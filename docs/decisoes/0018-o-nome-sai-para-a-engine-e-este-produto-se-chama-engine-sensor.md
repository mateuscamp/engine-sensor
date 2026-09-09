# ADR 0018 - O nome anterior sai para a engine, e este produto se chama engine-sensor

**Status:** Aceita
**Data:** 9 de setembro de 2026
**Decisor:** proprietário do engine-sensor
**Escopo:** identidade do produto — binário, pacote, contrato em disco, identificadores
de regra e variáveis do portão. **Não altera o verificador**: nenhuma regra, nenhum
adapter e nenhum código de diagnóstico muda por causa desta ADR.

## Contexto

A [ADR 0003](0003-nome-provisorio-da-camada.md) batizou esta camada com um nome
provisório e escreveu, no mesmo parágrafo, a condição de saída: *"se o projeto virar
engine, for publicado ou receber marca comercial, nome e marca exigirão uma nova
decisão."* A [ADR 0016](0016-a-engine-sai-de-casa-antes-do-g0-e-este-repositorio-e-o-sensor.md)
tirou o pré-projeto da engine de casa e deixou este repositório sendo o sensor, e só ele.

A condição se cumpriu por um caminho que a 0003 não previu: **não foi este produto que
virou engine — foi outro produto que nasceu com este nome.** O proprietário desenvolve
hoje uma engine chamada Sara, em `/home/mateus/sara`, com série de decisões própria. Dois
produtos com o mesmo nome em dois repositórios não é ambiguidade de marca: é um nome que
deixou de identificar qualquer um dos dois.

### O que a medição de hoje mostrou

O renome do acervo já tinha acontecido — a ADR 0016 §6 registrou a pendência e o README
registrou a conferência contra a API do GitHub em 30/08/2026. Ele parou no nome do
repositório. **O produto dentro dele continuou se chamando pelo nome antigo**, em 85 dos
133 arquivos rastreados, no binário, no contrato que o `init` escreve nos projetos e nos
identificadores de regra.

E o renome nem no acervo estava inteiro. Em 09/09/2026 o `remote.origin.url` deste clone
ainda era `git@github.com:mateuscamp/sara-engine.git`. Como o GitHub mantém o
redirecionamento do nome antigo, `fetch` e `push` funcionaram o tempo todo, nenhum comando
reclamou, e o README afirmava — corretamente sobre o lado do GitHub, falsamente sobre este
clone — que o remoto se chamava `engine-sensor`. **É a forma de defeito que este
repositório já nomeou duas vezes: verde por construção.** Um renome conferido no lugar
onde ele é fácil de conferir, e não conferido onde ele é usado.

## Opções consideradas

1. **Manter o nome nos dois produtos.** Recusada: nome que aponta para duas coisas não
   aponta para nenhuma. O custo aparece primeiro na citação — a matriz do legado da engine
   cita este acervo por `repositório@revisão caminho`, e um repositório ambíguo estraga
   a citação, que é o único elo entre os dois lados.
2. **Renomear só o acervo, e deixar o produto.** Recusada por já ter sido tentada: era
   exatamente o estado de 30/08/2026 a 09/09/2026, e ele produziu um README verdadeiro
   sobre o GitHub e falso sobre o clone.
3. **Renomear tudo, com camada de compatibilidade** — escrever `.engine-sensor/` e
   continuar lendo `.sara/`. Recusada: o `0.1.0` é interno, sem consumidor público, e o
   corpus inteiro vive numa máquina só. Uma migração de quatro projetos custa uma tarde;
   um caminho de legado custa para sempre, e é lido por quem nunca soube por que ele
   existe.
4. **Renomear integralmente, sem alias.** Adotada. É o precedente que a própria
   [ADR 0003](0003-nome-provisorio-da-camada.md) estabeleceu quando o codinome anterior
   saiu: *"o comando, a pasta local, a configuração e os identificadores mudam
   integralmente. Não haverá alias de compatibilidade."*

## Decisão

Nós adotamos a opção 4.

### 1. O antes e o depois, por inteiro

