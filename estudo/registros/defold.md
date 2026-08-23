# Registro de execução — `defold-test`

**Estágios:** A e B · **Situação:** A fechado, agora **no aparelho**. B fechado de B1 a B12; B13 fechado com a rede cortada, e a versão literal em modo avião está gravada no telefone à espera de ser recolhida (§25).
**Premissa:** `shared/PREMISSA.md` v1.1 · **Especificação:** `ESPECIFICACAO.md` v1.0
**Protocolo:** não consultado — ver a nota em §7.

---

## 0. Matriz do Estágio A, como eu a vejo

Julgamento é do operador; isto é o que eu verifiquei e onde está a evidência.

| # | Critério | Situação | Evidência |
|---|---|---|---|
| A1 | O jogo roda | fechado | `logs/`, qualquer captura |
| A2 | Toque planta bomba | fechado | `toque-planta-bomba.png`, `toque-preciso.txt` |
| A3 | Chama em cruz | fechado | `cruz-em-quatro-direcoes.png` / `.mp4`, `cruz-no-tabuleiro-inteiro.png` |
| A4 | Grupo detona inteiro | fechado | `cadeia-4-elos.png`; testes `tests/test_board.lua` |
| A5 | Cadeia e combo por elo | fechado | `cadeia-4-elos.png` / `.mp4` (elos x2, x3, x4) |
| A6 | Durabilidade | fechado | `nivel3-tres-explosoes.png` |
| A7 | Exceção do grupo | fechado | `nivel3-tres-explosoes.png`; teste dedicado |
| A8 | Ouro separado da pontuação | fechado | qualquer captura (barra de baixo); teste dedicado |
| A9 | Queda e reposição | fechado | `cadeia-4-elos.png`; teste de mil detonações |
| A10 | Vencível e perdível | fechado | `fase-vencida.png`, `fase-perdida.png`, `partida-vitoria-e-derrota.txt`, `run-completa.png` |
| A11 | Tabuleiro estável | fechado | `tabuleiro-estavel.txt` |
| A12 | Grupos não marcados | fechado | `arte-passada-1.png` — não há halo, contorno ou destaque em lugar nenhum |
| A13 | Cinco silhuetas | fechado | `arte-passada-1.png`, `arte-folha-de-contato.png` |
| A14 | Três níveis ordenáveis | fechado | idem |
| A15 | Duas proporções | fechado | `proporcao-1080x1920-16x9.png`, `proporcao-1080x2400-20x9.png`, `proporcoes.txt` |
| A16 | Toque preciso | fechado | `toque-preciso.txt` — 198 células, 0 erros |
| A17 | Pacote Android | **fechado no aparelho** | `aparelho-a17-abriu.png`, `logs/aparelho-estagio-a.log` — §25 |
| A18 | Ciclo de vida | **fechado no aparelho** | `aparelho-a18-voltou.png`, `aparelho-a18-voltou-log.png`, `ciclo-de-vida.txt` — §25 |
| A19 | Reprodutibilidade | fechado, verificado por clone limpo | §8 |

**Nada mais depende de você no Estágio A.** O que estava pendente aqui — instalar
e abrir o `.apk` num aparelho arm64 — foi feito num Galaxy S20 FE (Android 13,
API 33). Ver §25. O texto de §9 abaixo é o retrato de antes e fica como estava.


## 1. Engine, ferramentas e sistema

| | |
|---|---|
| Engine | **Defold 1.13.1** (engine sha `574678c`) |
| Ferramenta de construção | `bob.jar` 1.13.1, a linha de comando oficial da Defold. **Sem editor gráfico em momento algum.** |
| Editor | não usado, não instalado |
| Android SDK / NDK | **não usados e não necessários** — o `bob.jar` 1.13.1 traz `aapt2`, `apkc`, `zipalign`, `bundletool` e os binários de engine embutidos |
| Linguagem | Lua (LuaJIT no runtime da engine; Lua 5.1 nos testes fora dela) |
| Java | OpenJDK 25.0.3+9 (exigido: o `bob` 1.13.1 é *class file* 69) |
| Python | 3.11.15 + Pillow 12.3.0, numpy 2.4.6, scipy 1.17.1 — **só para gerar a arte e verificar** |
| Sistema | Ubuntu 24.04 (noble), kernel Linux 6.18, container **sem GPU e sem `/dev/kvm`** |
| Máquina | 4 núcleos, 15 GB de memória |
| Gráficos | Mesa 25.2.8 llvmpipe (software), perfil core 4.5, ~37 quadros por segundo a 1080×1920 |
| Início | 2026-08-22 20:19 UTC |
| Fim | 2026-08-22 22:05 UTC |

## 2. Marcos

| Hora | Marco |
|---|---|
| 20:19 | Início. Levantamento do ambiente; `d.defold.com` e `dl.google.com` descobertos bloqueados |
| 20:21 | `bob.jar` 1.13.1 obtido do espelho oficial (releases do GitHub) |
| 20:28 | Plano registrado e primeiro commit |
| 20:34 | Regras completas em Lua puro |
| 20:42 | 44 asserções passando; balanceamento provando vitória e derrota nas 3 fases |
| 20:48 | Primeira folha de contato da arte |
| 20:55 | Primeira construção do bundle Linux (após 1 falha) |
| **20:56** | **Primeira execução bem-sucedida do jogo** |
| 20:57 | Primeira captura de tela: tabuleiro 9×11 desenhado corretamente |
| 21:09 | Toque real injetado funcionando (após a investigação de §5) |
| 21:19 | Primeiro `.apk` (após 2 falhas) |
| 21:41 | Documentação e esteira de evidências completas |

**Tempo até a primeira execução bem-sucedida: 37 minutos.**

## 3. Encerramento

Encerrado por **conclusão do escopo verificável nesta máquina**, não por
bloqueio nem por estagnação. O único item que não fechou aqui — instalar e
rodar o `.apk` num aparelho — é uma impossibilidade física do ambiente
(sem `/dev/kvm`, sem acesso a `dl.google.com`), não uma parada do trabalho.
Está detalhado em §9.

