"use strict";
/* UTSim dashboard: pages and navigation. Drawing only; every number comes from the API
   (or from the embedded copy in the exported page, window.UTSIM_DADOS). */
(() => {
  const ESTATICO = typeof window.UTSIM_DADOS === "object" && window.UTSIM_DADOS !== null;
  const { esc, fmt, fin } = UT;
  const $ = (s, el = document) => el.querySelector(s);

  async function api(path) {
    if (ESTATICO) {
      if (path in window.UTSIM_DADOS) return window.UTSIM_DADOS[path];
      throw new Error("sem dados para " + path);
    }
    const r = await fetch(encodeURI(path), { cache: "no-store" });
    if (!r.ok) throw new Error(path + ": " + r.status);
    return r.json();
  }
  const arquivo = (nome) => (ESTATICO ? nome : "/api/export/" + nome);

  const S = { resumo: null, kpis: [], objetos: [], series: null, dist: null, pareto: null, gantt: null, estados: {}, detalhe: {}, ordem: { col: "nome", asc: true }, filtro: "" };

  const ABAS = [
    ["gestor", "Gestor"],
    ["eng/objetos", "Engenheiro: objetos"],
    ["eng/distribuicoes", "Distribuições"],
    ["eng/series", "Séries"],
    ["eng/gantt", "Gantt"],
    ["eng/estatistica", "Estatística"],
  ];

  async function carregar() {
    const [resumo, kpis, objetos, series, dist, pareto, gantt, estados] = await Promise.all(
      ["/api/resumo", "/api/kpis", "/api/objetos", "/api/series", "/api/distribuicoes", "/api/pareto", "/api/gantt", "/api/estados"].map(api)
    );
    Object.assign(S, { resumo, kpis: kpis.kpis, objetos: objetos.objetos, series, dist, pareto, gantt, estados });
    S.detalhe = {};
    document.title = resumo.titulo + " · Painel UTSim";
  }

  const kpi = (id) => S.kpis.find((k) => k.id === id);
  const pct1 = (x) => fmt(100 * x, 0) + " %";

  /* ---- manager view ---- */
  const GLIFO = { verde: "✔ Verde", amarelo: "▲ Amarelo", vermelho: "✖ Vermelho", sem_meta: "Sem meta", indefinido: "Indefinido" };

  function cartao(k) {
    const sit = k.situacao;
    let sem = "";
    if (k.meta) {
      const sinal = k.meta.sentido === "min" ? "≥" : "≤";
      let det = "";
      if (k.prob) {
        det = `Meta ${sinal} ${fmt(k.meta.valor)}: cumprida em ${pct1(k.prob.p)} das rodadas (${k.prob.k} de ${k.prob.n}; IC ${pct1(k.prob.lo)} a ${pct1(k.prob.hi)})`;
      } else {
        det = `Meta ${sinal} ${fmt(k.meta.valor)}: rode mais de uma replicação`;
      }
      sem = `<span class="sem ${sit}">${GLIFO[sit]}</span><div class="detalhe">${esc(det)}</div>`;
      if (k.indefinido) {
        sem += `<div class="detalhe"><b>Indefinido:</b> a precisão da estimativa ainda cruza a meta${k.replicacoes_faltam ? "; rode mais ~" + k.replicacoes_faltam + " replicações" : ""}.</div>`;
      }
    }
    const fat = k.fatores.length ? `<div class="fatores">${k.fatores.map((f) => `<span>${esc(f.nome)} ${esc(f.texto)} %</span>`).join("")}</div>` : "";
    return `<article class="card sit-${sit}" aria-label="${esc(k.nome)}">
      ${UT.minigrafico(k.valores, k.meta ? k.meta.valor : null)}
      <h3 title="${esc(k.formula)}">${esc(k.nome)}</h3>
      <div class="valor">${esc(k.texto)} <small>${esc(k.unidade)}</small></div>
      <div class="faixa">${k.faixa ? "faixa esperada numa rodada: " + esc(k.faixa) : "uma só replicação"}</div>
      ${fat}${k.nota ? `<div class="detalhe">${esc(k.nota)}</div>` : ""}${sem}
    </article>`;
  }

  function progresso() {
    const p = S.resumo.progresso;
    if (p.concluido) return p.cancelado ? `<p class="muted small">Estudo cancelado com ${p.feitas} de ${p.total} replicações.</p>` : "";
    const f = p.total ? (100 * p.feitas) / p.total : 0;
    return `<div class="no-print"><div class="progress"><i style="width:${f.toFixed(0)}%"></i></div>
      <div class="barra"><span>Rodando: ${p.feitas} de ${p.total} replicações. Os intervalos encolhem a cada replicação.</span>
      ${ESTATICO ? "" : '<button id="cancelar">Cancelar</button>'}</div></div>`;
  }

  function viewGestor() {
    const r = S.resumo;
    const cards = r.cartoes.map(kpi).filter(Boolean);
    const par = S.pareto.itens.map((i) => ({ rotulo: `${i.objeto}: ${i.rotulo}`, valor: i.tempo }));
    return `<p class="pergunta">Estamos bem? Vamos cumprir? Onde está o problema?</p>
      ${progresso()}
      <p class="frase">${esc(r.frase)}</p>
      ${r.avisos.length ? `<div class="avisos"><b>Atenção:</b> ${r.avisos.map(esc).join("; ")}.</div>` : ""}
      <div class="cards">${cards.map(cartao).join("")}</div>
      <h2>Onde o tempo se perde</h2>
      <p class="muted small">Tempo médio por rodada em que cada recurso ficou bloqueado, esperando operador ou transporte, parado ou em falha, em ordem decrescente, com o acumulado em linha.</p>
      <div class="panel">${par.length ? UT.pareto(par, { unidade: S.pareto.unidade, titulo: "Pareto das perdas de tempo" }) : '<p class="muted">Nenhuma perda de tempo registrada nesta rodada.</p>'}</div>
      <p class="glossario"><b>Como ler.</b> <i>Precisão da estimativa</i> é o intervalo de confiança de ${fmt(100 * r.nivel, 0)} % da média entre replicações (o número depois do ±). <i>Faixa esperada numa rodada</i> é o intervalo entre os percentis 5 e 95 das replicações (o termo técnico é intervalo de predição). O <i>semáforo</i> usa a probabilidade de cumprir a meta, a fração das rodadas que a cumpre: verde a partir de ${fmt(100 * r.limiares.green, 0)} %, amarelo a partir de ${fmt(100 * r.limiares.yellow, 0)} %, vermelho abaixo. Passe o mouse sobre o nome do cartão para ver a fórmula.</p>`;
  }

  /* ---- engineer view ---- */
  const th = (col, txt, cls = "") => `<th class="${cls}" data-ordem="${col}" ${S.ordem.col === col ? `aria-sort="${S.ordem.asc ? "ascending" : "descending"}"` : ""}>${txt}${S.ordem.col === col ? (S.ordem.asc ? " ▲" : " ▼") : ""}</th>`;
  const celula = (o) => (o ? o.texto : "-");

  function viewObjetos() {
    const val = (o, c) => (c === "nome" || c === "tipo" ? o[c] : o[c] ? o[c].valor : -Infinity);
    let linhas = S.objetos.filter((o) => (o.nome + " " + o.tipo).toLowerCase().includes(S.filtro.toLowerCase()));
    const { col, asc } = S.ordem;
    linhas.sort((a, b) => {
      const x = val(a, col), y = val(b, col);
      return (typeof x === "string" ? x.localeCompare(y, "pt-BR") : x - y) * (asc ? 1 : -1);
    });
    const util = S.objetos.filter((o) => o.ocupacao).map((o) => ({ rotulo: o.nome, valor: o.ocupacao.valor, lo: o.ocupacao.valor - o.ocupacao.meia, hi: o.ocupacao.valor + o.ocupacao.meia, texto: o.ocupacao.texto + " %", destaque: o.nome === S.resumo.gargalo }));
    return `<p class="pergunta">Por que o resultado é este? Qual objeto limita o sistema?</p>
      ${progresso()}
      <div class="barra no-print"><input type="search" id="filtro" placeholder="Filtrar objetos" value="${esc(S.filtro)}" aria-label="Filtrar objetos">
        <a class="btn" href="${arquivo("objetos.csv")}" download>CSV</a>
        <a class="btn" href="${arquivo("painel.xlsx")}" download>Planilha .xlsx</a></div>
      <table><thead><tr>${th("nome", "Objeto")}${th("tipo", "Tipo")}${th("entradas", "Entradas", "n")}${th("saidas", "Saídas", "n")}${th("conteudo", "Conteúdo médio", "n")}${th("permanencia", "Permanência", "n")}${th("ocupacao", "Ocupação %", "n")}${th("ocupacao_turno", "No turno %", "n")}</tr></thead>
      <tbody>${linhas.map((o) => `<tr class="link" data-objeto="${esc(o.nome)}"><td><a href="#/eng/objeto/${encodeURIComponent(o.nome)}">${esc(o.nome)}</a></td><td>${esc(o.tipo)}</td><td class="n">${celula(o.entradas)}</td><td class="n">${celula(o.saidas)}</td><td class="n">${celula(o.conteudo)}</td><td class="n">${celula(o.permanencia)}</td><td class="n">${celula(o.ocupacao)}</td><td class="n">${celula(o.ocupacao_turno)}</td></tr>`).join("")}</tbody></table>
      <h2>Ocupação dos recursos</h2>
      <div class="panel">${UT.barrasIC(util, { max: 100, unidade: "% do tempo trabalhando", titulo: "Ocupação por recurso, com intervalo de confiança" })}</div>
      <h2>Estados de cada objeto</h2>
      <div class="panel">${UT.empilhada(S.objetos.filter((o) => o.estados.length).map((o) => ({ rotulo: o.nome, estados: o.estados })), { titulo: "Fração do tempo em cada estado, por objeto" })}</div>`;
  }

  async function viewObjeto(nome) {
    if (!S.detalhe[nome]) S.detalhe[nome] = await api("/api/objeto/" + nome);
    const d = S.detalhe[nome];
    const estados = d.estados.map((e) => ({ estado: e.estado, rotulo: e.rotulo, cor: e.cor, fracao: e.fracao }));
    const gantt = d.gantt.length ? UT.gantt([{ rotulo: d.nome, segmentos: d.gantt }], S.estados, d.fim, { left: 120, titulo: "Gantt de estados de " + d.nome }) : '<p class="muted">Sem amostragem de estados nesta rodada.</p>';
    const linhasK = d.kpis.map((k) => `<tr><td>${esc(k.nome)}</td><td class="n">${esc(k.texto)} ${esc(k.unidade)}</td><td class="n">${k.ic ? fmt(k.ic[0]) + " a " + fmt(k.ic[1]) : "-"}</td><td class="n">${k.faixa ? esc(k.faixa) : "-"}</td></tr>`).join("");
    return `<p><a href="#/eng/objetos">&larr; Objetos</a></p>
      <h2 style="margin-top:.4rem">${esc(d.nome)} <span class="muted small">${esc(d.tipo)}</span></h2>
      <p class="pergunta">O que este objeto fez, e em que estado passou o tempo?</p>
      <div class="grid2"><div><table><thead><tr><th>Medida</th><th class="n">Média ± precisão</th><th class="n">IC</th><th class="n">Faixa P5 a P95</th></tr></thead><tbody>${linhasK}</tbody></table></div>
      <div class="panel">${UT.empilhada([{ rotulo: d.nome, estados }], { left: 70, titulo: "Fração do tempo em cada estado" })}</div></div>
      <h2>Gantt de estados (primeira replicação)</h2><div class="panel">${gantt}</div>
      <h2>Conteúdo ao longo do tempo</h2><div class="panel">${UT.linhaBanda([{ nome: "conteúdo", pontos: d.serie.pontos }], { unidade: "itens", unidadeX: S.resumo.unidade_tempo, titulo: d.serie.nome })}
        <p class="legenda"><i style="background:var(--s0);opacity:.3"></i>intervalo de confiança da média <i style="background:var(--s0);opacity:.13"></i>P5 a P95 entre replicações</p></div>
      ${d.histogramas.length ? "<h2>Distribuições</h2>" + d.histogramas.map((h) => `<div class="panel">${UT.histograma(h.histograma, { rotuloX: h.rotulo, unidadeX: S.resumo.unidade_tempo, titulo: "Histograma: " + h.rotulo })}</div>`).join("") : ""}
      ${d.extras.length ? "<h2>Outras medidas</h2><table><tbody>" + d.extras.map((e) => `<tr><td>${esc(e.nome)}</td><td class="n">${esc(e.resumo.texto)}</td></tr>`).join("") + "</tbody></table>" : ""}`;
  }

  function viewDistribuicoes() {
    const itens = S.kpis.filter((k) => k.n >= 2);
    const caixas = itens.map((k) => `<div class="panel"><h3>${esc(k.nome)} <span class="muted small">(${esc(k.unidade)})</span></h3>${UT.caixa([{ rotulo: "entre replicações", p5: k.p5, p25: k.p25, p50: k.p50, p75: k.p75, p95: k.p95, media: k.valor, ic: k.ic, valores: k.valores }], { meta: k.meta ? k.meta.valor : null, unidade: k.unidade, rowH: 70, left: 120, titulo: "Erro e risco: " + k.nome })}</div>`).join("");
    const hs = S.dist.histogramas.map((h) => `<div class="panel">${UT.histograma(h.histograma, { rotuloX: h.rotulo + " em " + h.objeto, unidadeX: S.resumo.unidade_tempo, marcas: marcasLead(), titulo: "Histograma: " + h.rotulo })}</div>`).join("");
    return `<p class="pergunta">Com que confiança vale o número, e que variação esperar de uma rodada para outra?</p>
      ${progresso()}
      <p class="muted small">Cada gráfico mostra, para um indicador: os pontos (uma replicação cada), a caixa do P25 ao P75, o bigode do P5 ao P95, o losango da média e, sob ele, o intervalo de confiança da média. A linha tracejada é a meta.</p>
      <div class="grid2">${caixas || '<p class="muted">São precisas ao menos duas replicações.</p>'}</div>
      ${hs ? "<h2>Distribuição dos itens</h2>" + hs : ""}`;
  }

  function marcasLead() {
    const m = [];
    const a = kpi("lead_time"), b = kpi("lead_time_p90");
    if (a) m.push({ x: a.valor, rotulo: "média" });
    if (b) m.push({ x: b.valor, rotulo: "P90" });
    return m;
  }

  function viewSeries() {
    const w = S.series.wip;
    return `<p class="pergunta">Como o estoque em processo evoluiu, e quanto ele varia entre replicações?</p>
      ${progresso()}
      <div class="panel">${UT.linhaBanda([{ nome: w.nome, pontos: w.pontos }], { unidade: "itens", unidadeX: S.resumo.unidade_tempo, titulo: w.nome })}
      <p class="legenda"><i style="background:var(--s0);opacity:.3"></i>intervalo de confiança da média <i style="background:var(--s0);opacity:.13"></i>P5 a P95 entre replicações. Aquecimento: ${fmt(S.resumo.aquecimento)} ${esc(S.resumo.unidade_tempo)}.</p></div>`;
  }

  function viewGantt() {
    const g = S.gantt;
    return `<p class="pergunta">O que cada objeto fez ao longo da rodada?</p>
      <p class="muted small">Estados amostrados na replicação ${g.replicacao}; a resolução é o intervalo de amostragem do modelo.</p>
      <div class="panel">${UT.gantt(g.linhas, S.estados, g.fim, { unidadeX: S.resumo.unidade_tempo, titulo: "Gantt de estados" })}</div>`;
  }

  function viewEstatistica() {
    const r = S.resumo, L = r.little;
    const linhas = S.kpis.map((k) => `<tr title="${esc(k.formula)}"><td>${esc(k.nome)}</td><td class="n">${k.n}</td><td class="n">${esc(k.texto)} ${esc(k.unidade)}</td><td class="n">${k.ip ? fmt(k.ip[0]) + " a " + fmt(k.ip[1]) : "-"}</td><td class="n">${fmt(k.p5)} a ${fmt(k.p95)}</td><td class="n">${k.replicacoes_2pct ?? "-"}</td><td class="n">${k.prob ? pct1(k.prob.p) + " (" + pct1(k.prob.lo) + " a " + pct1(k.prob.hi) + ")" : "-"}</td><td>${GLIFO[k.situacao]}${k.indefinido ? " (indefinido)" : ""}</td></tr>`).join("");
    return `<p class="pergunta">As conclusões se sustentam? Há replicações suficientes?</p>
      ${progresso()}
      <div class="barra no-print"><a class="btn" href="${arquivo("kpis.csv")}" download>CSV dos KPIs</a><a class="btn" href="${arquivo("painel.json")}" download>JSON</a><a class="btn" href="${arquivo("painel.xlsx")}" download>Planilha .xlsx</a></div>
      <table><thead><tr><th>KPI</th><th class="n">n</th><th class="n">Média ± IC ${fmt(100 * r.nivel, 0)} %</th><th class="n">Intervalo de uma rodada</th><th class="n">P5 a P95</th><th class="n">Replicações p/ ±2 %</th><th class="n">P(cumprir) e Wilson</th><th>Semáforo</th></tr></thead><tbody>${linhas}</tbody></table>
      <h2>Lei de Little</h2>
      <p>${L ? `WIP médio ${fmt(L.l)} contra vazão × lead time ${fmt(L.lambda_w)}: diferença relativa de ${fmt(100 * L.relativo, 1)} %. <b>${L.vale ? "A lei fecha." : "A lei não fecha: há itens fora das filas e estações contadas (transportadores, esteiras de transferência) ou a rodada é curta."}</b>` : "Precisa de ao menos duas replicações e de uma Sink."}</p>
      <h2>Como cada número é calculado</h2>
      <table><tbody>${S.kpis.map((k) => `<tr><td>${esc(k.nome)}</td><td class="muted">${esc(k.formula)}</td></tr>`).join("")}</tbody></table>`;
  }

  /* ---- shell ---- */
  const hash = () => location.hash.replace(/^#\/?/, "") || "gestor";

  async function desenhar() {
    const h = hash();
    let corpo;
    try {
      if (h === "gestor") corpo = viewGestor();
      else if (h === "eng/objetos") corpo = viewObjetos();
      else if (h.startsWith("eng/objeto/")) corpo = await viewObjeto(decodeURIComponent(h.slice("eng/objeto/".length)));
      else if (h === "eng/distribuicoes") corpo = viewDistribuicoes();
      else if (h === "eng/series") corpo = viewSeries();
      else if (h === "eng/gantt") corpo = viewGantt();
      else if (h === "eng/estatistica") corpo = viewEstatistica();
      else corpo = viewGestor();
    } catch (e) {
      corpo = `<p class="avisos">Não foi possível montar esta página: ${esc(e.message)}</p>`;
    }
    const r = S.resumo;
    const ativa = h.startsWith("eng/objeto/") ? "eng/objetos" : h;
    $("#app").innerHTML = `<header class="topo"><h1>${esc(r.titulo)}</h1>
      <div class="sub">${esc(r.modelo)} · ${r.progresso.feitas} de ${r.progresso.total} replicações · parada em ${fmt(r.parada)} ${esc(r.unidade_tempo)}, aquecimento ${fmt(r.aquecimento)} · semente ${r.semente}${ESTATICO ? " · página exportada" : ""}</div>
      <nav aria-label="Páginas">${ABAS.map(([id, nome]) => `<a href="#/${id}" ${ativa === id ? 'aria-current="page"' : ""}>${nome}</a>`).join("")}<span class="sp"></span><a href="#" id="imprimir" class="no-print">Imprimir / PDF</a></nav></header>
      <main>${corpo}</main>`;
    ligar();
  }

  function ligar() {
    const imp = $("#imprimir");
    if (imp) imp.onclick = (e) => { e.preventDefault(); window.print(); };
    const can = $("#cancelar");
    if (can) can.onclick = () => fetch("/api/cancelar", { method: "POST", headers: { "X-UTSim": "1" } });
    const f = $("#filtro");
    if (f) f.oninput = () => { S.filtro = f.value; const p = f.selectionStart; desenhar().then(() => { const n = $("#filtro"); n.focus(); n.setSelectionRange(p, p); }); };
    document.querySelectorAll("th[data-ordem]").forEach((t) => (t.onclick = () => {
      const c = t.dataset.ordem;
      S.ordem = { col: c, asc: S.ordem.col === c ? !S.ordem.asc : true };
      desenhar();
    }));
    document.querySelectorAll("tr.link").forEach((tr) => (tr.onclick = (e) => { if (e.target.tagName !== "A") location.hash = "#/eng/objeto/" + encodeURIComponent(tr.dataset.objeto); }));
  }

  async function acompanhar() {
    if (ESTATICO || S.resumo.progresso.concluido) return;
    let ultimo = S.resumo.progresso.feitas;
    const volta = setInterval(async () => {
      try {
        const p = await api("/api/progresso");
        if (p.feitas !== ultimo || p.concluido) {
          ultimo = p.feitas;
          const y = window.scrollY;
          await carregar();
          await desenhar();
          window.scrollTo(0, y);
        }
        if (p.concluido) clearInterval(volta);
      } catch (e) { clearInterval(volta); }
    }, 1200);
  }

  window.addEventListener("hashchange", desenhar);
  carregar().then(desenhar).then(acompanhar).catch((e) => {
    $("#app").innerHTML = `<p class="avisos" style="margin:1rem">Não foi possível carregar o painel: ${esc(e.message)}</p>`;
  });
})();
