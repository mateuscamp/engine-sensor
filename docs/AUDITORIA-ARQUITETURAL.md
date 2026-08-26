# Auditoria arquitetural e plano final de implementação

**Status:** auditoria concluída; Fase 0 e Fase 1 executadas e verificadas; A7 fechado por inteiro
**Data:** 23 de agosto de 2026
**Escopo:** conformidade do Sara `0.1.0` com o método declarado em
[`METODO-ARQUITETURAL.md`](METODO-ARQUITETURAL.md), medida contra as duas referências
**Referências:** Richards e Ford, *Fundamentals of Software Architecture* (FSA);
Ford, Richards, Sadalage e Dehghani, *Software Architecture: The Hard Parts* (HP)

## Resposta curta

O método está correto e é aplicado de verdade, não decorativamente. As ADRs, a matriz
de risco, as sete características prioritárias e a regra de fixar limiar antes do
experimento reproduzem fielmente os capítulos que o `METODO-ARQUITETURAL.md` cita — em
dois pontos com disciplina maior que a dos livros.

A auditoria encontra **oito achados**. Nenhum invalida o `0.1.0`. Um deles é
estrutural e barato agora, caro depois: o Marco 7 muda o quantum arquitetural do
Sara, e o plano trata isso como consequência em prosa, não como estrutura no código.
Três achados são o próprio método do projeto sendo violado pelo projeto.

O plano final consolidado está na seção 5. Ele não acrescenta escopo: acrescenta um
dia de governança executável antes do Marco 6 e uma decisão de empacotamento antes
do Marco 7.

## 0. O que foi executado

Tudo verificado: 33 testes verdes, `cargo clippy` sem aviso, `cargo fmt` aplicado,
build de release ligando só contra `libc` e `libgcc`, execução em namespace sem rede
terminando em código 0.

| Item | Achado | Resultado |
|---|---|---|
| `common::action_branches` recebe `BlockSyntax` em vez de `godot: bool` | A1 | `src/adapters/common.rs` não menciona nenhuma engine |
| `tests/governanca.rs` com treze fitness functions | A1, A2, A5, A6, A7 | 33 testes no total, contra 19 antes |
| ADR 0005 - foco em Godot, Defold congelado | A4 | matriz ponderada retirada do `ROTEIRO.md` |
| ADR 0006 - contrato estrito de relatório e códigos de saída | A6 | forma do JSON congelada por teste |
| ADR 0007 - `sara observe` como binário separado | A5 | `sara` permanece um quantum offline |
| ADR 0008 - gramáticas tree-sitter fixadas | Fase 3 | decisão que já existia de fato, agora registrada |
| Data de parada do Marco 6: 20/09/2026 | A3 | `ROTEIRO.md` e `USO-PESSOAL.md` |
| Caixa de decisão do agente | seção 3 | `kit/CONTRATO.md` e os dois fragmentos |
| Cabeçalho de papel nos três diários | A7 | `docs/USO-PESSOAL.md`, `kit/USOS.md` |
| Frase do núcleo corrigida | A1 | `docs/RESULTADO-0.1.0.md` |
| Diagrama de contêiner | A8 | `docs/arquitetura.svg` |

**F5 encontrou duas divergências reais na primeira execução.** O scanner aceita `.gd`
e `.render_script`; o contrato de compatibilidade publicado não declarava nenhum dos
dois. O documento foi corrigido. Uma fitness function que acha defeito no minuto em
que entra é o melhor argumento possível a favor dela.

---

## 1. O que a auditoria confirma

