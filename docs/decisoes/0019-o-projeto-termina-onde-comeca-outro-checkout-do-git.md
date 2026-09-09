# ADR 0019 - O projeto termina onde começa outro checkout do Git

**Status:** Aceita
**Data:** 9 de setembro de 2026
**Decisor:** proprietário do engine-sensor
**Escopo:** a descoberta de fontes — o que `engine-sensor check` conta como arquivo do projeto
pedido. **Não altera o verificador**: nenhuma regra, nenhum adapter, nenhum código de
diagnóstico e nenhum campo do relatório muda por causa desta ADR.

## Contexto

Em 09/09/2026, `engine-sensor check . --format json` no BomberBoom devolveu 143 arquivos e 48
avisos. Os 48 vinham de **quatro cópias do próprio projeto** em `.claude/worktrees/`: doze
achados, repetidos nas quatro. Nenhum apontava para código que alguém edita por aquele
caminho.

O scanner descia da raiz pedida até o fim, tirando uma lista fechada de **nomes** —
`.git`, `.godot`, `.engine-sensor`, `build`, `target` e mais alguns. Lista de nome não
consegue dizer *aqui começa outra árvore*, porque o nome da outra árvore é arbitrário: as
quatro cópias do BomberBoom se chamam `desenhista-branch-capacidades-461806`,
`extension-automation-bridge-1be538`, `goofy-tesla-ac2353` e `zen-wu-93a512`.

### O que o defeito custou, que não é o número

Diagnóstico a mais é visível, e por isso parece o defeito barato. Não foi.

O `engine-sensor` só tem uma saída para aviso que o projeto provou ser composição
deliberada: a exceção exata em `engine-sensor.toml`, por recurso. Exceção é por recurso
justamente para não virar baseline. O BomberBoom resolveu os oito avisos **dele** com oito
`[[allow]]`, cada um com prova em runtime, e escreveu no arquivo a frase que fechou o
caminho: *"Não abrange worktrees"*. Os outros 48 não têm resposta possível — não são dele,
e nem por isso podem ser excetuados um a um.

A ferramenta que existe para o agente não precisar adivinhar estava **ela mesma
adivinhando** o que é o projeto, e a conta caiu num projeto do corpus, no meio de uma
tarefa.

### Por que a fronteira é do Git, e não do caminho

O `.claude/worktrees/` é convenção de uma ferramenta, e ela não cobre o caso. O `gods`
tem `arte/` — outro repositório, no meio do projeto, fora de qualquer convenção de nome. E
o problema não é de nome: é que **existe outro checkout ali**.

O Git assinala a raiz de um checkout, e só a raiz, com um `.git`: diretório num
repositório comum, arquivo com `gitdir: ...` numa worktree vinculada e num submódulo. É a
única marca que o Git põe, é ele quem a põe, e ela está exatamente onde a fronteira está.

### As referências quebradas, que decidem o critério

Resolver o `gitdir` seria o reflexo, e é ele que erra aqui. **Três das quatro** cópias do
BomberBoom apontam para `/home/mateus/defold/bomberboom/`, que não existe desde o renome
do projeto; no `bomberboom-gd` são **sete das onze**, apontando para `~/Godot/`. Um
critério que exige o alvo existir deixa de fora justamente a maioria dos casos reais —
cópia velha é o estado normal de worktree de agente.

Perguntar ao binário do `git` (`git worktree list`) tem o mesmo furo, porque as cópias
velhas aparecem como `prunable` mas com o caminho antigo, e traz um custo próprio: um
processo externo dentro de uma ferramenta que é offline por decisão da
[ADR 0001](0001-validar-mecanismos-antes-da-engine-completa.md) e cuja medição de
desempenho é feita sem ele.

## Opções consideradas

1. **Não fazer, e pedir que o projeto conviva.** Recusada: não há como. Não existe opção
   de exclusão no CLI, e a única saída que existe — a exceção exata — é por recurso e não
   deve virar baseline. Foi o que o BomberBoom encontrou.
2. **Ignorar `.claude/worktrees/` por nome.** Recusada: é a mesma lista de nomes que
   falhou, com um nome a mais. Não cobre o `arte/` do `gods`, nem worktree criada fora
   daquela convenção, e amarra o verificador à convenção de uma ferramenta de terceiro.
3. **Ignorar todo diretório oculto.** Recusada: derruba fonte legítima por uma
   característica que não tem relação nenhuma com o defeito. `.claude/ferramentas/` com
   um `.script` dentro é fonte do projeto, e ficaria invisível.
4. **Resolver o `gitdir`, ou perguntar ao binário do `git`.** Recusada pelo argumento
   acima: deixa de fora a maioria das cópias reais e traz processo externo.
5. **Marca do Git pela presença, com o submódulo declarado como exceção.** Adotada.

## Decisão

Nós adotamos a opção 5.

### 1. A raiz pedida é sempre o projeto

O filtro isenta a profundidade 0. Uma worktree pedida como raiz é analisada por inteiro,
com os caminhos relativos a ela — e é dentro de worktree que o agente trabalha, então essa
metade não é detalhe: é o uso principal.

### 2. Abaixo da raiz, a marca do Git delimita

Encontrar `.git` num diretório abaixo da raiz pedida quer dizer que ali começa outro
checkout, e outro checkout não é fonte do projeto pedido. Vale para worktree vinculada,
para submódulo não declarado e para clone solto, nas duas formas da marca.

### 3. É a presença da marca, não o que ela aponta

Nada é resolvido, aberto ou consultado. Cópia cujo `gitdir` não existe mais continua sendo
outro checkout — e era essa a maioria dos casos medidos.

### 4. Submódulo declarado continua sendo fonte

