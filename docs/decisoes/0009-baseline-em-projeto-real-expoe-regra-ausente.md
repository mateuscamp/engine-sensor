# ADR 0009 - Baseline em projeto real também autoriza regra ausente

**Status:** Aceita
**Data:** 23 de agosto de 2026
**Decisor:** proprietário do Sara
**Escopo:** governança do Marco 6 e regra de posse de animação em Godot

## Contexto

A Fase 2 do plano da auditoria congela regra nova durante o Marco 6, com uma exceção:
"salvo quando uma mudança real expuser regra ausente". A exceção existe porque foi
assim que a fixture da cadeia fluente do `tween_property` apareceu — a realidade
manda, não o roteiro.

Na integração do `gods` e do `boomlitude`, a baseline do `gods` produziu um aviso
`SAR-OWN-001` em `src/entities/card.gd`. A classificação foi feita e o aviso é
**falso**: as seis trajetórias de `position` no arquivo seguem disciplina de dono
centralizado — cada escritor chama `_kill_active_tween()` antes de criar a sua e
guarda a nova em `_active_tween`. Existe um único Tween de posição por construção.

O Sara não via isso por duas lacunas em `src/adapters/godot.rs`:

1. `has_ordering_barrier` reconhecia apenas `variavel.kill()` literal entre as duas
   linhas. O `gods` cancela por método auxiliar, e a indireção não era seguida.
2. O aviso entre donos nunca consultava barreira nenhuma. Mesmo um `.kill()` literal
   não o calaria.

O detalhe que decide: a remediação que o próprio `SAR-OWN-001` imprime é *"centralize
o proprietário"*. O `card.gd` já faz exatamente isso. A ferramenta estava pedindo o
padrão que não sabia reconhecer, no maior projeto do corpus.

A força que exige decisão agora é de forma, não de conteúdo: a regra ausente foi
exposta por uma **baseline**, e baseline por definição não é mudança real. Ao pé da
letra, a Fase 2 não autoriza o conserto.

## Opções consideradas

1. Esperar uma mudança real tocar o `card.gd`, cumprindo a Fase 2 literalmente.
2. Registrar exceção `[[allow]]` no `sara.toml` do `gods`, calando o aviso sem tocar
   em regra.
3. Consertar sem registrar nada, tratando a baseline como coberta pela exceção.
4. Ampliar a exceção por escrito e consertar.

A opção 1 mantém um falso positivo conhecido de pé no maior projeto do corpus por até
quatro semanas, dentro da janela de medição. Cada mudança futura em `card.gd` tropeça
no mesmo ruído, e o desfecho previsível é o proprietário aprender a ignorar o Sara —
que é precisamente o fracasso que o Marco 6 existe para detectar. Produzi-lo de
propósito, com o defeito já diagnosticado, invalidaria a medição.

A opção 2 usa um mecanismo legítimo, mas no lugar errado: `[[allow]]` é para exceção
de projeto, e isto não é particularidade do `gods`. É um padrão comum em GDScript, e
deixá-lo assim reproduz o falso positivo em todo projeto que centralize o dono.

A opção 3 conserta e não deixa rastro. É exatamente o tipo de contorno silencioso que
a auditoria encontrou em oito lugares deste projeto.

## Decisão

Nós ampliamos a exceção da Fase 2: **baseline de integração em projeto real também
autoriza regra ausente**, com as mesmas condições que valem para mudança real — a
regra nasce com fixture, o corpus inteiro é reexecutado, e o caso vai para o diário
com a classificação escrita antes do conserto.

E nós consertamos a regra: duas trajetórias se serializam quando **as duas** encerram
o mesmo alvo antes de começar, seguindo um nível de indireção por método auxiliar.
Exigir os dois lados é o que separa esta regra de uma que cala aviso legítimo:
cancelar de um lado só não serializa nada.

O que a decisão **não** amplia: baseline não conta como uso do Marco 6. A contagem
continua sendo de mudanças reais, e continua em 1 de 10.

## Consequências

### Positivas

- O padrão de dono centralizado, que é a remediação recomendada pela própria regra,
  deixa de virar aviso falso.
- O Marco 6 começa sem falso positivo conhecido, o que é condição para a medição
  significar alguma coisa.
- A exceção fica escrita antes de ser usada de novo, em vez de ser inventada caso a
  caso.

### Negativas

- A superfície de silêncio do `SAR-OWN-001` aumenta. Um projeto que chame um método
  auxiliar de cancelamento sem que ele cancele o Tween certo não recebe mais o aviso.
  A fixture `godot_animation_uncancelled_owners_warn` limita o dano fixando que
  cancelamento de um lado só continua avisando.
- A Fase 2 fica com uma exceção a mais, e exceção acumulada é como escopo volta.
  Esta é a segunda e continua exigindo evidência de projeto real.
- O conserto vale só para Godot. O adapter Defold continua congelado pela ADR 0005 e
  não recebe a regra equivalente, mesmo tendo o padrão análogo.

## Conformidade

Fitness function automática, em `cargo test`:

- `godot_recognizes_centralized_owner_cancellation` em `tests/cli.rs` exige que a
  fixture `godot_animation_centralized_owner_green` termine sem diagnóstico e que a
  `godot_animation_uncancelled_owners_warn` continue avisando. Uma regra boa demais
  reprova na segunda.
- `tests/corpus.rs` reexecuta os cinco projetos pessoais.

Evidência registrada: nos cinco projetos do corpus, as declarações são idênticas antes
e depois, e a única mudança de diagnóstico é o aviso do `gods` desaparecendo. Os 12
avisos do BomberBoom Defold permanecem intactos.

## Critério de revisão

Se uma mudança real produzir sobreposição observada em cena entre duas trajetórias que
esta regra silenciou, a regra volta a avisar e a exceção da Fase 2 é revista junto. A
terceira exceção à Fase 2, se aparecer, obriga a rever a Fase 2 inteira em vez de
ampliá-la de novo.

## Notas

- Autor: proprietário do Sara
- Aprovada por: proprietário do Sara
- Substitui: nenhuma
- Última alteração: 23 de agosto de 2026
