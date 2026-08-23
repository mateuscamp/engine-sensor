# Registro do Marco 6 - uso pessoal

> Este é o registro **do próprio Sara**. O modelo distribuído por `sara init` está em
> [`kit/USOS.md`](../kit/USOS.md); a instância criada em cada projeto integrado fica em
> `.sara/USOS.md` daquele projeto. Três arquivos, três papéis distintos.

Preencher uma linha para cada mudança em que animação ou entrada possa ter mudado.

**Critério de conclusão:** dez mudanças reais em projetos Godot.
**Critério de parada:** 20 de setembro de 2026.

O que vier primeiro encerra o marco. Chegar à data com menos de dez mudanças é um
resultado, não um atraso: significa que a ferramenta não está no caminho do trabalho
real, e o encerramento legítimo passa a ser manter só o kit ou congelar a ferramenta.

Pela [ADR 0005](decisoes/0005-foco-em-godot-com-defold-congelado.md), mudança em projeto
Defold não conta para a contagem. O uso 1 abaixo é anterior à decisão e permanece
registrado.

**Estado atual:** 1 de 10 mudanças; 4 projetos Godot integrados; prazo aberto até 20/09/2026.

| # | Data | Projeto | Mudança | Tempo | Conflito | Aviso útil/falso | Inspeção humana necessária | Regra ausente |
|---:|---|---|---|---:|---|---|---|---|
| 1 | 2026-08-23 | porte BomberBoom (Godot) | bomba visual com Tween configurado em cadeia fluente | < 1 s | nenhum após correção | nenhum aviso; a primeira execução omitiu 2 declarações | sim, para comparar o inventário com o diff | parser perdia `tween_property` seguido de `set_trans`/`set_ease`; fixture adicionada |
| 2 | | | | | | | | |
| 3 | | | | | | | | |
| 4 | | | | | | | | |
| 5 | | | | | | | | |
| 6 | | | | | | | | |
| 7 | | | | | | | | |
| 8 | | | | | | | | |
| 9 | | | | | | | | |
| 10 | | | | | | | | |

O Sara pode permanecer privado ao final. Nova etapa pública exige nova decisão;
não é continuação automática deste registro.

## Baselines que não contam como mudança

| Data | Projeto | Arquivos | Declarações | Erros | Avisos | Motivo de não contar |
|---|---|---:|---:|---:|---:|---|
| 2026-08-23 | porte BomberBoom (Godot) | 69 | 5 | 0 | 0 | fotografia posterior ao uso 1; não é uma segunda mudança |
| 2026-08-23 | BomberBoom (Defold) | 27 | 76 | 0 | 19 | integração do segundo projeto, sem mudança de jogo associada |
| 2026-08-23 | BomberBoom (Defold), após calibração | 27 | 76 | 0 | 12 | refinamento da mesma baseline; não é mudança de jogo |
| 2026-08-23 | Gods (Godot) | 450 | 67 | 0 | 1 -> 0 | integração do terceiro projeto Godot, sem mudança de jogo associada; o aviso era falso e virou a ADR 0009 |
| 2026-08-23 | Boomlitude (Godot) | 97 | 6 | 0 | 0 | integração do quarto projeto Godot, sem mudança de jogo associada |

## Baseline do Gods: o único aviso, classificado

`SAR-OWN-001` em `animation:godot:src/entities/card.gd:self:position`, apontando
`card.gd::set_elevated` (1040) e `card.gd::_on_selection_end` (1078).

**Classificação: falso.** Não há conflito em runtime. As seis animações de `position`
no arquivo seguem disciplina de dono centralizado: cada uma chama `_kill_active_tween()`
antes de criar a sua e guarda a nova em `_active_tween`, então existe um único Tween de
posição por construção. `_mouse_enter` e `_mouse_exit` ainda saem cedo quando
`_selected or _dragging or _block_hand_spring or _hand_target_active`.

**Regra ausente, em duas partes.** O adapter reconhecia cancelamento só como
`variavel.kill()` literal entre as duas linhas, e não seguia a indireção do método
auxiliar; e o aviso entre donos nunca consultava barreira nenhuma. O que decide o caso:
a remediação que o próprio `SAR-OWN-001` imprime é *"centralize o proprietário"*, e é
exatamente o que o `card.gd` faz. A ferramenta pedia o padrão que não sabia reconhecer.

**Consertado** pela [ADR 0009](decisoes/0009-baseline-em-projeto-real-expoe-regra-ausente.md),
que também ampliou a exceção da Fase 2 para baseline em projeto real. Duas trajetórias
passam a se serializar quando **as duas** encerram o mesmo alvo antes de começar,
seguindo um nível de indireção. Fixtures `godot_animation_centralized_owner_green` e
`godot_animation_uncancelled_owners_warn` — a segunda existe para reprovar uma regra
boa demais, e reprova mesmo: foi testada por mutação.

Nos cinco projetos do corpus, as declarações são idênticas antes e depois, e a única
mudança de diagnóstico é este aviso desaparecendo. Os 12 avisos do BomberBoom Defold
permanecem intactos.

Varredura de 450 arquivos em 681 ms, mediana de cinco execuções — a medição do
`RESULTADO-0.1.0.md` era 0,69 s, então a fronteira da Fase 1 não custou desempenho.

**Isto não conta como uso do Marco 6.** Baseline não é mudança real, e a ADR 0009 diz
isso explicitamente. A contagem continua em 1 de 10.

## Classificação da baseline Defold

- **7 falsos removidos:** laços com variáveis locais independentes, cancelamento que
  domina ramos, animação iniciada no callback de conclusão e ramo exclusivo de `init`.
- **11 úteis mantidos:** transições de ciclo de vida que substituem deliberadamente
  uma animação sem chamar `cancel_animations`; continuam avisos, nunca bloqueios.
- **1 útil mantido:** `PAVIO[n]`, alvo dinâmico limitado por tabela que ainda não pode
  ser resolvido estaticamente.
- **Taxa atual:** 0 aviso falso em 76 declarações classificadas. A amostra ainda é
  pequena e será recalculada em cada uso Defold.
