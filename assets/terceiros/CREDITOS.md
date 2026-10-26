# Créditos dos ativos de terceiros

Os arquivos desta pasta não foram feitos pelo UTSim. Cada pacote fica em sua pasta, com o
texto ou o endereço da licença em `LICENCA.txt` (e a licença original do autor em
`LICENCA_ORIGINAL.txt`, quando o pacote traz uma). As licenças foram conferidas na página
oficial de cada pacote em 2026-10-25. As medidas, os eixos e os clipes de cada arquivo estão
em `catalogo.ron`, e o teste `crates/utsim-assets/tests/terceiros.rs` confere tudo.

CC0 não exige crédito (damos mesmo assim). **CC-BY 3.0 exige crédito**: ao redistribuir o
UTSim, mantenha a linha da empilhadeira e da paleteira abaixo.

| Pacote (pasta) | Autor | Licença | Endereço | Arquivos | Para que serve no UTSim |
|---|---|---|---|---|---|
| `quaternius_ual` | Quaternius | CC0 1.0 | https://quaternius.itch.io/universal-animation-library | `UAL1_Standard.glb` (7,6 MB) | Pessoa/operador: boneco com 43 clipes (Idle_Loop, Walk_Loop, Jog, Sprint, Push_Loop, Driving_Loop, Interact, PickUp_Table, Sitting) |
| `quaternius_ual2` | Quaternius | CC0 1.0 | https://quaternius.itch.io/universal-animation-library-2 | `UAL2_Standard.glb` (8,1 MB) | Mesmo esqueleto, mais clipes; traz `Walk_Carry_Loop` (andar carregando) |
| `kenney_mini_characters` | Kenney | CC0 1.0 | https://kenney.nl/assets/mini-characters | `character-male-a`, `male-d`, `female-a`, `female-d` | Pessoas leves e estilizadas (idle, walk, sprint, pick-up, holding-both, interact, drive, sit) para cenas com muitas pessoas |
| `kenney_factory_kit` | Kenney | CC0 1.0 | https://kenney.nl/assets/factory-kit | 21 arquivos: máquinas, braços robóticos, ponte rolante, caixas, esteiras (reta, longa, laterais, listra, curva), funis, pistão, alavanca, leitor | Processadores, estações robotizadas, esteiras, itens de fluxo |
| `kenney_conveyor_kit` | Kenney | CC0 1.0 | https://kenney-assets.itch.io/conveyor-kit | 4 esteiras e coberturas | Variantes de esteira (barras, guias, cerca, funil coberto) |
| `polypizza_cc0` | Quaternius, Kenney, MilkAndBanana (via Poly Pizza) | CC0 1.0 | https://poly.pizza (páginas por modelo em `LICENCA.txt`) | palete, carrinho, barril, caixote, duas estantes, estante de contêineres, bancada | Palete, armazenagem, bancada e itens de fluxo |
| `polypizza_ccby` | KolosStudios (via Poly Pizza) | **CC-BY 3.0** | https://poly.pizza/m/DTQBuenKJY e https://poly.pizza/m/snDq7v3GRO ; licença: https://creativecommons.org/licenses/by/3.0/ | `empilhadeira_kolosstudios.glb`, `paleteira_kolosstudios.glb` | Empilhadeira e paleteira (malha única, sem garfos separados) |

Atribuição exigida pela CC-BY 3.0: "Forklift" e "Pallet Truck" por KolosStudios, licença
CC-BY 3.0, via Poly Pizza.

Para regerar ou trocar um arquivo, veja a seção "Ativos de terceiros" em
`docs/plano_3d_realista.md`.
