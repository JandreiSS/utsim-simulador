"use strict";
/* SVG charts of the UTSim dashboard: plain JavaScript, no library, no build step.
   Every function returns an SVG string. Drawing only: the numbers come from the Rust side. */
const UT = (() => {
  const esc = (s) =>
    String(s).replace(/[&<>"']/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" })[c]);
  const fin = (x) => typeof x === "number" && isFinite(x);
  const fmt = (x, d) => {
    if (!fin(x)) return "-";
    if (x === 0) x = 0; /* no "-0" */
    if (d === undefined) {
      const a = Math.abs(x);
      d = a >= 100 ? 0 : a >= 10 ? 1 : a >= 1 ? 2 : 3;
    }
    return x.toLocaleString("pt-BR", { minimumFractionDigits: 0, maximumFractionDigits: d });
  };

  /* "Nice" axis ticks covering [lo, hi]. */
  function ticks(lo, hi, n = 5) {
    if (!fin(lo) || !fin(hi)) return [0, 1];
    if (hi <= lo) hi = lo + 1;
    const raw = (hi - lo) / n;
    const mag = Math.pow(10, Math.floor(Math.log10(raw)));
    const step = [1, 2, 2.5, 5, 10].map((m) => m * mag).find((s) => s >= raw) || raw;
    const out = [];
    /* From the multiple of the step at or below lo to the one at or above hi, so the
       axis always covers the data. */
    const first = Math.floor(lo / step + 1e-9) * step, last = Math.ceil(hi / step - 1e-9) * step;
    for (let v = first; v <= last + step * 1e-9; v += step) out.push(Math.abs(v) < step * 1e-9 ? 0 : +v.toPrecision(12));
    return out.length ? out : [lo, hi];
  }

  const linear = (d0, d1, r0, r1) => (v) => (d1 === d0 ? (r0 + r1) / 2 : r0 + ((v - d0) / (d1 - d0)) * (r1 - r0));

  const wrap = (w, h, title, body) =>
    `<svg class="chart" viewBox="0 0 ${w} ${h}" role="img" aria-label="${esc(title)}" xmlns="http://www.w3.org/2000/svg">` +
    `<title>${esc(title)}</title>${body}</svg>`;

  const text = (x, y, s, cls = "", anchor = "start") =>
    `<text x="${x.toFixed(1)}" y="${y.toFixed(1)}" class="${cls}" text-anchor="${anchor}">${esc(s)}</text>`;

  /* Horizontal axis with grid lines. */
  function xAxis(sx, tk, y0, y1, label, unit, f = fmt) {
    let s = "";
    for (const t of tk) {
      const x = sx(t);
      s += `<line class="grid" x1="${x.toFixed(1)}" x2="${x.toFixed(1)}" y1="${y0}" y2="${y1}"/>`;
      s += text(x, y1 + 14, f(t), "tick", "middle");
    }
    s += `<line class="axis" x1="${sx(tk[0]).toFixed(1)}" x2="${sx(tk[tk.length - 1]).toFixed(1)}" y1="${y1}" y2="${y1}"/>`;
    if (label) s += text((sx(tk[0]) + sx(tk[tk.length - 1])) / 2, y1 + 30, label + (unit ? ` (${unit})` : ""), "axis-label", "middle");
    return s;
  }

  function yAxis(sy, tk, x0, x1, label) {
    let s = "";
    for (const t of tk) {
      const y = sy(t);
      s += `<line class="grid" x1="${x0}" x2="${x1}" y1="${y.toFixed(1)}" y2="${y.toFixed(1)}"/>`;
      s += text(x0 - 6, y + 4, fmt(t), "tick", "end");
    }
    s += `<line class="axis" x1="${x0}" x2="${x0}" y1="${sy(tk[0]).toFixed(1)}" y2="${sy(tk[tk.length - 1]).toFixed(1)}"/>`;
    if (label) s += `<text class="axis-label" transform="translate(12 ${(sy(tk[0]) + sy(tk[tk.length - 1])) / 2}) rotate(-90)" text-anchor="middle">${esc(label)}</text>`;
    return s;
  }

  /* Bars with a confidence whisker. items: [{rotulo, valor, lo, hi, texto, destaque}] */
  function barrasIC(items, o = {}) {
    const W = 640, left = o.left || 170, right = 90, rowH = 28, top = 8;
    const H = top + items.length * rowH + 34;
    const max = o.max || Math.max(...items.map((i) => (fin(i.hi) ? i.hi : i.valor)), 1e-9) * 1.05;
    const tk = ticks(0, max, 5);
    const sx = linear(0, tk[tk.length - 1], left, W - right);
    let b = xAxis(sx, tk, top, top + items.length * rowH, "", o.unidade);
    items.forEach((it, k) => {
      const y = top + k * rowH;
      const x1 = sx(Math.max(0, it.valor));
      b += text(left - 8, y + rowH / 2 + 4, it.rotulo, "label", "end");
      b += `<rect class="${it.destaque ? "bar hot" : "bar"}" x="${left}" y="${y + 5}" width="${Math.max(0, x1 - left).toFixed(1)}" height="${rowH - 10}"/>`;
      if (fin(it.lo) && fin(it.hi)) {
        const a = sx(it.lo), c = sx(it.hi), m = y + rowH / 2;
        b += `<line class="whisker" x1="${a.toFixed(1)}" x2="${c.toFixed(1)}" y1="${m}" y2="${m}"/>`;
        b += `<line class="whisker" x1="${a.toFixed(1)}" x2="${a.toFixed(1)}" y1="${m - 5}" y2="${m + 5}"/>`;
        b += `<line class="whisker" x1="${c.toFixed(1)}" x2="${c.toFixed(1)}" y1="${m - 5}" y2="${m + 5}"/>`;
      }
      b += text(W - right + 6, y + rowH / 2 + 4, it.texto || fmt(it.valor), "value");
    });
    if (o.unidade) b += text((left + W - right) / 2, H - 4, o.unidade, "axis-label", "middle");
    return wrap(W, H, o.titulo || "Barras com intervalo de confiança", b);
  }

  /* Line with confidence band (dark) and P5-P95 band (light). pontos: [t, mean, lo, hi, p5, p95] */
  function linhaBanda(series, o = {}) {
    const W = 680, H = 280, L = 56, R = 14, T = 12, B = 44;
    const all = series.flatMap((s) => s.pontos);
    if (!all.length) return semDados(o.titulo);
    const t0 = Math.min(...all.map((p) => p[0])), t1 = Math.max(...all.map((p) => p[0]));
    let lo = Math.min(...all.map((p) => (fin(p[4]) ? p[4] : p[1]))), hi = Math.max(...all.map((p) => (fin(p[5]) ? p[5] : p[1])));
    if (fin(o.meta)) { lo = Math.min(lo, o.meta); hi = Math.max(hi, o.meta); }
    lo = Math.min(lo, 0);
    const ty = ticks(lo, hi, 5), tx = ticks(t0, t1, 6);
    const sy = linear(ty[0], ty[ty.length - 1], H - B, T), sx = linear(tx[0], tx[tx.length - 1], L, W - R);
    let b = yAxis(sy, ty, L, W - R, o.unidade) + xAxis(sx, tx, T, H - B, o.rotuloX || "Tempo", o.unidadeX);
    const area = (pts, i0, i1) => {
      const up = pts.filter((p) => fin(p[i1])).map((p) => `${sx(p[0]).toFixed(1)},${sy(p[i1]).toFixed(1)}`);
      const dn = pts.filter((p) => fin(p[i0])).map((p) => `${sx(p[0]).toFixed(1)},${sy(p[i0]).toFixed(1)}`).reverse();
      return up.concat(dn).join(" ");
    };
    series.forEach((s, k) => {
      const c = `s${k % 4}`;
      b += `<polygon class="band2 ${c}" points="${area(s.pontos, 4, 5)}"/>`;
      b += `<polygon class="band ${c}" points="${area(s.pontos, 2, 3)}"/>`;
      b += `<polyline class="line ${c}" points="${s.pontos.map((p) => `${sx(p[0]).toFixed(1)},${sy(p[1]).toFixed(1)}`).join(" ")}"/>`;
    });
    if (fin(o.meta)) b += `<line class="goal" x1="${L}" x2="${W - R}" y1="${sy(o.meta).toFixed(1)}" y2="${sy(o.meta).toFixed(1)}"/>` + text(W - R - 4, sy(o.meta) - 4, "meta " + fmt(o.meta), "tick", "end");
    return wrap(W, H, o.titulo || "Série com banda", b);
  }

  const semDados = (t) => wrap(300, 40, t || "Sem dados", text(8, 24, "Sem dados para este gráfico.", "label"));

  /* Histogram. h: {min, max, largura, contagens}; marcas: [{x, rotulo}] */
  function histograma(h, o = {}) {
    const c = h && h.contagens;
    if (!c || !c.length) return semDados(o.titulo);
    const W = 640, H = 240, L = 52, R = 14, T = 12, B = 44;
    const w = h.largura > 0 ? h.largura : 1;
    const x0 = h.min, x1 = h.largura > 0 ? h.min + w * c.length : h.min + 1;
    const total = c.reduce((a, b) => a + b, 0) || 1;
    const dens = c.map((v) => (o.contagem ? v : (100 * v) / total));
    const ty = ticks(0, Math.max(...dens) * 1.08, 4), tx = ticks(x0, x1, 6);
    const sy = linear(0, ty[ty.length - 1], H - B, T), sx = linear(Math.min(tx[0], x0), Math.max(tx[tx.length - 1], x1), L, W - R);
    let b = yAxis(sy, ty, L, W - R, o.contagem ? "Itens" : "% dos itens") + xAxis(sx, tx, T, H - B, o.rotuloX || "Valor", o.unidadeX);
    dens.forEach((v, k) => {
      const a = sx(x0 + w * k), z = sx(x0 + w * (k + 1));
      b += `<rect class="bar" x="${a.toFixed(1)}" y="${sy(v).toFixed(1)}" width="${Math.max(1, z - a - 1).toFixed(1)}" height="${(H - B - sy(v)).toFixed(1)}"><title>${fmt(x0 + w * k)} a ${fmt(x0 + w * (k + 1))}: ${fmt(v, 1)}${o.contagem ? "" : " %"}</title></rect>`;
    });
    (o.marcas || []).forEach((m, k) => {
      const x = sx(m.x);
      b += `<line class="goal" x1="${x.toFixed(1)}" x2="${x.toFixed(1)}" y1="${T}" y2="${H - B}"/>` + text(x + 3, T + 10 + k * 12, m.rotulo, "tick");
    });
    return wrap(W, H, o.titulo || "Histograma", b);
  }

  /* State Gantt. linhas: [{rotulo, segmentos:[[t0,t1,estado]]}]; estados: {nome:{cor,rotulo}} */
  function gantt(linhas, estados, fim, o = {}) {
    if (!linhas.length) return semDados(o.titulo);
    const W = 760, L = o.left || 130, R = 14, rowH = o.rowH || 22, T = 8;
    const used = new Set();
    linhas.forEach((l) => l.segmentos.forEach((s) => used.add(s[2])));
    const legend = [...used];
    const H = T + linhas.length * rowH + 54 + 18 * Math.ceil(legend.length / 4);
    let t0 = Math.min(...linhas.flatMap((l) => l.segmentos.map((s) => s[0])));
    let t1 = fin(fim) ? fim : Math.max(...linhas.flatMap((l) => l.segmentos.map((s) => s[1])));
    /* Zoom: only the window [o.t0, o.t1], segments clipped to it. */
    if (fin(o.t0) && fin(o.t1) && o.t1 > o.t0) { t0 = o.t0; t1 = o.t1; }
    const tx = ticks(t0, t1, 8).filter((t) => t >= t0 - 1e-9 && t <= t1 + 1e-9);
    if (tx.length < 2) tx.splice(0, tx.length, t0, t1);
    const sx = linear(t0, t1, L, W - R);
    let b = xAxis(sx, tx, T, T + linhas.length * rowH, "Tempo", o.unidadeX);
    linhas.forEach((l, k) => {
      const y = T + k * rowH;
      b += text(L - 8, y + rowH / 2 + 4, l.rotulo, "label", "end");
      for (const [a0, z0, st] of l.segmentos) {
        if (z0 <= t0 || a0 >= t1) continue;
        const a = Math.max(a0, t0), z = Math.min(z0, t1);
        const info = estados[st] || {};
        b += `<rect x="${sx(a).toFixed(1)}" y="${y + 3}" width="${Math.max(0.6, sx(z) - sx(a)).toFixed(1)}" height="${rowH - 6}" fill="${info.cor || "#9aa0a6"}"><title>${esc(l.rotulo)}: ${esc(info.rotulo || st)} (${fmt(a)} a ${fmt(z)})</title></rect>`;
      }
    });
    const ly = T + linhas.length * rowH + 54;
    legend.forEach((st, k) => {
      const info = estados[st] || {};
      const x = L + (k % 4) * 150, y = ly + Math.floor(k / 4) * 18;
      b += `<rect x="${x}" y="${y - 10}" width="12" height="12" fill="${info.cor || "#9aa0a6"}"/>` + text(x + 17, y, info.rotulo || st, "tick");
    });
    return wrap(W, H, o.titulo || "Gantt de estados", b);
  }

  /* "Error and risk" box: mean with CI (thick) over P5-P95 whisker and P25-P75 box, plus the
     value of each replication as a dot. items: [{rotulo, p5,p25,p50,p75,p95, media, ic, valores}] */
  function caixa(items, o = {}) {
    items = items.filter((i) => fin(i.media));
    if (!items.length) return semDados(o.titulo);
    const W = 640, left = o.left || 150, right = 20, rowH = o.rowH || 54, top = 6;
    const H = top + items.length * rowH + 34;
    const vals = items.flatMap((i) => [i.p5, i.p95, i.media, ...(i.valores || []).filter(fin), ...(i.ic || [])]).filter(fin);
    if (fin(o.meta)) vals.push(o.meta);
    let lo = Math.min(...vals), hi = Math.max(...vals);
    const pad = (hi - lo || Math.abs(hi) || 1) * 0.08;
    lo -= pad; hi += pad;
    const tk = ticks(lo, hi, 6);
    const sx = linear(tk[0], tk[tk.length - 1], left, W - right);
    let b = xAxis(sx, tk, top, top + items.length * rowH, "", o.unidade);
    items.forEach((it, k) => {
      const m = top + k * rowH + rowH / 2;
      b += text(left - 8, m + 4, it.rotulo, "label", "end");
      if (fin(it.p5) && fin(it.p95)) {
        b += `<line class="whisker" x1="${sx(it.p5).toFixed(1)}" x2="${sx(it.p95).toFixed(1)}" y1="${m}" y2="${m}"/>`;
        b += `<line class="whisker" x1="${sx(it.p5).toFixed(1)}" x2="${sx(it.p5).toFixed(1)}" y1="${m - 7}" y2="${m + 7}"/>`;
        b += `<line class="whisker" x1="${sx(it.p95).toFixed(1)}" x2="${sx(it.p95).toFixed(1)}" y1="${m - 7}" y2="${m + 7}"/>`;
      }
      if (fin(it.p25) && fin(it.p75)) {
        b += `<rect class="box" x="${sx(it.p25).toFixed(1)}" y="${m - 11}" width="${Math.max(1, sx(it.p75) - sx(it.p25)).toFixed(1)}" height="22"/>`;
        b += `<line class="median" x1="${sx(it.p50).toFixed(1)}" x2="${sx(it.p50).toFixed(1)}" y1="${m - 11}" y2="${m + 11}"/>`;
      }
      (it.valores || []).filter(fin).forEach((v) => (b += `<circle class="dot" cx="${sx(v).toFixed(1)}" cy="${m}" r="2.4"><title>${fmt(v)}</title></circle>`));
      if (it.ic) b += `<line class="ci" x1="${sx(it.ic[0]).toFixed(1)}" x2="${sx(it.ic[1]).toFixed(1)}" y1="${m + 16}" y2="${m + 16}"/>`;
      b += `<path class="mean" d="M${sx(it.media).toFixed(1)} ${m + 10} l5 6 l-5 6 l-5 -6 z"><title>média ${fmt(it.media)}</title></path>`;
    });
    if (fin(o.meta)) b += `<line class="goal" x1="${sx(o.meta).toFixed(1)}" x2="${sx(o.meta).toFixed(1)}" y1="${top}" y2="${top + items.length * rowH}"/>` + text(sx(o.meta) + 3, top + 10, "meta", "tick");
    return wrap(W, H, o.titulo || "Erro e risco", b);
  }

  /* Stacked bars of the fraction of time in each state. items: [{rotulo, estados:[{estado,rotulo,cor,fracao}]}] */
  function empilhada(items, o = {}) {
    if (!items.length) return semDados(o.titulo);
    const W = 680, left = o.left || 130, right = 16, rowH = 26, top = 6;
    const used = new Map();
    items.forEach((i) => i.estados.forEach((e) => used.set(e.estado, e)));
    const legend = [...used.values()];
    const H = top + items.length * rowH + 44 + 18 * Math.ceil(legend.length / 4);
    const sx = linear(0, 1, left, W - right);
    let b = xAxis(sx, [0, 0.25, 0.5, 0.75, 1], top, top + items.length * rowH, "Fração do tempo", "", (t) => fmt(t * 100) + "%");
    items.forEach((it, k) => {
      const y = top + k * rowH;
      b += text(left - 8, y + rowH / 2 + 4, it.rotulo, "label", "end");
      let acc = 0;
      for (const e of it.estados) {
        if (!(e.fracao > 0)) continue;
        const w = Math.max(0, sx(acc + e.fracao) - sx(acc));
        b += `<rect x="${sx(acc).toFixed(1)}" y="${y + 4}" width="${w.toFixed(1)}" height="${rowH - 8}" fill="${e.cor}"><title>${esc(it.rotulo)}: ${esc(e.rotulo)} ${fmt(100 * e.fracao, 1)}%</title></rect>`;
        acc += e.fracao;
      }
    });
    const ly = top + items.length * rowH + 44;
    legend.forEach((e, k) => {
      const x = left + (k % 4) * 140, y = ly + Math.floor(k / 4) * 18;
      b += `<rect x="${x}" y="${y - 10}" width="12" height="12" fill="${e.cor}"/>` + text(x + 17, y, e.rotulo, "tick");
    });
    return wrap(W, H, o.titulo || "Estados", b);
  }

  /* Pareto: bars sorted descending plus the cumulative share. itens: [{rotulo, valor}] */
  function pareto(itens, o = {}) {
    if (!itens.length) return semDados(o.titulo);
    const W = 680, H = 280, L = 56, R = 46, T = 12, B = 84;
    const total = itens.reduce((a, i) => a + i.valor, 0) || 1;
    const ty = ticks(0, Math.max(...itens.map((i) => i.valor)) * 1.1, 4), sy = linear(0, ty[ty.length - 1], H - B, T);
    const sc = linear(0, 100, H - B, T);
    const bw = (W - L - R) / itens.length;
    let b = yAxis(sy, ty, L, W - R, o.unidade);
    let acc = 0, pts = [];
    itens.forEach((it, k) => {
      const x = L + k * bw;
      b += `<rect class="bar" x="${(x + 3).toFixed(1)}" y="${sy(it.valor).toFixed(1)}" width="${Math.max(2, bw - 6).toFixed(1)}" height="${(H - B - sy(it.valor)).toFixed(1)}"><title>${esc(it.rotulo)}: ${fmt(it.valor)} ${o.unidade || ""}</title></rect>`;
      b += `<text class="tick" transform="translate(${(x + bw / 2).toFixed(1)} ${H - B + 10}) rotate(-35)" text-anchor="end">${esc(it.rotulo)}</text>`;
      acc += it.valor;
      pts.push(`${(x + bw / 2).toFixed(1)},${sc((100 * acc) / total).toFixed(1)}`);
    });
    b += `<polyline class="line s1" points="${pts.join(" ")}"/>`;
    for (const t of [0, 50, 100]) b += text(W - R + 6, sc(t) + 4, t + "%", "tick");
    return wrap(W, H, o.titulo || "Pareto", b);
  }

  /* Tiny bars: the value of each replication, with the goal as a line. */
  function minigrafico(valores, meta) {
    const v = valores.filter(fin);
    if (v.length < 2) return "";
    const W = 120, H = 28;
    let lo = Math.min(...v, fin(meta) ? meta : Infinity), hi = Math.max(...v, fin(meta) ? meta : -Infinity);
    if (hi === lo) { hi += 1; lo -= 1; }
    const sy = linear(lo, hi, H - 2, 2), bw = W / v.length;
    let b = "";
    v.forEach((x, k) => (b += `<rect class="spark" x="${(k * bw + 0.5).toFixed(1)}" y="${sy(x).toFixed(1)}" width="${Math.max(1, bw - 1).toFixed(1)}" height="${(H - 2 - sy(x)).toFixed(1)}"/>`));
    if (fin(meta)) b += `<line class="goal" x1="0" x2="${W}" y1="${sy(meta).toFixed(1)}" y2="${sy(meta).toFixed(1)}"/>`;
    return `<svg class="spark-svg" viewBox="0 0 ${W} ${H}" role="img" aria-label="valor em cada replicação" xmlns="http://www.w3.org/2000/svg"><title>Valor em cada replicação</title>${b}</svg>`;
  }

  /* Differences with simultaneous intervals around zero (forest plot).
     items: [{rotulo, valor, lo, hi, destaque}] */
  function intervalos(items, o = {}) {
    items = items.filter((i) => fin(i.valor));
    if (!items.length) return semDados(o.titulo);
    const longest = Math.max(...items.map((i) => String(i.rotulo).length));
    const W = 760, left = o.left || Math.min(340, 24 + 6.6 * longest), right = 20, rowH = 26, top = 8;
    const H = top + items.length * rowH + 40;
    const vals = items.flatMap((i) => [i.lo, i.hi, i.valor]).filter(fin).concat([0]);
    let lo = Math.min(...vals), hi = Math.max(...vals);
    const pad = (hi - lo || 1) * 0.06;
    const tk = ticks(lo - pad, hi + pad, 6);
    const sx = linear(tk[0], tk[tk.length - 1], left, W - right);
    let b = xAxis(sx, tk, top, top + items.length * rowH, o.rotuloX || "Diferença", o.unidade);
    b += `<line class="goal" x1="${sx(0).toFixed(1)}" x2="${sx(0).toFixed(1)}" y1="${top}" y2="${top + items.length * rowH}"/>`;
    items.forEach((it, k) => {
      const m = top + k * rowH + rowH / 2;
      b += text(left - 8, m + 4, it.rotulo, "label", "end");
      if (fin(it.lo) && fin(it.hi)) {
        b += `<line class="${it.destaque ? "ci" : "whisker"}" x1="${sx(it.lo).toFixed(1)}" x2="${sx(it.hi).toFixed(1)}" y1="${m}" y2="${m}"/>`;
      }
      b += `<circle class="${it.destaque ? "mean" : "dot"}" cx="${sx(it.valor).toFixed(1)}" cy="${m}" r="4"><title>${esc(it.rotulo)}: ${fmt(it.valor)} (${fmt(it.lo)} a ${fmt(it.hi)})</title></circle>`;
    });
    return wrap(W, H, o.titulo || "Intervalos", b);
  }

  /* Correlation matrix: blue for positive, orange for negative, stronger colour for a
     larger |r|. rotulos: names; valores: k x k (null for undefined). */
  function matriz(rotulos, valores, o = {}) {
    const k = rotulos.length;
    if (k < 2) return semDados(o.titulo);
    const cell = Math.max(30, Math.min(56, Math.floor(420 / k)));
    const left = o.left || 220, top = 8;
    const W = left + k * cell + 20, H = top + k * cell + 120;
    let b = "";
    for (let i = 0; i < k; i++) {
      b += text(left - 8, top + i * cell + cell / 2 + 4, rotulos[i], "label", "end");
      b += `<text class="tick" transform="translate(${(left + i * cell + cell / 2).toFixed(1)} ${top + k * cell + 8}) rotate(40)" text-anchor="start">${esc(rotulos[i])}</text>`;
      for (let j = 0; j < k; j++) {
        const v = valores[i][j];
        const x = left + j * cell, y = top + i * cell;
        if (!fin(v)) {
          b += `<rect class="cell-na" x="${x}" y="${y}" width="${cell - 1}" height="${cell - 1}"><title>${esc(rotulos[i])} x ${esc(rotulos[j])}: indefinida</title></rect>`;
          continue;
        }
        const op = Math.max(0.06, Math.min(1, Math.abs(v))).toFixed(2);
        b += `<rect class="${v >= 0 ? "cell-pos" : "cell-neg"}" fill-opacity="${op}" x="${x}" y="${y}" width="${cell - 1}" height="${cell - 1}"><title>${esc(rotulos[i])} x ${esc(rotulos[j])}: ${fmt(v, 3)}</title></rect>`;
        b += text(x + cell / 2, y + cell / 2 + 4, fmt(v, 2), Math.abs(v) > 0.6 ? "cell-text strong" : "cell-text", "middle");
      }
    }
    return wrap(W, H, o.titulo || "Matriz de correlação", b);
  }

  /* Sequential heat colour, pale yellow (0) to dark red (1): the same scale as the SVG
     written by `utsim analisar --mapa-calor`. */
  const HEAT = [[255, 245, 177], [254, 204, 92], [253, 141, 60], [227, 26, 28], [128, 0, 38]];
  function corCalor(v) {
    v = Math.max(0, Math.min(1, v)) * (HEAT.length - 1);
    const i = Math.min(HEAT.length - 2, Math.floor(v)), f = v - i;
    const c = [0, 1, 2].map((k) => Math.round(HEAT[i][k] + (HEAT[i + 1][k] - HEAT[i][k]) * f));
    return "#" + c.map((x) => x.toString(16).padStart(2, "0")).join("");
  }

  /* Heat map over the plan. d: /api/mapa_calor; camada: {nome, celulas:[[col,row,tempo,passagens]]};
     medida: "tempo" | "passagens". */
  function mapaCalor(d, camada, medida, o = {}) {
    if (!d || !camada) return semDados(o.titulo);
    const M = 30, W = 760;
    const wm = d.colunas * d.celula, hm = d.linhas * d.celula;
    const px = (W - 2 * M) / Math.max(wm, 1e-9);
    const H = hm * px + 2 * M + 34;
    const sx = (x) => M + (x - d.x0) * px, sy = (y) => M + hm * px - (y - d.y0) * px;
    const idx = medida === "passagens" ? 3 : 2;
    const max = Math.max(0, ...camada.celulas.map((c) => c[idx]));
    let b = `<rect class="plan-bg" x="${M}" y="${M}" width="${(wm * px).toFixed(1)}" height="${(hm * px).toFixed(1)}"/>`;
    const side = d.celula * px;
    for (const [col, row, t, n] of camada.celulas) {
      const v = medida === "passagens" ? n : t;
      if (!(v > 0) || !(max > 0)) continue;
      const x = d.x0 + col * d.celula, y = d.y0 + (row + 1) * d.celula;
      b += `<rect x="${sx(x).toFixed(2)}" y="${sy(y).toFixed(2)}" width="${side.toFixed(2)}" height="${side.toFixed(2)}" fill="${corCalor(v / max)}"><title>(${fmt(x + d.celula / 2)}; ${fmt(y - d.celula / 2)}) m: ${fmt(t)} de ocupação, ${n} passagens</title></rect>`;
    }
    for (const [x1, y1, x2, y2] of d.ligacoes) {
      b += `<line class="plan-link" x1="${sx(x1).toFixed(1)}" y1="${sy(y1).toFixed(1)}" x2="${sx(x2).toFixed(1)}" y2="${sy(y2).toFixed(1)}"/>`;
    }
    for (const ob of d.objetos) {
      const x = sx(ob.x), y = sy(ob.y), fim = x > 0.75 * W;
      b += `<rect class="plan-obj" x="${(x - 4).toFixed(1)}" y="${(y - 4).toFixed(1)}" width="8" height="8"><title>${esc(ob.nome)} (${esc(ob.tipo)})</title></rect>`;
      b += text(fim ? x - 6 : x + 6, y - 6, ob.nome, "tick", fim ? "end" : "start");
    }
    const ly = hm * px + 2 * M + 6;
    for (let i = 0; i < 5; i++) b += `<rect x="${M + 26 * i}" y="${ly.toFixed(1)}" width="26" height="12" fill="${corCalor((i + 0.5) / 5)}"/>`;
    b += text(M + 140, ly + 10, (medida === "passagens" ? "Passagens" : "Tempo de ocupação") + ` por célula de ${fmt(d.celula)} m: de 0 a ${fmt(max)}`, "tick");
    return wrap(W, H, o.titulo || "Mapa de calor", b);
  }

  /* Tornado: rank correlation of each activity with the total time, largest first.
     itens: [{rotulo, valor, extra}] with valor in [-1, 1]. */
  function tornado(itens, o = {}) {
    if (!itens.length) return semDados(o.titulo);
    const longest = Math.max(...itens.map((i) => String(i.rotulo).length));
    const W = 760, left = o.left || Math.min(340, 24 + 6.6 * longest), right = 70, rowH = 24, top = 8;
    const H = top + itens.length * rowH + 40;
    const sx = linear(-1, 1, left, W - right);
    let b = xAxis(sx, [-1, -0.5, 0, 0.5, 1], top, top + itens.length * rowH, o.rotuloX || "Correlação de postos com o tempo total", "");
    itens.forEach((it, k) => {
      const y = top + k * rowH;
      const a = sx(Math.min(0, it.valor)), z = sx(Math.max(0, it.valor));
      b += text(left - 8, y + rowH / 2 + 4, it.rotulo, "label", "end");
      b += `<rect class="${it.valor >= 0 ? "bar" : "bar hot"}" x="${a.toFixed(1)}" y="${y + 4}" width="${Math.max(1, z - a).toFixed(1)}" height="${rowH - 8}"><title>${esc(it.rotulo)}: ${fmt(it.valor, 3)}${it.extra ? "; " + esc(it.extra) : ""}</title></rect>`;
      b += text(W - right + 6, y + rowH / 2 + 4, fmt(it.valor, 2), "value");
    });
    return wrap(W, H, o.titulo || "Tornado", b);
  }

  /* Style of a chart downloaded alone (the page's CSS is not inside the file). */
  const SVG_CSS =
    "text{font:11px system-ui,sans-serif;fill:#1c1f24}.tick{fill:#5d6672}.axis,.whisker,.median,.ci{stroke:#1c1f24;fill:none}" +
    ".axis{stroke:#5d6672}.grid{stroke:#e1e5ea}.bar{fill:#2563eb;opacity:.85}.bar.hot{fill:#c2410c}.line{fill:none;stroke:#2563eb;stroke-width:2}" +
    ".band{fill:#2563eb;opacity:.28}.band2{fill:#2563eb;opacity:.12}.box{fill:#bcd0f7;stroke:#1c1f24}.dot{fill:#1c1f24;opacity:.6}" +
    ".mean{fill:#c2410c}.goal{stroke:#c2410c;stroke-dasharray:4 3;fill:none}.spark{fill:#2563eb}";

  return { esc, fmt, fin, barrasIC, linhaBanda, histograma, gantt, caixa, empilhada, pareto, minigrafico, intervalos, matriz, mapaCalor, corCalor, tornado, SVG_CSS, ticks };
})();
if (typeof module !== "undefined") module.exports = UT;
