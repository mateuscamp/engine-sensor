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

**Desde 24 de agosto de 2026 há um único projeto Godot em desenvolvimento ativo:** o
porte do BomberBoom. Gods, Boomlitude e MineBoom ficam parados como corpus de regressão.
As dez mudanças virão todas do porte, e isso é ao mesmo tempo a força e o risco do marco.

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
