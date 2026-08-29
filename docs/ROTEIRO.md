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

## O Marco 7 foi cancelado em 28/08/2026

O [ADR 0004](decisoes/0004-spike-de-visao-instrumentada-em-godot.md) autorizava, após o
Marco 6, um único experimento adicional — o spike de visão instrumentada em Godot. Ele
**não vai acontecer**, e o segundo binário `sara-observe` não nasce.

A [ADR 0011](decisoes/0011-marco-7-exige-comparacao-com-ferramenta-existente.md) exigiu uma
comparação antes de começar. A comparação é a
[ADR 0014](decisoes/0014-comparacao-do-marco-7-com-as-ferramentas-existentes.md), e ela
concluiu por cancelar. Em resumo, contra as sete fitness functions da ADR 0004:

- **cinco já estavam entregues** por ferramentas de terceiros — e a v0.12.0 do `godot-agent`,
  de 27/08/2026, traz recibo de captura ligando sessão, cena, estado e hash da saída, que é a
  unidade de evidência da ADR 0004 com mais rigor do que ela pediu;
- **uma falhou quando medida, e falhou em silêncio.** Apontar o nó ou propriedade causal é
  precisamente onde a ferramenta genérica errou no porte: ela leu uma propriedade que o motor
  publica e que **não desenha nada**, e reportou verde;
- **uma sobrevive**, e é a única que produz conhecimento: *imagem mais estado diagnostica
  melhor que imagem isolada?* Ela foi preservada como pergunta e **não precisa de binário**.

E havia um defeito no próprio critério de aceitação, que o [caso da aranha](CASO-DA-ARANHA.md)
expôs: **as sete fitness functions têm todas forma de regressão** — algo que estava certo e
ficou errado. A aranha nunca esteve certa, e o spike, como estava especificado, teria passado
nela. É o mesmo ponto cego que a Sentinela tem por construção, reproduzido dentro de uma
decisão que ainda não fora executada.

Responder essa objeção exige a **verdade de design declarada**, que a
[ADR 0012](decisoes/0012-sara-e-corpus-coevoluem.md) nomeou e deixou sem formato, lugar e
dono. A ADR 0014 registra que ela é pré-condição da pergunta que sobrou, e não o contrário.

A [ADR 0007](decisoes/0007-observe-como-binario-separado.md) continua sendo o freio efetivo:
`sara-observe` não está na lista de binários autorizados, e acrescentá-lo exige uma ADR que
substitua a 0014.

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
