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
| 6 | dez usos reais | utilidade sem falso bloqueio | manter privado, congelar ou propor nova ADR |

## Continuação aprovada, fora da versão 0.1.0

O [ADR 0004](decisoes/0004-spike-de-visao-instrumentada-em-godot.md) autoriza, após
o Marco 6, um único experimento adicional:

| Marco | Entrega | Fitness function | Decisão ao terminar |
|---|---|---|---|
| 7 | spike de visão instrumentada em Godot | agente detecta três regressões e localiza suas causas sem inspeção humana | encerrar, manter como ferramenta privada ou propor incorporação ao Sara |

O Marco 7 combina captura visual, estado da cena, entradas e logs. Ele não inclui
Defold, Android, SDK, runtime nem API pública e não conta como parte do lançamento
`0.1.0`.

## O que fica adiado

Publicação, código aberto, itch.io, preço, licença, marca Compositando, plugin de
editor, SARIF, daemon, SDK, protocolo público de consulta, perfil executável de
plataforma, runtime e engine própria. O protocolo provisório e local necessário ao
spike Godot é a única exceção autorizada.

## Matriz de foco da engine

Cada engine recebe nota de 0 a 5. Uma vantagem de pelo menos 10 pontos ponderados
muda o foco. Em empate, Godot vence por concentrar os projetos atuais. Falha funcional
no aparelho, toque incorreto ou prova de cena dependente de uma pessoa têm veto.

| Critério | Peso | Godot inicial | Defold inicial | Evidência atual |
|---|---:|---:|---:|---|
| ciclo sem atenção humana | 30% | 5 | 2 | Godot captura cena por script; Defold ainda exige janela |
| posse resolvida com certeza | 25% | a medir | a medir | sai dos adapters |
| precisão do diagnóstico | 20% | a medir | a medir | sai do corpus |
| velocidade e estabilidade | 15% | 5 | 2 | portões medidos em 0,6 s e 32 s |
| fidelidade no aparelho | 10% | pendente | pendente | medição Android tem veto |

O placar inicial é hipótese incompleta, não autorização para retirar suporte da outra
engine.
