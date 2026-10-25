// Smoke test: loads the built wasm in Node and runs the M/M/1 example.
// Usage: bash scripts/web.sh && node web/smoke.mjs
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const glue = readFileSync(join(here, "dist", "utsim.js"), "utf8");
const wasm = readFileSync(join(here, "dist", "utsim_bg.wasm"));
const wb = new Function(`${glue}\nreturn wasm_bindgen;`)();
await wb({ module_or_path: wasm });

const assert = (cond, msg) => {
  if (!cond) {
    console.error("FALHOU:", msg);
    process.exit(1);
  }
};

const examples = JSON.parse(wb.examples());
const mm1 = examples.find((e) => e.name === "mm1").text;
assert(JSON.parse(wb.validate_model(mm1)).length === 0, "mm1 deve ser válido");
assert(JSON.parse(wb.validate_model("Model(")).length > 0, "sintaxe inválida deve dar erro");

const r = JSON.parse(wb.run_model(mm1, 3));
assert(r.ok && r.replications === 3, "run_model");
// M/M/1 with rho = 0.8: mean time in queue Wq = 3.2 (theory).
const wq = r.summary["fila.staytime.mean"].mean;
assert(Math.abs(wq - 3.2) < 0.5, `permanência na fila ${wq} longe da teoria 3,2`);

const rp = JSON.parse(wb.replay(mm1, 20));
assert(rp.ok && rp.replay.records.length > 0, "replay");

const html = wb.fit_data("t\n1.2\n0.8\n2.5\n0.4\n1.9\n3.1\n0.7\n1.1\n2.2\n0.9\n1.6\n0.5\n", '{"bootstrap":20}');
assert(html.includes("<"), "relatório HTML");
let threw = false;
try { wb.fit_data("", ""); } catch { threw = true; }
assert(threw, "fit_data com dados vazios deve lançar");

console.log(`OK: wasm rodou mm1 (Wq = ${wq.toFixed(3)}), validou, reproduziu e ajustou.`);

// The same engine behind web/worker.js (the page's Web Worker), driven through its
// message protocol with a fake `self`: init, examples, validate, run, fit, replay, errors.
{
  const workerSrc = readFileSync(join(here, "worker.js"), "utf8");
  const waiting = new Map();
  const fakeSelf = { postMessage: (m) => waiting.get(m.id)(m) };
  new Function("self", `${glue}\n${workerSrc}`)(fakeSelf);
  let seq = 0;
  const ask = (op, ...args) =>
    new Promise((resolve) => {
      const id = ++seq;
      waiting.set(id, resolve);
      fakeSelf.onmessage({ data: { id, op, args } });
    });

  const early = await ask("validate", mm1);
  assert(!early.ok && /iniciado/.test(early.error), "operação antes do init deve falhar");
  assert((await ask("init", wasm)).ok, "worker init");
  const ex = await ask("examples");
  assert(ex.ok && JSON.parse(ex.result).length === examples.length, "worker examples");
  const val = await ask("validate", "Model(");
  assert(val.ok && JSON.parse(val.result).length > 0, "worker validate");
  const run = await ask("run", mm1, 2);
  assert(run.ok && JSON.parse(run.result).replications === 2, "worker run");
  const rep = await ask("replay", mm1, 20);
  assert(rep.ok && JSON.parse(rep.result).ok, "worker replay");
  const fit = await ask("fit", "t\n1\n2\n3\n4\n5\n6\n7\n8\n", '{"bootstrap":5}');
  assert(fit.ok && fit.result.includes("<"), "worker fit");
  const bad = await ask("fit", "", "");
  assert(!bad.ok && bad.error.length > 0, "worker fit com erro devolve a mensagem");
  assert(!(await ask("nada")).ok, "operação desconhecida");
  console.log("OK: o worker respondeu a init, examples, validate, run, replay, fit e erros.");
}