| Prática do Sara | Mecanismo na referência | Veredito |
|---|---|---|
| `METODO §1` separa objetivo, evidência, referência e mercado | FSA cap. 2: arquitetura é o que não se pode pesquisar; HP: não existem melhores práticas universais | Correto, e mais rigoroso que os livros: os livros não avisam para não virarem requisito |
| "Registrar o porquê antes do como" | FSA, Segunda Lei: *why is more important than how* | Fiel |
| "Combinação menos ruim de trade-offs para um contexto explícito" | FSA cap. 5: nunca busque a melhor arquitetura, busque a menos ruim | Fiel |
| Sete características prioritárias com definição operacional | FSA cap. 4-6: limite de aproximadamente sete; característica sem medida não governa | Fiel |
| `docs/modelos/ADR.md` | FSA cap. 19: Título, Status, Contexto, Decisão, Consequências, mais Conformidade e Notas | Fiel, com a extensão *Alternatives* que o próprio capítulo autoriza |
| Status `Substituída por NNNN` e campo `Substitui` | FSA cap. 19: `Superseded` em ambas as direções | Fiel |
| `RISCOS.md`: impacto x probabilidade 1-3, nota 1-9, mitigação com sinal de parada | FSA cap. 20: matriz de risco; impacto primeiro, probabilidade depois | Fiel |
| Opções sempre incluem "não fazer" e "ferramenta existente" | HP cap. 15: lista MECE, coletivamente exaustiva | Fiel |
| "Devem ser fixados antes de executar o experimento que irão julgar" (`DIAGNOSTICO §8`) | FSA cap. 6: fitness function | **Mais forte que a referência.** Os livros não impõem essa ordem |
| `RESULTADOS §4`: proibição de pontuar antes de toda evidência comparável existir | HP cap. 15: armadilha do fora de contexto | **Mais forte que a referência** |
| Um binário Rust, offline, sem runtime externo | HP cap. 2: quantum arquitetural independentemente implantável | Fiel, e medido |
| `estudo/` como evidência bruta separada de conclusão | FSA cap. 21: apego irracional ao artefato | Imunizado por desenho |

Três capítulos das referências aparecem no `METODO §7` e estão de fato aplicados:
FSA 2, 4-6, 19, 20 e HP 15. Os achados abaixo vêm sobretudo dos capítulos que o
método **não** cita e que se aplicam ao que o projeto virou: FSA 3 (connascência),
FSA 22 (dar orientação), HP 2 (acoplamento e quantum), HP 7 (granularidade) e
HP 14 (contratos).

---

## 2. Achados

### A1 - O núcleo não era agnóstico de engine - CORRIGIDO

**Evidência.** `RESULTADO-0.1.0.md` afirma: "O núcleo comum mantém descoberta,
configuração, modelo, ordenação e apresentação fora dos adapters." O código diverge em
quatro módulos:

| Local | Semântica de engine no núcleo |
|---|---|
| `src/scanner.rs:19,25,28,66,106-107` | `project.godot`, `game.project`, extensões por engine |
| `src/parser.rs:38-48,217` | gramática por engine, `ascii_shape` só para Godot, regex de `func` do GDScript |
| `src/init.rs:44,58` | ramo por engine |
| `src/adapters/common.rs:146,153,167,191` | parâmetro `godot: bool` com quatro ramificações |

**Referência.** FSA cap. 3. Os `match Engine` de `scanner`, `parser` e `init` são
connascência de nome e de tipo: explícitos, localizáveis por busca, refatoráveis por
ferramenta. São aceitáveis. `common::action_branches(function, calls, regex, godot:
bool)` é outra coisa: um booleano posicional cujo significado — "sintaxe de bloco com
`:`/`elif` em vez de `then`/`elseif`/`end`" — precisa ser conhecido igualmente pelos
dois adapters e pelo código compartilhado. É connascência de significado e de posição
atravessando a fronteira de encapsulamento, exatamente o que a segunda regra de
Page-Jones manda minimizar.

**Consequência.** A propriedade que o `RESULTADO` vende — "preserva a troca de foco
sem reescrever o CLI" — e a mitigação do risco "Duas engines duplicam a implementação:
modelo comum e semântica isolada por adapter" dependem dessa fronteira. Uma terceira
engine transforma o booleano em `match` dentro do código compartilhado, que é o
oposto do que foi prometido.

**Correção aplicada.** `common::action_branches` agora recebe `common::BlockSyntax`,
um descritor que cada adapter declara como constante: prefixos que abrem ramo, marca
que encerra a condição, prefixos e linhas exatas que fecham o corpo. `GDSCRIPT_BLOCKS`
vive em `godot.rs`; `LUA_BLOCKS` vive em `defold.rs`. A connascência de significado
virou connascência de tipo, que é a *Rule of Degree* de Weirich, e o código
compartilhado deixou de conhecer engine. Comportamento idêntico: os 19 testes que
existiam continuam passando sem alteração.

Os `match Engine` de `scanner`, `parser`, `init`, `config` e `model` continuam onde
estavam, por decisão: são connascência de nome e de tipo, explícitos e enumeráveis. F3
fixa a lista deles.

A frase do `RESULTADO-0.1.0.md` foi corrigida, com nota de correção datada: a variação
por engine existe, está concentrada em cinco arquivos nomeados, e o conjunto é fechado
por teste.

