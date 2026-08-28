# Roteiro do lançamento interno 0.1.0

## Resultado pretendido

Um binário Linux chamado `sara` que Codex e Claude Code executam antes de concluir
mudanças em jogos Godot 4.7 ou Defold 1.13. Ele falha apenas quando consegue provar
dois donos para uma propriedade animada ou dois canais físicos distintos chegando ao
mesmo efeito de entrada.

## Marcos e portões

| Marco | Entrega | Fitness function | Decisão ao terminar |
|---|---|---|---|
| 0 | ADRs, escopo, riscos e fixtures | critérios existem antes do código | iniciar ou parar |
| 1 | kit AI-first para agentes | adoção local em até 15 minutos | kit já é útil sozinho? |
| 2 | núcleo Rust e inventário | nenhum arquivo relevante é omitido | parsers sustentam o corpus? |
| 3 | regras Defold | dois defeitos históricos falham e correções passam | Portão 0, teto acumulado de 40 h |
| 4 | regras Godot | sequência e concorrência são distinguidas | escolher engine de foco |
| 5 | binário 0.1.0 | precisão, desempenho, determinismo e offline | lançar internamente |
| 6 | dez usos reais em Godot, **ou 20/09/2026** | utilidade sem falso bloqueio | manter privado, congelar ou propor nova ADR |

O Marco 6 tem critério de conclusão **e** critério de parada. Dez mudanças reais em
projetos Godot o encerram por conclusão; 20 de setembro de 2026 o encerra por data.
Chegar à data com menos de dez mudanças não é atraso: é a evidência de que a ferramenta
não está no caminho do trabalho real, e o encerramento legítimo passa a ser manter só o
kit ou congelar a ferramenta privada.

Pela [ADR 0005](decisoes/0005-foco-em-godot-com-defold-congelado.md), mudança em projeto
Defold não conta para o marco. O BomberBoom Defold permanece no corpus de falso positivo
bloqueante e suas duas regressões históricas continuam sendo o Portão 0.

**Desde 28 de agosto de 2026 a Sara muda durante o marco.** A
[ADR 0012](decisoes/0012-sara-e-corpus-coevoluem.md) reviu a Fase 2: os dez casos deixam
de ser uma série controlada e passam a ser uma série histórica, com a versão usada
declarada em cada um. A medição de utilidade continua, e ganha uma segunda ao lado — se o
que nasce num projeto generaliza para o corpus.

**Desde 24 de agosto de 2026 há um único projeto Godot em desenvolvimento ativo:** o
porte do BomberBoom. Gods, Boomlitude e MineBoom ficam parados como corpus de regressão.
As dez mudanças vieram todas do porte, e isso é ao mesmo tempo a força e o risco do marco.
**Em 28/08/2026 elas somaram 13, a contagem fechou vinte e três dias antes da data, e o
portão decidiu: manter privado**, pela [ADR 0013](decisoes/0013-manter-a-sara-privada-ao-fim-do-marco-6.md).
É o desfecho que a previsão datada de 25/08 antecipou, e as duas condições que ela listou
como capazes de derrubá-la não aconteceram. A evidência julgada está em
[`USO-PESSOAL.md`](USO-PESSOAL.md).

A exigência de "pelo menos dois projetos", que o `RESULTADO-0.1.0.md` carregava desde 23/08,
foi corrigida no mesmo dia: ela era contradição não riscada contra este roteiro, não critério
vivo. O que ela protegia virou **limitação declarada do julgamento** — treze mudanças de um
jogo só não são treze projetos.

A força: o mesmo jogo existe em Defold e em Godot, então toda regra do Sara pode ser
conferida contra o original. Foi assim que a [ADR 0010](decisoes/0010-canal-fisico-de-entrada-sem-mapa-de-acoes.md)
nasceu — o porte reproduziu, em Godot, a regressão de entrada que o original tinha em
Defold, e o Sara não a via porque o eixo de entrada exigia mapa de ações.

O risco: um alvo só. Se o porte parar, o marco para junto e a data decide sozinha. Isso
não muda o critério, muda o que a data significa quando chegar.

## Continuação aprovada, fora da versão 0.1.0

O [ADR 0004](decisoes/0004-spike-de-visao-instrumentada-em-godot.md) autoriza, após
o Marco 6, um único experimento adicional:

| Marco | Entrega | Fitness function | Decisão ao terminar |
|---|---|---|---|
| 7 | spike de visão instrumentada em Godot | agente detecta três regressões e localiza suas causas sem inspeção humana | encerrar, manter como ferramenta privada ou propor incorporação ao Sara |

O Marco 7 combina captura visual, estado da cena, entradas e logs. Ele não inclui
Defold, Android, SDK, runtime nem API pública e não conta como parte do lançamento
`0.1.0`.

### As duas perguntas que o Marco 7 precisa responder antes de começar

A primeira é da [ADR 0011](decisoes/0011-marco-7-exige-comparacao-com-ferramenta-existente.md):
a unidade de evidência que a ADR 0004 manda construir já existe pronta, e o spike não começa
sem a ADR que compara.

**A segunda nasceu do [caso da aranha](CASO-DA-ARANHA.md), em 28/08/2026, e é sobre o
próprio critério de aceitação.** As sete fitness functions da ADR 0004 têm todas a forma de
**regressão** — três regressões visuais injetadas, detectadas, com a causa localizada. Regressão
é algo que estava certo e ficou errado.

A aranha nunca esteve certa. O mecanismo foi construído, passou por 303 casos verdes, pela
Sentinela, pelo portão de cena e pelo `sara check`, e **não funcionava em condição nenhuma**.
Nenhuma das sete fitness functions pergunta se a peça alguma vez existiu, e por isso **o
spike, exatamente como está especificado, teria passado nele.**

É o mesmo ponto cego que a Sentinela tem por construção — referência pega tela que estava
certa e ficou errada — reproduzido dentro de uma decisão que ainda não foi executada. Fica
escrito antes de o spike ser construído, porque depois vira racionalização.

Isto **não altera a ADR 0004**: acrescenta uma pergunta ao momento em que ela for revista
pela ADR 0011. Quem escrever a ADR de comparação responde as duas juntas.

Pela [ADR 0007](decisoes/0007-observe-como-binario-separado.md), `observe` nasce como um
segundo binário, `sara-observe`, e não como subcomando de `sara`. O binário `sara`
continua sendo um quantum único, offline, sem exigir Godot instalado; essa propriedade
está medida e não pode degradar por causa de um experimento.

## O que fica adiado

Publicação, código aberto, itch.io, preço, licença, marca Compositando, plugin de
editor, SARIF, daemon, SDK, protocolo público de consulta, perfil executável de
plataforma, runtime e engine própria. O protocolo provisório e local necessário ao
spike Godot é a única exceção autorizada.

## Foco de engine

Resolvido pela [ADR 0005](decisoes/0005-foco-em-godot-com-defold-congelado.md): **Godot
é o foco declarado** e o adapter Defold está congelado como corpus de regressão.

A matriz ponderada que ocupava esta seção foi retirada. Ela fixava pesos depois de o
estudo ter sido lido, o que o próprio protocolo em [`RESULTADOS.md`](../RESULTADOS.md)
proíbe, e transformava em aritmética uma comparação que não suporta aritmética. O que
fazia o trabalho de decisão eram os vetos, e eles continuam valendo integralmente:

- falha funcional no aparelho;
- toque incorreto no aparelho;
- prova de cena que dependa de uma pessoa olhar.

A medição Android continua sendo a evidência comparável que falta e mantém poder de
veto sobre qualquer conclusão de foco.
