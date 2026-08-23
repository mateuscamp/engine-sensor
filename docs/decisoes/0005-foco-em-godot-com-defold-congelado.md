# ADR 0005 - Foco de desenvolvimento em Godot, com Defold congelado

**Status:** Aceita
**Data:** 23 de agosto de 2026
**Decisor:** proprietário do Sara
**Escopo:** alocação de esforço entre os dois adapters

## Contexto

A auditoria arquitetural registrada em
[`AUDITORIA-ARQUITETURAL.md`](../AUDITORIA-ARQUITETURAL.md) mostrou que o núcleo não
era agnóstico de engine como o `RESULTADO-0.1.0.md` afirmava. A pergunta imediata foi
se valia manter duas engines.

Duas evidências puxam para lados opostos:

- Godot concentra quatro dos cinco projetos do corpus, fecha mais do ciclo de cena sem
  atenção humana e é a única engine do spike autorizado pela ADR 0004.
- **Todos os verdadeiros positivos medidos do Sara são Defold.** Os 667 arquivos Godot
  do corpus produziram zero erro e zero aviso. As duas regressões históricas que
  definem o Portão 0 — `defold_animation_red` e `defold_input_red` — são Defold, e são
  a única prova causal de que o verificador detecta alguma coisa em código real.

Remover o Defold agora deixaria o `0.1.0` sem teste causal até que regressões reais
novas aparecessem em Godot. A ADR 0002 já registrava exatamente essa assimetria, e a
matriz do roteiro já avisava que o placar inicial "não é autorização para retirar
suporte da outra engine".

## Opções consideradas

1. Remover o adapter Defold, as fixtures e o projeto Defold do corpus.
2. Manter as duas engines em desenvolvimento paralelo, como no `0.1.0`.
3. Congelar o Defold como corpus de regressão e concentrar todo o esforço novo em
   Godot.

## Decisão

Nós adotaremos a opção 3.

- O adapter Defold **não recebe regra nova, fixture nova nem calibração nova**. Ele
  continua compilando, testado e distribuído.
- As quatro fixtures históricas e os dois cenários de CLI que as exercitam permanecem
  no repositório permanentemente. Elas são o Portão 0.
- Todo esforço novo — Marco 6, Marco 7, regras, perfis, diagnóstico — é Godot.
- O projeto BomberBoom Defold sai da contagem de mudanças do Marco 6 e continua no
  corpus de falso positivo bloqueante.
- A matriz de foco do `ROTEIRO.md` deixa de ser um empate a medir: Godot é o foco
  declarado. Os vetos continuam valendo, inclusive o veto da medição Android.

Descongelar o Defold, acrescentar regra a ele ou remover o adapter exige nova ADR.

## Consequências

### Positivas

- superfície de manutenção menor num projeto solo, que é o motor de negócio 4 do
  diagnóstico;
- o esforço vai para onde estão quatro dos cinco projetos e o único spike autorizado;
- a prova causal do Portão 0 é preservada por inteiro;
- a fronteira núcleo/adapter continua exercitada por dois consumidores, o que impede
  que ela apodreça em silêncio até o Marco 7 chegar.

### Negativas

- o adapter Defold envelhece: uma mudança do Defold 1.13 para diante pode quebrá-lo
  sem que ninguém perceba, porque não há uso ativo;
- os 12 avisos úteis já classificados da baseline Defold param de crescer, e com eles
  para a única série que media taxa de aviso falso;
- se o foco voltar ao Defold, o custo de retomada é maior do que teria sido manter.

## Conformidade

`tests/governanca.rs::adr_0005_regressoes_historicas_defold_continuam_no_repositorio`
reprova quando qualquer uma das quatro fixtures históricas ou qualquer um dos dois
cenários de CLI desaparece.

Congelamento não tem verificação automática: é uma regra de alocação de esforço.
O sinal de violação é uma fixture Defold nova aparecer no diff sem esta ADR ter sido
substituída.

## Critério de revisão

Revisar quando ocorrer o primeiro destes eventos:

- o Marco 6 fechar e o portão autorizar continuidade;
- uma mudança real em projeto Defold produzir defeito de posse que o Sara deixou passar;
- o adapter Godot acumular verdadeiros positivos suficientes para substituir o Portão 0;
- o Defold publicar versão que quebre o adapter congelado.

## Notas

- Autor: proprietário do Sara
- Origem: achado A1 de `docs/AUDITORIA-ARQUITETURAL.md`
- Substitui: nenhuma. Complementa a ADR 0002, que permanece aceita