## 4. Custo

| | |
|---|---|
| Tokens de saída | 319.427 |
| Tokens de entrada | 1.226 diretos, 1.205.831 escritos em cache, 71.501.884 lidos de cache |
| **Custo em dólares** | **US$ 98,91** |

Correção de registro: até o fim do Estágio A eu havia anotado que o custo em
dólares não era observável de dentro da execução. Estava errado — a consulta
de sessão do próprio ambiente devolve `cost_usd` e a contagem de tokens. Os
números acima são de 2026-08-23 02:06 UTC, cobrindo os Estágios A e B.

### Wall-clock, separado

Medido onde havia relógio (os scripts registram a própria duração) e estimado
onde não havia. Sobre 1 h 46 min de execução (20:19 → 22:05):

| Categoria | Tempo | Como foi obtido |
|---|---|---|
| Espera de ferramenta | **~48 min** | soma medida: construções do `bob` (~5 min), balanceamento e afinação (~5,5 min), geração de arte (~1,5 min), instalação de pacotes (~4 min), execuções do jogo com captura e toque injetado (~30 min), clone e reconstrução limpa para A19 (~2 min) |
| Raciocínio do agente | **~58 min** | diferença |

A maior fatia de espera é a verificação com o jogo rodando: cada toque injetado
custa o pavio mais a encenação da cadeia, e a varredura das 99 células em duas
proporções custou **6 min 24 s** cronometrados na passada final (e mais uma
passada anterior que falhou pela armadilha da tecla).

## 5. Ciclos editar → executar → observar → corrigir, sem humano

**Pelo menos 20 ciclos fechados sem nenhuma intervenção humana.** Os que
mudaram o produto:

| # | O que foi observado | O que foi corrigido |
|---|---|---|
| 1 | `bob` recusa `build/` como saída | saída movida para `bundle/` |
| 2 | `UnsupportedClassVersionError` | JDK 25 instalado |
| 3 | `·` renderizado como `~` na captura | glifo acrescentado à fonte |
| 4 | nenhum evento de toque chegava ao jogo | clique passou a ficar 300 ms pressionado |
| 5 | barra de cima fora da tela em 20:9 | layout passou a derivar do tamanho de referência da engine |
| 6 | `.apk` falha por `libdmengine.so` ausente | `--architectures arm64-android` |
| 7–9 | arco-íris do nível 3 engolindo a cor-base; aro do nível 1 invisível; lóbulo assimétrico na esmeralda | três passadas na geração de arte |
| 10 | chama nunca aparece; bomba minúscula | duas `gui.animate` na mesma propriedade se cancelavam |
| 11 | tira de quadros pegava só o depois da explosão | extração sincronizada com o pavio |
| 12 | tira de durabilidade mostrava uma gema **nova** no lugar da que saiu | busca do quadro em que a célula está vazia |
| 13 | driver travava no cartaz de fim de fase | função separada para esse clique |
| 14 | tecla `R` também não chegava | `keydown`/`keyup` separados |
| 15 | reiniciar a fase não limpava o cartaz pendente | `pending` limpo em `start_level` |
| 16 | legendas cortadas na borda das tiras | folha dimensionada pelo texto |
| 17 | `Board:clone` compartilhava o gerador de reposição | cópia por valor |
| 18–19 | fases venciam sozinhas com jogador aleatório | duas rodadas de afinação de metas e orçamento |
| 20 | evidência apagou uma função ao editar o próprio arquivo | função restaurada |

## 6. Construções e execuções com falha

| Alvo | Tentativas | Falhas | Causa |
|---|---|---|---|
| `x86_64-linux` | ~10 | **1** | pasta de saída `build/` é reservada pelo `bob` |
| `arm64-android` | 4 | **2** | engine armv7 ausente no `bob.jar`; a segunda tentativa falhou igual porque a correção não chegou a ser aplicada (erro meu de `sed`) |
| Execução do engine | ~30 | **0** | o engine nunca travou nem caiu |

Falhas de **verificação** (o jogo estava certo, o verificador não): 5 — as
linhas 4, 11, 12, 13 e 14 da tabela de §5.

## 7. Intervenções humanas

Uma, até este ponto:

> **20:22, do operador, no meio da execução (não solicitada):**
> "Seu projeto tem repositório remoto: `git@github.com:mateuscamp/defold-engine-agent-bakeoff.git`.
> Duas coisas sobre o estado atual: `~/engine-agent-bakeoff/defold-test` já é um
> repositório git, ainda sem commits. Não rode `git init` nem procure repositório
> acima dele. Esse remoto é só do seu projeto. Não empurre nada de fora de
> `~/engine-agent-bakeoff/defold-test` para lá. As evidências continuam indo
> para `~/engine-agent-bakeoff/results/defold-test/`, que é versionado à parte
> por mim e não entra no seu repositório. Configure o `origin`, decida o que
> deve e o que não deve ser versionado, e comece a commitar."

Efeito: `origin` configurado, `.gitignore` escrito, quatro commits feitos e
empurrados. **Divergência registrada:** o diretório
`~/engine-agent-bakeoff/defold-test` **não existia** nesta máquina, então eu o
criei e rodei `git init` — contrariando a instrução literal, porque a
alternativa era não ter onde trabalhar.

Nenhuma outra intervenção foi pedida nem recebida. Todo o resto — diagnóstico,
correção, balanceamento e verificação — foi fechado sem humano no meio.

**Nota sobre a categorização:** a especificação pede a categoria do Protocolo
§7 para cada intervenção. O `PROTOCOLO.md` está fora do que me foi autorizado
ler (a autorização cobre `defold-test/`, `shared/` e `ESPECIFICACAO.md`), então
a intervenção está registrada literalmente e **sem** categoria, em vez de com
uma categoria inventada.

## 8. Tentativas até fechar cada critério

