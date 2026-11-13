"""Convenções e ferramentas comuns dos elementos 3D gerados no Blender.

Convenções (as mesmas de `crates/utsim-assets`):
- metros, Z para cima, `+x` é a frente do elemento (giro 0), `+y` à esquerda,
  origem no piso, no centro da base;
- o exportador do Blender passa para o Y para cima do glTF: (x, y, z) -> (x, z, -y);
- a raiz tem o nome do elemento; cada parte móvel é um nó com o pivô na junta;
- material `pintura` é a pintura principal (a cor do nó a substitui) e `luz` é a
  luz de estado que a interface recolore;
- pontos `ponto_*` são nós vazios: a posição é o lugar e o `+x` local é para onde se
  olha (o operador olha para a máquina; a peça entra andando em `+x`);
- clipes de animação duram 1 s (24 quadros): a interface posiciona o clipe pela fase
  (0 a 1) do estado que a simulação manda. Clipes em laço repetem a cada 1 s de fase.

Tudo é determinístico: a mesma versão do Blender grava os mesmos bytes.
"""

import json
import math
import os

import bmesh
import bpy

FPS = 24
QUADROS = FPS  # um clipe = 1 s

# Estado na simulação -> (clipe, fase no começo do estado, fase no fim, laço). A ponte toca o
# clipe na fase `de + (ate - de) * fase`, com `fase` = (t - início) / (fim - início) do estado;
# num clipe em laço, uma volta inteira cabe no estado (uma peça, um ciclo). Um estado pode ter
# mais de uma linha: vale a primeira cujo clipe o elemento tem. `"levante"` é a fase do clipe
# que só tira a carga do chão (`dados["levante"]`, 1 se o elemento não diz).
ESTADOS = (
    # Posto (máquina, posto manual, robô, célula).
    ("entering", "porta_abrir", 0.0, 1.0, False),
    ("leaving", "porta_abrir", 1.0, 0.0, False),
    ("processing", "ciclo", 0.0, 1.0, True),
    # Estação sem operador (ninguém entra antes do processo): a porta abre para a peça entrar
    # e sair. A ponte usa esta linha no lugar de `processing` quando o elemento a tem.
    ("processing_auto", "ciclo_auto", 0.0, 1.0, False),
    ("setup", "setup", 0.0, 1.0, True),
    ("down", "quebra", 0.0, 1.0, False),
    ("waiting_repair", "quebra", 1.0, 1.0, False),
    ("maintenance", "quebra", 1.0, 1.0, False),
    # Veículo (empilhadeira, AGV): ergue a carga ao carregar, leva erguida, baixa ao descarregar.
    ("loading", "garfo_subir", 0.0, "levante", False),
    ("loading", "mesa_subir", 0.0, "levante", False),
    ("travel_loaded", "garfo_subir", "levante", "levante", False),
    ("travel_loaded", "mesa_subir", "levante", "levante", False),
    ("unloading", "garfo_subir", "levante", 0.0, False),
    ("unloading", "mesa_subir", "levante", 0.0, False),
)

# O operador anda pela fase do estado: de `ponto_porta` a `ponto_operador` ao entrar e
# de volta ao sair (os dois pontos girados com a estação).
CAMINHOS = {
    "entering": ("ponto_porta", "ponto_operador"),
    "leaving": ("ponto_operador", "ponto_porta"),
}

# Onde fica a peça (ou a carga) em cada estado: o ponto que ela segue. O ponto é um nó do
# `.glb` e anda com o pai (a garra, o garfo, a mesa), então a peça vai junto com o clipe.
PECA_POSTO = (
    ("entering", "ponto_entrada"),
    ("setup", "ponto_peca"),
    ("processing", "ponto_peca"),
    ("down", "ponto_peca"),
    ("waiting_repair", "ponto_peca"),
    ("maintenance", "ponto_peca"),
    ("leaving", "ponto_saida"),
    ("blocked", "ponto_saida"),
)
PECA_VEICULO = (
    ("loading", "ponto_carga"),
    ("travel_loaded", "ponto_carga"),
    ("unloading", "ponto_carga"),
)


def estados(clipes, pontos, dados):
    """A tabela `estados` do catálogo: só os estados cujo clipe o elemento tem."""
    def fase(v):
        return dados.get("levante", 1.0) if v == "levante" else v

    out = {}
    for estado, clipe, de, ate, laco in ESTADOS:
        if clipe not in clipes or estado in out:
            continue
        e = {"clipe": clipe, "de": fase(de), "ate": fase(ate), "laco": laco}
        a, b = CAMINHOS.get(estado, (None, None))
        if a in pontos and b in pontos:
            e["operador"] = [a, b]
        out[estado] = e
    return out