Caminho listado em `.gitmodules` na raiz é conteúdo que o **próprio projeto reivindica**, e
a engine o carrega junto. A declaração é o que separa o submódulo da worktree, e ela vive
na raiz, onde continua legível mesmo quando o `gitdir` da cópia não existe.

Sem este item a correção trocaria um defeito por outro: pararia de contar quatro vezes o
mesmo achado e passaria a não contar nenhuma vez o código que a engine carrega junto com o
projeto. Nenhum dos cinco projetos do corpus tem submódulo hoje; o item entra pela
[ADR 0006](0006-contrato-estrito-de-relatorio-e-codigos-de-saida.md) e pelo que a saída 2
promete, e não por um caso medido.

### 5. Nada é suprimido, e os doze continuam alcançáveis

O que sai não é diagnóstico escondido: é fonte de **outro projeto**. Pedir aquela worktree
como raiz devolve os mesmos doze achados, com os caminhos relativos a ela. Medido nas duas
cópias do BomberBoom, a de metadado válido e a de referência quebrada: 23 arquivos, 12
diagnósticos, nenhum caminho fora da raiz pedida.

### 6. O contrato do relatório não muda, e o pulo não é declarado

`REPORT_SCHEMA_VERSION` fica em 1. `files_scanned` é valor dentro da forma existente, e a
ADR 0006 diz que valor não sobe versão.

Declarar as raízes puladas seria campo novo, e campo novo sobe a versão pela mesma ADR —
com o contrato que o `init` escreve a migrar em cada projeto integrado. **Não fazemos isso
agora**, e a consequência está registrada abaixo como negativa, não como omissão: o pulo é
silencioso, e quem quiser conferir o conta por fora.

## Consequências

### Positivas

- O BomberBoom volta a ter um relatório sobre o BomberBoom: 143 arquivos e 48 avisos viram
  51 e 0, sem que nenhum diagnóstico novo entre. Os oito avisos do projeto principal
  continuam sendo produzidos — medido numa cópia de trabalho com as exceções neutralizadas.
- O critério é do Git e não da árvore de ninguém: cobre `.claude/worktrees/`, o `arte/` do
  `gods` e qualquer clone solto, sem lista para manter.
- Cobre igualmente os dois estados de metadado, que é onde os critérios óbvios falham.
- No corpus, a leitura cai de 3.951 arquivos para 819, e uma enumeração independente do
  sistema de arquivos bate com `files_scanned` nos cinco projetos.
- Diretório oculto que não seja checkout continua lido, e há teste que reprova se parar.

### Negativas

- **O pulo é silencioso.** `files_scanned` cai e o relatório não diz por quê. Quem tiver um
  clone solto com fonte real dentro do projeto perde cobertura sem aviso — e a saída 2
  promete provar que o projeto inteiro foi analisado. É o item 6, e é a dívida desta ADR.
- **A fronteira é uma convenção do Git.** Projeto que guarde fonte de produção dentro de
  uma worktree aninhada deixa de ser lido por inteiro. Nenhum dos cinco faz isso, e a
  saída, se aparecer, é declarar o caminho como submódulo.
- Um `.git` de qualquer natureza — inclusive link simbólico quebrado — encerra a varredura
  naquele ramo. É deliberado: alguma coisa pôs uma marca de checkout ali.
- Custa um `symlink_metadata` por diretório visitado. No `gods`, o projeto mais fundo do
  corpus, a suíte não mudou de tempo mensurável.

## Conformidade

Três fitness functions em `tests/cli.rs`. A primeira reprovava antes da correção, com
`leu 5 arquivo(s); esperado 2`:

- `nested_checkouts_are_not_sources_of_the_requested_project` monta a árvore com os três
  casos — worktree de metadado válido, cópia de `gitdir` quebrado e clone solto — mais um
  diretório oculto que **não** é checkout, e exige que só o projeto e o diretório oculto
  sejam lidos;
- `a_worktree_is_analyzed_when_it_is_the_requested_root` guarda o item 1, nos dois estados
  de metadado;
- `declared_submodule_stays_project_content` guarda o item 4.

`adr_0012_o_binario_publicado_responde_como_o_codigo` passou a montar também um projeto com
checkout aninhado. A razão é específica: **a marca do Git não pode virar fixture
versionada**, porque o Git recusa indexar caminho que contenha um componente `.git`. Sem
essa montagem em tempo de execução, o portão do artefato ficava cego exatamente para a
capacidade que esta ADR introduz — e foi ele que reprovou o `dist/` atrasado aqui.

O critério está publicado em [`docs/COMPATIBILIDADE.md`](../COMPATIBILIDADE.md), na seção
*O que conta como fonte do projeto*, que é o que um consumidor lê.

## Critério de revisão

Revisar quando ocorrer o primeiro destes eventos:

- **um caso perdido pelo silêncio do item 6.** Se alguém descobrir tarde que uma fonte não
  foi lida, a dívida cobrou, e a revisão é do item 6 — declarar as raízes puladas, subindo
  `REPORT_SCHEMA_VERSION` para 2 pela ADR 0006.
- **um projeto do corpus passar a ter submódulo de verdade.** O item 4 entrou sem caso
  medido, e a ADR 0012 §3 exige confronto com os cinco antes de incorporar capacidade.
- **alguém precisar analisar as worktrees a partir da raiz.** Hoje a resposta é pedir cada
  uma como raiz. Se isso virar rotina, a pergunta é sobre uma opção de CLI, e opção de CLI
  é contrato.

## Notas

- Autor: proprietário do engine-sensor
- Aprovada por: proprietário do engine-sensor
- Substitui: nenhuma. Delimita o alcance da saída 2 definida pela
  [ADR 0006](0006-contrato-estrito-de-relatorio-e-codigos-de-saida.md), que continua
  valendo integralmente.
- Última alteração: 9 de setembro de 2026