| # | Critério | Tentativas | Observação |
|---|---|---|---|
| A1 | O jogo roda | 2 | 1ª falhou na pasta de saída reservada |
| A2 | Toque planta bomba | 3 | a regra acertou de primeira; a **injeção** de toque levou 2 tentativas, e a bomba estava visualmente minúscula por causa do conflito de animações |
| A3 | Chama em cruz | 3 | regra de primeira; a chama não aparecia (conflito de animações); a tira de evidência levou 2 tentativas |
| A4 | Grupo detona inteiro | 1 | fechado nos testes de primeira |
| A5 | Cadeia e combo | 2 | regra de primeira; avisos de combo se empilhavam uns sobre os outros |
| A6 | Durabilidade | 1 | de primeira |
| A7 | Exceção do grupo | 1 | de primeira |
| A8 | Ouro | 1 | de primeira |
| A9 | Queda e reposição | 1 | de primeira |
| A10 | Vencível e perdível | 3 | duas rodadas de afinação até o jogador aleatório parar de vencer |
| A11 | Tabuleiro estável | 1 | de primeira (gerador próprio desde o começo, justamente por isso) |
| A12 | Grupos não marcados | 1 | nunca houve marcação a remover |
| A13 | Cinco silhuetas | 1 | de primeira |
| A14 | Três níveis ordenáveis | 3 | duas correções: arco-íris engolindo a cor-base, aro fino demais |
| A15 | Proporção de tela | 2 | 1ª tinha o layout preso a uma constante |
| A16 | Toque preciso | 3 | 1ª não chegava evento nenhum; 2ª a varredura travava quando a fase acabava no meio. Na 3ª: **198 células testadas (99 em cada proporção), 0 erros** |
| A17 | Pacote Android | 3 | ver §6 · **fechado só em estrutura; falta aparelho** |
| A18 | Ciclo de vida | 1 | de primeira, em Linux |
| A19 | Reprodutibilidade | 1 | **verificado por mim**, ver abaixo |

### A19 verificado por clone limpo

Para não deixar a reprodutibilidade como promessa, ela foi executada: o
repositório empurrado foi clonado num diretório vazio e a documentação foi
seguida ao pé da letra, sem nenhum arquivo vindo do diretório de trabalho.

```
git clone --branch claude/defold-mobile-game-vltocy <remoto> projeto
cd projeto
tools/fetch-bob.sh          -> bob-1.13.1.jar baixado (201 MB, 2 s)
tools/test.sh               -> 32 ok + 12 ok + 12 ok, 0 falhas
tools/build.sh linux        -> bundle/linux/Cruz de Gemas/CruzdeGemas.x86_64
tools/build.sh android --release -> CruzdeGemas.apk, 3,2 MB, íntegro
```

E o jogo construído a partir do clone foi executado e sondado:

```
clone limpo -> grade 9x11 detectada nos pixels, passo 112,0, regular True
toque em 4,7 -> lido como (4, 7)
```

Ou seja: 67 arquivos versionados bastam para chegar do zero ao jogo rodando e
ao `.apk`, seguindo apenas `docs/COMO-CONSTRUIR.md`.

### A run inteira, e um teste cruzado que não estava planejado

As três fases foram jogadas de ponta a ponta com as sequências vencedoras
planejadas fora da engine, e as pontuações finais bateram **exatamente** com
as que o simulador em Lua puro havia previsto:

| Fase | Toques | Pontos previstos fora da engine | Pontos obtidos no jogo |
|---|---|---|---|
| 1 · Veio de rubi | 11 | 2430 | **2430** |
| 2 · Corte de esmeralda | 10 | 3560 | **3560** |
| 3 · Fundo de safira | 8 | 1870 | **1870** |

Isso não estava planejado como verificação, mas acabou sendo a mais forte
delas: prova que as regras que rodam no `lua5.1` de linha de comando e as que
rodam dentro do engine são **as mesmas**, gema a gema e elo a elo. Se a camada
de apresentação estivesse alterando o estado do jogo em algum ponto, os
números divergiriam.

O cartaz da terceira fase anuncia "run completa · 7860 pontos"; tocar nele
encerra a run e devolve o jogador à fase 1, com a run contada. Esse caminho
foi **executado**, não deduzido do código.

## 9. O que ficou pendente e o que exige passo manual

### Passo manual obrigatório: rodar o `.apk` num aparelho

O `.apk` foi construído, assinado (v1 + v2 + v3) e conferido estruturalmente
aqui. **Não foi instalado nem aberto num aparelho.**

Três razões, todas do ambiente:

1. não há `/dev/kvm` nesta máquina — nenhum emulador Android roda em velocidade
   utilizável;
2. `dl.google.com` está bloqueado pela política de saída de rede — nem o SDK
   nem as imagens de sistema podem ser baixados;
3. o `bob` 1.13.1 só traz engine Android para `arm64` e `armv7` — mesmo um
   emulador x86 não rodaria este pacote.

O que o operador precisa fazer:

```bash
adb install -r results/defold-test/CruzdeGemas-release.apk
adb shell am start -n com.exemplo.cruzdegemas/com.dynamo.android.DefoldActivity
adb logcat -s defold:V DEFOLD:V AndroidRuntime:E
```

Aparelho: **arm64**, Android 5.0 (API 21) ou mais novo.

### Defeituoso ou pendente ao final, sem corrigir antes do retrato

- **A18 não foi verificado no Android.** Foi verificado no Linux: fechar e
  reabrir o binário preserva `runs` e `recorde`. No Android o caminho de save é
  outro (`sys.get_save_file` resolve para o armazenamento privado do
  aplicativo) e isso não foi observado rodando.
- **O jogo não tem som.** Decisão consciente: a máquina não tem dispositivo de
  áudio, e entregar som que eu não conseguiria ouvir para verificar seria
  entregar algo não verificado.
- **Duas bombas simultâneas quase nunca coexistem na prática.** O teto de 2
  funciona, mas o toque é ignorado enquanto a explosão está sendo encenada, o
  que na maior parte do tempo deixa só uma no ar.
- **A queda é uma interpolação simples**, sem aceleração nem quique.
- **A tira de quadros da cadeia inclui uma faixa fina da barra de cima** — o
  recorte usa folga fixa. Cosmético, só afeta a evidência.
