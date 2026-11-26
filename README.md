# UTSim

Este repositório tem o simulador de processos produtivos por eventos discretos: linha de comando (`utsim`), interface 3D no Godot (`utsim-godot` e `godot/`), visualizador, página web, painel e gêmeo digital. Ele faz parte da família UTSim: cada app tem o seu repositório,
e as bibliotecas que os apps compartilham ficam em
[utsim-familia](https://github.com/JandreiSS/utsim-familia).

As versões prontas para instalar ficam na página de
[Releases](https://github.com/JandreiSS/utsim-simulador/releases). São pacotes cifrados: instale-os
pelo UTSim Launcher.

## Onde fica cada coisa

- `crates/`: o código deste repositório (Rust).
- `idiomas/<língua>/*.ftl`: os textos da interface nas cinco línguas (pt-BR, en, es, fr, it).
- `scripts/verificar.sh`: a verificação completa. Rode antes de cada commit.
- O núcleo usado por este app está na família: núcleo da simulação (`utsim-core`, `utsim-model`, `utsim-objects`, `utsim-stats`, `utsim-scene` e outros). Os apps conversam entre si
  (um abre os dados do outro), por isso o que mais de um app usa mora na família.

## Compilar e verificar

Precisa do Rust estável (https://rustup.rs) e do Git.

```bash
cargo build --release          # compila
bash scripts/verificar.sh      # formatação, lints, testes e build; termina com "Tudo certo"
```

Na primeira compilação, o Cargo baixa a família do GitHub, na tag fixada no `Cargo.toml`
(hoje `v0.1.0`). Todos os crates da família usam a mesma tag.

## Mudar o núcleo junto com o app

Quando a mudança precisa mexer na família:

```bash
git clone https://github.com/JandreiSS/utsim-familia ../utsim-familia
bash scripts/familia-local.sh liga         # usa ../utsim-familia em vez da tag
# ... mude os dois, rode os testes dos dois ...
```

Depois:

1. Na família: `bash scripts/verificar.sh`, commit, tag nova (por exemplo `v0.2.0`) e push.
2. Aqui: `bash scripts/familia-local.sh desliga`, depois
   `bash scripts/atualizar-familia.sh v0.2.0`, depois `bash scripts/verificar.sh`, e o commit.

O `verificar.sh` recusa rodar com a família local ligada, para nada ir ao GitHub apontando
para uma pasta do seu computador.

## Regras do projeto

- Não há GitHub Actions: a verificação roda na máquina, com `scripts/verificar.sh`.
- Integração só na branch `main`. Trabalhe numa branch sua e junte em `main` depois de
  verificar.
- Não cite outros softwares de simulação ou de estatística pelo nome (código, comentários,
  docs, commits).
- Textos da interface vêm sempre de `idiomas/` (veja `docs/traducao.md` na família).
- Não acrescente arquivo ou texto de licença.

## Licença

Licença ainda não definida; para reutilizar, fale com o autor.

## Publicar uma versão (autor)

`scripts/publicar-*.sh` gera o pacote cifrado e o manifesto local; depois
`utsim-publicar github` sobe o pacote como release deste repositório e o manifesto assinado
para o repositório do launcher.
