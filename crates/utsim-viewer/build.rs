//! Embute o ícone e as informações de versão no `utsim-view.exe` (somente no Windows).

fn main() {
    println!("cargo:rerun-if-changed=../../assets/icone/utsim.ico");
    #[cfg(windows)]
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut recurso = winresource::WindowsResource::new();
        recurso.set_icon("../../assets/icone/utsim.ico");
        recurso.set("ProductName", "UTSim");
        recurso.set(
            "FileDescription",
            "UTSim: editor e visualizador 3D de processos",
        );
        recurso.set("LegalCopyright", "GPL-3.0-or-later");
        if let Err(e) = recurso.compile() {
            println!("cargo:warning=não foi possível embutir o ícone: {e}");
        }
    }
}