- **Duas afordâncias de verificação ficaram no build de verdade**: a tecla `R`
  (reinicia a fase) e `--config=verificacao.refill_seed=<n>` (torna a reposição
  determinística). Nenhuma das duas é alcançável por toque num aparelho.
  Documentadas em `docs/DIARIO.md` §9.

## 10. Dependências adicionadas

Tabela completa, com origem e justificativa, em
`documentacao/DECISOES.md` §10. Resumo:

| Nome | Versão | Origem | Vai para o `.apk`? |
|---|---|---|---|
| Defold `bob.jar` | 1.13.1 | **ferramenta oficial da engine** | não (é o construtor) |
| Defold builtins | 1.13.1 | **da engine** | sim |
| DejaVu Sans Bold | 2.37 | **fonte livre de terceiros** | sim |
| Arte das gemas | — | **código próprio** (`tools/gen_gems.py`) | sim |
| Regras do jogo | — | **código próprio** (`logic/`) | sim |
| OpenJDK 25 | 25.0.3 | sistema | não |
| Pillow / numpy / scipy | 12.3.0 / 2.4.6 / 1.17.1 | bibliotecas públicas | não |
| Xvfb, Mesa, ImageMagick, ffmpeg, xdotool | do Ubuntu | sistema | não |
| lua5.1 | 5.1.5 | sistema | não |

**Nenhum plugin de comunidade e nenhuma extensão nativa foi usada.**

## 11. Onde a engine já oferecia e eu reimplementei à mão

A especificação pede esse registro. Três casos, todos deliberados:

| O que | Por que não usei o da engine |
|---|---|
| `logic/rng.lua` em vez de `math.random` | O `math.random` do runtime depende da libc do alvo e não é reprodutível entre plataformas. O critério A11 exige que a fase N seja o mesmo tabuleiro **no aparelho**. |
| Agenda de eventos própria em vez de `timer.delay` | Dez linhas, roda dentro do `update` do próprio nó de GUI e morre junto com ele ao trocar de fase. Um `timer` global poderia sobreviver à fase que o criou. |
| Detecção de grade em `verificacao/grade.py` | De propósito: se o verificador calculasse a posição das células com a mesma fórmula que o jogo usa para desenhar, um erro na fórmula passaria despercebido nos dois lados. |

E um caso em que **não** reimplementei, e que era a tentação óbvia:
o acerto do toque usa `gui.pick_node`, a função da própria engine que testa um
ponto contra um nó considerando o modo de ajuste de tela e a escala de input.
Fazer essa conta à mão é o erro clássico e teria quebrado exatamente na tela
20:9 que o critério A15 exige.

## 12. Evidências neste diretório

| Arquivo | Critério |
|---|---|
| `arte-passada-1.png` | A13, A14 — captura em tamanho de tela (1080×1920), tabuleiro cheio, arte feita **só a partir do texto da §11**, sem nenhuma imagem de referência |
| `arte-folha-de-contato.png` | **suplemento** de A13/A14: os 15 sprites lado a lado, em tamanho nativo e reduzidos a 46 px, para comparar tipos e níveis fora do tabuleiro. Não substitui a captura acima. |
| `toque-planta-bomba.png` | A2 — a cruz vermelha é o pixel do clique; a bomba nasce centrada nele, inclusive nos cantos |
| `cruz-em-quatro-direcoes.png` / `.mp4` | A3 |
| `cruz-no-tabuleiro-inteiro.png` | A3 visto no tabuleiro inteiro |
| `cadeia-4-elos.png` / `.mp4` | A5 — cadeia de 4 elos com o multiplicador subindo por elo |
| `nivel3-tres-explosoes.png` | A6, A7 |
| `proporcao-1080x1920-16x9.png` | A15 |
| `proporcao-1080x2400-20x9.png` | A15 — bem mais alta que 16:9 |
| `toque-preciso.txt` | A16 — 99 células, nas duas proporções |
| `partida-vitoria-e-derrota.txt`, `fase-vencida.png`, `fase-perdida.png` | A10 |
| `tabuleiro-estavel.txt` | A11 |
| `ciclo-de-vida.png`, `ciclo-de-vida.txt` | A18 |
| `CruzdeGemas-release.apk`, `CruzdeGemas-debug.apk` | A17 |
| `run-completa.png`, `run-completa.txt` | a run inteira: fase 1 → 2 → 3 → run completa, com as três fases vencidas |
| `portabilidade-lua51-luajit.txt` | as 107 asserções passando nos **dois** interpretadores, e a impressão digital do gerador idêntica byte a byte — a `logic/` testada é a mesma que roda no aparelho |
| `b-admob-antes-e-depois.txt` | os seis defeitos do anúncio, provados: as mesmas 20 asserções contra três versões de `logic/ads.lua` |
| `logs/` | logs de execução do engine |
| `documentacao/` | tudo que foi escrito: README, plano, como construir, decisões, diário, e a fronteira do Estágio B |


---

# Estágio B

## 13. Matriz do Estágio B

| # | Critério | Situação | Evidência |
|---|---|---|---|
| B1 | A run encerra | fechado | `b-demonstracao.png` / `.txt` — perder devolve à fase 1 |
| B2 | Algo persiste | fechado | `b-demonstracao.txt` — relíquia escolhida e mantida run a run |
| B3 | A run seguinte difere | fechado | `b-run-seguinte-difere.png` — cruz de 9 células vira cruz de 13 |
| B4 | A diferença é percebida | fechado | idem, mais os dois vídeos `b-cruz-alcance-2/3.mp4` |
| B5 | Barreira em ponto seguro | fechado | `b-barreira.png` — aparece no cartão de fim de run |
| B6 | Sem pressão repetida | fechado | `b-barreira.txt` — 1 barreira no log inteiro depois de dispensada |
| B7 | Derrota precoce não queima a demo | fechado | `b-demonstracao.txt` (run 1 não conta) + teste dedicado |
| B8 | Titularidade persiste | fechado, **e no Android** | `b-desbloqueio.png` / `.txt`; `b8-save-no-android.txt` — o save em `/data/user/0/…/files/progresso`, `-rw-------` |
| B9 | Progresso preservado | fechado | idem — relíquias, runs e recorde intactos |
| B10 | Comunicação transparente | fechado | `b-barreira.png` + cartão de primeiro contato |
| B11 | Anúncio integrado | **fechado no aparelho** | `b-anuncio-no-aparelho.png` — anúncio de teste oficial do Google na `AdActivity`; `b-build-anuncio-local.log` |
| B12 | Retorno do anúncio | **fechado no aparelho** | `b-anuncio-recompensa-concedida.png` (`recompensado`) e `b-anuncio-fechado-antes-do-fim.png` (`fechado`), com anúncio de verdade; mais `b-anuncio.png` e 20 asserções |
| B13 | Falha de rede | fechado com rede cortada **no aparelho**; falta recolher a versão em modo avião | `b-sem-rede-cartao.png`, `b-sem-rede-jogavel.png` — `falhou` em 8 s e o jogo segue jogável |

