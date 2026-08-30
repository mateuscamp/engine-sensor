# Dossiê do portão do corpus

Registro de execução do portão que a [ADR 0012](decisoes/0012-sara-e-corpus-coevoluem.md) §3
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
| Branch | `claude/corpus-test-parametrize-4a8602`, publicada em `origin` em 30/08/2026 |
| Árvore de trabalho | limpa antes e depois de cada execução (`git status --porcelain` vazio) |

> **Limite desta revisão, dito antes que alguém a cite.** Em 30/08/2026 a branch está
> publicada — `9b5cc58` é alcançável a partir de
> `origin/claude/corpus-test-parametrize-4a8602` —, mas **ainda não a partir de
> `origin/main`**, que segue em `f62cdbe`. Quem cita por revisão cita a branch, e branch
> se apaga; a citação só fica estável quando isto entrar em `main`, e mesclar é ato do
> proprietário. Confira antes de fechar o item:
>
> ```bash
> git merge-base --is-ancestor 9b5cc58 origin/main
> ```

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
Sara: corpus APROVADO — os cinco projetos do corpus foram lidos e nenhum tem conflito bloqueante.
  bomberboom-df — /home/mateus/defold/bomberboom-df
  bomberboom-gd — /home/mateus/godot/bomberboom-gd
  boomlitude — /home/mateus/godot/boomlitude
  mineboom — /home/mateus/godot/mineboom
  gods — /home/mateus/godot/gods
```

A fotografia do corpus no dia, tirada com `sara 0.1.0` (esquema de relatório 1), somando os
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

## Os quatro modos, exercitados na mesma revisão

Portão que nunca reprovou não foi mostrado capaz de reprovar. Os quatro foram exercitados
antes de fechar, no mesmo commit e na mesma máquina descritos acima.

| Modo | Comando | Saída | Estado |
|---|---|---:|---|
| Corpus presente e íntegro | `tools/check_corpus.sh` | **0** | aprovado |
| Corpus inteiro ausente | `SARA_CORPUS_RAIZ=<diretório vazio> tools/check_corpus.sh` | **2** | inconclusivo |
| Um projeto movido ou renomeado | `SARA_CORPUS_GODS=/home/mateus/godot/gods-renomeado tools/check_corpus.sh` | **2** | inconclusivo |
| Conflito bloqueante no corpus | as cinco variáveis apontadas para fixtures, com `godot_animation_red` no lugar do `gods` | **1** | reprovado |

### Corpus inteiro ausente — saída 2

`SARA_CORPUS_RAIZ` apontado para um diretório vazio. Os cinco caminhos são nomeados, cada um
com a origem que o produziu:

```text
┌─ SARA-CORPUS: INCONCLUSIVO ─────────────────────────────────────────────
│ 5 de 5 projetos do corpus fora do lugar declarado:
│   bomberboom-df — /tmp/tmp.bV7Q3VncgB/defold/bomberboom-df (via $SARA_CORPUS_RAIZ)
│   bomberboom-gd — /tmp/tmp.bV7Q3VncgB/godot/bomberboom-gd (via $SARA_CORPUS_RAIZ)
│   boomlitude — /tmp/tmp.bV7Q3VncgB/godot/boomlitude (via $SARA_CORPUS_RAIZ)
│   mineboom — /tmp/tmp.bV7Q3VncgB/godot/mineboom (via $SARA_CORPUS_RAIZ)
│   gods — /tmp/tmp.bV7Q3VncgB/godot/gods (via $SARA_CORPUS_RAIZ)
│
│ Não poder conferir não é ter conferido. [...]
└─────────────────────────────────────────────────────────────────────────
test five_personal_projects_have_no_blocking_false_positive ... ok

Sara: corpus INCONCLUSIVO — não foi possível conferir. Faltou:
  [os cinco caminhos, de novo]
Registre isto como bloqueio externo, não como aprovação: não poder conferir não é ter conferido.
```

O bloco aparece **num `cargo test` comum**, sem `--nocapture`, porque ele é escrito no
descritor real do processo. O teste em si passa no arnês — ausência de corpus não é defeito
da Sara —, e é o código 2 do script que carrega o veredito.

### Um projeto movido ou renomeado — saída 2

É a reprodução exata da defasagem de 28/08/2026, que na época passou invisível. Os outros
quatro projetos são lidos normalmente; o que faltou é nomeado, com o caminho e a variável:

```text
│ 1 de 5 projetos do corpus fora do lugar declarado:
│   gods — /home/mateus/godot/gods-renomeado (via $SARA_CORPUS_GODS)
```

**Este é o critério que compra o conserto.** Nesta forma, a migração de `~/Godot` para
`~/godot` teria sido visível no dia em que aconteceu.

### Conflito bloqueante — saída 1

As cinco variáveis apontadas para fixtures, quatro verdes e uma vermelha no lugar do `gods`:

```text
Sara: corpus REPROVADO — o confronto encontrou conflito bloqueante:
  gods — .../tests/fixtures/godot_animation_red produziu erro(s): [Diagnostic {
    rule: "SAR-OWN-001", severity: Error,
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

- A revisão ainda não sai de `origin/main`. A branch está publicada, o que já é mais do
  que estava; a citação estável, porém, é a que sobrevive a apagar a branch.
- A parte manual da ADR 0012 §3 continua manual: **ler o diff de diagnóstico** é de quem lê,
  e nenhum teste prova leitura. Esta execução não a substitui — nesta mudança não há diff a
  ler, porque nenhuma regra, nenhum adapter e nenhum código de diagnóstico mudou.
- Corpus existe numa máquina só. O portão agora diz isso em voz alta em qualquer outra, o
  que é diferente de resolver.

## Como reproduzir

```bash
git checkout 9b5cc587da3184b4528b422fedd6e59010fcafd5
tools/check_corpus.sh; echo "saída: $?"
```

Sem o corpus na máquina, a saída esperada é **2**, e ela é o resultado correto — não uma
falha da reprodução.