### A2 - A conformidade da ADR 0001 era uma lista de proibições sem verificação - CORRIGIDO

**Evidência.** A ADR 0001 proíbe iniciar editor, renderizador, áudio, física geral,
loja de assets e cadeia multiplataforma. Nenhuma dessas proibições tem verificação.

**Referência.** FSA cap. 19 exige que a seção de conformidade decida entre checagem
manual e fitness function automática, e descreva como a automática seria escrita. FSA
cap. 6 explica por que isso importa: modularidade é importante mas não urgente, e
urgência domina. O `METODO §2` já diz a mesma coisa em uma linha: "trocar regras
lembradas por mecanismos executáveis quando o custo se justificar".

**Consequência.** A ADR que segura o escopo inteiro do projeto é a única sem
mecanismo. Ela depende da memória do proprietário em uma base que agentes editam.

**Correção aplicada.** `tests/governanca.rs` declara `DEPENDENCIAS_AUTORIZADAS` com as
treze dependências atuais. Dois testes: um reprova quando aparece dependência fora da
lista, citando a ADR 0001 e o que ela proíbe; o outro reprova quando a lista guarda uma
entrada que o manifesto não usa mais, para que ela continue sendo a verdade e não um
histórico. É o equivalente em Rust ao exemplo ArchUnit do cap. 19.

### A3 - O Marco 6 tinha critério de conclusão, mas não de parada - CORRIGIDO

**Evidência.** `USO-PESSOAL.md`: "O marco permanece aberto até existirem nove mudanças
reais adicionais." Estado atual: 1 de 10. Não há data.

**Referência.** `METODO §5`, regra do próprio projeto: "um experimento deve ter uma
hipótese principal e um critério de parada". `METODO §2`: "adiar decisões
irreversíveis até o último momento responsável, sem usar isso como desculpa para
paralisia".

**Consequência.** O marco que abre todos os outros — Marco 7 depende dele por ADR
0004 — pode ficar aberto indefinidamente. E a ausência de uso voluntário já está
listada em `RESULTADO-0.1.0.md` como encerramento legítimo: sem data, esse resultado
nunca é declarado, apenas adiado.

**Correção aplicada.** **20 de setembro de 2026**, quatro semanas, registrado no
`ROTEIRO.md` e no `USO-PESSOAL.md`. Se dez mudanças reais em Godot não acontecerem até
lá, isso **é** o resultado: a ferramenta não está no caminho do trabalho real. Encerrar
em kit apenas, ou congelar.

### A4 - A matriz de foco era quantitativa onde as referências pedem qualitativa - RESOLVIDO POR OUTRO CAMINHO

**Evidência.** `RESULTADOS §4`: o protocolo "proíbe fixar pesos depois de ler os
resultados — que é onde o estudo parou". O `ROTEIRO.md`, datado depois do estudo, fixa
pesos (30/25/20/15/10), notas iniciais e um limiar de "10 pontos ponderados".

**Referência.** HP cap. 15: "virtually none of our trade-off tables are quantitative
[...] two architectures will always differ enough to prevent true quantitative
comparisons [...] we recommend you hone the skill of performing qualitative analysis".

**O que a matriz acerta.** Os vetos — falha funcional no aparelho, toque incorreto,
prova de cena dependente de uma pessoa — são qualitativos, binários e decisivos. São
eles que estão fazendo o trabalho de decisão. As duas linhas com evidência real
("ciclo sem atenção humana", "velocidade e estabilidade") também.

**Como ficou.** A ADR 0005 tornou a matriz desnecessária: Godot é o foco declarado
por decisão escrita, com contexto, consequências negativas e critério de revisão. Isso
é mais honesto que uma soma ponderada cujos pesos foram fixados depois de ler o
resultado. Os vetos — falha no aparelho, toque incorreto, prova dependente de pessoa —
continuam valendo integralmente, inclusive o veto da medição Android. A soma ponderada
sai do `ROTEIRO.md`.

### A5 - O Marco 7 muda o quantum arquitetural - DECIDIDO (ADR 0007)

**Evidência.** A propriedade mais forte e mais bem medida do `0.1.0`: binário único de
3.836.984 bytes, código 0 em namespace sem rede, sem Python nem Node. A ADR 0004
registra a perda corretamente entre as consequências negativas: "a prova deixa de ser
um único binário operacional e passa a exigir Godot no ambiente". Mas o registro é
textual; não há decisão de empacotamento.