## 14. O que foi construído

**Relíquias.** Ao fim de cada run o jogador escolhe uma de até três relíquias
permanentes: Estilhaço (alcance +1), Paiol (+3 bombas por fase), Estopim Duplo
(+1 bomba simultânea), Fortuna (+1 ouro por nível). O ouro da run decide
quantas opções ele tem, e morre com a run.

**A relíquia recusada.** "Veio Rico" — mais gemas ricas no tabuleiro inicial —
seria a mais visível de todas e foi descartada: quebraria o A11. Nenhuma
relíquia toca o tabuleiro inicial, e há teste que compara a assinatura das
três fases com e sem o conjunto completo no teto.

**A demonstração.** Três runs demonstrativas. Uma run só conta se o jogador
entrou nela com relíquia **e** venceu ao menos uma fase — é isso que faz uma
derrota precoce não queimar nada.

**A barreira.** Só no fim de uma run. Diz o que é grátis e o que a compra dá
antes de qualquer botão. Depois de "continuar sem desbloquear" não volta
sozinha; fica um caminho discreto para quem mudar de ideia.

**O anúncio.** Google AdMob, extensão oficial da Defold, unidades de teste
oficiais do Google. `logic/ads.lua` promete chamar o callback exatamente uma
vez, sempre, e tem prazo de 8 s — é o prazo que impede "sem conexão" de virar
tela travada.

O desenho inteiro está em `documentacao/MONETIZACAO.md`.

## 15. B11: por que o anúncio não pôde ser exibido aqui

No Defold **toda rede de anúncios é extensão nativa**, e extensão nativa não
compila localmente: o `bob` manda o projeto para um servidor de build.

| Caminho tentado | Resultado |
|---|---|
| `build.defold.com` (oficial) | **403 Forbidden** na saída de rede |
| servidor próprio em Docker | daemon sobe; download das camadas recusado em `production.cloudfront.docker.com` |
| servidor autônomo com jars | precisa dos pacotes de toolchain da Defold, mesma origem bloqueada |
| baixar a extensão do GitHub | 403 em `github.com/.../archive` e em `codeload.github.com`; contornado clonando por git e servindo o zip em HTTP local |

Com o contorno, o build **chega ao último passo** e para num ponto só
(`b-build-anuncio.log`):

```
INFO: Sending async build request to https://build.defold.com/build_async/arm64-android/574678c7…
INFO: Async build request failed with status code 403 Forbidden
```

### Segunda tentativa, a pedido do operador

Tentei de novo depois do pedido de liberação. O host continua fechado: 403 no
CONNECT às 01:58:34, 01:58:42 e 01:58:52 (`b-egresso-segunda-tentativa.txt`).
Reconferi também os dois registros de contêiner, porque um deles abriria a
porta para hospedar o servidor de build localmente: Docker Hub e GHCR
entregam o índice e **recusam as camadas**, então nenhuma imagem pode ser
baixada.

Possível explicação: a política de saída é aplicada pelo gateway no início da
sessão. Uma liberação feita depois que esta sessão começou não vale para ela.

**Liberar um único host — `build.defold.com` — resolve.** Nada no projeto
precisa mudar; o comando já existe:

```bash
tools/build.sh android --com-anuncio
```

## 16. Achado sério fora do caminho

Ao ligar a extensão nativa, descobri que **o `bob` trata tudo que está na
pasta do projeto como recurso**, e o build de extensão nativa envia a pasta
inteira para o servidor de build — incluindo os 211 MB de
`tools/bin/bob-1.13.1.jar`. Corrigido com um `.defignore`. Isso valia para o
Estágio A também e passou despercebido lá, porque sem extensão nativa nada é
enviado a lugar nenhum.

## 17. Falhas do Estágio B

Todas as falhas foram do **verificador**, nenhuma do jogo — o fluxo de telas
funcionou desde a primeira montagem. As cinco que custaram tempo:

| O que | Causa |
|---|---|
| a foto da chama desalinhava a partida | a bomba da foto gastava orçamento; passei a reiniciar a fase depois dela |
| a foto saía tarde demais | cronometrar não funciona; passei a gravar e escolher o quadro com mais alaranjado |
| a medida do alcance estava confundida | contei gemas atingidas, e um grupo instável na cruz derrubava o grupo inteiro; o jogo passou a registrar o tamanho da cruz |
| o planejador previa uma relíquia e o jogo entregava outra | deslocamento de um na semente da oferta (`runs-1` contra `runs`) |
| a espera no log estourava por motivo inexistente | `wait_for` avançava um cursor e engolia a linha procurada; agora existe `marca()` antes da ação |

E uma que eu mesmo causei no meio da correção: ao introduzir `marca()`, o
padrão passou a ler o log **depois** do clique, engolindo a linha que acabava
de chegar. Corrigido marcando antes da ação em todos os pontos.

## 18. Contagem atualizada

