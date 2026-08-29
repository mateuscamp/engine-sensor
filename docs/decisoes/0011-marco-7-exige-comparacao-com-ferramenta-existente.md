# ADR 0011 - O Marco 7 não começa sem comparação com ferramenta existente

**Status:** **Cumprida** — a ADR de comparação que ela exigiu é a
[0014](0014-comparacao-do-marco-7-com-as-ferramentas-existentes.md), de 28/08/2026, e ela
concluiu por cancelar o spike. Este freio custou uma ADR e nenhuma hora de implementação.
**Data:** 26 de agosto de 2026
**Decisor:** proprietário do Sara
**Escopo:** pré-condição do Marco 7; restringe a [ADR 0004](0004-spike-de-visao-instrumentada-em-godot.md)

## Contexto

A ADR 0004 autoriza, após o Marco 6, um spike de visão instrumentada em Godot. Ela fixa
a unidade de evidência do experimento:

```text
imagem + estado semântico + sequência de entradas + instante + logs
```

Em 26 de agosto de 2026 apareceu evidência externa de que essa unidade já existe
construída, e por terceiros com mais braço que este projeto.

**A Defold publicou a `extension-automation-bridge`**, extensão nativa oficial descrita
como *"debug-only native extension for inspecting and controlling a running Defold game
from local automation clients such as Codex"*. Ela expõe consultas de cena e de
elementos, entrada FIFO com recibo, capturas de tela e gravação de vídeo, e um canal Lua
para eventos, estado, comandos, anotações semânticas e confirmação de entradas. É a
lista da ADR 0004, item por item.

O balanço de meio de ano da Defold declara a intenção com o mesmo vocabulário que o
`RESULTADOS.md` deste projeto elegeu como eixo decisivo: ajudar quem incorpora agentes
ao fluxo de trabalho a **fechar o ciclo**.

**Em Godot, que é a engine do spike, existem pelo menos quatro implementações
comunitárias.** O `godot-agent` tem 446 commits e dois modos, um deles dirigindo o jogo
em execução para inspecionar estado, simular entrada e capturar tela. O
`satelliteoflove/godot-mcp` se anuncia como olhos e mãos para agentes, com injeção de
entrada, playtesting determinístico e estado vivo. Há ainda um com mais de trezentas
ferramentas, um com diagnóstico de GDScript e erros de runtime, e um na Asset Library
oficial.

**A ADR 0004 não tem seção de "Opções consideradas".** O modelo do próprio projeto, em
`docs/modelos/ADR.md`, manda incluir "não fazer" e ferramenta existente quando forem
alternativas reais. Das dez ADRs anteriores, três não têm a seção; a 0004 é a única em
que a ferramenta existente era alternativa viva no momento da decisão. Ela autorizou
construir sem perguntar se já existia.

A evidência chegou **antes de o spike começar**, que é o momento mais barato possível.
O Marco 7 está represado atrás do Marco 6 e não consumiu uma hora.

## Opções consideradas

1. **Não fazer nada:** iniciar o Marco 7 como a ADR 0004 o escreveu, quando ele
   começar.
2. **Cancelar o Marco 7** e retirar a autorização do spike.
3. **Substituir o spike pela adoção de uma ferramenta existente**, medindo com ela em
   vez de construir.
4. **Exigir uma comparação escrita antes de iniciar**, e deixar o resultado dela
   escolher entre 1, 2 e 3.

A opção 1 repete o erro que a ausência da seção de opções produziu: decidir construir
sem olhar o que existe. Custaria as horas do spike para descobrir no fim.

A opção 2 joga fora a pergunta boa junto com a redundante. As ferramentas entregam
**capacidade**; nenhuma delas mede se a capacidade ajuda. A sétima fitness function da
ADR 0004 — "imagem mais estado produz diagnóstico mais preciso que imagem isolada em
pelo menos um caso previamente definido" — continua sem resposta pública, e é a única
das sete que produz conhecimento em vez de infraestrutura.

