# ADR 0012 - Sara e corpus coevoluem, e a evolução do instrumento é registrada

**Status:** Aceita
**Data:** 28 de agosto de 2026
**Decisor:** proprietário do Sara
**Escopo:** método da Fase 2 e do Marco 6. Revisão obrigada pelo gatilho da
[ADR 0009](0009-baseline-em-projeto-real-expoe-regra-ausente.md)

## Contexto

A Fase 2 do plano da auditoria congela regra nova durante o Marco 6, com uma exceção
para quando um projeto real expuser regra ausente. A ADR 0009 ampliou a exceção para
baselines e escreveu o gatilho: **uma terceira exceção obriga a rever a Fase 2 inteira
em vez de ampliá-la de novo.**

O terceiro caso chegou, e ele não é uma regra. É um **modo de trabalho**: desenvolver a
Sara ao mesmo tempo em que se desenvolvem os testes, porque o domínio é novo e não se
sabe de antemão o que precisa ser medido.

Três evidências deste mês sustentam que o congelamento descreve mal o processo real:

- A ADR 0009 nasceu de um aviso falso no `gods`. A ADR 0010 nasceu de o porte não
  declarar mapa de ações. Nenhuma das duas foi imaginada em mesa.
- A regra da ADR 0010, ao entrar, voltou aos projetos parados e encontrou 4 declarações
  invisíveis no `gods` e 6 no `boomlitude` — **sem produzir um único diagnóstico falso**.
  O que nasceu num projeto se sustentou nos outros.
- O **sentinela**, construído no `bomberboom-gd`, é um aparelho de observação mais
  completo que o spike que a ADR 0004 especifica: relógio fixo, 25 telas com tabela de
  delta por tela, memória por assinatura dos arquivos que desenham, detecção de troca de
  driver, e cinco famílias de afirmação conferidas **injetando o defeito**. Ele não foi
  planejado por nenhuma ADR. Ele nasceu da pressão de um jogo real.

### A objeção que foi derrubada

Este documento registra que a posição do assistente era **manter o congelamento**, com o
argumento "medir uma ferramenta que muda não mede nada". O proprietário a refutou, e a
refutação é registrada porque ela é o conteúdo da decisão:

> Isso seria verdade se estivéssemos tentando fazer um benchmark controlado entre dez
> casos usando exatamente o mesmo instrumento. Mas não acho mais que esse seja o
> experimento. Estamos num domínio novo e desenvolvendo o instrumento ao mesmo tempo em
> que descobrimos o que precisa ser medido. Congelar produziria uma comparação
> metodologicamente limpa, mas artificial: descobriríamos quanto uma versão arbitrária e
> incompleta da Sara ajuda durante dez mudanças, enquanto deliberadamente impediríamos
> que problemas reais dessas mudanças melhorassem a ferramenta.

O argumento vence porque as ADRs 0009 e 0010 são a prova empírica dele. A evolução do
instrumento não é contaminação do experimento; **começa a parecer o próprio experimento.**

## Opções consideradas

1. **Terceira exceção pontual.** Recusada pelo gatilho da própria ADR 0009, e com razão:
   é assim que um freio vira decoração.
2. **Manter o congelamento** e recusar o desenvolvimento em paralelo. Recusada pelo
   argumento acima.
3. **Substituir a medição de utilidade pela de generalização.** Recusada: são duas
   perguntas diferentes e as duas importam. "Uma regra que nasceu num jogo continua
   válida nos outros?" mede generalização. "Usar a Sara durante mudanças reais reduz meu
   trabalho ou melhora minha capacidade de achar problema?" mede utilidade.
4. **Rever o método:** manter as dez mudanças e a medição de utilidade, acrescentar a
   série histórica do instrumento, e exigir confronto com o corpus antes de incorporar.

## Decisão

Nós adotamos a opção 4. A obrigação metodológica **deixa de ser "não altere o
instrumento" e passa a ser "não altere o instrumento sem deixar evidência de por que ele
mudou, o que mudou, e o que aconteceu quando a mudança foi confrontada com o corpus".**

### 1. O Marco 6 não muda de tamanho

Dez mudanças reais em Godot, ou 20 de setembro de 2026. A medição de utilidade
permanece, e permanece sendo o que o portão julga.

### 2. Cada uso registra qual instrumento foi usado

O diário ganha a coluna **Sara**, com a versão ou o commit usado naquele caso. Quando o
caso alterar a Sara, uma nota numerada abaixo da tabela registra três coisas: o que
faltava, o que mudou, e o efeito da mudança no corpus inteiro.

Assim não se finge que os dez casos usaram o mesmo instrumento. **Preserva-se a série
histórica**, e a evolução fica explícita: se o caso três gerar capacidade nova, o caso
quatro usa uma Sara melhor, e isso está escrito.

### 3. Toda capacidade generalizável é confrontada com o corpus antes de entrar

A palavra é **capacidade**, e não regra, de propósito. Hoje é regra estática de posse;
amanhã pode ser observação de runtime, entrada, determinismo, estado visual, ou algo que
ainda não sabemos nomear.

O confronto é contra o **corpus aplicável** — os cinco projetos para regra estática, e o
subconjunto que puder executar para capacidade de runtime. Essa imprecisão é deliberada:
fixar hoje qual corpus serve a uma capacidade que não existe seria inventar.

