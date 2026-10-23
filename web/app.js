// UTSim in the browser: UI glue around the WebAssembly module (no framework).
// Works both from web/dist (wasm fetched next to the page) and from the
// self-contained utsim.html (wasm inlined as base64 in window.UTSIM_WASM_B64).
(function () {
  "use strict";

  const $ = (id) => document.getElementById(id);
  const fmt = new Intl.NumberFormat("pt-BR", { maximumFractionDigits: 4 });
  const LABELS = {
    input: "entradas",
    output: "saídas",
    "content.mean": "conteúdo médio",
    "content.max": "conteúdo máx.",
    "staytime.mean": "permanência média",
  };
  const FIXED = Object.keys(LABELS);

  let api = null;
  let examples = [];

  function setStatus(text) {
    $("status").textContent = text;
  }

  function showErrors(list, ok) {
    const ul = $("erros");
    ul.innerHTML = "";
    ul.classList.toggle("ok", !!ok);
    for (const e of list) {
      const li = document.createElement("li");
      li.textContent = e;
      ul.appendChild(li);
    }
    ul.hidden = list.length === 0;
  }

  function decodeBase64(b64) {
    const bin = atob(b64);
    const bytes = new Uint8Array(bin.length);
    for (let i = 0; i < bin.length; i++) bytes[i] = bin.charCodeAt(i);
    return bytes;
  }

  async function loadWasm() {
    // `wasm_bindgen` is the global created by dist/utsim.js (wasm-bindgen --target no-modules).
    const source = window.UTSIM_WASM_B64
      ? decodeBase64(window.UTSIM_WASM_B64)
      : "dist/utsim_bg.wasm";
    await wasm_bindgen({ module_or_path: source });
    return wasm_bindgen;
  }

  // ---- Simular ------------------------------------------------------------

  function cell(summary, key) {
    const s = summary[key];
    if (!s) return "<td>-</td>";
    const hw = s.half_width;
    const pm = hw === null || hw === undefined ? "" : ` <span class="pm">± ${fmt.format(hw)}</span>`;
    return `<td>${fmt.format(s.mean)}${pm}</td>`;
  }

  function esc(text) {
    const d = document.createElement("div");
    d.textContent = text;
    return d.innerHTML;
  }

  function renderResult(r) {
    const keys = new Set();
    r.blocks.forEach((b) => Object.keys(b.metrics).forEach((k) => keys.add(k)));
    const columns = FIXED.filter((k) => keys.has(k)).concat(
      [...keys].filter((k) => !FIXED.includes(k)).sort()
    );
    let html = "<table><thead><tr><th>objeto</th><th>tipo</th>";
    for (const k of columns) html += `<th>${esc(LABELS[k] || k)}</th>`;
    html += "</tr></thead><tbody>";
    for (const b of r.blocks) {
      html += `<tr><td>${esc(b.name)}</td><td class="tipo">${esc(b.kind)}</td>`;
      for (const k of columns) {
        html += cell(b.metrics, k);
      }
      html += "</tr>";
    }
    html += "</tbody></table>";
    $("titulo-resultado").textContent = `Resultados: ${r.name}`;
    $("resumo").textContent =
      `${r.replications} replicação(ões), t até ${fmt.format(r.stop_time)}` +
      (r.warmup ? ` (aquecimento ${fmt.format(r.warmup)})` : "") +
      `, ${fmt.format(r.events)} eventos. Valores: média ± meia-largura do IC de 95%.`;
    $("resultado").innerHTML = html;
  }

  function validate() {
    const errors = JSON.parse(api.validate_model($("modelo").value));
    if (errors.length === 0) showErrors(["Modelo válido."], true);
    else showErrors(errors, false);
    return errors.length === 0;
  }

  function simulate() {
    const reps = parseInt($("replicacoes").value, 10) || 0;
    $("simular").disabled = true;
    setStatus("Simulando…");
    // Let the browser paint the status before the (synchronous) run starts.
    setTimeout(() => {
      try {
        const t0 = performance.now();
        const r = JSON.parse(api.run_model($("modelo").value, reps));
        if (!r.ok) {
          showErrors(r.errors, false);
          setStatus("O modelo tem problemas; corrija e tente de novo.");
        } else {
          showErrors([], true);
          renderResult(r);
          setStatus(`Pronto em ${fmt.format((performance.now() - t0) / 1000)} s.`);
        }
      } catch (e) {
        showErrors([String(e)], false);
        setStatus("Erro inesperado na simulação.");
      } finally {
        $("simular").disabled = false;
      }
    }, 30);
  }

  function loadExample() {
    const ex = examples.find((e) => e.name === $("exemplos").value);
    if (!ex) return;
    $("modelo").value = ex.text;
    const m = /replications:\s*(\d+)/.exec(ex.text);
    if (m) $("replicacoes").value = m[1];
    $("resultado").innerHTML = "";
    $("titulo-resultado").textContent = "Resultados";
    showErrors([], true);
  }

  // ---- Ajustar distribuição -----------------------------------------------

  function fitData() {
    const text = $("dados").value;
    const options = {
      column: $("coluna").value.trim() || null,
      timestamps: $("instantes").checked,
      bootstrap: Math.max(0, parseInt($("bootstrap").value, 10) || 0),
    };
    $("ajustar").disabled = true;
    $("fit-erro").hidden = true;
    setStatus("Ajustando distribuições…");
    setTimeout(() => {
      try {
        const html = api.fit_data(text, JSON.stringify(options));
        const frame = $("relatorio");
        frame.srcdoc = html;
        frame.hidden = false;
        setStatus("Relatório pronto.");
      } catch (e) {
        $("relatorio").hidden = true;
        $("fit-erro").textContent = e && e.message ? e.message : String(e);
        $("fit-erro").hidden = false;
        setStatus("Não foi possível ajustar.");
      } finally {
        $("ajustar").disabled = false;
      }
    }, 30);
  }

  // ---- Abas ---------------------------------------------------------------

  const TABS = [["tab-sim", "painel-sim"], ["tab-fit", "painel-fit"], ["tab-3d", "painel-3d"]];

  function selectTab(which) {
    for (const [tab, panel] of TABS) {
      const on = tab === which;
      $(tab).setAttribute("aria-selected", String(on));
      $(panel).hidden = !on;
    }
    // A Bevy app cannot be restarted and keeps rendering while hidden: drop the 3D frame
    // when leaving the tab (it is created again by "Gerar visualização 3D").
    if (which !== "tab-3d") close3d();
  }

  // ---- 3D ---------------------------------------------------------------------
  // The viewer (Bevy, WebGL2) is a separate, large wasm built into dist/viewer/ and shown
  // in an iframe (viewer.html), one fresh instance per run. The self-contained utsim.html
  // does not carry it: there the tab only explains how to get it.

  let viewerAvailable = null; // null = not checked yet

  async function checkViewer() {
    if (viewerAvailable !== null) return viewerAvailable;
    try {
      const r = await fetch("dist/viewer/utsim_viewer_bg.wasm", { method: "HEAD" });
      viewerAvailable = r.ok;
    } catch (e) {
      viewerAvailable = false;
    }
    return viewerAvailable;
  }

  function close3d() {
    const frame = $("quadro3d");
    if (!frame.hidden) {
      frame.src = "about:blank";
      frame.hidden = true;
    }
  }

  function note3d(text) {
    $("nota3d").textContent = text;
    $("nota3d").hidden = !text;
  }

  function stop3d(ron) {
    const typed = parseFloat($("parada3d").value);
    if (typed > 0) return typed;
    const m = /stop_time\s*:\s*([0-9.eE+-]+)/.exec(ron);
    const model = m ? parseFloat(m[1]) : NaN;
    return Math.min(model > 0 ? model : 2000, 2000);
  }

  function generate3d() {
    const ron = $("modelo").value;
    const stop = stop3d(ron);
    $("gerar3d").disabled = true;
    note3d("");
    setStatus("Gravando a replicação para o 3D…");
    setTimeout(() => {
      let text;
      try {
        text = api.replay(ron, stop);
        const r = JSON.parse(text);
        if (!r.ok) {
          note3d("O modelo tem problemas: " + r.errors.join("; "));
          setStatus("Corrija o modelo na aba Simular.");
          $("gerar3d").disabled = false;
          return;
        }
      } catch (e) {
        note3d("Erro ao gravar a replicação: " + e);
        $("gerar3d").disabled = false;
        return;
      }
      const frame = $("quadro3d");
      const onMessage = (ev) => {
        if (ev.source !== frame.contentWindow || !ev.data || ev.data.type !== "utsim-viewer-ready") return;
        window.removeEventListener("message", onMessage);
        frame.contentWindow.postMessage({ type: "utsim-replay", text }, "*");
        setStatus(`Visualizador 3D iniciado (t até ${fmt.format(stop)}).`);
        $("gerar3d").disabled = false;
      };
      window.addEventListener("message", onMessage);
      frame.hidden = false;
      frame.src = "viewer.html";
      setStatus("Carregando o visualizador 3D…");
    }, 30);
  }

  async function prepare3d() {
    if (await checkViewer()) {
      $("gerar3d").disabled = false;
    } else {
      note3d(
        "O visualizador 3D (WebAssembly com Bevy, vários MB) não está nesta página. " +
          "Ele só funciona servido por HTTP: rode scripts/web.sh (ou web.ps1) e sirva a pasta web/ " +
          "(python3 -m http.server -d web), depois abra http://localhost:8000/. " +
          "A versão autocontida utsim.html não inclui o 3D."
      );
    }
  }

  async function main() {
    $("tab-sim").addEventListener("click", () => selectTab("tab-sim"));
    $("tab-fit").addEventListener("click", () => selectTab("tab-fit"));
    $("tab-3d").addEventListener("click", () => selectTab("tab-3d"));
    $("gerar3d").addEventListener("click", generate3d);
    $("arquivo").addEventListener("change", async (ev) => {
      const file = ev.target.files[0];
      if (file) $("dados").value = await file.text();
    });
    try {
      api = await loadWasm();
    } catch (e) {
      setStatus(
        "Não foi possível carregar o WebAssembly. Se abriu dist/ direto do disco, use o utsim.html " +
          "(autocontido) ou sirva a pasta web/ com um servidor local. Detalhe: " + e
      );
      return;
    }
    examples = JSON.parse(api.examples());
    const select = $("exemplos");
    for (const ex of examples) {
      const opt = document.createElement("option");
      opt.value = ex.name;
      opt.textContent = ex.name;
      select.appendChild(opt);
    }
    select.addEventListener("change", loadExample);
    $("validar").addEventListener("click", validate);
    $("simular").addEventListener("click", simulate);
    $("ajustar").addEventListener("click", fitData);
    loadExample();
    prepare3d();
    for (const id of ["validar", "simular", "ajustar"]) $(id).disabled = false;
    setStatus("Pronto. Tudo roda localmente, sem enviar nada para servidores.");
  }

  main();
})();
