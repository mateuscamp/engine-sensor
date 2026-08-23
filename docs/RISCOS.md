# Matriz de riscos do Sara 0.1.0

Impacto e probabilidade variam de 1 a 3. Pontuação de 6 a 9 exige mitigação antes do
lançamento interno.

| Risco | Impacto | Prob. | Pontos | Mitigação e sinal de parada |
|---|---:|---:|---:|---|
| Gramática comunitária não entende GDScript 4.7 | 3 | 2 | 6 | corpus obrigatório; erro explícito; não prometer o adapter se houver omissão |
| Análise estática acusa concorrência inexistente | 3 | 2 | 6 | só prova bloqueia; fluxo incerto vira aviso; exceção exata e justificada |
| Análise estática deixa alvo dinâmico passar | 3 | 2 | 6 | aviso `SAR-PARSE-001`; registrar cobertura de declarações resolvidas |
| Duas engines duplicam a implementação | 2 | 2 | 4 | modelo comum e semântica isolada por adapter |
| Kit diverge entre Codex e Claude | 2 | 2 | 4 | contrato canônico único e fragmentos mínimos |
| Linter vira produto amplo antes de provar posse | 3 | 2 | 6 | ADR bloqueia runtime, consulta, telemetria, loja e monetização |
| Configuração esconde problemas novos | 3 | 1 | 3 | sem baseline global; exceção exige recurso, donos e motivo exatos |
| Portão é lento demais para uso por agente | 2 | 2 | 4 | mediana menor que 2 s no Gods; sem iniciar a engine |
| Ferramenta depende de rede ou runtime externo | 3 | 1 | 3 | binário Rust único; teste com rede indisponível |

## Riscos do spike visual aprovado

Estes riscos pertencem ao Marco 7 e não alteram o aceite do verificador `0.1.0`.

| Risco | Impacto | Prob. | Pontos | Mitigação e sinal de parada |
|---|---:|---:|---:|---|
| Screenshot correto esconder estado causal incorreto | 3 | 2 | 6 | sempre parear imagem com estado, entrada e instante |
| Referência visual variar por GPU, fonte ou plataforma | 3 | 3 | 9 | ambiente fixado; comparação de pixels e estado; Android continua com veto |
| Instrumentação mudar a cena observada | 3 | 2 | 6 | probe mínimo, removível e testado contra execução sem captura |
| Julgamento multimodal produzir falsa aprovação | 3 | 2 | 6 | regressões injetadas, oráculo independente e nenhuma reprovação baseada só em opinião visual |
| Spike crescer até SDK ou runtime | 3 | 2 | 6 | ADR 0004 limita uma cena, uma engine e uma interface provisória |
| Execução passar a depender de serviço remoto | 3 | 1 | 3 | Sara apenas produz artefatos locais; o agente consumidor fica fora do binário |
