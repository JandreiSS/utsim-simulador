# Pacote da linha

Para quem cria uma linha nova (a próxima prevista é serviços e atendimento). Uma linha é uma
pasta `linhas/<linha>/` com dois arquivos: a biblioteca e o vocabulário.

```ron
Library(
    line: "lean_office",          // identificador, o valor de Model.line
    name: "Lean Office",          // nome exibido
    description: "...",
    models: [(file: "examples/aprovacao_de_compras.ron", name: "Aprovação de compras")],
    presets: [
        (
            id: "posto",          // a predefinição grava preset: "lean_office/posto"
            name: "Posto de trabalho",
            icon: "mesa_computador",
            role: Work,           // Entry, Inbox, Work, Approval, Check, Transit, WaitInfo,
                                  // Batch, Meeting, Team, Person ou Done
            name_base: "posto",
            fields: [(key: "tempo", label: "Tempo de trabalho", default: "Exp(mean: 20.0)")],
            objects: [r#"(name: "${name}", kind: Processor(process: ${tempo}))"#],
            connections: [],
        ),
    ],
)
```

Cada objeto de uma predefinição é o texto RON de um objeto comum com marcadores `${chave}`:
`${name}` é o nome escolhido; as outras chaves são os campos, com o valor padrão em RON. O
primeiro objeto se chama `${name}` e os outros começam por ele. O UTSim confere o pacote ao
carregar: todo marcador tem campo, todo campo é usado, e cada predefinição monta objetos
válidos com os valores padrão. Os modelos listados em `models` precisam ser da linha, ter o
mesmo nome da lista e ter `preset` e `department` em todos os objetos.

O vocabulário é `Vocabulary(line: "...", terms: {"item": "caso", ...})`: cada chave é um termo
geral do UTSim (um nome de objeto do núcleo, uma medida ou um estado) e o valor é o termo da
linha. A interface e o relatório da linha usam esses termos; o núcleo e o `.ron` não mudam de
nome. Os capítulos de uma linha no manual também escrevem com o vocabulário: um teste
automático confere que os termos gerais não sobram no texto (fora dos trechos de código) e que
os links entre os capítulos levam a arquivos e seções que existem.
