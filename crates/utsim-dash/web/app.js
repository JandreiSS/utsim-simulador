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
  /* Routes that exist only after the analyses (or only with some settings). */
  async function apiOpcional(path) {
    if (ESTATICO) return path in window.UTSIM_DADOS ? window.UTSIM_DADOS[path] : null;
    const r = await fetch(encodeURI(path), { cache: "no-store" });
    return r.ok ? r.json() : null;
  }

  const S = {
    resumo: null, kpis: [], objetos: [], series: null, dist: null, pareto: null, gantt: null, estados: {}, detalhe: {}, ordem: { col: "nome", asc: true }, filtro: "",
    correlacao: null, mapa: null, cenarios: null, pert: null,
    zoom: null, corr: "pearson", vistaMapa: { grupo: "recurso", camada: "todos", medida: "tempo" },
  };

  const ABAS = [
    ["gestor", "Gestor"],
    ["eng/objetos", "Engenheiro: objetos"],
    ["eng/distribuicoes", "Distribuições"],
    ["eng/series", "Séries"],
    ["eng/gantt", "Gantt"],
    ["eng/estatistica", "Estatística"],
    ["eng/cenarios", "Cenários"],
    ["eng/correlacao", "Correlação"],
    ["eng/mapa", "Mapa de calor"],
    ["eng/pert", "PERT"],
  ];

  async function carregar() {
    const [resumo, kpis, objetos, series, dist, pareto, gantt, estados] = await Promise.all(
      ["/api/resumo", "/api/kpis", "/api/objetos", "/api/series", "/api/distribuicoes", "/api/pareto", "/api/gantt", "/api/estados"].map(api)
    );
    const [correlacao, mapa, cenarios, pert] = await Promise.all(["/api/correlacao", "/api/mapa_calor", "/api/cenarios", "/api/pert"].map(apiOpcional));
    Object.assign(S, { resumo, kpis: kpis.kpis, objetos: objetos.objetos, series, dist, pareto, gantt, estados, correlacao, mapa, cenarios, pert });
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
    const msg = p.etapa ? `Replicações prontas; agora ${esc(p.etapa)}.` : `Rodando: ${p.feitas} de ${p.total} replicações. Os intervalos encolhem a cada replicação.`;
    return `<div class="no-print"><div class="progress"><i style="width:${f.toFixed(0)}%"></i></div>
      <div class="barra"><span>${msg}</span>
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

  const LEGENDA_BANDA = `<i style="background:var(--s0);opacity:.3"></i>intervalo de confiança da média <i style="background:var(--s0);opacity:.13"></i>P5 a P95 entre replicações`;

  function viewSeries() {
    const w = S.series.wip, v = S.series.vazao, grupos = S.series.grupos || [];
    const ut = S.resumo.unidade_tempo;
    const vaz = v && v.pontos.length
      ? `<h2>Vazão por janela</h2><div class="panel">${UT.linhaBanda([{ nome: v.nome, pontos: v.pontos }], { unidade: v.unidade, unidadeX: ut, titulo: v.nome })}
        <p class="legenda">${LEGENDA_BANDA}. Chegadas às saídas em janelas de ${fmt(v.janela)} ${esc(ut)}, contadas depois do aquecimento; cada ponto fica no fim da sua janela.</p></div>`
      : "";
    const gs = grupos.map((g) => `<div class="panel">${UT.linhaBanda([{ nome: g.nome, pontos: g.pontos }], { unidade: g.unidade, unidadeX: ut, titulo: g.nome })}
        <p class="legenda">${LEGENDA_BANDA}. Objetos: ${g.objetos.map(esc).join(", ")}.</p></div>`).join("");
    return `<p class="pergunta">Como o estoque em processo e a vazão evoluíram, e quanto variam entre replicações?</p>
      ${progresso()}
      <div class="panel">${UT.linhaBanda([{ nome: w.nome, pontos: w.pontos }], { unidade: "itens", unidadeX: ut, titulo: w.nome })}
      <p class="legenda">${LEGENDA_BANDA}. Aquecimento: ${fmt(S.resumo.aquecimento)} ${esc(ut)}.</p></div>
      ${vaz}${gs ? "<h2>WIP por grupo de objetos</h2>" + gs : '<p class="muted small">Para ver o WIP de um grupo de objetos, declare <code>groups: {"nome": ["obj1", "obj2"]}</code> no arquivo do painel.</p>'}`;
  }

  function viewGantt() {
    const g = S.gantt;
    const exato = g.fonte === "rastro";
    const [a, z] = S.zoom || [0, g.fim];
    return `<p class="pergunta">O que cada objeto fez ao longo da rodada?</p>
      <p class="muted small">${exato ? `Estados exatos da replicação ${g.replicacao}, lidos do rastro (cada mudança de estado no instante em que ocorreu).` : `Estados amostrados na replicação ${g.replicacao}; a resolução é o intervalo de amostragem do modelo. O Gantt exato, lido do rastro, aparece quando as replicações terminam.`} Mesma paleta de estados da cena 3D.</p>
      <div class="barra no-print"><label>De <input type="number" id="z0" value="${+a.toPrecision(6)}" step="any"></label>
        <label>até <input type="number" id="z1" value="${+z.toPrecision(6)}" step="any"></label>
        <button id="zaplicar">Aplicar</button><button id="zmais">Aproximar</button><button id="zmenos">Afastar</button><button id="ztudo">Tudo</button>
        ${fin(g.aquecimento) && g.aquecimento > 0 ? `<button id="zaq">Depois do aquecimento</button>` : ""}</div>
      <div class="panel">${UT.gantt(g.linhas, S.estados, g.fim, { unidadeX: S.resumo.unidade_tempo, titulo: "Gantt de estados", t0: a, t1: z })}</div>`;
  }

  /* ---- scenarios ---- */
  const pTexto = (p) => (!fin(p) ? "-" : p < 1e-4 ? "< 0,0001" : fmt(p, 4));

  function tabelaPares(pares) {
    return `<table><thead><tr><th>Par</th><th class="n">Diferença</th><th class="n">Intervalo simultâneo</th><th class="n">p ajustado</th><th>Conclusão</th></tr></thead><tbody>${pares.map((q) => `<tr><td>${esc(q.a)} − ${esc(q.b)}</td><td class="n">${fmt(q.diferenca)}</td><td class="n">${fmt(q.lo)} a ${fmt(q.hi)}</td><td class="n">${pTexto(q.p)}</td><td>${q.difere ? '<span class="ruim">diferem</span>' : '<span class="muted">sem diferença detectada</span>'}</td></tr>`).join("")}</tbody></table>`;
  }

  function viewCenarios() {
    const c = S.cenarios;
    const nivel = fmt(100 * S.resumo.nivel, 0);
    if (!c) {
      return `<p class="pergunta">Qual cenário é melhor, e a diferença é real ou é acaso?</p>${progresso()}
        <p>Nenhum experimento foi indicado. Declare no arquivo do painel <code>scenarios: Some("experimento.ron")</code> (o formato de <code>utsim experiment</code>); os cenários rodam sobre este modelo com as mesmas sementes quando as replicações terminam. Na linha de comando: <code>utsim analisar modelo.ron --comparar experimento.ron</code>.</p>`;
    }
    if (c.erro) return `<div class="avisos">Os cenários não puderam ser rodados: ${esc(c.erro)}</div>`;
    const blocos = c.metricas.map((m) => {
      const linhas = c.cenarios.map((nome, i) => `<tr><td>${esc(nome)}</td><td class="n">${esc(m.resumos[i].texto)}</td></tr>`).join("");
      const a = m.anova;
      const frase = a ? `ANOVA de um fator: F(${a.gl1}, ${a.gl2}) = ${fmt(a.f, 2)}, p ${a.p < 1e-4 ? "< 0,0001" : "= " + fmt(a.p, 4)}. <b>${a.p < 1 - S.resumo.nivel ? "Os cenários diferem nesta medida." : "Não há diferença detectada entre os cenários nesta medida."}</b>` : "A ANOVA precisa de ao menos duas replicações por cenário.";
      const pares = (lista) => lista.map((q) => ({ rotulo: `${q.a} − ${q.b}`, valor: q.diferenca, lo: q.lo, hi: q.hi, destaque: q.difere }));
      const bon = m.bonferroni ? `<h3 style="margin-top:.8rem">Pares com Bonferroni (números aleatórios comuns)</h3>
          <p class="muted small">${m.bonferroni.comparacoes} comparações, cada intervalo a ${fmt(100 * m.bonferroni.nivel_cada, 2)} % para que todos valham juntos a ${nivel} %; pares formados replicação a replicação (${m.bonferroni.n} pares). É a leitura recomendada: as mesmas sementes tiram o ruído comum.</p>
          ${UT.intervalos(pares(m.bonferroni.pares), { titulo: "Diferenças pareadas com Bonferroni: " + m.rotulo })}${tabelaPares(m.bonferroni.pares)}` : "";
      const tk = m.tukey ? `<h3 style="margin-top:.8rem">Tukey HSD</h3>
          <p class="muted small">Intervalos simultâneos de ${nivel} % pela amplitude estudentizada (q = ${fmt(m.tukey.q, 3)}), tratando os cenários como independentes; costuma ser mais largo que o pareado.</p>
          ${UT.intervalos(pares(m.tukey.pares), { titulo: "Diferenças de Tukey: " + m.rotulo })}${tabelaPares(m.tukey.pares)}` : "";
      return `<div class="panel"><h3>${esc(m.rotulo)} <span class="muted small">${esc(m.metrica)}</span></h3>
        <table><thead><tr><th>Cenário</th><th class="n">Média ± IC ${nivel} %</th></tr></thead><tbody>${linhas}</tbody></table>
        <p>${frase}</p>${bon}${tk}</div>`;
    }).join("");
    return `<p class="pergunta">Qual cenário é melhor, e a diferença é real ou é acaso?</p>${progresso()}
      <p class="muted small">Experimento <b>${esc(c.nome)}</b>: ${c.cenarios.map(esc).join(", ")}. Um intervalo que não cruza o zero (traço vermelho) indica diferença; o p ajustado já considera todas as comparações.</p>${blocos}`;
  }

  /* ---- correlation ---- */
  function viewCorrelacao() {
    const c = S.correlacao;
    if (!c || c.metricas.length < 2 || c.n < 3) {
      return `<p class="pergunta">Que medidas andam juntas?</p>${progresso()}<p class="muted">São precisas ao menos três replicações e duas medidas que variem entre elas.</p>`;
    }
    const m = S.corr === "spearman" ? c.spearman : c.pearson;
    const pk = S.corr === "spearman" ? "p_spearman" : "p_pearson";
    const fortes = c.fortes.map((f) => `<tr><td>${esc(f.a)}</td><td>${esc(f.b)}</td><td class="n">${fmt(f.pearson, 3)}</td><td class="n">${pTexto(f.p_pearson)}</td><td class="n">${fmt(f.spearman, 3)}</td><td class="n">${pTexto(f.p_spearman)}</td></tr>`).join("");
    return `<p class="pergunta">Que medidas andam juntas? Fila longa num posto acompanha vazão menor?</p>${progresso()}
      <div class="barra no-print"><label>Coeficiente <select id="corr"><option value="pearson" ${S.corr === "pearson" ? "selected" : ""}>Pearson (linear)</option><option value="spearman" ${S.corr === "spearman" ? "selected" : ""}>Spearman (postos, monotônica)</option></select></label></div>
      <div class="panel">${UT.matriz(c.rotulos, m, { titulo: "Correlação entre medidas nas " + c.n + " replicações" })}
        <p class="legenda"><i style="background:var(--s0)"></i>positiva <i style="background:var(--s1)"></i>negativa; cor mais forte, correlação mais forte. Valores-p de "${pk === "p_pearson" ? "Pearson" : "Spearman"}" na tabela abaixo.</p></div>
      <h2>Pares mais fortes</h2>
      <table><thead><tr><th>Medida</th><th>Medida</th><th class="n">Pearson</th><th class="n">p</th><th class="n">Spearman</th><th class="n">p</th></tr></thead><tbody>${fortes}</tbody></table>
      <p class="glossario"><b>Como ler.</b> Cada replicação é uma observação do par de medidas. Pearson mede a relação linear; Spearman, a monotônica (mais robusta a valores extremos). O valor-p testa "correlação zero" e não está corrigido para muitas comparações. Correlação não é causa: as duas medidas podem depender de uma terceira, como a chegada de itens.</p>`;
  }

  /* ---- heat map ---- */
  function viewMapa() {
    const d = S.mapa;
    if (!d) return `<p class="pergunta">Por onde os recursos andam?</p>${progresso()}<p class="muted">O mapa de calor aparece quando as replicações terminam (ele lê o rastro da replicação 0).</p>`;
    if (d.erro) return `<div class="avisos">O mapa de calor não pôde ser montado: ${esc(d.erro)}</div>`;
    const v = S.vistaMapa;
    const camadas = v.grupo === "classe" ? d.classe : d.recurso;
    const atual = v.camada === "todos" ? d.total : camadas.find((l) => l.nome === v.camada) || d.total;
    if (d.total.movimento <= 0) return `<p class="pergunta">Por onde os recursos andam?</p><p class="muted">Nada se deslocou no intervalo: o modelo não tem transportadores nem ligações com <code>transfer</code>.</p>`;
    const opc = [`<option value="todos">todos</option>`].concat(camadas.map((l) => `<option value="${esc(l.nome)}" ${l.nome === v.camada ? "selected" : ""}>${esc(l.nome)}</option>`)).join("");
    const idx = v.medida === "passagens" ? 3 : 2;
    const top = [...atual.celulas].sort((a, b) => b[idx] - a[idx]).slice(0, 8);
    const resumo = camadas.map((l) => `<tr><td>${esc(l.nome)}</td><td class="n">${fmt(l.distancia)}</td><td class="n">${fmt(l.movimento)}</td></tr>`).join("");
    return `<p class="pergunta">Por onde os recursos andam, e onde o piso fica mais ocupado?</p>${progresso()}
      <div class="barra no-print"><label>Camadas <select id="mgrupo"><option value="recurso" ${v.grupo === "recurso" ? "selected" : ""}>por recurso</option><option value="classe" ${v.grupo === "classe" ? "selected" : ""}>por classe</option></select></label>
        <label>Camada <select id="mcamada">${opc}</select></label>
        <label>Medida <select id="mmedida"><option value="tempo" ${v.medida === "tempo" ? "selected" : ""}>tempo de ocupação</option><option value="passagens" ${v.medida === "passagens" ? "selected" : ""}>passagens</option></select></label></div>
      <div class="panel">${UT.mapaCalor(d, atual, v.medida, { titulo: "Mapa de calor de deslocamentos: " + atual.nome })}
        <p class="legenda">Replicação ${d.replicacao}, de ${fmt(d.de)} a ${fmt(d.ate)} ${esc(S.resumo.unidade_tempo)} (depois do aquecimento). Cada deslocamento de um recurso (e cada item numa ligação com <code>transfer</code>) passa pelas células a velocidade constante; o tempo é o que ele leva atravessando a célula, e cada entrada conta uma passagem.</p></div>
      <div class="grid2"><div><h2>Camadas</h2><table><thead><tr><th>Camada</th><th class="n">Distância (m)</th><th class="n">Tempo em movimento</th></tr></thead><tbody>${resumo}</tbody></table></div>
      <div><h2>Células mais ${v.medida === "passagens" ? "percorridas" : "ocupadas"}</h2><table><thead><tr><th>Centro (m)</th><th class="n">Ocupação</th><th class="n">Passagens</th></tr></thead><tbody>${top.map((c) => `<tr><td>(${fmt(d.x0 + (c[0] + 0.5) * d.celula)}; ${fmt(d.y0 + (c[1] + 0.5) * d.celula)})</td><td class="n">${fmt(c[2])}</td><td class="n">${c[3]}</td></tr>`).join("")}</tbody></table></div></div>`;
  }

  /* ---- PERT ---- */
  function viewPert() {
    const p = S.pert;
    if (!p) {
      return `<p class="pergunta">Que atividade mais pesa no prazo de um trecho?</p>${progresso()}
        <p>Nenhum trecho foi indicado. Declare no arquivo do painel, por exemplo, <code>pert: Some((from: Some("corte"), to: Some("embalagem"), deadline: Some(480.0)))</code>: o painel roda o Monte Carlo da rede do PERT/CPM (como <code>utsim pert --modo montecarlo</code>) e mostra o tornado da sensibilidade.</p>`;
    }
    if (p.erro) return `<div class="avisos">O PERT/CPM não pôde ser calculado: ${esc(p.erro)}</div>`;
    const tor = p.tornado.map((t) => ({ rotulo: t.atividade, valor: t.correlacao, extra: `criticidade ${fmt(100 * t.criticidade, 1)} %, crucialidade ${fmt(t.crucialidade, 3)}` }));
    const crit = p.criticidade.filter((c) => c.indice > 0).sort((a, b) => b.indice - a.indice)
      .map((c) => ({ rotulo: c.atividade, valor: 100 * c.indice, lo: 100 * (c.indice - c.meia), hi: 100 * (c.indice + c.meia), texto: fmt(100 * c.indice, 1) + " %" }));
    const prazo = p.prazo ? `<p>Probabilidade de cumprir o prazo de ${fmt(p.prazo.prazo)}: <b>${fmt(100 * p.prazo.p, 1)} %</b> (IC de 95 %: ${fmt(100 * p.prazo.lo, 1)} a ${fmt(100 * p.prazo.hi, 1)} %).</p>` : "";
    return `<p class="pergunta">Que atividade mais pesa no prazo do trecho (${esc(p.selecao)})?</p>${progresso()}
      <p>Tempo total (${p.amostras} sorteios): <b>${esc(p.total.texto)}</b> ${esc(S.resumo.unidade_tempo)}; percentis ${p.percentis.map(([q, v]) => "P" + fmt(100 * q, 0) + " " + fmt(v)).join(", ")}.</p>${prazo}
      ${p.avisos.length ? `<div class="avisos"><b>Hipóteses:</b> ${p.avisos.map(esc).join("; ")}.</div>` : ""}
      <h2>Sensibilidade (tornado)</h2>
      <div class="panel">${UT.tornado(tor, { titulo: "Tornado: correlação de postos de cada duração com o tempo total" })}
        <p class="legenda">Ordem pela crucialidade (criticidade × correlação). Barras maiores: atividades cuja variação mais move o tempo total; reduzir a variação delas é o que mais estreita o prazo.</p></div>
      <h2>Índice de criticidade</h2>
      <div class="panel">${UT.barrasIC(crit, { max: 100, unidade: "% dos sorteios no caminho crítico", titulo: "Índice de criticidade por atividade" })}</div>`;
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
      else if (h === "eng/cenarios") corpo = viewCenarios();
      else if (h === "eng/correlacao") corpo = viewCorrelacao();
      else if (h === "eng/mapa") corpo = viewMapa();
      else if (h === "eng/pert") corpo = viewPert();
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
    /* Gantt zoom */
    const fimG = S.gantt ? S.gantt.fim : 0;
    const janela = () => S.zoom || [0, fimG];
    const zoom = (a, z) => { a = Math.max(0, a); z = Math.min(fimG, z); S.zoom = z > a ? [a, z] : null; desenhar(); };
    const bt = (id, f) => { const e = $("#" + id); if (e) e.onclick = f; };
    bt("zaplicar", () => zoom(+$("#z0").value, +$("#z1").value));
    bt("zmais", () => { const [a, z] = janela(), m = (a + z) / 2, h = (z - a) / 4; zoom(m - h, m + h); });
    bt("zmenos", () => { const [a, z] = janela(), m = (a + z) / 2, h = z - a; zoom(m - h, m + h); });
    bt("ztudo", () => { S.zoom = null; desenhar(); });
    bt("zaq", () => zoom(S.gantt.aquecimento, fimG));
    /* Correlation and heat map selectors */
    const sel = (id, f) => { const e = $("#" + id); if (e) e.onchange = () => { f(e.value); desenhar(); }; };
    sel("corr", (v) => (S.corr = v));
    sel("mgrupo", (v) => { S.vistaMapa.grupo = v; S.vistaMapa.camada = "todos"; });
    sel("mcamada", (v) => (S.vistaMapa.camada = v));
    sel("mmedida", (v) => (S.vistaMapa.medida = v));
  }

  async function acompanhar() {
    if (ESTATICO || S.resumo.progresso.concluido) return;
    let ultimo = S.resumo.progresso.feitas;
    const volta = setInterval(async () => {
      try {
        const p = await api("/api/progresso");
        if (p.feitas !== ultimo || p.concluido || p.etapa !== S.resumo.progresso.etapa) {
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