| Antes | Depois |
|---|---|
| binário `sara` | `engine-sensor` |
| binário reservado `sara-observe` ([ADR 0007](0007-observe-como-binario-separado.md)) | `engine-sensor-observe` |
| pacote `sara-ai-first`, crate `sara_ai_first` | `engine-sensor`, `engine_sensor` |
| artefato `dist/sara-linux-x86_64` | `dist/engine-sensor-linux-x86_64` |
| contrato em disco `.sara/` | `.engine-sensor/` |
| configuração `sara.toml` | `engine-sensor.toml` |
| ids de regra `SAR-OWN-001`, `SAR-OWN-002`, `SAR-PARSE-001` | `ESN-OWN-001`, `ESN-OWN-002`, `ESN-PARSE-001` |
| variáveis do portão `SARA_CORPUS_*` (seis) e `SARA_CORPUS_VEREDITO` | `ENGINE_SENSOR_CORPUS_*`, `ENGINE_SENSOR_CORPUS_VEREDITO` |
| veredito escrito pelo portão, `SARA-CORPUS:` | `ENGINE-SENSOR-CORPUS:` |
| tipo `SaraConfig` | `SensorConfig` |
| `docs/decisoes/0003-sara-como-nome-provisorio.md` | `0003-nome-provisorio-da-camada.md` |
| `docs/decisoes/0012-sara-e-corpus-coevoluem.md` | `0012-o-sensor-e-o-corpus-coevoluem.md` |
| `docs/decisoes/0013-manter-a-sara-privada-ao-fim-do-marco-6.md` | `0013-manter-o-sensor-privado-ao-fim-do-marco-6.md` |
| `remote.origin.url` para `mateuscamp/sara-engine` | `mateuscamp/engine-sensor` |

O esquema JSON do relatório **não muda**: nenhuma chave carregava o nome, e por isso
`REPORT_SCHEMA_VERSION` fica onde está. O contrato estrito da
[ADR 0006](0006-contrato-estrito-de-relatorio-e-codigos-de-saida.md) atravessa este renome
intacto, e essa é a diferença entre trocar um nome e quebrar uma interface.

### 2. Os quatro projetos do corpus migram no mesmo dia

`bomberboom-df`, `bomberboom-gd`, `boomlitude` e `gods` têm `.sara/` e `sara.toml` em
disco. Sem migração, o verificador deixa de encontrar o contrato **em silêncio** — cai no
padrão e ignora os blocos `[[allow]]`, que é a pior forma de falhar: continua saindo 0.
`mineboom` nunca foi inicializado e não migra.

A migração acontece no mesmo dia desta decisão, e o portão do corpus é reexecutado depois
dela. Migração conferida por reexecução, não por intenção.

### 3. Citação por id e por caminho quebra, e a quebra fica mapeada

Esta é a consequência que precisa estar escrita porque ela atinge quem está de fora:
**`SAR-OWN-001` deixa de existir, e quem citou esse id citou algo que não resolve mais.**
O mesmo vale para os três nomes de arquivo de ADR. A tabela do item 1 é o mapa do conserto,
e ela é a razão de a tabela existir em vez de uma frase dizendo "renomeamos tudo".

Citação **por revisão** sobrevive intacta: `engine-sensor@eb2908d docs/decisoes/0015-…`
resolve na revisão em que foi fixada, e é assim que a matriz do legado da engine cita este
acervo. A disciplina que a [ADR 0016](0016-a-engine-sai-de-casa-antes-do-g0-e-este-repositorio-e-o-sensor.md)
impôs à fronteira é o que faz este renome não quebrá-la.

### 4. O nome anterior só aparece onde explica esta migração

É a exceção que a ADR 0003 já tinha escrito para o codinome que saiu antes, e ela vale
para este pelo mesmo motivo: um documento que não pode nomear o que mudou não consegue
explicar a mudança. Fora desta ADR, o nome anterior não aparece em código, teste,
documentação, binário ou projeto integrado.

### 5. O relatório de pesquisa da engine sai do sensor