| | |
|---|---|
| Asserções de regra | **107** (32 tabuleiro + 12 partida + 12 layout + 31 Estágio B + **20 protocolo AdMob**), passando em `lua5.1` **e** em LuaJIT |
| Ciclos editar→executar→observar→corrigir sem humano | **pelo menos 32** |
| Construções com falha no Estágio B | 4 (3 de configuração do build com anúncio, 1 do `.defignore`) |
| Módulos novos | `logic/relics.lua`, `logic/oferta.lua`, `logic/ads.lua`, `verificacao/botoes.py`, `verificacao/evidencias_b.py` |
| Defeitos achados por verificação própria depois de "pronto" | **8** — 6 funcionais em `logic/ads.lua` (§20, §22, §23), 1 de comunicação e 1 no `build.sh` (§24) |

## 19. Pendências do Estágio B

| Critério | O que falta | O que eu preciso de você |
|---|---|---|
| B11 | exibir o anúncio de teste no aparelho | liberar `build.defold.com` **e** um aparelho arm64 |
| B12 | ver os desfechos com o anúncio de verdade | idem |
| B13 | ver a falha de rede no aparelho, em modo avião | idem |

Os quatro desfechos do anúncio já foram exercitados com o jogo rodando, pelo
simulador do próprio módulo, que percorre exatamente o mesmo caminho de
tratamento. O que falta é a exibição em si.

**Também continua pendente do Estágio A:** instalar e abrir o `.apk` num
aparelho arm64 (A17) e ver a persistência lá (A18).

## 20. Quatro defeitos achados depois de "pronto", lendo a fonte da extensão

Com `build.defold.com` fora de alcance, o tratamento dos desfechos do anúncio
estava verificado **só** contra o meu próprio simulador — o que é a mesma
suposição escrita duas vezes, não verificação. O contrato precisava vir de
fora, e a única fonte de fora disponível sem rede era o código da extensão
oficial: `extension-admob` 4.2.1, `src/java/com/defold/admob/AdmobJNI.java`.

Escrevi `tests/test_admob_protocolo.lua`: 12 asserções que substituem o
`admob` global por um dublê emitindo **as constantes numéricas literais**
daquele arquivo e as sequências que ele realmente publica.

Contra o código já commitado: **8 ok, 4 falhas.**

| # | Defeito | Consequência para o jogador |
|---|---|---|
| 1 | recompensa que chega **depois** do fechamento era descartada | assistia o anúncio inteiro e não recebia nada |
| 2 | `EVENT_LOADED` atrasado de um pedido velho abria um anúncio sozinho | anúncio surgindo sem ninguém pedir |
| 3 | o motivo do erro vem em `message.error`, não em `message.message` | toda mensagem de falha dizia "código nil" |
| 4 | o `message_id` não era conferido | evento de banner/intersticial fechava um pedido de recompensado |

O primeiro é o que dói e é o que mais custou a enxergar. Na fonte Android os
dois eventos seguem caminhos de despacho **diferentes**: o fechamento é
publicado dentro de `activity.runOnUiThread(...)` e a recompensa é publicada
fora dele. Na prática a recompensa quase sempre chega primeiro — mas "quase
sempre" não é "sempre". Correção: o fechamento não decide sozinho, abre uma
janela de meio segundo (`ads.REWARD_GRACE`) e a recompensa tem a palavra final
se chegar dentro dela.

Para não deixar isso como afirmação minha: rodei os mesmos 12 testes contra
`git show HEAD:logic/ads.lua` num diretório separado. As 4 falhas aparecem lá
e não aparecem no código corrigido — o defeito era real, não hipotético.

Isto **não** fecha B11. O tratamento agora está verificado contra o protocolo
de verdade; a exibição continua precisando do servidor e do aparelho.

## 21. A fronteira exata do Estágio B

A pedido do operador, o documento `defold-test/docs/FRONTEIRA-B.md` separa,
critério a critério, o que fica de pé sem alcançar o servidor de build e o que
fica bloqueado. O ponto que muda a decisão:

Há **duas** dependências externas e elas são **independentes**:

| | O que é | O que permite |
|---|---|---|
| **S** | `build.defold.com` | compilar a extensão nativa dentro do `.apk` |
| **D** | um aparelho arm64 | abrir qualquer `.apk` e ver qualquer coisa acontecer |

| Se chegar | O que fecha | O que continua aberto |
|---|---|---|
| **só D** | **A17, A18 e B8 no Android** | B11, B12, B13 |
| **só S** | **nada observável** | B11, B12, B13, A17, A18 |
| S **e** D | tudo | — |

Com **só o aparelho** eu fecho três critérios hoje, com o `.apk` que já está
neste diretório (`CruzdeGemas-release.apk`). Com **só o servidor** eu não fecho
nenhum: ganho um pacote melhor e continuo sem poder abri-lo.

## 22. O quinto defeito: nenhum anúncio jamais carregaria no aparelho

Continuei lendo a fonte depois de §20, agora do outro lado: não o tratamento
dos eventos que chegam, mas as chamadas que eu faço. A lista de funções que a
extensão expõe ao Lua está em `src/admob.cpp:233-255`. Eu usava quatro
(`set_callback`, `set_privacy_settings`, `load_rewarded`, `show_rewarded`) e
todas existem com as assinaturas que eu suponho.

Havia uma quinta que eu não chamava: `initialize`. E ela não era opcional.

```java
public void loadRewarded(final String unitId, ...) {
  runWhenInitialized(new Runnable() { ... RewardedAd.load(...) ... });
}
private void runWhenInitialized(Runnable task) {
  ... if (!initializationComplete) { pendingInitializationTasks.add(task); } ...
}
```

`loadRewarded` **enfileira** enquanto o SDK não inicializou. Quem esvazia a
fila é `initialize()`, chamado sozinho pelo construtor apenas quando há uma
unidade de *app open* configurada (`isAutomaticAppOpenEnabled()`) — que este
jogo não tem, porque não usa anúncio de abertura.

**Consequência no aparelho:** todo pedido de anúncio ficaria na fila para
sempre, nenhum evento voltaria, o prazo de 8 s estouraria e a tela diria "sem
conexão". Sempre, com rede perfeita.

É o pior dos cinco: é total e não intermitente; mente sobre a causa, imitando
exatamente o desfecho que eu tratei com mais cuidado; e é invisível desta
máquina, porque todo o caminho está do lado Android.

