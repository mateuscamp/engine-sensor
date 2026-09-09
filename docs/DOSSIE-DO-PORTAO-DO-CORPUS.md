# Dossiê do portão do corpus

Registro de execução do portão que a [ADR 0012](decisoes/0012-o-sensor-e-o-corpus-coevoluem.md) §3
exige e que a [ADR 0017](decisoes/0017-o-portao-do-corpus-roda-sempre-e-ausencia-e-inconclusivo.md)
consertou. Quatro campos por execução — **comando, revisão, máquina e resultado** —, porque
número copiado de documento não é evidência: sem os quatro, uma execução relatada vale como
inconclusiva.

Este arquivo é acervo deste repositório. Quem estiver de fora cita **por caminho e revisão**,
como a fronteira da [ADR 0016](decisoes/0016-a-engine-sai-de-casa-antes-do-g0-e-este-repositorio-e-o-sensor.md)
manda; nada aqui depende de ler o outro lado.

---

## Execução de 29 de agosto de 2026

### Revisão

| | |
|---|---|
| Commit | `9b5cc587da3184b4528b422fedd6e59010fcafd5` |
| Assunto | *O portao do corpus para de esperar ser lembrado, e ausencia vira inconclusivo* |
| Branch | `claude/corpus-test-parametrize-4a8602`, mesclada em `main` pelo [#15](https://github.com/mateuscamp/engine-sensor/pull/15) em 30/08/2026 |
| Árvore de trabalho | limpa antes e depois de cada execução (`git status --porcelain` vazio) |

> **Esta revisão é citável.** Em 30/08/2026 `9b5cc58` passou a ser alcançável a partir de
> `origin/main`, que ficou em `679d5c4`. A distinção importa porque branch se apaga e
> commit de branch apagada some com ela; o que sobrevive é o que está no tronco. Quem
> importar este dossiê confere por conta própria, e o comando é um só:
>
> ```bash
> git merge-base --is-ancestor 9b5cc587da3184b4528b422fedd6e59010fcafd5 origin/main
> ```
>
> *(Este parágrafo já foi reescrito duas vezes em 30/08/2026: dizia "não sai de
> `origin/main`", depois "a branch está publicada, `main` não", agora isto. Cada versão
> ficou falsa em horas. É o argumento a favor de o dossiê carregar o **comando** em vez de
> carregar a resposta — a resposta envelhece, o comando não.)*

### Máquina

| | |
|---|---|
| Sistema | Arch Linux |
| Núcleo | `Linux 7.2.0-1-cachyos x86_64` |
| Máquina | `arch` — Intel Core i7-14700K |
| Toolchain | `rustc 1.96.0 (ac68faa20 2026-05-25)`, `cargo 1.96.0 (30a34c682 2026-05-25)` |
| Corpus | presente nos cinco caminhos padrão, sob `/home/mateus` |

É a máquina do proprietário, e é o único lugar onde o corpus existe. Isso não é detalhe de
rodapé: é a razão de os caminhos terem virado configuração na ADR 0017, e o motivo de o
estado **inconclusivo** existir para toda máquina que não seja esta.

### Resultado

**Aprovado.** O comando abaixo saiu **0**, e os cinco projetos foram lidos.

```text
$ tools/check_corpus.sh
engine-sensor: corpus APROVADO — os cinco projetos do corpus foram lidos e nenhum tem conflito bloqueante.
  bomberboom-df — /home/mateus/defold/bomberboom-df
  bomberboom-gd — /home/mateus/godot/bomberboom-gd
  boomlitude — /home/mateus/godot/boomlitude
  mineboom — /home/mateus/godot/mineboom
  gods — /home/mateus/godot/gods
```

A fotografia do corpus no dia, tirada com `engine-sensor 0.1.0` (esquema de relatório 1), somando os
perfis `desktop` e `android`:

| Projeto | Engine | Arquivos | Declarações | Erros | Avisos |
|---|---|---:|---:|---:|---:|
| bomberboom-df | Defold | 119 | 345 | 0 | 60 |
| bomberboom-gd | Godot | 2806 | 826 | 0 | 0 |
| boomlitude | Godot | 97 | 12 | 0 | 0 |
| mineboom | Godot | 51 | 0 | 0 | 0 |
| gods | Godot | 450 | 79 | 0 | 0 |

*Esta tabela não é comparável com a de [`RESULTADO-0.1.0.md`](RESULTADO-0.1.0.md), de
23/08/2026. O corpus cresceu — o porte tem hoje 2.806 arquivos relevantes contra 69 —, e o
eixo de entrada da [ADR 0010](decisoes/0010-canal-fisico-de-entrada-sem-mapa-de-acoes.md)
entrou no meio. O que as duas afirmam em comum, e é o que este portão julga, é a coluna de
erros: zero, então como agora.*

---

## Reexecução de 30 de agosto de 2026, no topo de `main`

A execução acima aconteceu em `9b5cc58`, que é um commit **dentro** do que foi mesclado.
Depois dele entraram o dossiê e as quatro correções do inventário P-2, e um portão só vale
onde ele está de fato. A reexecução confere isso.

| | |
|---|---|
| Comando | `tools/check_corpus.sh` |
| Revisão | `679d5c4` — `Merge pull request #15`, `origin/main` |
| Máquina | a mesma da execução acima |
| Resultado | **aprovado**, saída **0**, cinco projetos lidos, zero erro |

Nenhuma das quatro correções do P-2 toca o verificador: elas mexem em manifesto, README,
gabarito, três ADRs e um comentário, mais uma fitness function de gabarito. O resultado
igual é o esperado, e medi-lo é o que separa esperar de saber.

---

## Os quatro modos, exercitados na mesma revisão

Portão que nunca reprovou não foi mostrado capaz de reprovar. Os quatro foram exercitados
antes de fechar, no mesmo commit e na mesma máquina descritos acima.

| Modo | Comando | Saída | Estado |
|---|---|---:|---|
| Corpus presente e íntegro | `tools/check_corpus.sh` | **0** | aprovado |
| Corpus inteiro ausente | `ENGINE_SENSOR_CORPUS_RAIZ=<diretório vazio> tools/check_corpus.sh` | **2** | inconclusivo |
| Um projeto movido ou renomeado | `ENGINE_SENSOR_CORPUS_GODS=/home/mateus/godot/gods-renomeado tools/check_corpus.sh` | **2** | inconclusivo |
| Conflito bloqueante no corpus | as cinco variáveis apontadas para fixtures, com `godot_animation_red` no lugar do `gods` | **1** | reprovado |

### Corpus inteiro ausente — saída 2

`ENGINE_SENSOR_CORPUS_RAIZ` apontado para um diretório vazio. Os cinco caminhos são nomeados, cada um
com a origem que o produziu:

```text
┌─ ENGINE-SENSOR-CORPUS: INCONCLUSIVO ─────────────────────────────────────────────
│ 5 de 5 projetos do corpus fora do lugar declarado:
│   bomberboom-df — /tmp/tmp.bV7Q3VncgB/defold/bomberboom-df (via $ENGINE_SENSOR_CORPUS_RAIZ)
│   bomberboom-gd — /tmp/tmp.bV7Q3VncgB/godot/bomberboom-gd (via $ENGINE_SENSOR_CORPUS_RAIZ)
│   boomlitude — /tmp/tmp.bV7Q3VncgB/godot/boomlitude (via $ENGINE_SENSOR_CORPUS_RAIZ)
│   mineboom — /tmp/tmp.bV7Q3VncgB/godot/mineboom (via $ENGINE_SENSOR_CORPUS_RAIZ)
│   gods — /tmp/tmp.bV7Q3VncgB/godot/gods (via $ENGINE_SENSOR_CORPUS_RAIZ)
│
│ Não poder conferir não é ter conferido. [...]
└─────────────────────────────────────────────────────────────────────────
test five_personal_projects_have_no_blocking_false_positive ... ok

engine-sensor: corpus INCONCLUSIVO — não foi possível conferir. Faltou:
  [os cinco caminhos, de novo]
Registre isto como bloqueio externo, não como aprovação: não poder conferir não é ter conferido.
```

O bloco aparece **num `cargo test` comum**, sem `--nocapture`, porque ele é escrito no
descritor real do processo. O teste em si passa no arnês — ausência de corpus não é defeito
do engine-sensor —, e é o código 2 do script que carrega o veredito.

### Um projeto movido ou renomeado — saída 2

É a reprodução exata da defasagem de 28/08/2026, que na época passou invisível. Os outros
quatro projetos são lidos normalmente; o que faltou é nomeado, com o caminho e a variável:

```text
│ 1 de 5 projetos do corpus fora do lugar declarado:
│   gods — /home/mateus/godot/gods-renomeado (via $ENGINE_SENSOR_CORPUS_GODS)
```

**Este é o critério que compra o conserto.** Nesta forma, a migração de `~/Godot` para
`~/godot` teria sido visível no dia em que aconteceu.

### Conflito bloqueante — saída 1

As cinco variáveis apontadas para fixtures, quatro verdes e uma vermelha no lugar do `gods`:

```text
engine-sensor: corpus REPROVADO — o confronto encontrou conflito bloqueante:
  gods — .../tests/fixtures/godot_animation_red produziu erro(s): [Diagnostic {
    rule: "ESN-OWN-001", severity: Error,
    resource: "animation:godot:main.gd:$Bomb:scale", ...
    explanation: "dois Tweens distintos começam na mesma função sobre a mesma propriedade" }]
```

Um quinto caso foi exercitado de propósito, porque é regra que a ADR 0017 §4 escreve e
regra não exercitada é regra suposta: **com um projeto ausente e outro reprovando ao mesmo
tempo, o resultado é reprovado**, saída 1, com o ausente listado junto. Conflito comprovado
é fato; ausência é a falta de um, e a falta de um não apaga o outro.

---

## O que esta execução fecha, e o que ela não fecha

**Fecha:** o portão do corpus deste repositório foi executado nele, com resultado registrado
nos quatro campos. Não é indisponibilidade, não é aprovação implícita, e não é número
copiado de documento.

**Não fecha:**

- ~~A revisão ainda não sai de `origin/main`.~~ **Fechado em 30/08/2026** pelo
  [#15](https://github.com/mateuscamp/engine-sensor/pull/15). Fica riscado em vez de
  apagado: o limite existiu, e um dossiê que some com os próprios limites depois de
  resolvê-los ensina a confiar no que ele afirma hoje.
- A parte manual da ADR 0012 §3 continua manual: **ler o diff de diagnóstico** é de quem lê,
  e nenhum teste prova leitura. Esta execução não a substitui — nesta mudança não há diff a
  ler, porque nenhuma regra, nenhum adapter e nenhum código de diagnóstico mudou.
- Corpus existe numa máquina só. O portão agora diz isso em voz alta em qualquer outra, o
  que é diferente de resolver.

## Como reproduzir

```bash
git checkout 679d5c4   # ou 9b5cc58, a revisão da primeira execução
tools/check_corpus.sh; echo "saída: $?"
```

Sem o corpus na máquina, a saída esperada é **2**, e ela é o resultado correto — não uma
falha da reprodução.

---

## Execução de 9 de setembro de 2026 — depois do renome e da migração do contrato

A [ADR 0018](decisoes/0018-o-nome-sai-para-a-engine-e-este-produto-se-chama-engine-sensor.md)
renomeou o produto e, com ele, o contrato que vive em disco nos projetos do corpus. Um
renome que muda o nome do arquivo de configuração é a mudança mais fácil de aprovar por
engano: sem migração, o verificador não acha o contrato, cai no padrão, ignora os blocos
`[[allow]]` e **continua saindo 0**. Esta execução é o que separa migrado de parecido com
migrado.

| | |
|---|---|
| Comando | `tools/check_corpus.sh` |
| Revisão | `da2440c` — *O nome anterior sai para a engine, e este produto passa a se chamar engine-sensor*, árvore limpa antes e depois |
| Máquina | Arch Linux, `Linux 7.2.2-1-cachyos x86_64`, Intel Core i7-14700K, `rustc 1.96.0` / `cargo 1.96.0` |
| Resultado | **aprovado**, saída **0**, cinco projetos lidos, zero erro |

A migração aconteceu antes, nos quatro projetos que tinham contrato: `bomberboom-df`,
`bomberboom-gd`, `boomlitude` e `gods` passaram a ter `.engine-sensor/` e
`engine-sensor.toml`, com os ids `ESN-*` dentro. O antes de cada um está na tabela da
[ADR 0018 §1](decisoes/0018-o-nome-sai-para-a-engine-e-este-produto-se-chama-engine-sensor.md),
que é o único lugar desta árvore onde o nome antigo ainda aparece — a fitness function
`adr_0018_o_nome_anterior_nao_volta` reprovou a primeira versão deste parágrafo por
escrevê-lo aqui, e ela está certa: mapa de renome em dois lugares vira dois mapas.
`mineboom` nunca foi inicializado e não migrou.

### O diff de diagnóstico, que a ADR 0012 §3 manda ler

A parte manual não foi pulada, e desta vez ela tinha o que ler: **os avisos do
`bomberboom-df` caíram de 60, medidos em 29/08/2026, para 0.** Queda dessa ordem no dia de
um renome tem duas explicações possíveis, e elas se parecem no relatório: o projeto mudou,
ou a ferramenta parou de ver. A pergunta se responde rodando **o binário anterior ao
renome** contra a árvore de hoje:

| | claims desktop | claims android | avisos | arquivos |
|---|---:|---:|---:|---:|
| binário anterior ao renome | 357 | 392 | 0 | 141 |
| binário de `da2440c` | 357 | 392 | 0 | **139** |

Os dois concordam em tudo que é diagnóstico. A única divergência são **dois arquivos**, e
ela é explicada até o fim: são `.engine-sensor/defold/padroes_ai_first.lua` e
`portao_ai_first.lua`, que o binário novo ignora por serem o contrato e o antigo varre por
só conhecer o nome antigo da pasta. **O renome é inerte no diagnóstico, e a queda de 60
avisos é trabalho feito no projeto** — o `bomberboom-df` está sendo desenvolvido, e a
medição de 29/08 é de uma árvore que não existe mais.

### A fotografia do corpus no dia

Somando os perfis `desktop` e `android`, com `engine-sensor 0.1.0` (esquema de relatório 1):

| Projeto | Engine | Arquivos | Declarações | Erros | Avisos |
|---|---|---:|---:|---:|---:|
| bomberboom-df | Defold | 139 | 357 | 0 | 0 |
| bomberboom-gd | Godot | 1552 | 420 | 0 | 0 |
| boomlitude | Godot | 97 | 12 | 0 | 0 |
| mineboom | Godot | 51 | 0 | 0 | 0 |
| gods | Godot | 2108 | 381 | 0 | 0 |

*As três primeiras colunas não são comparáveis com a tabela de 29/08/2026, e não é ruído de
medição: os projetos mudaram entre uma data e outra — o `gods` cresceu de 450 para 2.108
arquivos relevantes, o `bomberboom-gd` encolheu de 2.806 para 1.552. O que este portão
julga, e o que as duas tabelas afirmam em comum, é a coluna de erros: zero antes, zero
agora.*

### O que esta execução fecha, e o que ela não fecha

**Fecha:** a migração do contrato foi conferida por reexecução, e não por intenção. E o
renome foi mostrado inerte por comparação com o binário anterior, que é diferente de ser
declarado inerte pela ADR que o fez.

**Não fecha:**

- Os quatro projetos migraram **na árvore de trabalho**; nenhum commit foi feito nos
  repositórios deles. Enquanto essa mudança não for commitada, um `git checkout` naqueles
  projetos desfaz a migração e o contrato volta a não ser encontrado — em silêncio, com
  saída 0.
- Os documentos históricos daqueles projetos (carimbos, auditorias, diários) continuam
  citando o comando pelo nome antigo. São registros deles, não deste acervo, e reescrevê-los
  não estava no escopo desta decisão.
- Corpus continua existindo numa máquina só. Nada aqui muda isso.

### Como reproduzir

```bash
git checkout da2440c
tools/check_corpus.sh; echo "saída: $?"
```

Sem o corpus na máquina, a saída esperada é **2**, e ela é o resultado correto.