A opção 3 é o mesmo erro invertido: adotar antes de medir. E há uma consequência
arquitetural que precisa ser examinada antes, não depois — a ADR 0007 separou
`sara-observe` em binário próprio justamente para que o quantum offline do `sara` não
herdasse dependência de ambiente. Adotar um servidor MCP acrescenta runtime e, em
alguns casos, processo em segundo plano. Isso é decisão, não detalhe.

## Decisão

Nós exigimos que **o Marco 7 não comece sem uma ADR de comparação**, que confronte o
spike da ADR 0004 contra as ferramentas existentes, item a item contra as sete fitness
functions daquela ADR.

Essa ADR de comparação deverá, no mínimo:

- listar as ferramentas candidatas em Godot com data, versão e maturidade observadas;
- dizer, para cada uma das sete fitness functions da ADR 0004, se a ferramenta já a
  satisfaz, se não satisfaz, ou se a pergunta não se aplica a ela;
- registrar o que a adoção acrescentaria de acoplamento — runtime, processo, rede,
  credencial — e o que isso faz com a ADR 0007;
- concluir explicitamente por construir, adotar ou cancelar.

Esta ADR **não cancela** a ADR 0004 nem autoriza começar o Marco 7. Ela põe uma
pré-condição: enquanto a ADR de comparação não existir, o segundo binário não nasce.

## Consequências

### Positivas

- O erro da ADR 0004 fica corrigido antes de custar qualquer hora de implementação.
- A evidência externa entra no registro com fonte e data, em vez de virar lembrança.
- A pergunta que sobrou é a interessante — se a evidência combinada diagnostica melhor
  que a evidência visual isolada — e ela fica isolada do trabalho de infraestrutura que
  outros já fizeram.

### Negativas

- Uma ADR a mais entre a decisão e o experimento. Se o portão do Marco 6 aprovar
  seguir, o Marco 7 começa mais tarde.
- A comparação envelhece: um ecossistema com quatro implementações em movimento pode
  estar diferente quando o Marco 6 fechar, e a ADR de comparação terá de ser feita com
  os dados daquele momento, não com os deste registro.
- Nada disso resolve o incômodo de fundo, e é honesto dizê-lo: a lacuna que o projeto
  nomeia está sendo fechada por quem faz as engines. Isso valida a tese e reduz o
  espaço do produto ao mesmo tempo.

## Conformidade

Fitness function automática, `adr_0011_observe_exige_adr_de_comparacao` em
`tests/governanca.rs`: se um binário `sara-observe` aparecer no `Cargo.toml` sem que
exista uma ADR de comparação em `docs/decisoes/`, o teste reprova citando esta decisão.

O mecanismo é um **freio**, não escopo novo. A Fase 2 proíbe regra de posse nova durante
o Marco 6; uma fitness function que impede trabalho futuro não é uma regra de posse e não
consome exceção nenhuma.

## Critério de revisão

Se as ferramentas existentes forem descontinuadas, ou se a medição mostrar que nenhuma
entrega a unidade de evidência da ADR 0004 num projeto real, a comparação conclui por
construir e esta ADR terá cumprido o papel sem mudar o destino.

Se o portão do Marco 6 decidir congelar ou encerrar, esta ADR se torna irrelevante junto
com a 0004, e nenhuma das duas precisa de substituição — o Marco 7 simplesmente não
acontece.

## Notas

- Autor: proprietário do Sara
- Aprovada por: proprietário do Sara
- Substitui: nenhuma. Restringe a ADR 0004.
- Fontes consultadas em 26/08/2026: `github.com/defold/extension-automation-bridge`,
  `defold.com/2026/06/30/Defold-H1-2026/`, `github.com/aigengame/godot-agent`,
  `github.com/satelliteoflove/godot-mcp`, `github.com/Fulviuus/defold-mcp`
- Última alteração: 26 de agosto de 2026