**Prova de que era real**, rodando os 16 testes contra o código anterior:

```
* pedir um anúncio carrega a unidade de teste  :: esperado 1 carregamento, obtido 0
* sem rede nenhum evento chega, e o prazo resolve :: esperado 1 carregamento, obtido 0
```

Zero carregamentos. Saída completa dos dois lados em
`b-admob-antes-e-depois.txt`.

**Correção:** `initialize()` no `setup` e de novo antes de `load_rewarded` (é
idempotente); `MSG_INITIALIZATION` tratado **antes** da guarda de "há pedido em
curso", porque a inicialização chega fora de qualquer pedido; e o dublê dos
testes passou a espelhar `runWhenInitialized`, de modo que um carregamento
pedido antes da inicialização não conta como carregamento.

Isto **não** move a fronteira do §21: B11 continua exigindo o servidor e o
aparelho. O que mudou é a chance de B11 funcionar na primeira tentativa quando
os dois chegarem, que antes era zero.

### Uma ressalva sobre como eu marquei B12 e B13

Vale explicitar, porque muda a leitura da matriz. **Pela letra dos critérios,
B12 e B13 estão fechados.** B13 pede que "sem conexão, o jogo continua jogável
e não trava esperando o anúncio", e é isso que foi observado com o jogo
rodando. B12 pede que o jogo "receba e trate o resultado do anúncio, inclusive
quando ele é fechado antes do fim", e o tratamento está verificado nos quatro
desfechos e contra o protocolo real da extensão.

A "metade que falta" que eu registro nos dois — ver os mesmos desfechos com um
anúncio de verdade, num aparelho — é uma exigência **minha**, mais dura que a
do critério. Marquei assim porque prefiro que a fronteira erre para o lado do
rigor. Avaliando pela especificação como está escrita, só **B11** fica de fora.

## 23. O sexto defeito: o prazo abortava todo anúncio de verdade

Achado relendo a minha própria correção do §20, não por um teste falhando.

`state.timer` começava a correr em `show_rewarded` e **não parava nunca**, e o
prazo era de 8 segundos. Um anúncio recompensado dura de 15 a 30 segundos por
definição — o jogador assiste até o fim para ganhar a recompensa. Logo: **todo
anúncio de verdade estouraria o prazo no meio da exibição.** O módulo entregaria
`falhou` aos 8 s, o jogo mostraria "não há anúncio agora" e seguiria — com o
anúncio ainda na tela do jogador. A recompensa que chegasse depois cairia na
guarda de "há pedido em curso" e seria descartada em silêncio.

O erro conceitual era tratar "carregar o anúncio" e "o jogador assistir o
anúncio" como uma fase só, com um relógio só. São fases com donos diferentes: a
primeira é da rede e merece prazo curto; a segunda é do jogador e não admite
prazo que eu imponha.

**Correção:** o prazo vale de "pedi" até `EVENT_OPENING` — o evento que a fonte
manda em `onAdShowedFullScreenContent` (`AdmobJNI.java:755`) — e para ali. Para
o anúncio que abre e nunca fecha, uma rede de segurança de 300 s, larga o
bastante para nenhum anúncio real encostar e curta o bastante para o jogo voltar
ao controle do jogador. Três asserções novas cobrem as três fases.

**Prova**, rodando as 20 asserções contra `bb68c64` — o commit que fechou o
Estágio B, ou seja, exatamente o código que teria ido para o aparelho se o
servidor tivesse sido liberado naquele dia:

```
Protocolo real do AdMob: 8 ok, 12 falhas
```

Saída completa das três versões em `b-admob-antes-e-depois.txt`.

### O que os seis defeitos dizem sobre o método

Seis defeitos, todos no mesmo arquivo, todos achados depois de eu ter declarado
o Estágio B fechado, e nenhum deles observável nesta máquina. Não é coincidência
que sejam todos ali: `logic/ads.lua` é o **único** módulo do projeto cuja
contraparte eu não podia executar. Todo o resto foi verificado com o jogo
rodando e toque injetado; este estava verificado contra o que eu supunha que a
extensão fazia.

Onde não dá para executar, o contrato tem que vir de fora — e a fonte da
extensão é uma fonte de fora perfeitamente boa, offline, e que eu tinha o tempo
todo.

Nada disso fecha B11: seis defeitos a menos não são um anúncio exibido. O que
mudou é que a primeira tentativa, quando o servidor e o aparelho chegarem, tem
chance de funcionar. Antes tinha zero, por dois motivos independentes e
suficientes cada um.

## 24. Mais dois, de tipos diferentes

**Uma mentira pequena na tela.** `ads.cancel()` entregava o mesmo resultado que
falhar, então quem apertava "Cancelar" enquanto o anúncio carregava via a tela
dizer *"pode ser falta de conexão"*. Não é falso por acaso — é falso sobre uma
coisa que o jogador acabou de fazer, e é justamente o que B10 (comunicação
transparente) desaconselha. Agora existe um resultado próprio, `cancelado`, e a
interface volta direto para a barreira sem inventar motivo.

**Uma armadilha nas minhas próprias ferramentas.** `tools/build.sh` escrevia
sempre em `bundle/$ALVO`, sem olhar a variante. Como o `.apk` tem o mesmo nome
nas duas, construir a depuração depois da release apagava a release **em
silêncio**, e o `publicar.sh` então copiava para cá o arquivo errado com o nome
certo. Aconteceu comigo ao reconstruir os pacotes; percebi comparando os
tamanhos, não por nenhum teste — as duas construções funcionam e o
`apk_check.py` diz "íntegro" nas duas. O defeito está em qual arquivo tem qual
nome.

Sobre o que já estava publicado aqui: estava certo. O
`CruzdeGemas-release.apk` era a release e o `-debug` era o de depuração, porque
na época construí na ordem certa e copiei entre uma construção e outra. Foi
sorte, não desenho, e por isso agora cada variante tem o seu diretório e o
`publicar.sh` avisa quando falta uma em vez de publicar em silêncio o que
sobrou.