**Referência.** HP cap. 2: um quantum é independentemente implantável, com alta coesão
funcional e alto acoplamento estático. Exigir uma instalação de Godot é acoplamento
estático novo. HP cap. 7, desintegradores de granularidade aplicados a `check` e
`observe`:

| Desintegrador | `sara check` x `sara observe` |
|---|---|
| Escopo e função | análise estática de texto x instrumentação de cena em execução |
| Volatilidade | estável, com corpus x experimental, muda a cada repetição |
| Tolerância a falha | uma queda do Godot não pode derrubar o portão |
| Extensibilidade | contrato congelado x interface provisória por desenho |

Quatro desintegradores. Os integradores existentes — `model`, esquema do relatório,
vocabulário do CLI — pedem biblioteca compartilhada, não binário compartilhado.

**Consequência.** Se `observe` entrar no mesmo binário, `sara check` herda dependência
de Godot. A característica 5, "Possibilidade de posse", e o risco "Ferramenta depende
de rede ou runtime externo" degradam sem que nenhum teste reclame.

**Decisão aplicada.** [ADR 0007](decisoes/0007-observe-como-binario-separado.md):
`sara-observe` nasce como binário separado no mesmo workspace, compartilhando `model` e
`report` pela biblioteca. O binário `sara` continua sem Godot, sem rede e sem runtime
externo. `adr_0007_apenas_binarios_autorizados` reprova quando aparece um `[[bin]]` fora
da lista. O desenho está em [`arquitetura.svg`](arquitetura.svg).

### A6 - O contrato JSON e os códigos de saída eram estritos de fato, sem ADR - CORRIGIDO

**Evidência.** `src/model.rs:7` define `REPORT_SCHEMA_VERSION: u32 = 1`. Os códigos
0, 1 e 2 e o formato JSON são consumidos por Codex e Claude Code através de
`.sara/CONTRATO.md` e dos fragmentos. Não há ADR.

**Referência.** HP cap. 14, contratos estritos x frouxos. A escolha do Sara é estrita,
e está certa: o consumidor é um agente que não pode adivinhar. Mas contrato estrito
sem decisão escrita não tem regra de quebra nem de versionamento.

**Correção aplicada.** [ADR 0006](decisoes/0006-contrato-estrito-de-relatorio-e-codigos-de-saida.md):
contrato **estrito**. Qualquer mudança de forma — inclusive campo acrescentado — sobe
`REPORT_SCHEMA_VERSION`; mudança de valor dentro da forma não sobe. Os 38 caminhos de
chave do relatório estão congelados em `FORMA_DO_RELATORIO`, e três testes guardam a
forma, a coerência da versão e a existência de cenário vivo para cada código de saída.

### A7 - Segunda fonte de verdade em compatibilidade e no diário de uso - CORRIGIDO

**Evidência.** `docs/COMPATIBILIDADE.md` lista extensões e construções também
codificadas em `src/scanner.rs:106-107` e nos adapters. Existem três arquivos de
diário — `docs/USO-PESSOAL.md`, `kit/USOS.md` e `.sara/USOS.md` — sem cabeçalho
dizendo qual é qual.

**Referência.** `METODO §5`, de novo regra do próprio projeto: "informação operacional
pertence ao teste ou ao código, não a uma segunda fonte de verdade em prosa".

**Correção aplicada.** F5 extrai as extensões do corpo de `scanner::supported`
e exige que cada uma apareça em `docs/COMPATIBILIDADE.md`. Na primeira execução o teste
reprovou com `["gd", "render_script"]`: o contrato publicado não declarava a extensão
principal do Godot nem uma das quatro do Defold. O documento foi corrigido.

Os três diários receberam cabeçalho: `docs/USO-PESSOAL.md` é o registro do próprio
Sara, `kit/USOS.md` é o modelo distribuído pelo `init`, e `.sara/USOS.md` é a instância
de cada projeto integrado.

**A metade que faltava, fechada depois.** As extensões eram só metade da duplicação: as
*construções* — `tween_property`, `go.animate`, `gui.animate`, cancelamento, os pontos de
entrada — viviam em prosa no documento e em literais espalhados pelos adapters, sem nada
ligando os dois. Cada adapter agora declara `CONSTRUCTS`, no mesmo espírito do
`BlockSyntax` da Fase 1, e a tabela do `COMPATIBILIDADE.md` é conferida nos dois sentidos:
o documento não cala o que o código faz, nem promete o que ele não faz. Um contrato que
promete a mais é pior que contrato ausente, porque é acreditado.

