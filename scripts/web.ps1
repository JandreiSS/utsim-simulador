# Builds UTSim for the browser (PowerShell version of scripts/web.sh).
# Output: web/dist/utsim.js + utsim_bg.wasm and the self-contained web/dist/utsim.html.
param([switch]$Sem3d)
$ErrorActionPreference = "Stop"
Set-Location (Join-Path $PSScriptRoot "..")

function Run($cmd, $arguments) {
    & $cmd @arguments
    if ($LASTEXITCODE -ne 0) { throw "FALHOU: $cmd $arguments" }
}

rustup target add wasm32-unknown-unknown | Out-Null

$lock = Get-Content Cargo.lock
$idx = [array]::IndexOf($lock, 'name = "wasm-bindgen"')
$want = ($lock[$idx + 1] -replace '[^0-9.]', '')
$have = ""
try { $have = ((wasm-bindgen --version) -split ' ')[1] } catch { }
if ($have -ne $want) {
    Write-Host "==> instalando wasm-bindgen-cli $want (encontrado: $have)"
    Run cargo @("install", "wasm-bindgen-cli", "--version", $want, "--locked", "--force")
}

Write-Host "==> compilando utsim-web (perfil web, wasm32)"
Run cargo @("build", "--profile", "web", "--target", "wasm32-unknown-unknown", "-p", "utsim-web")

Write-Host "==> wasm-bindgen"
if (Test-Path web/dist) { Remove-Item -Recurse -Force web/dist }
Run wasm-bindgen @("target/wasm32-unknown-unknown/web/utsim_web.wasm", "--target", "no-modules",
    "--no-typescript", "--out-dir", "web/dist", "--out-name", "utsim")

# The 3D viewer (Bevy + WebGL2) is large: it goes to web/dist/viewer/ and is NOT inlined
# in utsim.html. Pass -Sem3d to skip it. web/viewer.html loads it.
if (-not $Sem3d) {
    Write-Host "==> compilando utsim-viewer (perfil web-3d, wasm32; demora)"
    Run cargo @("build", "--profile", "web-3d", "--target", "wasm32-unknown-unknown", "-p", "utsim-viewer")
    Write-Host "==> wasm-bindgen (visualizador)"
    Run wasm-bindgen @("target/wasm32-unknown-unknown/web-3d/utsim-view.wasm", "--target", "web",
        "--no-typescript", "--out-dir", "web/dist/viewer", "--out-name", "utsim_viewer")
}

Write-Host "==> gerando web/dist/utsim.html (autocontido)"
$utf8 = New-Object System.Text.UTF8Encoding($false)
$b64 = [Convert]::ToBase64String([IO.File]::ReadAllBytes("web/dist/utsim_bg.wasm"))
$css = [IO.File]::ReadAllText("web/style.css")
$glue = [IO.File]::ReadAllText("web/dist/utsim.js")
$app = [IO.File]::ReadAllText("web/app.js")
$worker = [IO.File]::ReadAllText("web/worker.js")
foreach ($part in @($glue, $worker, $app)) {
    if ($part.Contains("</script")) { throw "FALHOU: um dos scripts contém '</script' e não pode ser embutido" }
}
$out = New-Object System.Text.StringBuilder
foreach ($line in [IO.File]::ReadAllLines("web/index.html")) {
    if ($line.Contains('href="style.css"')) {
        [void]$out.Append("<style>`n$css`n</style>`n")
    } elseif ($line.Contains('src="app.js"')) {
        # The engine runs in a Web Worker created from a Blob: glue + worker.js in inert scripts.
        [void]$out.Append("<script type=`"text/plain`" id=`"utsim-glue`">`n$glue`n</script>`n")
        [void]$out.Append("<script type=`"text/plain`" id=`"utsim-worker-src`">`n$worker`n</script>`n")
        [void]$out.Append("<script>window.UTSIM_WASM_B64=`"$b64`";</script>`n<script>`n$app`n</script>`n")
    } else {
        [void]$out.Append("$line`n")
    }
}
[IO.File]::WriteAllText((Join-Path (Get-Location) "web/dist/utsim.html"), $out.ToString(), $utf8)

Get-ChildItem web/dist/utsim_bg.wasm, web/dist/utsim.html, web/dist/viewer/utsim_viewer_bg.wasm -ErrorAction SilentlyContinue | ForEach-Object { "{0,10} bytes  {1}" -f $_.Length, $_.Name }
Write-Host "Pronto. Abra web/dist/utsim.html no navegador."
