// UTSim engine in a Web Worker: the page never freezes during a long run.
//
// Protocol (all messages are plain objects):
//   page -> worker   {id, op, args}   op: "init" | "examples" | "validate" | "run" | "fit" | "replay"
//   worker -> page   {id, ok: true, result}  or  {id, ok: false, error: "text"}
// "init" must come first; its args[0] may carry the wasm bytes (self-contained utsim.html);
// otherwise the module is fetched from dist/utsim_bg.wasm, next to this script.
// Cancelling a run = terminating the worker (the page starts a new one).
//
// This file is also inlined as a Blob by utsim.html, with the wasm-bindgen glue placed in
// front of it, so `wasm_bindgen` may already exist when it starts.
(function () {
  "use strict";

  if (typeof wasm_bindgen === "undefined") {
    // `wasm_bindgen` is the global created by dist/utsim.js (--target no-modules).
    importScripts("dist/utsim.js");
  }

  let ready = null;

  const OPS = {
    examples: () => wasm_bindgen.examples(),
    validate: (ron) => wasm_bindgen.validate_model(ron),
    run: (ron, reps) => wasm_bindgen.run_model(ron, reps),
    fit: (csv, options) => wasm_bindgen.fit_data(csv, options),
    replay: (ron, stop) => wasm_bindgen.replay(ron, stop),
  };

  async function handle(op, args) {
    if (op === "init") {
      const wasm = args[0] || "dist/utsim_bg.wasm";
      ready = wasm_bindgen({ module_or_path: wasm });
      await ready;
      return true;
    }
    if (!ready) throw new Error("o motor ainda não foi iniciado");
    await ready;
    const fn = OPS[op];
    if (!fn) throw new Error("operação desconhecida: " + op);
    return fn(...args);
  }

  self.onmessage = async (ev) => {
    const { id, op, args } = ev.data;
    try {
      const result = await handle(op, args || []);
      self.postMessage({ id, ok: true, result });
    } catch (e) {
      self.postMessage({ id, ok: false, error: e && e.message ? e.message : String(e) });
    }
  };
})();
