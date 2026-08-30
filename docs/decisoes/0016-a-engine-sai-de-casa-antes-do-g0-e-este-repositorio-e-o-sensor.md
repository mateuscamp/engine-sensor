# ADR 0016 - A engine sai de casa antes do G0, e este repositório é o sensor

**Status:** Aceita
**Data:** 29 de agosto de 2026
**Decisor:** proprietário do Sara
**Escopo:** empacotamento, fronteira de produto e sede das decisões. **Não altera o
verificador**: nenhuma regra, nenhum adapter, nenhum código de diagnóstico muda por
causa desta ADR.

## Contexto

O pré-projeto de uma possível engine foi escrito dentro deste repositório, na branch
`licao-da-aranha`. Quatro rodadas de revisão externa depois, a convivência produziu um
defeito que nenhum dos dois revisores viu até a última passagem.

### O defeito, e a forma dele

Todo o pré-projeto, as três respostas do Codex, as três revisões e o inventário do G0
foram escritos e conferidos contra uma árvore que **não continha o acervo inteiro**. A
branch estava rebaseada e atrasada; uma ADR aceita, a fitness function dela e um eixo
inteiro do verificador existiam só no `main`.

É a forma exata que este projeto existe para nomear: **verde por construção, porque a
pergunta foi feita a um recorte que não continha a resposta.** O mesmo defeito do
[caso da aranha](../CASO-DA-ARANHA.md), dois andares acima — não num carimbo, nem no
registro de decisões, mas na árvore que a auditoria leu.

### A medição, refeita sobre o acervo completo em 29/08/2026

A revisão externa da passagem 4 — hoje no repositório da engine,
`docs/engine/REVISAO-EXTERNA-PASSAGEM-4.md` — mediu o defeito contra o `main` **local**, que estava ele próprio dezenove commits atrás do
`origin/main`. Refeita a medição contra o remoto, o número muda de grau e não de tipo:

| Árvore | Revisão | Testes definidos | Passam | Ignorados |
|---|---|---:|---:|---:|
| `licao-da-aranha` (a árvore auditada) | `d8afce8` | 39 | 38 | 1 |
| `main` local (o que a passagem 4 chamou de "real") | `174da11` | 41 | 40 | 1 |
| **`origin/main` (o acervo)** | **`819b64e`** | **42** | **41** | **1** |

A distância auditada era de onze commits. **A distância real era de trinta.**

Os três testes ausentes da branch têm nome, e não são órfãos:

- `adr_0012_o_binario_publicado_responde_como_o_codigo`
- `adr_0015_as_sete_fitness_functions_da_0004_continuam_sete`
- `godot_inventories_who_decides_draw_order`

A passagem 4 os encontrou num `target/` que classificou como obsoleto e contaminado, por
não corresponder nem à branch nem ao `main` que ela conhecia. Não havia terceiro estado:
**aquele `target/` era uma compilação do `main` verdadeiro**, e a contagem de 42 que ele
devolveu era a contagem certa. O que estava desalinhado eram as duas árvores de leitura,
não o artefato de build.

Os oito commits próprios da branch são reescritas com hash diferente dos mesmos commits
do `main`: `git diff` entre cada par devolve vazio. **A branch não continha uma linha que
o `main` já não tivesse.** Ela custou quatro rodadas de auditoria e não carregava conteúdo.

### O que a convivência já cobrou

Uma discussão foi aberta no backlog da engine para decidir o que a ADR 0015 já decidiu;
um rascunho de ADR foi numerado 0015 sobre um número ocupado; o inventário do G0 disse
catorze ADRs quando são quinze; e três contagens de teste conviveram na mesma máquina.

Nenhum desses erros é sobre a engine ou sobre o verificador. Todos são sobre **dois
produtos compartilhando uma árvore, uma série de decisões e um portão**.

O desenho já reconhecia a fronteira sem consumá-la: o verificador fora do workspace da
engine, com lockfile, target e comandos próprios; governança por manifesto de produto;
prefixo `SARE-*` porque `SAR-*` já é daqui; portão do corpus com dono separado. São
quatro contornos para a mesma fronteira, e cada um custa vigilância permanente.

## Opções consideradas

1. **Não separar.** Um repositório, com o workspace `engine/` que o pré-projeto desenha.
2. **Separar antes do G0.**
3. **Separar no portão do G0**, como primeiro ato depois da ratificação.
4. **Separar depois do S1**, quando existir código de engine para mover.

A opção 1 é o estado que produziu o defeito acima, e continua exigindo quatro mecanismos
de vigilância para manter separado o que já é separado.

