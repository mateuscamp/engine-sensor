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

## Riscos do spike visual aprovado — os três se realizaram, e estão medidos

Estes riscos foram escritos em 23/08/2026 para um spike que **nunca foi construído**: o
Marco 7 aconteceu sem ter sido planejado, e é a Sentinela do porte
([ADR 0014, adendo](decisoes/0014-comparacao-do-marco-7-com-as-ferramentas-existentes.md)).
Eles não alteram o aceite do verificador `0.1.0`.

**Mas eles não envelheceram: eles aconteceram.** Os três foram vividos pelo aparelho que
ocupou o lugar do spike, e a pontuação que este documento deu antes da evidência pode agora
ser conferida contra ela. Fica pontuado abaixo, em 29/08/2026, sem tocar na tabela original —
previsão corrigida depois da evidência não prova nada.

| Risco | Impacto | Prob. | Pontos | Mitigação e sinal de parada |
|---|---:|---:|---:|---|
| Screenshot correto esconder estado causal incorreto | 3 | 2 | 6 | sempre parear imagem com estado, entrada e instante |
| Referência visual variar por GPU, fonte ou plataforma | 3 | 3 | 9 | ambiente fixado; comparação de pixels e estado; Android continua com veto |
| Instrumentação mudar a cena observada | 3 | 2 | 6 | probe mínimo, removível e testado contra execução sem captura |

### O placar, em 29/08/2026

| risco | pontos | aconteceu? | evidência |
|---|---:|---|---|
| Screenshot correto esconder estado causal incorreto | 6 | **sim, e é o caso central do projeto** | no [caso da aranha](CASO-DA-ARANHA.md), a Sentinela deu **0 de 25 telas mudaram** sobre um mecanismo em que **0 de 36 roubos completavam**. Screenshot correto, estado causal errado, portão verde |
| Referência visual variar por GPU, fonte ou plataforma | **9** | **sim** | nove das 25 capturas mudavam sem relógio fixo; o `--fixed-fps 60` as levou a zero, e o `sentinela.txt` passou a anotar a GPU e o driver. Uso 12 do [registro](USO-PESSOAL.md) |
| Instrumentação mudar a cena observada | 6 | **sim, e na direção errada** | ver abaixo |

**O de 9 pontos era o mais alto da matriz inteira, e a mitigação escrita antes funcionou**:
*"ambiente fixado; comparação de pixels e estado"*. É o único dos três em que a previsão
acertou risco, causa e remédio.

**O de 6 pontos sobre o screenshot acertou o risco e subestimou o remédio.** A mitigação
escrita era *"sempre parear imagem com estado, entrada e instante"*, e a
[pergunta 7](PERGUNTA-7.md) mediu o quanto ela vale: no defeito de cena, o par
imagem-contra-estado localizou e deu o número; **no defeito de tempo, a imagem não contribuiu
com nada** — quem respondeu foi o estado sozinho. Parear é necessário e não é suficiente,
porque há defeito em que um dos dois termos não entra na conta.

**O de instrumentação aconteceu numa direção que a linha não previa.** Ela dizia *"probe
mínimo, removível"*, e imaginava a sonda mexendo na **cena**. O que aconteceu foi a sonda
mexendo no **domínio**: os 18 casos da aranha e a sonda que produziu a frase *"o roubo
funciona"* rodavam com `bomba.pavio = 999.0`, e **era esse botão que escondia a falha**.
Instrumentação removível não protege contra isso — a sonda saiu limpa, e o defeito ficou.

Na direção prevista, a mitigação valeu: o `godot-agent` instalou addon e autoload, e o
`daemon uninstall` devolveu o `project.godot` aos bytes originais, com `git diff` vazio
conferido. Removível de verdade, e ainda assim cego — ele leu uma propriedade **ligada em
nada** e reportou verde ([ADR 0014](decisoes/0014-comparacao-do-marco-7-com-as-ferramentas-existentes.md)).

**A lição que a matriz não tinha:** um risco de instrumentação escrito só sobre a cena deixa
de fora o parâmetro. A forma geral é *o instrumento muda alguma coisa para poder observar, e
essa coisa pode ser exatamente a que decide.*
| Julgamento multimodal produzir falsa aprovação | 3 | 2 | 6 | regressões injetadas, oráculo independente e nenhuma reprovação baseada só em opinião visual |
| Spike crescer até SDK ou runtime | 3 | 2 | 6 | ADR 0004 limita uma cena, uma engine e uma interface provisória |
| Execução passar a depender de serviço remoto | 3 | 1 | 3 | Sara apenas produz artefatos locais; o agente consumidor fica fora do binário |
| Ferramenta existente já entregar a unidade de evidência do spike | 3 | 3 | 9 | **materializado em 26/08/2026**: a Defold publicou a `extension-automation-bridge` oficial e há pelo menos quatro implementações comunitárias em Godot. Mitigação: [ADR 0011](decisoes/0011-marco-7-exige-comparacao-com-ferramenta-existente.md) exige comparação escrita antes de o Marco 7 começar |
