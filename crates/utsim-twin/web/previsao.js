// Página de previsão do gêmeo: só desenha o JSON de /api/gemeo/previsao (todo cálculo é no Rust).
"use strict";

function esc(s) {
  return String(s).replace(/[&<>"]/g, c => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" }[c]));
}
function num(x, d) {
  if (x === null || x === undefined || !isFinite(x)) return "—";
  return Number(x).toLocaleString("pt-BR", { maximumFractionDigits: d === undefined ? 2 : d });
}
function pct(p) { return (p === null || p === undefined) ? "—" : num(100 * p, 0) + " %"; }
function table(head, rows) {
  if (!rows.length) return '<p class="vazio">Nada.</p>';
  return "<table><tr>" + head.map(h => "<th>" + esc(h) + "</th>").join("") + "</tr>" +
    rows.map(r => "<tr>" + r.map(c => "<td>" + c + "</td>").join("") + "</tr>").join("") + "</table>";
}
function luz(s) {
  const t = { verde: "verde", amarelo: "amarelo", vermelho: "vermelho", sem_meta: "sem meta", indefinido: "indefinido" }[s] || s;
  return '<span class="luz ' + esc(s) + '">' + esc(t) + "</span>";
}

// Leque: P5–P95 e P25–P75 em faixas, mediana, média com a precisão e a meta.
function band(serie) {
  const p = serie.pontos;
  if (!p.length) return "";
  const w = 640, h = 200, l = 48, r = 8, t = 8, b = 24;
  const xs = p.map(q => q.t);
  let lo = Math.min(...p.map(q => q.p5)), hi = Math.max(...p.map(q => q.p95));
  if (serie.meta !== undefined && serie.meta !== null) { lo = Math.min(lo, serie.meta); hi = Math.max(hi, serie.meta); }
  if (!(hi > lo)) { hi = lo + 1; }
  const x0 = xs[0], x1 = xs[xs.length - 1] > x0 ? xs[xs.length - 1] : x0 + 1;
  const X = v => (l + (v - x0) / (x1 - x0) * (w - l - r)).toFixed(1);
  const Y = v => (t + (hi - v) / (hi - lo) * (h - t - b)).toFixed(1);
  const area = (a, z) => p.map(q => X(q.t) + "," + Y(q[a])).join(" ") + " " +
    p.slice().reverse().map(q => X(q.t) + "," + Y(q[z])).join(" ");
  const line = f => p.filter(q => isFinite(f(q))).map(q => X(q.t) + "," + Y(f(q))).join(" ");
  let s = '<svg class="leque" viewBox="0 0 ' + w + " " + h + '" role="img" aria-label="' + esc(serie.nome) + '">';
  for (const v of [lo, (lo + hi) / 2, hi]) {
    s += '<line x1="' + l + '" x2="' + (w - r) + '" y1="' + Y(v) + '" y2="' + Y(v) + '" class="grade"/>' +
      '<text x="' + (l - 4) + '" y="' + Y(v) + '" class="eixo" text-anchor="end" dominant-baseline="middle">' + num(v, 1) + "</text>";
  }
  s += '<text x="' + l + '" y="' + (h - 6) + '" class="eixo">t = ' + num(x0, 1) + "</text>" +
    '<text x="' + (w - r) + '" y="' + (h - 6) + '" class="eixo" text-anchor="end">t = ' + num(x1, 1) + "</text>";
  s += '<polygon class="p90" points="' + area("p5", "p95") + '"/>';
  s += '<polygon class="p50" points="' + area("p25", "p75") + '"/>';
  s += '<polyline class="mediana" points="' + line(q => q.p50) + '"/>';
  s += '<polyline class="media" points="' + line(q => q.media) + '"/>';
  s += '<polyline class="ic" points="' + line(q => q.ic ? q.ic[0] : NaN) + '"/>';
  s += '<polyline class="ic" points="' + line(q => q.ic ? q.ic[1] : NaN) + '"/>';
  if (serie.meta !== undefined && serie.meta !== null) {
    s += '<line class="meta" x1="' + l + '" x2="' + (w - r) + '" y1="' + Y(serie.meta) + '" y2="' + Y(serie.meta) + '"/>';
  }
  return s + "</svg>";
}

