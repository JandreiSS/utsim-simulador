// Página do gêmeo em modo sombra: só desenha o JSON de /api/gemeo (todo cálculo é no Rust).
"use strict";

function esc(s) {
  return String(s).replace(/[&<>"]/g, c => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" }[c]));
}
function num(x, d) {
  if (x === null || x === undefined || !isFinite(x)) return "—";
  return Number(x).toLocaleString("pt-BR", { maximumFractionDigits: d === undefined ? 3 : d });
}
function table(head, rows) {
  if (!rows.length) return '<p class="vazio">Nada ainda.</p>';
  return "<table><tr>" + head.map(h => "<th>" + esc(h) + "</th>").join("") + "</tr>" +
    rows.map(r => "<tr>" + r.map(c => "<td>" + c + "</td>").join("") + "</tr>").join("") + "</table>";
}
// Resíduos padronizados (real − modelo) da série, de −4 a +4.
function spark(z) {
  if (!z || !z.length) return "";
  const w = 120, h = 24, n = z.length, clamp = v => Math.max(-4, Math.min(4, v));
  const pts = z.map((v, i) => ((n === 1 ? w / 2 : i * w / (n - 1)).toFixed(1)) + "," +
    (h / 2 - clamp(v) * h / 8).toFixed(1)).join(" ");
  return '<svg class="spark" width="' + w + '" height="' + h + '" role="img" aria-label="resíduos">' +
    '<line x1="0" x2="' + w + '" y1="' + h / 2 + '" y2="' + h / 2 + '" stroke="currentColor" stroke-opacity=".25"/>' +
    '<polyline fill="none" stroke="currentColor" stroke-width="1.5" points="' + pts + '"/></svg>';
}
function nivel(n) { return '<span class="' + esc(n) + '">' + esc(n) + "</span>"; }

function draw(d) {
  const nj = d.janelas.length;
  document.getElementById("resumo").textContent =
    "Modelo " + d.modelo + " · tempo " + num(d.tempo, 1) + " · " + nj + " janela(s) de " + num(d.janela) +
    " · " + d.recebidas + " leituras (" + d.nao_mapeadas + " sem regra) · " + d.testes + " testes; " +
    d.limites.m + " KPIs (BH a " + num(100 * d.limites.fdr, 1) + " %, CUSUM h = " + num(d.limites.cusum_h, 2) +
    ", EWMA L = " + num(d.limites.ewma_l, 2) + ") · modelo: " + d.replicacoes_simuladas + " replicações × " +
    d.janelas_por_replicacao + " janelas";
  document.getElementById("alertas").innerHTML = table(["janela", "nível", "teste", "mensagem"],
    d.alertas.slice().reverse().map(a => [a.janela + 1, nivel(a.nivel), esc(a.teste), esc(a.mensagem)]));
  document.getElementById("kpis").innerHTML = table(
    ["KPI", "nível", "último real", "modelo (média ± dp)", "p", "z", "CUSUM ↑/↓", "resíduos"],
    Object.entries(d.kpis).map(([k, s]) => [esc(k), nivel(s.nivel), num(s.ultimo_real),
      num(s.simulado_media) + " ± " + num(s.simulado_dp), num(s.ultimo_p), num(s.ultimo_z, 2),
      num(s.cusum_alta, 2) + " / " + num(s.cusum_baixa, 2), spark(s.z)]));
  document.getElementById("objetos").innerHTML = table(
    ["objeto", "estado", "desde", "conteúdo", "entradas", "saídas", "posição", "atualizado"],
    Object.entries(d.objetos).map(([k, o]) => [esc(k), esc(o.estado || "—"), num(o.estado_desde, 1),
      num(o.conteudo), num(o.entradas, 0), num(o.saidas, 0),
      o.posicao ? o.posicao.map(v => num(v, 2)).join(", ") : "—", num(o.atualizado, 1)]));
  document.getElementById("conexoes").innerHTML = table(
    ["conexão", "conectada", "recebidas", "descartadas", "inválidas", "lacunas seq", "último erro"],
    Object.entries(d.conexoes || {}).map(([k, h]) => [esc(k), h.conectado ? "sim" : "não",
      h.recebidas, h.descartadas, h.invalidas, h.lacunas_seq, esc(h.ultimo_erro || "")]));
  document.getElementById("regras").innerHTML = table(
    ["conexão", "tag", "objeto", "medida", "último valor", "qualidade", "recebidas", "eventos", "descartadas"],
    d.regras.map(r => [esc(r.fonte), esc(r.tag), esc(r.objeto), esc(r.medida),
      esc(r.ultimo_valor === undefined ? "—" : JSON.stringify(r.ultimo_valor)), esc(r.ultima_qualidade),
      r.recebidas, r.eventos, r.qualidade_ruim + r.nao_numericas + r.fora_da_faixa + r.estado_desconhecido]));
}

async function tick() {
  try {
    const r = await fetch("/api/gemeo", { cache: "no-store" });
    if (r.ok) draw(await r.json());
  } catch (e) {
    document.getElementById("resumo").textContent = "Sem resposta do gêmeo: " + e;
  }
  setTimeout(tick, 2000);
}
tick();
