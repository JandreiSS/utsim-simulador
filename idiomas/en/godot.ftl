## UTSim 3D (cliente Godot): configurações e campos de distribuição

barra-configuracoes = Settings
barra-configuracoes-dica = UTSim 3D preferences on this computer
configuracoes-titulo = Settings
configuracoes-distribuicoes = Distribution fields open as
configuracoes-formulario = Form
configuracoes-codigo = Code
configuracoes-distribuicoes-ajuda = Applies to every distribution field (process time, arrivals, loading, unloading...). The {"{"} {"}"} button of each field still switches only that field, until another object is chosen.
configuracoes-fechar = Close
props-editar_como_texto = Edit as code (RON text)
props-editar_como_formulario = Back to the form

## Painéis ao vivo em janela própria

vivo-soltar = Window
vivo-soltar-dica = Opens this panel in a window of its own, which can go to another monitor; it stays live
vivo-prender = Back
vivo-prender-dica = Closes the window and puts the panel back over the 3D view
vivo-janela-titulo = UTSim 3D: { $objeto }

## Idioma

configuracoes-idioma = Language
configuracoes-idioma-ajuda = For now only the Settings window, the live panels and the Quality, Safety and Maintenance tabs of the nodes change language; the other screens stay in Portuguese.

## Curva de fadiga da pessoa

fadiga-secao = Fatigue curve (time study)
fadiga-nota = Read only. The readings and the fit are edited in UTSim Estatística or with the command utsim pessoa ajustar.
fadiga-tarefa = Task { $tarefa }: { $leituras } readings at { $horarios } times
fadiga-tarefa-metodo = Task { $tarefa }, method { $metodo }: { $leituras } readings at { $horarios } times
fadiga-resumo = Rested: { $t0 }. At maximum fatigue, { $acrescimo } % more. Fatigue λ = { $lambda } per hour; recovery μ = { $mu } per hour ({ $origem }).
fadiga-estimada = estimated
fadiga-padrao = default
fadiga-hora = hour of the shift
fadiga-pausa = Break
fadiga-curva = Fitted curve
fadiga-media = Measured mean
fadiga-erro = Not fitted: { $erro }
fadiga-abrir = Open in UTSim Estatística
fadiga-abrir-ajuda = Opens the readings of this time study in a UTSim Estatística spreadsheet (one row per measured time: task, method, hour of the shift and time) for analysis. The model does not change.

## Abas dos nós (Qualidade, Segurança, Manutenção) e painéis ao vivo

aba-so-leitura = { $quem }. Read-only until that person closes the model.
aba-abrir-qualidade = Open in UTSim Quality
aba-abrir-qualidade-dica = Opens this node in UTSim Quality (defects, inspection, costs, SPC). The model is saved first and stays with the other app for editing; UTSim 3D reloads what it saves.
aba-abrir-seguranca = Open in UTSim Safety
aba-abrir-seguranca-dica = Opens this node in UTSim Occupational Safety (hazards, risk matrix, PPE). The model is saved first and stays with the other app for editing; UTSim 3D reloads what it saves.
aba-calculando = Computing the simulation figures...
aba-nenhuma = none
aba-nenhum-escolha = (none)
aba-nenhum = none
aba-opcoes = Separate with commas. Options: { $opcoes }.
aba-vazio-nao-usa = empty: not used
aba-adicionar = + Add
aba-adicionar-dica = Adds an item to { $lista }
aba-abrir-item = Opens the item
aba-fechar-item = Closes the item
aba-sem-nome = (no name)
aba-remover = Remove
aba-remover-dica = Takes this item off the list
vivo-fechar = Close
vivo-fechar-dica = Close the panel (Esc closes the most recent)
vivo-fixar = Pin
vivo-fixado = Pinned
vivo-fixar-dica = Pin keeps this panel; the next click on an object opens another one beside it
vivo-sem-numeros = No figures for this object.
vivo-eixo-tempo = time
vivo-eixo-itens = items
vivo-sem-dados = No data yet.
vivo-estados = States in the window
vivo-sem-estados = no states