function draw(d) {
  const f = d.ultima;
  document.getElementById("situacao").textContent =
    (d.ativa ? "Previsão ligada" : "Previsão desligada (use --previsao)") +
    " · tempo do gêmeo " + num(d.tempo, 1) +
    (d.proxima !== null && d.proxima !== undefined ? " · próxima em t = " + num(d.proxima, 1) : " · só sob pedido") +
    (d.na_fila ? " · " + d.na_fila + " calculando" : "");
  document.getElementById("agora").disabled = !d.ativa;
  if (!f) return;
  document.getElementById("resumo").textContent = f.resumo;
  document.getElementById("resumo").className = "";
  const metas = f.kpis.filter(k => k.meta);
  document.getElementById("cartoes").innerHTML = metas.map(k =>
    '<div class="cartao ' + esc(k.situacao) + '"><div class="nome">' + esc(k.nome) + "</div>" +
    '<div class="valor">' + esc(k.texto) + " " + esc(k.unidade) + "</div>" +
    '<div class="faixa">faixa: ' + esc(k.faixa) + "</div>" +
    "<div>meta " + (k.meta.sentido === "min" ? "≥ " : "≤ ") + num(k.meta.valor) + ": " +
    (k.prob ? pct(k.prob.p) + " (" + pct(k.prob.lo) + " a " + pct(k.prob.hi) + ")" : "—") + "</div>" +
    "<div>" + luz(k.situacao) + (k.indefinido ? " · indefinido: rode mais replicações" : "") + "</div></div>").join("");
  document.getElementById("series").innerHTML = f.series.map(s =>
    "<h3>" + esc(s.nome) + " (" + esc(s.unidade) + ")</h3>" + band(s)).join("");
  document.getElementById("kpis").innerHTML = table(
    ["indicador", "média ± precisão", "faixa P5 a P95", "meta", "probabilidade (Wilson)", "semáforo"],
    f.kpis.map(k => [esc(k.nome), esc(k.texto) + " " + esc(k.unidade), esc(k.faixa),
      k.meta ? (k.meta.sentido === "min" ? "≥ " : "≤ ") + num(k.meta.valor) : "—",
      k.prob ? pct(k.prob.p) + " (" + pct(k.prob.lo) + " a " + pct(k.prob.hi) + ")" : "—", luz(k.situacao)]));
  document.getElementById("eventos").innerHTML = table(
    ["objeto", "limite", "probabilidade (Wilson)", "instante P5", "mediana", "P95"],
    f.eventos.map(e => [esc(e.objeto), esc(e.sentido) + " de " + num(e.limite),
      pct(e.prob.p) + " (" + pct(e.prob.lo) + " a " + pct(e.prob.hi) + ")",
      num(e.instante_p5, 1), num(e.instante_p50, 1), num(e.instante_p95, 1)]));
  const rows = [];
  for (const c of f.cenarios) {
    if (c.erro) { rows.push([esc(c.nome), "—", "—", "—", "—", esc(c.erro)]); continue; }
    for (const k of c.kpis) {
      rows.push([esc(c.nome), esc(k.nome), esc(k.texto) + " " + esc(k.unidade), esc(k.texto_diferenca),
        k.prob ? pct(k.prob.p) + " " + luz(k.situacao) : "—",
        k.melhora === true ? '<span class="luz verde">melhora</span>' :
          k.melhora === false ? '<span class="luz vermelho">piora</span>' : (k.significativa ? "difere" : "≈")]);
    }
  }
  document.getElementById("cenarios").innerHTML = table(
    ["cenário", "indicador", "valor", "diferença para o como está", "meta", "efeito"], rows);
  const av = (f.avisos || []).concat(f.notas_estado || []).concat(d.erros || []);
  document.getElementById("avisos").innerHTML = av.length ?
    "<ul>" + av.map(a => "<li>" + esc(a) + "</li>").join("") + "</ul>" : '<p class="vazio">Nenhum.</p>';
}

async function tick() {
  try {
    const r = await fetch("/api/gemeo/previsao", { cache: "no-store" });
    if (r.ok) draw(await r.json());
  } catch (e) {
    document.getElementById("situacao").textContent = "Sem resposta do gêmeo: " + e;
  }
  setTimeout(tick, 3000);
}
document.getElementById("agora").addEventListener("click", async () => {
  try {
    await fetch("/api/gemeo/previsao", { method: "POST", headers: { "X-UTSim": "1" } });
  } catch (e) { /* a próxima leitura mostra o erro */ }
});
tick();