A opção 4 paga o custo mais alto: mover código escrito, com histórico, lockfile e
artefatos, em vez de nascer no lugar certo.

A opção 3 é a que um rascunho desta ADR escolheu, e a objeção que ele levantou contra a
opção 2 era boa: *"separar antes faz o repositório novo nascer com um inventário vazio e
uma promessa de olhar outro lugar — que é exatamente a forma de lacuna declarada em prosa
que a ADR 0015 existe para impedir."*

## Decisão

Nós adotamos a **opção 2**, e ela já está executada: o pré-projeto saiu deste repositório
em 29/08/2026, e este repositório é o sensor.

### 1. Por que o tempo mudou

A objeção da opção 3 supunha que o custo de separar cedo era o inventário vazio. O custo
de separar tarde é maior e já foi pago uma vez: **a causa do defeito P1 é a árvore
compartilhada, e o G0 é justamente o trabalho mais sensível a ela.** Manter os dois
produtos juntos durante o G0 é manter a causa viva exatamente enquanto se classifica o
acervo que ela corrompeu.

A objeção foi respondida em vez de ignorada: a separação não nasceu com inventário vazio,
nasceu com uma **ponte escrita no mesmo ato** — o documento de herança do acervo, que já
lista o que atravessa como requisito, o que deixou de alcançar e o que ficou aberto. Não
é ainda a matriz do legado do G0; é o inventário de partida que a opção 3 dizia não
existir.

### 2. O que fica de cada lado

**Este repositório** é o sensor. Ele conserva o verificador, o corpus, o kit, os
artefatos distribuídos, os estudos, os artigos e a série de ADRs 0001–0016 inteira. Ele
**não contém `docs/engine/`**, nem a matriz de riscos da engine, nem proposta de ADR da
engine.

**O repositório da engine** conserva a constituição, o plano de primeiro código, o
backlog, os riscos da engine, as quatro revisões externas com suas respostas e a matriz
do legado que o G0 vai produzir. Ele não recebe cópia de ADR, de estudo, de registro do
bakeoff nem do caso da aranha.

### 3. A ponte é uma só, e ela já mostrou o preço

A única ligação entre os dois é a **matriz do legado**, que mora lá e cita este por
caminho e revisão.

O preço que a opção 3 previu é real e apareceu em menos de um dia: o documento de herança
já cita `a3fe810` para `docs/COMPATIBILIDADE.md` — um commit que existe **só na branch
incompleta** — e chama `174da11` de `main`. As duas citações precisam ser reancoradas em
`origin/main` (`819b64e`), e o equivalente de `a3fe810` no acervo é `fae1485`.

Isso não derruba a decisão; qualifica a regra. **Citação de acervo por revisão envelhece,
e citação por revisão de uma árvore incompleta nasce errada.** A regra da ponte passa a
exigir revisão *alcançável a partir de `origin/main`*, e não só revisão.

### 4. Duas séries de decisão, e referência qualificada

Este repositório continua sua série a partir de **0017**, para decisões do sensor. O
repositório da engine abre série própria em 0001.

**A partir daqui, nenhuma referência a ADR usa apenas o número.** Toda citação entre
repositórios nomeia repositório, caminho e revisão. Um número sozinho passa a ser
ambíguo, e ambiguidade em referência de decisão é o defeito que esta ADR corrige.

### 5. O sensor tem dono, cadência e data

A separação não decide congelar nem manter vivo o sensor, mas **obriga a dizer qual dos
dois**. O critério de revisão datado da [ADR 0013](0013-manter-a-sara-privada-ao-fim-do-marco-6.md)
— 20 de setembro de 2026 — continua valendo e passa a valer mais, porque um produto em
outra pasta é mais fácil de abandonar sem admitir.

### 6. O nome

O diretório de trabalho passou a ser `engine-sensor`, e o documento de herança já cita o
sensor por esse nome. **O repositório remoto ainda se chama `sara-engine`** — o nome do
produto que saiu de casa. A [ADR 0003](0003-sara-como-nome-provisorio.md) registrou que o
sufixo do repositório não muda a natureza do produto; com dois produtos, ele passa a
nomear o errado. Renomear o remoto é ato do proprietário e fica pendente.

*(Conferido em 30/08/2026: o remoto **é** `mateuscamp/engine-sensor`, pela API do GitHub.
A pendência está cumprida; em que dia ela foi cumprida, ninguém registrou — só se sabe que
em 29/08 esta seção ainda a descrevia como pendente. O nome antigo continua resolvendo pelo
redirecionamento do GitHub, o que é conveniência de quem já tinha o clone e não o nome do
acervo — e é por isso que a divergência sobreviveu: pelos dois nomes tudo funciona. O
`repository` do `Cargo.toml`, que ainda apontava para o antigo, foi junto.)*