O diff de diagnóstico vai para o diário. Foi o que já se fez nas ADRs 0009 e 0010, por
hábito; passa a ser obrigação.

### 4. Contrafactual é permitido, e nunca reescreve

Voltar a um caso anterior com a versão nova e perguntar "esta capacidade, se existisse
no caso dois, teria detectado aquilo?" é medição da evolução do instrumento. Ela é
registrada como contrafactual, com data, e **não altera o resultado original do caso**.

### 5. Duas linhas de evidência

- **Longitudinal:** como a Sara se comporta acompanhando trabalho real ao longo dos dez
  casos.
- **Transversal:** se o que nasce num projeto generaliza para o corpus.

O portão do Marco 6 julga as duas, e elas podem discordar. Uma Sara que generaliza bem e
não reduz trabalho é um resultado; o inverso também.

## A verdade de design

Registrado aqui porque apareceu junto com esta decisão e a sustenta, sem estar
operacionalizado: **o agente precisa de uma verdade de design declarada para ter com o
que comparar.**

Sem ela, um agente que executa conclui "rodou, logo funcionou" — que é a classe de
defeito que este projeto inteiro existe para nomear. As 25 capturas do sentinela são uma
forma dessa verdade, por referência. As cinco famílias são outra, por afirmação: elas
dizem o que **tem** de ser verdade, sem precisar de referência.

A distinção entre as duas formas importa e está medida no `bomberboom-gd`: referência
pega tela que estava certa e ficou errada; afirmação pega tela que nasceu errada. Nenhuma
das duas pega o que ninguém pensou em afirmar, e nenhuma julga estética.

Esta ADR **não decide** o formato, o lugar nem o dono da verdade de design. Registra que
ela é a peça que faltava nomear, e que decidir sobre ela exige decisão própria.

*A decisão própria é a [ADR 0015](0015-a-verdade-de-design-sao-tres-campos-no-carimbo.md),
de 28/08/2026, escrita depois do [caso da aranha](../CASO-DA-ARANHA.md). Este parágrafo fica
como está: ele continua descrevendo o que esta ADR decidiu e o que não decidiu.*

## Consequências

### Positivas

- O método passa a descrever o processo que já estava acontecendo, em vez de proibi-lo.
- As duas perguntas — utilidade e generalização — ficam medidas em vez de confundidas.
- A evolução do instrumento vira auditável: qualquer pessoa consegue reconstruir qual
  Sara respondeu a qual caso.
- O que nasce no jogo tem caminho declarado para subir: dor local, hipótese de
  generalização, confronto com o corpus, incorporação ou recusa.

### Negativas

- **Os dez casos deixam de ser uma série controlada**, e isso é aceito explicitamente em
  vez de escondido. O portão terá de julgar uma série com instrumento variável, o que é
  mais difícil e mais honesto.
- O diário fica mais pesado. Se ninguém preencher, o método falhou.
- **"Capacidade generalizável" é porta mais larga que "regra"**, e escopo entra por porta
  larga. O confronto com o corpus é o que segura; a ADR 0001 continua valendo integralmente
  e nada aqui autoriza editor, renderizador, áudio, física geral, loja ou runtime próprio.
- A Fase 2 perde o freio simples que tinha. O gatilho da ADR 0009 fica gasto; se este
  método também precisar ser revisto, a revisão é da decisão inteira, não de um item.

## Conformidade

Fitness function automática, `adr_0012_diario_declara_a_versao_usada` em
`tests/governanca.rs`: a tabela de usos do `docs/USO-PESSOAL.md` precisa ter a coluna
`Sara`, e toda linha preenchida precisa declarar qual instrumento respondeu àquele caso.
Linha de uso sem versão reprova.

O confronto com o corpus continua sendo `tools/check_corpus.sh` mais o diff de
diagnóstico registrado à mão — não há como um teste provar que a comparação foi *lida*.
Essa parte é conformidade manual, e está declarada como manual em vez de fingida.

*(Acrescentado em 29/08/2026: o `tools/check_corpus.sh` deste parágrafo apontava para um
teste `#[ignore]` com cinco caminhos literais já defasados, e o confronto que este item
exige não acontecia havia dias sem que nada avisasse. A
[ADR 0017](0017-o-portao-do-corpus-roda-sempre-e-ausencia-e-inconclusivo.md) tirou o
`#[ignore]`, parametrizou os cinco caminhos por variável de ambiente e deu à ausência de
corpus um terceiro estado — inconclusivo, código 2 — em vez de deixá-la passar por
aprovação. A parte manual continua sendo esta, e continua sendo manual: ler o diff é de
quem lê.)*

## Critério de revisão

- Se o diário virar papelada que ninguém preenche, o método falhou e o congelamento
  volta.
- Se uma capacidade entrar sem confronto com o corpus, esta decisão foi violada e a
  violação vale mais que o argumento que a produziu.
- Se o portão do Marco 6 concluir congelar ou encerrar, esta ADR se encerra junto: ela
  governa um marco em curso, não o projeto inteiro.

## Notas

- Autor: proprietário do Sara
- Aprovada por: proprietário do Sara
- Substitui: nenhuma. Revisa a Fase 2 do plano em `AUDITORIA-ARQUITETURAL.md §5` por
  obrigação do gatilho escrito na ADR 0009.
- Última alteração: 28 de agosto de 2026