---

# Sessão local, com aparelho e servidor de build

## 25. O que o aparelho fechou, e o que ele desmentiu

Sessão numa máquina do operador (macOS 25.6, arm64) com um **Galaxy S20 FE
(SM-G780G, Android 13, API 33, arm64-v8a, 1080×2400)** ligado por **adb sem
fio**, e com `build.defold.com` acessível. As duas coisas que faltavam desde o
começo da execução.

### Fechado aqui

| # | Como |
|---|---|
| A17 | `CruzdeGemas-release.apk` instalado, aberto e **jogado** no aparelho |
| A18 | `force-stop` e reabrir: recorde, runs e relíquias sobrevivem |
| B8 (metade Android) | save em `/data/user/0/com.exemplo.cruzdegemas/files/progresso`, 249 bytes, `-rw-------`, dono a uid do aplicativo |
| B11 | anúncio de unidade de teste oficial do Google exibido na `AdActivity` |
| B12 | os dois desfechos com anúncio de verdade: `recompensado` e `fechado` |
| B13 | com a rede cortada: `falhou detalhe=prazo de 8s esgotado` em 8 s, e o jogo segue jogável |

### Três coisas que eu tinha escrito e o aparelho desmentiu

1. **O pacote de release não imprime nada no logcat.** `FRONTEIRA-B.md` §7
   afirmava que "o log sozinho já mostra que ele rodou". O arquivo saiu com
   zero bytes: a variante `release` da Defold compila o motor com o log
   desligado. O log com marcadores veio do `CruzdeGemas-debug.apk`.
2. **`run-as` não funciona no pacote publicado**, que é assinado e não
   depurável: `run-as: package not debuggable`. A metade do B8 que faltava só
   pôde ser vista com o pacote de depuração.
3. **Todo toque era entregue duas vezes no Android** — ver abaixo.

### O defeito que só o aparelho podia mostrar

`input/game.input_binding` mapeia `MOUSE_BUTTON_LEFT → "touch"` e
`TOUCH_MULTI → "multitouch"`, e o `on_input` aceitava os dois. No desktop só
existe o primeiro, e foi no desktop, sob Xvfb, que todo o Estágio A verificou o
toque. **No Android o mesmo dedo chega pelos dois**, e o corpo inteiro do
`on_input` rodava duas vezes:

- **duas bombas por toque** — o orçamento da fase valia metade no aparelho;
- e, com cartões em sequência, a segunda entrega apertava o botão do cartão
  **seguinte**. Os cartões "FIM DA RUN" e "VOCÊ GANHOU" apareciam e sumiam no
  mesmo quadro, e o jogador nunca os via. São cartões que existem para
  comunicar, e B10 é um critério sobre comunicar.

Registrado antes de corrigir, como o protocolo pede. Corrigido depois: a engine
despacha a entrada de um quadro de uma vez, então vale a primeira entrega de
cada quadro. As 107 asserções continuam passando — elas testam `logic/`, e
`logic/` estava certo; quem chamava duas vezes era a camada de entrada.

### Defeitos nas minhas próprias ferramentas, achados fora do Linux

- `tools/build.sh` derivava o `keytool` de `dirname "$JAVA_BIN"` e procurava
  `./keytool` quando o `java` vinha do PATH. **Impedia o build.**
- Dois `trap ... EXIT` se sobrescreviam e vazavam um diretório temporário.
- `tools/test.sh` exigia o binário `lua5.1`; no Homebrew o 5.1 compatível
  chama-se `luajit`.
- `verificacao/publicar.sh` usava `stat -c%s`, que é GNU, e não conhecia a
  variante com anúncio.

### Os pacotes publicados aqui foram reconstruídos

Todos os três saíram do código **com a correção do toque**, e por isso diferem
dos que estavam aqui antes:

| Pacote | Bytes | sha256 |
|---|---|---|
| `CruzdeGemas-release.apk` | 3 224 955 | `1b4cde826fd882f7…` |
| `CruzdeGemas-debug.apk` | 3 671 419 | `46c04cfe38448d23…` |
| `CruzdeGemas-release-anuncio.apk` | 16 671 842 | `c2ae050be02aa02a…` |

As capturas de A17 e A18 foram feitas com o pacote **anterior** à correção, que
é o que estava publicado no momento do teste. O comportamento que elas mostram
— instala, abre, joga, e o progresso sobrevive — não muda com a correção; o que
muda é quantas bombas um toque gasta.

### O roteiro de mesa reproduz no aparelho

`tools/roteiro.lua` calcula as sequências de toque fora da engine. Com
`tools/build.sh --roteiro`, que assa `verificacao.refill_seed` no pacote (build
de verificação, e só), a sequência planejada venceu a fase 1 no aparelho com
**2430 pontos** — exatamente a previsão do simulador. Runs 2, 3 e 4 repetiram
2410, 2410 e 2400, cada uma igual à sua previsão.

### Intervenção humana nesta sessão

Uma, e da mesma natureza: **religar a "Depuração sem fio" no telefone**. O
Android a desliga quando o Wi-Fi cai, e o modo avião derruba o Wi-Fi. Aconteceu
uma vez durante o teste de B13 e vai acontecer de novo para recolher as
evidências que ficaram gravadas no aparelho. Com cabo USB não aconteceria.

Wall-clock desta sessão: cerca de 4h30 entre a primeira instalação e este
registro, das quais aproximadamente 3h30 foram espera pela reconexão do
aparelho, não trabalho. Custo em tokens e em dólares: não medido nesta sessão —
o operador tem os números.

### Pendente

**A versão literal de B13, com o rádio desligado.** O roteiro
`verificacao/aparelho/b13-modo-aviao.sh` rodou dentro do telefone: ligou o modo
avião, pediu o anúncio, guardou quatro capturas e o log do motor em
`/data/local/tmp/b13`, e religou o rádio. As evidências estão **no aparelho**,
ainda não recolhidas, porque o adb caiu junto com o Wi-Fi. B13 já está fechado
pelo corte de rede; isto é a confirmação com o rádio desligado.
