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

**Estado atual:** 1 de 10 mudanças; prazo aberto até 20/09/2026.

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

## Classificação da baseline Defold

- **7 falsos removidos:** laços com variáveis locais independentes, cancelamento que
  domina ramos, animação iniciada no callback de conclusão e ramo exclusivo de `init`.
- **11 úteis mantidos:** transições de ciclo de vida que substituem deliberadamente
  uma animação sem chamar `cancel_animations`; continuam avisos, nunca bloqueios.
- **1 útil mantido:** `PAVIO[n]`, alvo dinâmico limitado por tabela que ainda não pode
  ser resolvido estaticamente.
- **Taxa atual:** 0 aviso falso em 76 declarações classificadas. A amostra ainda é
  pequena e será recalculada em cada uso Defold.