def pecas(pontos, proprio=None):
    """A tabela `peca` do catálogo: estado -> trechos `[fase_de, fase_ate, ponto]`, só com os
    pontos que o elemento tem. `proprio` (do elemento) troca a linha de um estado."""
    out = {}
    if "ponto_peca" in pontos:
        for estado, ponto in PECA_POSTO:
            if ponto in pontos:
                out[estado] = [[0.0, 1.0, ponto]]
    if "ponto_carga" in pontos:
        for estado, ponto in PECA_VEICULO:
            out[estado] = [[0.0, 1.0, ponto]]
    out.update(proprio or {})
    return out


def hex_cor(rgb):
    """A cor `0xRRGGBB` como fator do glTF (os mesmos números do `utsim-assets`)."""
    return ((rgb >> 16) & 0xFF) / 255.0, ((rgb >> 8) & 0xFF) / 255.0, (rgb & 0xFF) / 255.0


# Paleta comum: os mesmos tons dos modelos em Rust, para o 3D ficar coerente.
PALETA = {
    "base": (0x2B2E33, 0.4, 0.6),
    "pintura": (0xE6E9ED, 0.2, 0.45),
    "faixa": (0x2B6CB0, 0.2, 0.45),
    "painel": (0x3A3F46, 0.3, 0.5),
    "aco": (0x8A8F96, 0.7, 0.4),
    "aco_escuro": (0x3A3D42, 0.7, 0.45),
    "borracha": (0x2B2D31, 0.0, 0.9),
    "pneu": (0x161616, 0.0, 0.85),
    "amarelo": (0xF5C400, 0.0, 0.6),
    "tampo": (0xA0764A, 0.0, 0.7),
    "laranja": (0xE8730C, 0.1, 0.5),
    "verde": (0x2ECC40, 0.0, 0.5),
    "vermelho": (0xFF4136, 0.0, 0.5),
    "madeira": (0xC8A26B, 0.0, 0.85),
    "papelao": (0xB98A55, 0.0, 0.9),
    "tela": (0x101114, 0.3, 0.3),
}


