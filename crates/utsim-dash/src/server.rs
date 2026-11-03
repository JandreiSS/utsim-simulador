//! The minimal HTTP server of the dashboard, on `std::net` only (no web framework).
//!
//! Read-only: `GET` of the page, its static files and the JSON API, plus one `POST`
//! (`/api/cancelar`) to stop a running study. It listens on `127.0.0.1` only and refuses
//! any request whose `Host` is not the loopback address (a page on the internet cannot
//! read the dashboard through DNS rebinding). One thread per connection, at most
//! [`MAX_CONNECTIONS`] at once, every connection closed after its response.

use std::io::{self, Read, Write};
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::export;
use crate::page;
use crate::study::Study;

/// Simultaneous connections served; further ones are closed at once.
pub const MAX_CONNECTIONS: usize = 64;
/// Longest request head accepted, in bytes.
const MAX_HEAD: usize = 16 * 1024;
const READ_TIMEOUT: Duration = Duration::from_secs(5);
const WRITE_TIMEOUT: Duration = Duration::from_secs(10);

/// Extra read-only routes served next to the dashboard's own (the digital twin's page
/// and API, `utsim gemeo --painel`). Asked first for every `GET`.
pub trait Extra: Send + Sync {
    /// Content type and body for `path`, or `None` to fall through.
    fn get(&self, path: &str) -> Option<(&'static str, Vec<u8>)>;

    /// Answer to a `POST` of `path` (an action such as "forecast now"), or `None` to
    /// fall through. Only asked when the request carries the `X-UTSim` header, which a
    /// cross-site form cannot send.
    fn post(&self, _path: &str) -> Option<(&'static str, Vec<u8>)> {
        None
    }
}

/// A running dashboard server.
pub struct Server {
    addr: SocketAddr,
    stop: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl Server {
    /// Starts listening on `127.0.0.1:port` (`0` picks a free port) and serves `study`.
    pub fn start(study: Study, port: u16) -> io::Result<Server> {
        Server::start_with(study, port, None)
    }

    /// Like [`Server::start`], also serving the routes of `extra`.
    pub fn start_with(
        study: Study,
        port: u16,
        extra: Option<Arc<dyn Extra>>,
    ) -> io::Result<Server> {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, port))?;
        let addr = listener.local_addr()?;
        let stop = Arc::new(AtomicBool::new(false));
        let flag = Arc::clone(&stop);
        let active = Arc::new(AtomicUsize::new(0));
        let handle = thread::spawn(move || {
            for conn in listener.incoming() {
                if flag.load(Ordering::Relaxed) {
                    break;
                }
                let Ok(stream) = conn else { continue };
                if active.fetch_add(1, Ordering::AcqRel) >= MAX_CONNECTIONS {
                    active.fetch_sub(1, Ordering::AcqRel);
                    continue;
                }
                let (study, active) = (study.clone(), Arc::clone(&active));
                let extra = extra.clone();
                thread::spawn(move || {
                    let _ = serve(stream, &study, addr.port(), extra.as_deref());
                    active.fetch_sub(1, Ordering::AcqRel);
                });
            }
        });
        Ok(Server {
            addr,
            stop,
            handle: Some(handle),
        })
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    /// Address to open in the browser.
    pub fn url(&self) -> String {
        format!("http://127.0.0.1:{}/", self.addr.port())
    }

    /// Stops accepting connections and waits for the acceptor thread.
    pub fn stop(mut self) {
        self.shutdown();
    }

