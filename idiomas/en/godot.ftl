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
configuracoes-idioma-ajuda = For now only the Settings window and the live panels change language; the other screens stay in Portuguese.

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