### 7. O que isto não decide

Nada aqui autoriza a engine, escolhe provider, adiciona dependência, resolve o nome do
produto ou altera o verificador.

## Consequências

### Positivas

- Quatro mecanismos de vigilância viram uma fronteira física: lockfile, target, suíte e
  manifesto deixam de poder se contaminar porque não se encontram.
- A colisão de numeração de ADR desaparece por construção.
- O `SARE-*` deixa de ser contorno e passa a ser o prefixo daquele produto.
- A reprodutibilidade do binário distribuído sai do risco de resolução compartilhada de
  dependências.
- O G0 deixa de ser exercício de permissão e passa a ser exercício de herança: a metade
  que buscava autorização desapareceu com a separação.

### Negativas

- **Dois repositórios são dois lugares para um projeto solo esquecer.** Mesma classe do
  risco de nota 9 já registrado em [docs/DIAGNOSTICO-INICIAL.md](../DIAGNOSTICO-INICIAL.md)
  sobre a capacidade de um projeto solo, aplicada a manutenção.
- O sensor sai do caminho, e sair do caminho é como um produto morre sem decisão. O item
  5 existe por isso e pode não bastar.
- A matriz do legado será escrita **fora** desta árvore, sobre evidência que continua
  mudando. O item 3 já registra a primeira citação envelhecida, no primeiro dia.
- Perde-se a conveniência de um `grep` único sobre método e produto.
- Enquanto o remoto não for renomeado, o repositório do sensor é chamado pelo nome da
  engine.

## Conformidade

Fitness functions automáticas, neste repositório:

- `adr_0016_o_sensor_nao_hospeda_o_pre_projeto_da_engine`, em `tests/governanca.rs`:
  `docs/engine/` não existe, nenhuma ADR de `docs/decisoes/` é da engine, e não há
  `docs/RISCOS-ENGINE.md`. Reprova antes de o pré-projeto voltar por descuido de merge.
- A suíte inteira continua passando sem alteração de regra. Medida em build limpa em
  29/08/2026: **42 definidos, 41 passando, 1 ignorado** em `origin/main` (`819b64e`), e
  **43, 42 e 1** depois da fitness function acima. O teste negativo foi exercitado: com
  `docs/engine/` presente, e com uma ADR da engine em `docs/decisoes/`, ele reprova.

**Declarado manual, porque não há como automatizar sem fingir:** que a matriz do legado
descreva o acervo com fidelidade, e que o portão do corpus continue sendo executado por
alguém. A evidência disponível é a série de execuções registradas em
[docs/USO-PESSOAL.md](../USO-PESSOAL.md), e **uma série que para é o sinal**.

Fica registrado como dívida o item 6 da passagem 4, que esta ADR não resolve e que
pertence ao portão do G0, no repositório da engine: *"nenhuma ADR aceita existente fora
da árvore auditada, verificado por execução"*. A conferência que este repositório pode
fazer sozinho — que a árvore de trabalho contém o `origin/main` — não substitui aquela.

## Critério de revisão

- **20 de setembro de 2026**, a data da [ADR 0013](0013-manter-a-sara-privada-ao-fim-do-marco-6.md).
  Se até lá o sensor não tiver recebido capacidade nova nem execução registrada, a
  separação terá acelerado um abandono em vez de organizar dois produtos, e a revisão
  passa a ser entre congelar de verdade e encerrar.
- **Se o repositório da engine passar a citar este fora da matriz do legado**, a ponte
  está vazando.
- **Se uma decisão passar a ser difícil de achar** porque está no outro repositório, a
  série dupla custou mais do que resolveu.
- **Se uma segunda citação envelhecida aparecer na ponte**, o item 3 não bastou e a regra
  precisa de mecanismo, não de redação.

## Notas

- Autor: reconciliação de `licao-da-aranha` com `origin/main`, a pedido do proprietário
- Aprovada por: proprietário do Sara, em 29 de agosto de 2026
- Substitui: nenhuma. Substitui um **rascunho** homônimo que escolhia a opção 3 e supunha
  a separação no portão do G0; o rascunho nunca foi aceito e não está no histórico.
  Complementa a [ADR 0007](0007-observe-como-binario-separado.md), que separou binários
  dentro de um repositório, levando a mesma lógica ao nível de produto; e força a decisão
  de nome pendente na [ADR 0003](0003-sara-como-nome-provisorio.md).
- Última alteração: 29 de agosto de 2026