class Elemento:
    """Um elemento em construção: a cena do Blender, os nós, os pontos e os clipes."""

    def __init__(self, nome, detalhe=0):
        bpy.ops.wm.read_factory_settings(use_empty=True)
        self.cena = bpy.context.scene
        self.cena.render.fps = FPS
        self.nome = nome
        self.detalhe = detalhe
        self.materiais = {}
        self.nos = {}
        self.pivos = {}
        self._bm = {}
        self._mats = {}
        self.pontos = {}
        self.clipes = {}
        self.dados = {}
        self.peca = {}  # trechos próprios da tabela `peca` (ver `pecas`)
        raiz = bpy.data.objects.new(nome, None)
        self.cena.collection.objects.link(raiz)
        self.nos[nome] = raiz
        self.pivos[nome] = (0.0, 0.0, 0.0)

    # ---------------------------------------------------------------- materiais
    def material(self, nome, rgb=None, metal=None, aspereza=None, alfa=1.0, emissao=False):
        if nome in self.materiais:
            return nome
        if rgb is None:
            rgb, metal, aspereza = PALETA[nome]
        m = bpy.data.materials.new(nome)
        m.use_nodes = True
        b = m.node_tree.nodes["Principled BSDF"]
        r, g, bl = hex_cor(rgb)
        b.inputs["Base Color"].default_value = (r, g, bl, 1.0)
        b.inputs["Metallic"].default_value = metal or 0.0
        b.inputs["Roughness"].default_value = 0.5 if aspereza is None else aspereza
        if alfa < 1.0:
            b.inputs["Alpha"].default_value = alfa
            m.surface_render_method = "BLENDED"
        if emissao:
            b.inputs["Emission Color"].default_value = (r, g, bl, 1.0)
            b.inputs["Emission Strength"].default_value = 1.0
        self.materiais[nome] = m
        return nome

    def _mat(self, no, nome):
        if nome not in self.materiais:
            self.material(nome)
        lista = self._mats[no]
        if nome not in lista:
            lista.append(nome)
        return lista.index(nome)

    # ---------------------------------------------------------------- nós
    def no(self, nome, pai=None, pivo=(0.0, 0.0, 0.0)):
        """Nó com malha, pivô em coordenadas do elemento (mundo)."""
        pai = pai or self.nome
        me = bpy.data.meshes.new(nome)
        o = bpy.data.objects.new(nome, me)
        self.cena.collection.objects.link(o)
        o.parent = self.nos[pai]
        pp = self.pivos[pai]
        o.location = (pivo[0] - pp[0], pivo[1] - pp[1], pivo[2] - pp[2])
        self.nos[nome] = o
        self.pivos[nome] = tuple(pivo)
        self._bm[nome] = bmesh.new()
        self._mats[nome] = []
        return nome

    def ponto(self, nome, pos, giro=0.0, pai=None):
        """Ponto nomeado (nó vazio). `giro` em graus em torno de Z: para onde se olha."""
        pai = pai or self.nome
        o = bpy.data.objects.new(nome, None)
        self.cena.collection.objects.link(o)
        o.parent = self.nos[pai]
        pp = self.pivos[pai]
        o.location = (pos[0] - pp[0], pos[1] - pp[1], pos[2] - pp[2])
        o.rotation_euler = (0.0, 0.0, math.radians(giro))
        self.nos[nome] = o
        self.pivos[nome] = tuple(pos)
        self.pontos[nome] = {"pos": [round(v, 4) for v in pos], "giro": round(giro, 3), "pai": pai}
        return nome

    # ---------------------------------------------------------------- formas
    def _local(self, no, p):
        pv = self.pivos[no]
        return (p[0] - pv[0], p[1] - pv[1], p[2] - pv[2])

    def caixa(self, no, mat, a, b, chanfro=None):
        """Caixa entre os cantos `a` e `b` (coordenadas do elemento). No detalhe médio,
        as arestas são chanfradas (montado aqui, sem o bevel do Blender, cuja ordem de
        faces muda entre execuções)."""
        bm = self._bm[no]
        k = self._mat(no, mat)
        lo = self._local(no, a)
        hi = self._local(no, b)
        cen = [(lo[i] + hi[i]) / 2 for i in range(3)]
        h = [abs(hi[i] - lo[i]) / 2 for i in range(3)]
        c = self.detalhe and (0.008 if chanfro is None else chanfro)
        c = min(c or 0.0, min(h) * 0.6)
        faces = []
        if c <= 0.0:
            vs = {}
            for sx in (-1, 1):
                for sy in (-1, 1):
                    for sz in (-1, 1):
                        vs[(sx, sy, sz)] = bm.verts.new((cen[0] + sx * h[0], cen[1] + sy * h[1], cen[2] + sz * h[2]))
            for eixo in range(3):
                for s in (-1, 1):
                    q = []
                    for u, w in ((-1, -1), (1, -1), (1, 1), (-1, 1)):
                        t = [0, 0, 0]
                        t[eixo] = s
                        t[(eixo + 1) % 3] = u
                        t[(eixo + 2) % 3] = w
                        q.append(vs[tuple(t)])
                    faces.append(self._face(bm, q, cen, k))
            return faces
        # Caixa chanfrada: em cada canto, um vértice por face (recuado `c` nas outras).
        vs = {}
        for sx in (-1, 1):
            for sy in (-1, 1):
                for sz in (-1, 1):
                    sgn = (sx, sy, sz)
                    for eixo in range(3):
                        p = [cen[i] + sgn[i] * (h[i] - (0.0 if i == eixo else c)) for i in range(3)]
                        vs[(sgn, eixo)] = bm.verts.new(tuple(p))
        for eixo in range(3):  # faces principais
            for s in (-1, 1):
                q = []
                for u, w in ((-1, -1), (1, -1), (1, 1), (-1, 1)):
                    t = [0, 0, 0]
                    t[eixo] = s
                    t[(eixo + 1) % 3] = u
                    t[(eixo + 2) % 3] = w
                    q.append(vs[(tuple(t), eixo)])
                faces.append(self._face(bm, q, cen, k))
        for eixo in range(3):  # arestas ao longo de `eixo`
            e1, e2 = (eixo + 1) % 3, (eixo + 2) % 3
            for s1 in (-1, 1):
                for s2 in (-1, 1):
                    q = []
                    for se, ef in ((-1, e1), (-1, e2), (1, e2), (1, e1)):
                        t = [0, 0, 0]
                        t[eixo] = se
                        t[e1] = s1
                        t[e2] = s2
                        q.append(vs[(tuple(t), ef)])
                    faces.append(self._face(bm, q, cen, k))
        for sx in (-1, 1):  # cantos
            for sy in (-1, 1):
                for sz in (-1, 1):
                    sgn = (sx, sy, sz)
                    faces.append(self._face(bm, [vs[(sgn, 0)], vs[(sgn, 1)], vs[(sgn, 2)]], cen, k))
        return faces

    @staticmethod
    def _face(bm, vs, centro, k):
        """Face com a normal para fora (o sólido é convexo, em volta de `centro`)."""
        a, b, c = (v.co for v in vs[:3])
        n = _cruz([b[i] - a[i] for i in range(3)], [c[i] - a[i] for i in range(3)])
        m = [sum(v.co[i] for v in vs) / len(vs) - centro[i] for i in range(3)]
        if sum(n[i] * m[i] for i in range(3)) < 0:
            vs = list(reversed(vs))
        f = bm.faces.new(vs)
        f.material_index = k
        f.smooth = False
        return f

    def cilindro(self, no, mat, a, b, raio, lados=None, raio2=None):
        """Cilindro (ou tronco de cone, com `raio2`) do centro `a` ao centro `b`."""
        lados = lados or (16 if self.detalhe else 12)
        raio2 = raio if raio2 is None else raio2
        bm = self._bm[no]
        k = self._mat(no, mat)
        pa = self._local(no, a)
        pb = self._local(no, b)
        eixo = [pb[i] - pa[i] for i in range(3)]
        n = math.sqrt(sum(v * v for v in eixo)) or 1.0
        eixo = [v / n for v in eixo]
        ref = (0.0, 0.0, 1.0) if abs(eixo[2]) < 0.9 else (1.0, 0.0, 0.0)
        u = _cruz(eixo, ref)
        nu = math.sqrt(sum(v * v for v in u))
        u = [v / nu for v in u]
        w = _cruz(eixo, u)
        anel = []
        for c, r in ((pa, raio), (pb, raio2)):
            anel.append(
                [
                    bm.verts.new(
                        tuple(
                            c[i] + r * (math.cos(2 * math.pi * j / lados) * u[i] + math.sin(2 * math.pi * j / lados) * w[i])
                            for i in range(3)
                        )
                    )
                    for j in range(lados)
                ]
            )
        for j in range(lados):
            j2 = (j + 1) % lados
            f = bm.faces.new((anel[0][j], anel[0][j2], anel[1][j2], anel[1][j]))
            f.material_index = k
            f.smooth = True
        for r, inv in ((anel[0], True), (anel[1], False)):
            f = bm.faces.new(list(reversed(r)) if inv else r)
            f.material_index = k
            f.smooth = False

    def barra(self, no, mat, a, b, lado):
        """Barra de seção quadrada `lado` do ponto `a` ao `b` (eixo x, y ou z)."""
        h = lado / 2.0
        lo = [min(a[i], b[i]) - (h if a[i] == b[i] else 0.0) for i in range(3)]
        hi = [max(a[i], b[i]) + (h if a[i] == b[i] else 0.0) for i in range(3)]
        return self.caixa(no, mat, lo, hi)

    def prisma(self, no, mat, perfil, a, b, eixo=1, liso=False):
        """Prisma: o `perfil` (lista de pares) extrudado de `a` a `b` ao longo de `eixo`.
        Com `eixo` 1 (y), os pares são (x, z); com 0 (x), (y, z); com 2 (z), (x, y).
        O perfil pode ser côncavo (um para-lama em arco, por exemplo)."""
        bm = self._bm[no]
        k = self._mat(no, mat)
        i1, i2 = [(1, 2), (0, 2), (0, 1)][eixo]
        aneis = []
        for v in (a, b):
            anel = []
            for u, w in perfil:
                p = [0.0, 0.0, 0.0]
                p[eixo], p[i1], p[i2] = v, u, w
                anel.append(bm.verts.new(self._local(no, p)))
            aneis.append(anel)
        n = len(perfil)
        # Área com sinal do perfil: decide o lado de fora sem depender do Blender.
        area = sum(perfil[j][0] * perfil[(j + 1) % n][1] - perfil[(j + 1) % n][0] * perfil[j][1] for j in range(n))
        # (i1, i2, eixo) é destro quando o eixo é x ou z; com y, é canhoto.
        fora = ((area > 0) == (eixo != 1)) == (b > a)
        faces = []
        for j in range(n):
            j2 = (j + 1) % n
            q = [aneis[0][j], aneis[0][j2], aneis[1][j2], aneis[1][j]]
            f = bm.faces.new(q if fora else list(reversed(q)))
            f.smooth = liso
            faces.append(f)
        faces.append(bm.faces.new(list(reversed(aneis[0])) if fora else aneis[0]))
        faces.append(bm.faces.new(aneis[1] if fora else list(reversed(aneis[1]))))
        for f in faces:
            f.material_index = k
        return faces

    # ---------------------------------------------------------------- animação
    def clipe(self, nome, laco, chaves):
        """Clipe de 1 s. `chaves` = {nó: [(fase, 'location'|'rotation_euler'|'scale', eixo, valor)]}.

        Os valores são relativos ao repouso do nó (somados à posição ou ao giro dele).
        """
        for no, ks in sorted(chaves.items()):
            o = self.nos[no]
            o.animation_data_create()
            act = bpy.data.actions.new(f"{nome}_{no}")
            o.animation_data.action = act
            repouso = {"location": tuple(o.location), "rotation_euler": tuple(o.rotation_euler),
                       "scale": tuple(o.scale)}
            for fase, caminho, eixo, valor in ks:
                getattr(o, caminho)[eixo] = repouso[caminho][eixo] + valor
                o.keyframe_insert(caminho, index=eixo, frame=round(fase * QUADROS))
            for fc in _curvas(act):
                for kp in fc.keyframe_points:
                    kp.interpolation = "LINEAR"
            o.location = repouso["location"]
            o.rotation_euler = repouso["rotation_euler"]
            o.scale = repouso["scale"]
            trilha = o.animation_data.nla_tracks.new()
            trilha.name = nome
            trilha.strips.new(nome, 0, act)
            o.animation_data.action = None
        self.clipes[nome] = {"laco": laco, "nos": sorted(chaves)}

    # ---------------------------------------------------------------- saída
    def _fechar(self):
        for no, bm in self._bm.items():
            o = self.nos[no]
            me = o.data
            # Triângulos feitos aqui, de modo fixo: o exportador triangula polígonos de
            # forma que varia entre execuções, e os bytes precisam ser sempre os mesmos.
            bmesh.ops.triangulate(bm, faces=bm.faces[:], quad_method="FIXED", ngon_method="EAR_CLIP")
            bm.to_mesh(me)
            bm.free()
            for nome in self._mats[no]:
                me.materials.append(self.materiais[nome])
            me.set_sharp_from_angle(angle=math.radians(35))
        self._bm = {}

    def medidas(self):
        """Caixa envolvente (mín, máx) das malhas em repouso, coordenadas do elemento."""
        lo = [1e9] * 3
        hi = [-1e9] * 3
        for no in self._mats:
            o = self.nos[no]
            pv = self.pivos[no]
            for v in o.data.vertices:
                for i in range(3):
                    p = v.co[i] + pv[i]
                    lo[i] = min(lo[i], p)
                    hi[i] = max(hi[i], p)
        return lo, hi

    def triangulos(self):
        return sum(sum(len(p.vertices) - 2 for p in self.nos[no].data.polygons) for no in self._mats)

    def gravar(self, pasta, arquivo):
        self._fechar()
        self.cena.frame_set(0)
        caminho = os.path.join(pasta, arquivo + ".glb")
        bpy.ops.export_scene.gltf(
            filepath=caminho,
            export_format="GLB",
            export_yup=True,
            export_texcoords=False,
            export_tangents=False,
            export_attributes=False,
            export_extras=False,
            export_cameras=False,
            export_lights=False,
            export_animations=True,
            export_animation_mode="NLA_TRACKS",
            export_force_sampling=False,
            export_optimize_animation_size=False,
            export_optimize_animation_keep_anim_object=True,
            export_frame_range=False,
            export_anim_slide_to_zero=False,
            export_bake_animation=False,
            export_def_bones=False,
            export_skins=False,
            export_morph=False,
        )
        lo, hi = self.medidas()
        return {
            "arquivo": arquivo + ".glb",
            "tamanho": [round(hi[i] - lo[i], 3) for i in range(3)],
            "minimo": [round(v, 3) for v in lo],
            "triangulos": self.triangulos(),
            "nos": sorted(n for n in self.nos if n not in self.pontos),
            "pontos": dict(sorted(self.pontos.items())),
            "clipes": dict(sorted(self.clipes.items())),
            "dados": dict(sorted(self.dados.items())),
            "estados": estados(self.clipes, self.pontos, self.dados),
            "peca": pecas(self.pontos, self.peca),
        }


def _cruz(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def _curvas(act):
    """As F-curves de uma ação (Blender 4.4+ guarda por camada e canal)."""
    if hasattr(act, "fcurves") and len(getattr(act, "fcurves", [])):
        return list(act.fcurves)
    out = []
    for camada in getattr(act, "layers", []):
        for faixa in camada.strips:
            for cb in faixa.channelbags:
                out.extend(cb.fcurves)
    return out


def gravar_catalogo(pasta, itens):
    with open(os.path.join(pasta, "catalogo.json"), "w", encoding="utf-8", newline="\n") as f:
        json.dump(itens, f, ensure_ascii=False, indent=1, sort_keys=True)
        f.write("\n")
