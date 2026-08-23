# Registro do Marco 6 - uso pessoal

Preencher uma linha para cada mudança em que animação ou entrada possa ter mudado.
Dez usos em pelo menos dois projetos encerram o marco.

**Estado atual:** 1 de 10 mudanças; 2 de 2 projetos integrados. O marco permanece
aberto até existirem nove mudanças reais adicionais.

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