    fn shutdown(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        // Wakes the blocked `accept`.
        let _ = TcpStream::connect(self.addr);
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.shutdown();
    }
}

struct Request {
    method: String,
    path: String,
    host: String,
    csrf_header: bool,
}

fn read_request(stream: &mut TcpStream) -> io::Result<Option<Request>> {
    let mut buf = Vec::with_capacity(1024);
    let mut chunk = [0u8; 1024];
    loop {
        let n = stream.read(&mut chunk)?;
        if n == 0 {
            return Ok(None);
        }
        buf.extend_from_slice(&chunk[..n]);
        if buf.windows(4).any(|w| w == b"\r\n\r\n") {
            break;
        }
        if buf.len() > MAX_HEAD {
            return Ok(None);
        }
    }
    let head = String::from_utf8_lossy(&buf).into_owned();
    let mut lines = head.split("\r\n");
    let mut first = lines.next().unwrap_or("").split_whitespace();
    let (Some(method), Some(target)) = (first.next(), first.next()) else {
        return Ok(None);
    };
    let (mut host, mut csrf_header) = (String::new(), false);
    for line in lines {
        if let Some((k, v)) = line.split_once(':') {
            match k.trim().to_ascii_lowercase().as_str() {
                "host" => host = v.trim().to_ascii_lowercase(),
                "x-utsim" => csrf_header = true,
                _ => {}
            }
        }
    }
    let path = target.split(['?', '#']).next().unwrap_or("/").to_string();
    Ok(Some(Request {
        method: method.to_string(),
        path,
        host,
        csrf_header,
    }))
}

/// Whether a `Host` header names the loopback address (with or without the port).
fn host_is_local(host: &str, port: u16) -> bool {
    let (name, port_text) = if let Some(rest) = host.strip_prefix('[') {
        match rest.split_once(']') {
            Some((n, after)) => (n, after.strip_prefix(':')),
            None => return false,
        }
    } else {
        match host.rsplit_once(':') {
            Some((n, p)) => (n, Some(p)),
            None => (host, None),
        }
    };
    let port_ok = port_text.is_none_or(|p| p.parse::<u16>().is_ok_and(|p| p == port));
    port_ok && matches!(name, "127.0.0.1" | "localhost" | "::1")
}

/// Decodes `%XX` escapes (object names may carry spaces and accents).
pub fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%'
            && let Some(hex) = s.get(i + 1..i + 3)
            && let Ok(v) = u8::from_str_radix(hex, 16)
        {
            out.push(v);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

struct Response {
    status: &'static str,
    content_type: &'static str,
    body: Vec<u8>,
    filename: Option<&'static str>,
}

impl Response {
    fn new(status: &'static str, content_type: &'static str, body: impl Into<Vec<u8>>) -> Self {
        Response {
            status,
            content_type,
            body: body.into(),
            filename: None,
        }
    }

    fn json(value: &serde_json::Value) -> Self {
        Response::new(
            "200 OK",
            "application/json; charset=utf-8",
            serde_json::to_vec(value).unwrap_or_default(),
        )
    }

    fn error(status: &'static str, text: &str) -> Self {
        Response::new(
            status,
            "application/json; charset=utf-8",
            serde_json::json!({ "erro": text }).to_string(),
        )
    }

    fn download(mut self, name: &'static str) -> Self {
        self.filename = Some(name);
        self
    }
}

fn serve(
    mut stream: TcpStream,
    study: &Study,
    port: u16,
    extra: Option<&dyn Extra>,
) -> io::Result<()> {
    stream.set_read_timeout(Some(READ_TIMEOUT))?;
    stream.set_write_timeout(Some(WRITE_TIMEOUT))?;
    let Some(req) = read_request(&mut stream)? else {
        return Ok(());
    };
    let response = if !host_is_local(&req.host, port) {
        Response::error(
            "403 Forbidden",
            "o painel só atende o endereço local (127.0.0.1)",
        )
    } else {
        let decoded = percent_decode(&req.path);
        match extra.and_then(|x| match req.method.as_str() {
            "GET" | "HEAD" => x.get(&decoded),
            "POST" if req.csrf_header => x.post(&decoded),
            _ => None,
        }) {
            Some((content_type, body)) => Response::new("200 OK", content_type, body),
            None => route(&req, study),
        }
    };
    let mut head = format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: no-store\r\n\
         X-Content-Type-Options: nosniff\r\nReferrer-Policy: no-referrer\r\n\
         Content-Security-Policy: default-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:\r\n\
         Connection: close\r\n",
        response.status,
        response.content_type,
        response.body.len()
    );
    if let Some(name) = response.filename {
        head += &format!("Content-Disposition: attachment; filename=\"{name}\"\r\n");
    }
    head += "\r\n";
    stream.write_all(head.as_bytes())?;
    if req.method != "HEAD" {
        stream.write_all(&response.body)?;
    }
    stream.flush()
}

fn route(req: &Request, study: &Study) -> Response {
    if req.method == "POST" && req.path == "/api/cancelar" {
        // A custom header cannot be sent by a plain cross-site form.
        if !req.csrf_header {
            return Response::error("403 Forbidden", "cabeçalho X-UTSim ausente");
        }
        study.cancel();
        return Response::json(&serde_json::json!({"ok": true}));
    }
    if req.method != "GET" && req.method != "HEAD" {
        return Response::error("405 Method Not Allowed", "só leitura");
    }
    let path = req.path.as_str();
    match path {
        "/" | "/index.html" => {
            return Response::new("200 OK", "text/html; charset=utf-8", page::live_html());
        }
        "/app.css" => return Response::new("200 OK", "text/css; charset=utf-8", page::CSS),
        "/charts.js" => {
            return Response::new("200 OK", "text/javascript; charset=utf-8", page::CHARTS_JS);
        }
        "/app.js" => {
            return Response::new("200 OK", "text/javascript; charset=utf-8", page::APP_JS);
        }
        "/api/progresso" => {
            return Response::json(&serde_json::to_value(study.progress()).unwrap_or_default());
        }
        _ => {}
    }
    let package = study.package();
    if let Some(name) = path.strip_prefix("/api/export/") {
        return export_response(name, &package);
    }
    if path.starts_with("/api/") {
        let key = percent_decode(path);
        return match package.routes.get(&key) {
            Some(v) => Response::json(v),
            None => Response::error("404 Not Found", "rota desconhecida"),
        };
    }
    Response::error("404 Not Found", "não encontrado")
}

fn export_response(name: &str, package: &crate::data::Package) -> Response {
    match name {
        "kpis.csv" => Response::new(
            "200 OK",
            "text/csv; charset=utf-8",
            export::kpis_csv(package),
        )
        .download("kpis.csv"),
        "objetos.csv" => Response::new(
            "200 OK",
            "text/csv; charset=utf-8",
            export::objects_csv(package),
        )
        .download("objetos.csv"),
        "painel.json" => Response::new(
            "200 OK",
            "application/json; charset=utf-8",
            export::json(package),
        )
        .download("painel.json"),
        "painel.xlsx" => match export::xlsx(package) {
            Ok(bytes) => Response::new(
                "200 OK",
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
                bytes,
            )
            .download("painel.xlsx"),
            Err(e) => Response::error("501 Not Implemented", &e),
        },
        _ => Response::error("404 Not Found", "exportação desconhecida"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_loopback_host_is_served() {
        assert!(host_is_local("127.0.0.1:8080", 8080));
        assert!(host_is_local("localhost:8080", 8080));
        assert!(host_is_local("localhost", 8080));
        assert!(host_is_local("[::1]:8080", 8080));
        assert!(!host_is_local("127.0.0.1:9999", 8080));
        assert!(!host_is_local("evil.example.com", 8080));
        assert!(!host_is_local("evil.example.com:8080", 8080));
        assert!(!host_is_local("", 8080));
    }

    #[test]
    fn percent_escapes_decode() {
        assert_eq!(
            percent_decode("/api/objeto/M%C3%A1quina%201"),
            "/api/objeto/Máquina 1"
        );
        assert_eq!(percent_decode("a%2"), "a%2");
        assert_eq!(percent_decode("100%"), "100%");
    }
}