`deep-research-report.md` era o pré-projeto da engine, byte a byte idêntico à cópia que
vive em `/home/mateus/sara`, sem nenhuma referência apontando para ele nesta árvore. Ele
ficou porque a fitness function da ADR 0016 vigiava `docs/engine`, `docs/RISCOS-ENGINE.md`
e a série de decisões, e um arquivo na raiz não estava na lista. Ele sai, e a fitness
function passa a vigiar a raiz também.

Este item não é acessório ao renome: era **o único arquivo desta árvore onde o nome
anterior estava correto**, porque falava do produto que hoje se chama assim.

## Consequências

### Positivas

- Cada nome aponta para um produto. A citação entre os dois repositórios volta a ser
  inequívoca, que é a condição de a fronteira da ADR 0016 funcionar.
- O renome passa a ser conferível onde ele é usado, e não só onde é fácil de conferir: uma
  fitness function reprova a volta do nome antigo, coisa que a fitness function em prosa da
  ADR 0003 nunca conseguiu fazer.
- O contrato em disco dos quatro projetos e o verificador voltam a concordar, e a
  concordância foi medida pelo portão do corpus depois da migração.

### Negativas

- **Citação externa por id de regra ou por caminho de ADR quebra.** O item 3 mapeia; ele
  não desfaz.
- O binário passou de quatro para treze caracteres. Quem digita `engine-sensor check` todo
  dia paga isso, e o preço fica registrado aqui em vez de virar reclamação depois.
- Os registros do estudo — que mediram uma ferramenta que naquele dia tinha outro nome —
  passam a nomeá-la pelo nome de hoje. É a escolha deliberada do proprietário, e o custo é
  que a leitura de um registro histórico não bate mais, ao pé da letra, com a saída que o
  binário daquele dia produzia.
- Um clone antigo, com `.sara/` no projeto, deixa de ser reconhecido sem aviso especial. A
  migração dos quatro projetos do corpus fecha isso na máquina do proprietário e em lugar
  nenhum além dela.

## Conformidade

Uma fitness function automática em `tests/governanca.rs`:

- `adr_0018_o_nome_anterior_nao_volta` varre `src/`, `tests/`, `tools/`, `docs/`,
  `Cargo.toml` e os documentos da raiz, e reprova quando o nome anterior reaparece como
  palavra, como prefixo de id (`SAR-`) ou como nome de variável (`SARA_`). A exceção é esta
  ADR, pelo item 4. Palavras da língua que contêm as quatro letras — *usaram*, *passaram* —
  não contam, e é por isso que o teste confere fronteira de palavra em vez de subcadeia.

E uma alteração na fitness function existente da ADR 0016:

- `adr_0016_o_sensor_nao_hospeda_o_pre_projeto_da_engine` passa a vigiar também a raiz da
  árvore, e não só `docs/`. Foi por ali que `deep-research-report.md` entrou e ficou.

Continua **manual**, como a ADR 0012 §3 já declarava: ler o diff de diagnóstico. Este
renome não muda nenhuma regra, e a reexecução do portão depois da migração é o que mostra
isso em vez de afirmar.

## Critério de revisão

- Se o nome anterior voltar a aparecer em algum lugar que não seja esta ADR, a fitness
  function reprova e a pergunta é por que ele voltou — não como calar o teste.
- Se um terceiro produto aparecer e disputar `engine-sensor`, esta decisão se repete com
  outro nome, e o preço já está medido aqui: uma tarde e quatro projetos.
- Se alguém precisar de um alias de compatibilidade para `.sara/` ou para `SAR-*`, a opção 3
  volta à mesa — e volta com o caso concreto que a justifique, que hoje não existe.

## Notas

- Autor: proprietário do engine-sensor
- Aprovada por: proprietário do engine-sensor
- Substitui: [ADR 0003](0003-nome-provisorio-da-camada.md), na parte que fixa o nome. O
  resto dela continua valendo: o nome segue provisório, segue sem alias de
  compatibilidade, e virar engine, publicar ou receber marca comercial continua exigindo
  nova decisão.
- Última alteração: 9 de setembro de 2026