A primeira versão de F7 nasceu vacuosa e o teste de mutação a pegou: como `CONSTRUCTS`
mora no mesmo arquivo que o teste vasculhava, todo token declarado se encontrava a si
mesmo, e um token inventado passava. O teste agora remove o bloco da declaração antes de
buscar. Fica registrado porque é o argumento a favor de mutar toda fitness function nova:
verde não é prova até falhar quando deve.

### A8 - Nenhum diagrama - CORRIGIDO

FSA cap. 21. [`docs/arquitetura.svg`](arquitetura.svg) mostra os dois painéis: o
pipeline atual com a semântica de engine contida nos adapters, e onde o Marco 7
acrescenta a dependência de Godot sem tocar no portão. A1 e A5 ficam visíveis de
relance.

---

## 3. Uma importação que falta, e que serve à tese do projeto

FSA cap. 22 descreve como o arquiteto dá orientação sem virar gargalo: classificar
bibliotecas de terceiros em três caixas e dizer, para cada caixa, quem decide.

| Categoria | Quem decide, no livro |
|---|---|
| Propósito específico | o desenvolvedor decide sozinho |
| Propósito geral | o desenvolvedor analisa e propõe; o arquiteto aprova |
| Framework | o arquiteto decide; o time nem analisa |

Num projeto cujo usuário primário é um agente, essa caixa é o mecanismo mais útil do
capítulo e o `.sara/CONTRATO.md` não o tem. O contrato hoje lista oito regras de
código, mas não diz **o que o agente pode decidir sozinho**. A tradução é direta:

| Decisão do agente | Regime |
|---|---|
| Nome, estrutura interna, teste, refatoração dentro de um módulo | decide sozinho |
| Nova dependência de propósito específico | decide sozinho e registra no diff |
| Nova dependência de propósito geral, nova fronteira entre módulos | propõe com justificativa; a pessoa aprova |
| Framework, runtime, serviço remoto, mudança no contrato JSON ou nos códigos de saída | exige ADR; o agente não decide |

Isso pertence ao `kit/CONTRATO.md` e à skill. É a peça que faltava para o kit deixar
de ser só uma lista de proibições.

---

## 4. Fitness functions executáveis propostas

Todas locais, todas em `cargo test`, nenhuma depende de rede.

Todas em `tests/governanca.rs`, todas locais, nenhuma depende de rede. Treze testes.

| # | Governa | Reprova quando |
|---|---|---|
| F1 | ADR 0001, escopo | aparece dependência fora da lista autorizada, ou a lista guarda entrada que o manifesto não usa mais |
| F2 | ADR 0006, contrato | a forma do JSON muda sem subir `REPORT_SCHEMA_VERSION`; a versão emitida diverge da constante; algum código de saída fica sem cenário vivo |
| F3 | A1, fronteira | `common.rs` conhece engine, ou surge ramo por engine fora dos cinco arquivos declarados |
| F4 | ADR 0007, quantum | aparece um `[[bin]]` fora de `sara` e `sara-observe` |
| F5 | A7, fonte única | o scanner aceita extensão que o `COMPATIBILIDADE.md` não declara |
| F6 | ADR 0005, Defold congelado | some qualquer uma das quatro fixtures ou dos dois cenários históricos |
| F7 | A7, fonte única | adapter e `COMPATIBILIDADE.md` divergem sobre as construções reconhecidas, em qualquer um dos dois sentidos; token declarado que não existe no fonte; a lista do Defold cresce |
| F8 | ADR 0011, freio do Marco 7 | o binário `sara-observe` aparece sem existir a ADR que compara o spike contra as ferramentas que já entregam a mesma unidade de evidência |

F3 é a que merece explicação. Ela não proíbe variação por engine: declara a lista dos
lugares onde ela pode existir e falha quando a lista cresce em silêncio. É a diferença
entre uma regra lembrada e um mecanismo, que é o critério do próprio `METODO §2`.

---

## 5. Plano final de implementação

O plano não muda de direção. A sequência da ADR 0001 permanece: linter, protocolo de
consulta, perfil de plataforma, SDK, e só então reavaliar engine. O que muda é a
ordem de duas coisas baratas.

### Fase 0 - governança executável - CONCLUÍDA

Quatro ADRs novas, dez fitness functions, a data de parada, a caixa de decisão do
agente, os cabeçalhos dos diários, a frase corrigida do `RESULTADO-0.1.0.md`, a matriz
ponderada fora do `ROTEIRO.md` e o diagrama. Ver seção 0.

### Fase 1 - fronteira - CONCLUÍDA

`common::BlockSyntax` no lugar do booleano, F3 guardando a fronteira, 33 testes
verdes, clippy sem aviso.

### Fase 2 - Marco 6, só Godot, sem nada em paralelo

7. Dez mudanças reais em projetos Godot, ou a data. O que vier primeiro. Pela ADR
   0005, mudança em BomberBoom Defold não conta mais; o projeto continua no corpus de
   falso positivo.
8. Nenhuma regra nova durante o marco, salvo quando uma mudança real **ou uma
   baseline de integração em projeto real** expuser regra ausente. A primeira metade é
   como a fixture da cadeia fluente do `tween_property` apareceu; a segunda foi
   acrescentada pela [ADR 0009](decisoes/0009-baseline-em-projeto-real-expoe-regra-ausente.md),
   quando a baseline do `gods` mostrou que o padrão de dono centralizado — a própria
   remediação que o `SAR-OWN-001` recomenda — virava aviso falso. Uma terceira exceção
   obriga a rever a Fase 2 inteira em vez de ampliá-la de novo.
9. O corpus Godot era quase mudo quando esta linha foi escrita: `mineboom` com zero
   declaração e `boomlitude` com quatro. A ADR 0010 mudou parte disso — o eixo de
   entrada passou a enxergar, e o `boomlitude` foi para doze, metade delas de entrada.
   O `mineboom` continua em zero. A conclusão que a linha carregava permanece de pé e
   fica mais afiada: se as dez mudanças não produzirem nenhum verdadeiro positivo em
   Godot, isso é um resultado sobre o Sara, não sobre os projetos — e agora sem a
   desculpa de que ele estava olhando com um olho só.

   *(Atualizado em 25/08/2026. Os números originais eram de antes da ADR 0010 e
   descreviam um corpus que a ferramenta ainda não conseguia ler inteiro.)*

### Portão do Marco 6

Decidir por escrito, com o `USO-PESSOAL.md` na mão, entre: manter e seguir; congelar a
ferramenta e manter só o kit; encerrar. `RESULTADO-0.1.0.md` já autoriza as três.

### Fase 3 - só se o portão aprovar

10. Medição Android. Tem poder de veto e é a evidência comparável que falta.
11. Marco 7, com `sara-observe` como binário separado desde o primeiro commit, pela
    ADR 0007. Quando o segundo binário existir, a lista de dependências autorizadas
    passa a ser por binário e o teste offline continua sendo executado contra `sara`.

A ADR 0008, sobre as gramáticas tree-sitter, foi antecipada para a Fase 0: ela
registrava uma decisão que já estava tomada de fato e custou quinze minutos.

### O que permanece adiado

Sem alteração: publicação, código aberto, itch.io, preço, licença, marca, plugin de
editor, SARIF, daemon, SDK, protocolo público, runtime e engine própria.

---

## 6. O que esta auditoria não conclui

- Não avalia a corretude das regras de posse. Isso é papel do corpus e do Marco 6.
- Não avalia mercado, preço nem disposição a pagar. A seção 9 do
  `DIAGNOSTICO-INICIAL.md` continua sendo a única análise econômica, e continua
  datada.
- Não julga qual engine é tecnicamente melhor. A ADR 0005 é uma decisão de alocação
  de esforço num projeto de uma pessoa, não um veredito sobre as engines. A medição
  Android continua sendo a evidência comparável que falta, e continua com poder de veto.
- Não recomenda construir a engine. Nada na auditoria mexe na ADR 0001.
- A ADR 0005 não diz que o Defold é pior. Diz que o esforço de um projeto solo cabe
  em uma engine, e que a prova causal do Portão 0 continua sendo Defold justamente por
  isso.
- Oito achados em um projeto com quatro ADRs, matriz de risco, corpus de cinco
  projetos e fitness functions medidas é um resultado bom. Cinco dos oito são o
  método do projeto sendo aplicado com mais rigor do que o próprio projeto aplicou.
