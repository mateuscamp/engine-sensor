# Registro de execução — Godot

**Execução 1 de 5** · Protocolo v2 · Premissa **v1.0 → v1.1 em 2026-08-22** · Especificação v1.0

> Preenchido pelo agente a pedido do operador. Onde há *(operador)*, é dado que
> não tenho como medir de dentro da sessão.

## Condições

| Campo | Valor |
|---|---|
| Engine | Godot |
| Versão da engine | **4.7.2.stable.official.ed1daf0bf** |
| Linguagem | GDScript + Java (plugin Android) |
| Diretório | `~/engine-agent-bakeoff/godot-test` |
| Remoto | `git@github.com:mateuscamp/godot-engine-agent-bakeoff.git` — 23 commits, `main` |
| Agente | Claude Opus 5, raciocínio extra high |
| Sistema | macOS (Darwin 25.6.0), Apple Silicon, 8 GB de RAM |
| Início | **2026-08-21 23:27 -03** |
| Fim | **2026-08-22 12:20 -03** |
| Encerramento | **conclusão** — Estágios A e B fechados |

## Marcos

| Marco | Quando | Observação |
|---|---|---|
| Primeira folha de arte renderizada e conferida a olho | 2026-08-21 23:43 | achou a ametista côncava logo de cara |
| Primeiro teste headless verde | 2026-08-21 23:58 | 57 asserções; a única falha era do próprio teste |
| **Primeira execução bem-sucedida do jogo** | **2026-08-22 00:03** | janela no computador, tabuleiro cheio, cruz explodindo |
| Balanceamento fechado por medição | 2026-08-22 00:14 | 4 iterações |
| Primeiro `.apk` exportado | 2026-08-22 00:17 | saiu de primeira |
| **Primeiro pacote Android instalado e rodando** | **2026-08-22 00:25** | emulador; exigiu trocar Vulkan por Compatibility |
| Arte refeita sob a premissa v1.1 | 2026-08-22 ~01:05–02:05 | sistema anterior descartado por inteiro |
| **Estágio A fechado** | **2026-08-22 03:58** | inclui A19 conferido em clone limpo do remoto |
| **Estágio B iniciado** | **2026-08-22 08:24** | |
| Plugin Android de anúncio compilando | 2026-08-22 09:08 | `.aar` próprio, build por Gradle ligado |
| Primeiro pacote instalado no **aparelho físico** | 2026-08-22 09:36 | troca de alvo |
| A15/A16/A17 refeitos no aparelho | 2026-08-22 10:28 | 9/9 de toque nas duas proporções |
| **Estágio B fechado** | **2026-08-22 12:18** | treze critérios B no aparelho |

**Tempo até a primeira execução bem-sucedida:** ~36 min.
**Tempo até o primeiro pacote rodando no aparelho (emulador):** ~58 min.

## Custo

| Métrica | Valor |
|---|---|
| Tokens de saída | *(operador)* — não mensurável de dentro da sessão |
| Custo em dólares | *(operador)* |
| Wall-clock total | **12 h 53 min** (23:27 → 12:20) |
| — raciocínio do agente | ~30% *(estimativa minha, não medição)* |
| — espera de ferramenta: emulador e aparelho | ~40% |
| — espera de ferramenta: exportação, build Gradle, instalação | ~20% |
| — espera de ferramenta: downloads (templates 1,09 GB, imagem de sistema, JDK) | ~10% |
| Ciclos editar→executar→observar→corrigir fechados sem humano | **pelo menos 40** (lista abaixo) |
| Exportações do `.apk` com falha | **0** de 12 |
| Execuções / tentativas com falha | **~31** (tabela abaixo) |

### Ciclos sem humano no meio — amostra

Arte v1.0: gota côncava, excesso de facetas, espaçamento das cópias. Regras: o
teste que falhava por defeito do próprio teste. Balanceamento: fases que se
ganhavam sozinhas → ferramenta de medição → 3 ajustes. Save gravado antes de ser
lido. `--import` sem cena principal. Erro de parse por inferência de tipo.
`class_name` sem cache. Vulkan morrendo no emulador. Atividade renomeada no
Godot 4.7. Diálogos do sistema cobrindo a tela. 1–2 fps → atlas de textura →
diagnóstico por resolução reduzida. Arte v1.1: família de silhuetas por perfil
radial, aro e dispersão (3 ajustes), bandas de faceta se fundindo, facetas finas
como cabelo. Filtro de log que escondia `[estado]`. Espera por linha velha de
`[perf]`. Layout mal distribuído em tela alta. Estágio B: painel que engolia
toque, `_enter_level` fechando painel sem registrar, orçamento de bombas
esgotado lido como toque errado, anúncio que nunca era recarregado depois de uma
falha, `[layout]` sumido do log.

### Execuções com falha

| O quê | Quantas | Causa |
|---|---|---|
| `--import` com falha grave (signal 11) | 1 | `run/main_scene` apontando para `.tscn` inexistente |
| Erro de parse do GDScript | 3 | inferência de tipo; `class_name` sem cache; transição de API interna |
| Ferramenta de arte falhou | 1 | classes globais não registradas antes do primeiro `--import` |
| Execução com tela preta no aparelho | 1 | `Couldn't present to Vulkan queue (VkResult error 5)` |
| `am start` recusado | 1 | atividade renomeada para `GodotAppLauncher` no Godot 4.7 |
| `adb install` / comando com `device offline` ou `not found` | 7 | daemon do adb caindo sob pressão de memória; depois, porta mDNS trocando |
| Emulador morto no meio da bateria | 4 | host com 7,5 de 8 GB em uso |
| Diálogo do sistema engolindo toque | 2 | confirmação de tela cheia e ANR da SystemUI |
| Captura de evidência silenciosamente vazia | 1 | três golpes não chegaram ao jogo; quatro capturas idênticas sem reclamar |
| Abertura estourando o tempo limite | 1 | 1080×2400 por software demora mais |
| Defeito no meu próprio verificador | 6 | filtro de log, espera por linha velha, orçamento, foco de janela, logcat limpo, BACK fechando o jogo |
| Aparelho inalcançável | 1 | **eu** liguei modo avião num aparelho conectado por Wi-Fi |
| **Total aproximado** | **~31** | |

## Intervenções humanas

Literais, com categoria do Protocolo §7. **Teto de três direções técnicas
(categoria 5).** Direções técnicas usadas: **0 / 3**.

> Renumerei em ordem cronológica e mantive o texto literal das que já estavam
> aqui. A correção de especificação (nova nº 2) aconteceu **entre** as duas que
> constavam antes.

| # | Momento | Categoria | O que o agente pediu | O que foi respondido |
|---|---|---|---|---|
| 1 | 2026-08-21, início | 2 — Ambiente | Qual alvo usar para verificar A17 e A15/A16 | «O alvo de verificação é emulador. / O SDK Android está em /opt/homebrew/share/android-commandlinetools. adb, emulator, sdkmanager e avdmanager estão disponíveis no terminal. / Existem dois AVDs, e são exatamente os dois que os critérios A15 e A16 exigem. Use estes, não crie outros: bakeoff-16x9 1080x1920, proporção 9:16; bakeoff-20x9 1080x2400, proporção 20:9. / Você pode iniciar, parar, instalar, desinstalar, capturar tela e ler log dos emuladores por conta própria — não precisa me pedir autorização para nenhuma dessas coisas. Se faltar algum componente do SDK, instale. / Aviso sobre a máquina: são 8 GB de RAM e o emulador roda com renderização por software. Ele vai ser lento. Se algo demorar, é o emulador, não necessariamente o seu código. / Produzir o pacote e instalá-lo no emulador é parte da sua tarefa.» |
| 2 | 2026-08-22, ~01h | *(a classificar — correção de especificação, não direção técnica)* | *(não solicitado)* | «Correção de especificação — a premissa mudou para v1.1 e a §11 (arte das gemas) foi reescrita por inteiro. Releia shared/PREMISSA.md. / A direção anterior estava errada e o erro é da especificação, não do seu trabalho: mandava expressar a riqueza repetindo cópias menores da mesma gema. Visto rodando, isso deixa vãos entre as cópias, o tabuleiro lê como esburacado, e a gema rica acaba ocupando menos célula que a pobre — o oposto do que a riqueza deveria comunicar. / A regra nova: 1) A SILHUETA identifica o tipo, mas o que a identifica é a ORIENTAÇÃO e as PONTAS, não o contorno exato. O contorno muda com o nível; a orientação não. [rubi ponta embaixo → pera; topázio lados arqueiam; esmeralda cantos viram chanfro em degraus; ametista ponta em cima, mais bojuda; safira segue círculo] O rubi aponta para baixo e a ametista aponta para cima. É só isso que impede que as duas se confundam quando ambas estão bojudas no nível 3. A orientação é inegociável; o contorno é livre para engordar. / 2) O ACABAMENTO identifica o nível, numa pedra só: nível 1 simples, não cintila; nível 2 facetada, cintila só na própria matiz; nível 3 irisada, dispersão em arco-íris. O aro engrossa a cada nível. / 3) PROIBIDO: nenhuma cópia, nenhuma repetição de peças, nenhum vão interno. Nenhuma variação de tamanho entre os níveis. / Os critérios A13 e A14 acompanharam a mudança. O A14 não é mais contagem: agora é ordenação. / Refaça a arte das gemas sob essa direção. O tempo gasto neste retrabalho será registrado à parte, porque ele paga um defeito da especificação e não conta contra você.» |
| 3 | 2026-08-22, ~02h | 2 — Ambiente | *(não solicitado)* | Repositório remoto próprio do projeto: `git@github.com:mateuscamp/godot-engine-agent-bakeoff.git`. Avisado que `godot-test` já é repositório local sem commits, que o remoto é exclusivo do próprio projeto, e que as evidências em `results/godot-test/` são versionadas à parte pelo operador. Configurar o `origin` e decidir o que versionar ficou com o agente. |
| 4 | 2026-08-22, ~08h20 | 1 — Fluxo | O agente pediu o Estágio B ao fechar o A | «Estágio B liberado, pode começar» |
| 5 | 2026-08-22, ~09h20 | 2 — Ambiente | *(não solicitado)* | «Mudança de alvo de verificação. O emulador Android saiu; a verificação passa a ser num aparelho físico, conectado por depuração sem fio e já pareado. / Use `adb devices` para achá-lo. Registre modelo e versão do Android no seu diário. / Os critérios A15, A16 e A17 precisam ser refeitos no aparelho — o que você verificou no emulador não vale mais, porque as três execuções têm de ser julgadas no mesmo alvo. O APK já existe; é reinstalar e recapturar. / A resolução nativa do aparelho é 1080x2400 (20:9). Para a outra proporção, force por software antes de capturar e reverta depois: `wm size 1080x1920` + `wm density 420`, e `reset` no fim. / Sempre reverta ao terminar — o aparelho é de uso pessoal. / Substitua as capturas correspondentes em results/godot-test/evidencias/. O tempo desta reverificação será registrado à parte: ela paga uma mudança de ambiente, não um defeito seu.» |
| 6 | 2026-08-22, ~11h50 | 3 — Desbloqueio operacional | **Sim.** O agente pediu para desligar o modo avião: ele próprio deixara o aparelho inalcançável ao rodar `airplane-mode enable` num aparelho conectado por Wi-Fi | «aparelho esta correto, conectado ao wifi e com depuração aprovada» |
| 7 | 2026-08-22, ~12h05 | 1 — Fluxo | *(não solicitado)* | «commit e push» — nada havia pendente; `main` já estava em dia com o remoto |
| 8 | 2026-08-22, ~12h25 | 1 — Fluxo | *(não solicitado)* | «atualize o REGISTRO.md com os dados finais» |

**Perguntas do agente ao operador** (via caixa de escolha, todas de ambiente ou
desbloqueio, nenhuma pedindo direção técnica): alvo de verificação Android;
aparelho físico ou emulador; forma de pareamento sem fio; como destravar o
aparelho depois do erro do modo avião.

## Matriz de aceitação — Estágio A

Tentativas até fechar cada critério. Aplicada por observador externo, sem ler código.
**Alvo final: aparelho físico Samsung SM-G780G.**

| # | Critério | Fechou? | Tentativas | Nota |
|---|---|---|---|---|
| A1 | O jogo roda | ✅ | 1 no computador, **3 no aparelho** | tela preta com Vulkan, atividade renomeada, diálogo do sistema por cima |
| A2 | Toque planta bomba na célula tocada | ✅ | 1 | um toque, uma bomba; sem arrasto |
| A3 | Chama em cruz | ✅ | 1 | teste headless confere que as 4 diagonais vizinhas ficam de fora |
| A4 | Grupo de 3+ detona inteiro | ✅ | 1 | e gema fora de grupo não abre cadeia |
| A5 | Cadeia e combo por elo | ✅ | 2 | a 1ª falha era do próprio teste; cadeia de **7 elos** no aparelho |
| A6 | Nível 2 exige 2 explosões, nível 3 exige 3 | ✅ | 1 | tira comparativa da mesma célula em 4 momentos |
| A7 | Gema rica sobrevive dentro do grupo | ✅ | 1 | |
| A8 | Ouro por nível removido, inclusive sem destruir | ✅ | 1 | ouro com moeda própria, separado da pontuação |
| A9 | Queda e reposição | ✅ | 1 | tabuleiro nunca esvazia |
| A10 | Fase vencível e perdível | ✅ | **4** | as 3 fases se ganhavam sozinhas; fechado por medição, não por chute |
| A11 | Tabuleiro inicial estável | ✅ | 2 | semente da fase 1 trocada para servir às evidências |
| A12 | Grupos não marcados | ✅ | 1 | |
| A13 | Cinco silhuetas distinguíveis pela forma | ✅ | **2 direções, ~10 iterações** | v1.0 fechada e depois refeita sob a v1.1 |
| A14 | Três níveis ordenáveis (v1.1) | ✅ | **2 direções, ~8 iterações** | contagem de cópias → ordenação por acabamento e aro |
| A15 | Duas proporções de tela | ✅ | 2 no emulador + **2 no aparelho** | 20:9 nativo e 16:9 forçado; folga vertical redistribuída |
| A16 | Toque acerta a célula sob o dedo | ✅ | 3 no aparelho | **9/9 nas duas proporções**, cantos incluídos |
| A17 | Pacote Android instala e roda | ✅ | 1 para exportar, 3 para rodar | 85 MB, arm64, build por Gradle |
| A18 | Ciclo de vida preserva o que persiste | ✅ | 3 | 1 defeito real no jogo, 2 no meu script |
| A19 | Reprodutibilidade pela documentação deixada | ✅ | 1 | clone limpo do remoto: importa, 85 testes passam, exporta, instala e roda |

## Matriz de aceitação — Estágio B

Percorrida **no aparelho, só com toques**, por `tools/verify_loja.py` e
`tools/verify_ads.py`.

| # | Critério | Fechou? | Tentativas | Nota |
|---|---|---|---|---|
| B1 | A run encerra e devolve ao começo | ✅ | 1 | `RUN ENCERRADA (bombas esgotadas)` |
| B2 | Algo persiste | ✅ | 1 | lapidação escolhida ao fim de cada run |
| B3 | A run seguinte difere | ✅ | 1 | `bombas_total` 10 → 11; alcance da cruz 2 → 3 |
| B4 | A diferença é percebida | ✅ | 1 | fileira de bolinhas maior, número em BOMBAS, chama mais longa |
| B5 | Barreira em ponto seguro | ✅ | 1 | só no retorno entre runs, ao fim da 3ª run demonstrativa |
| B6 | Sem pressão repetida | ✅ | 1 | sozinha uma vez; depois só `(a pedido)` pelo botão |
| B7 | Derrota precoce não queima a demo | ✅ | 1 | 1ª run: `demonstrativa=false, cota 0/3` |
| B8 | Titularidade persiste | ✅ | 1 | reabriu com `desbloqueado=true` |
| B9 | Progresso preservado | ✅ | 1 | `PAVIO EXTRA ×3, runs 5, recorde 2560` |
| B10 | Comunicação transparente | ✅ | 1 | aviso no primeiro contato + texto da barreira |
| B11 | Anúncio integrado | ✅ | 2 | `AdActivity` do AdMob no aparelho, criativo de teste com rótulo «Anúncio de teste» |
| B12 | Retorno do anúncio | ✅ | **5** | ver ressalva abaixo |
| B13 | Falha de rede | ✅ | 2 | ver ressalva abaixo |

**Duas ressalvas honestas sobre o anúncio:**

- **B12** — o vídeo premiado de teste do AdMob **intercepta o BACK** durante a
  reprodução e a recompensa dispara por volta dos 8 s, então não dá para fechá-lo
  cedo por tecla. O caminho "fechado antes do fim" foi exercitado **abandonando
  pelo HOME**, que é o que um jogador faz ao desistir. Os dois desfechos ficaram
  observados: `recompensa=true` assistindo até o fim, `recompensa=false`
  abandonando.
- **B13** — conferido quebrando a rede **do aplicativo** com um proxy inválido, e
  **não** com modo avião: o aparelho fala com o computador por depuração sem fio,
  e o modo avião derruba o próprio adb. É **falha de rede simulada**, e está dito
  assim em toda a documentação.

## Dependências adicionadas

| Nome | Versão | Origem | Institucional | Justificativa |
|---|---|---|---|---|
| Godot Engine | 4.7.2.stable | engine | engine | escolhida pelo enunciado |
| GDScript | da engine | engine | engine | sem etapa de compilação no ciclo de conferência |
| `Geometry2D` (`offset_polygon`, `merge_polygons`) | da engine | engine | engine | aro que engrossa para dentro; união de polígono |
| `Tween`, `SubViewport`, `RandomNumberGenerator`, `FileAccess`, `JSON` | da engine | engine | engine | animação, atlas de textura, sorteio, save |
| `DisplayServer.get_display_safe_area()` | da engine | engine | engine | recorte de câmera do aparelho |
| Templates de exportação Android | 4.7.2.stable | oficial do Godot | extensão oficial | empacota o `.apk` |
| Template de build Android (Gradle) | 4.7.2.stable | engine | engine | plugin v2 só existe nesse caminho |
| `org.godotengine:godot` | 4.7.2.stable | MavenCentral, oficial | extensão oficial | o plugin compila contra ela; entra como `compileOnly` |
| Google Mobile Ads SDK (`play-services-ads`) | 24.9.0 | Google | biblioteca de terceiro | vídeo premiado, só com unidades de **teste oficiais** |
| Android Gradle Plugin | 8.6.1 | Google | ferramenta de sistema | build do `.aar` e do aplicativo |
| Gradle | 8.11.1 | wrapper do template | ferramenta de sistema | idem |
| Android SDK build-tools / platform-tools / emulator | 36.0.0 / 37.0.1 / 37.1.11 | Google | ferramenta de sistema | assinatura, `adb`, emulador (até a troca de alvo) |
| JDK Temurin | 25.0.4 e **17.0.20.1** | Adoptium | ferramenta de sistema | 25 para `keytool`/`apksigner`; **17 obrigatório** para o build Gradle |
| Python 3, ffmpeg 9.0.1 | do sistema | — | ferramenta de sistema | scripts de conferência e montagem de evidência |
| `plugin_admob/` (plugin Android em Java) | — | — | **código próprio** | embrulha o SDK e expõe como singleton `PedraPavioAds` |
| `addons/pedrapavio_ads/` (export plugin) | — | — | **código próprio** | registra o `.aar` e a dependência Maven |
| Todo o jogo, testes e ferramentas | — | — | **código próprio** | regra, arte, interface, verificação |

**Nenhum plugin de comunidade. Nenhum asset de terceiro** — nenhuma imagem, som
ou fonte importada. As gemas são polígonos calculados em tempo de execução.

Instalados por mim: templates de exportação 4.7.2 (1,09 GB), `android-platform-tools`,
imagem de sistema `android-36` (**ficou sem uso** — os AVDs já traziam a deles),
JDK 17 pelo **tarball** da Adoptium (o cask do Homebrew pede senha de
administrador, que eu não tenho), e uma chave de depuração gerada com `keytool`.

**Configuração de máquina alterada por mim:**
`~/Library/Application Support/Godot/editor_settings-4.7.tres` (caminho do SDK e
do JDK), com cópia de segurança ao lado em `.bak-pedraepavio`. No emulador,
`immersive_mode_confirmations` e `anr_show_background`. No aparelho, nada ficou
alterado: resolução, densidade e proxy foram revertidos e conferidos.

## Observações qualitativas

- **Capacidade de observar o estado do editor e do jogo:** boa, e por caminho
  próprio. O Godot não oferece introspecção remota do jogo rodando, então o jogo
  publica no log o que a conferência precisa ver: `[layout]` (geometria do
  tabuleiro), `[toque]` (célula que recebeu a bomba), `[jogo]` (ondas, ouro,
  pontos, combo), `[estado]`/`[board]` (fase, orçamento, metas, hash e tabuleiro
  inteiro), `[meta]`/`[loja]`/`[ads]` (progressão, barreira, anúncio),
  `[overlay]` (retângulo de cada botão de painel) e `[perf]` (fps). Toda
  conferência externa lê isso por `adb logcat` e toca pelas coordenadas que **o
  jogo** publicou, nunca por número escrito no script.

- **Capacidade de verificar as próprias alterações:** alta. 85 asserções em
  runner headless escrito à mão (`--headless --script`), simulador de
  balanceamento que joga milhares de partidas, busca de semente por força bruta,
  folha de arte renderizada no tamanho de célula real para conferência a olho, e
  uma **reimplementação independente das regras em Python** que confere o
  GDScript a cada bomba — bateu em 9 de 9 explosões na sessão de vitória. A
  fraqueza apareceu no verificador, não no jogo: ele chegou a reportar "fiz"
  quando devia reportar "o alvo não confirmou", e entregou quatro capturas
  idênticas como se fossem evidência. Corrigido para conferir foco de janela,
  confirmar cada toque no log e comparar hashes.

- **Passos manuais ainda necessários:** apontar SDK, JDK 17 e chave de depuração
  nas configurações do editor (arquivo global, fora do projeto — o `README.md`
  traz as linhas exatas); gerar a chave de depuração se não existir; instalar os
  templates de exportação e o template de build Android; e conectar o aparelho.
  Nada disso é do projeto; tudo está documentado com comando pronto.

- **Reimplementou à mão algo que a engine já oferecia?** Em três lugares, todos
  deliberados: runner de teste próprio em vez do GUT (~40 linhas, evita
  dependência); interface montada em código em vez do editor de cena (o layout
  precisa responder a proporções muito distintas e eu queria a conta explícita);
  e a reimplementação das regras em Python, que é duplicação **de propósito** —
  as duas implementações se conferem. Em sentido contrário, deixei a engine
  trabalhar onde ela resolvia: `Geometry2D` para o aro, `Tween` para todas as
  animações, `SubViewport` para assar o atlas, `stretch/canvas_items` para a
  adaptação de proporção, área segura do `DisplayServer`, e o empacotador Android
  da própria engine.

- **Defeitos restantes ao final** *(sem corrigir antes do retrato)*:
  1. o `.apk` é de **depuração** e **não há integração de faturamento** — o
     desbloqueio acontece no aparelho, sem cobrança; a barreira diz isso com
     todas as letras, mas uma versão de loja precisaria do Play Billing;
  2. **85 MB** de pacote, por causa do build por Gradle (era 57 MB antes);
  3. **restos de apoio à verificação no binário entregue** — `--autoplay`,
     `--shots`, `--freshrun` e as linhas de log, inertes no Android porque nenhum
     argumento é passado, mas o log é verboso e `[board]` publica o tabuleiro
     inteiro. Num build de loja isso sairia;
  4. **sem áudio**, nenhum som e nenhuma vibração;
  5. **só português**;
  6. a folga vertical em 20:9 é grande — o tabuleiro é limitado pela largura e
     não pode crescer;
  7. a heurística do jogador "forte" na simulação é míope: em duas das três fases
     o "casual" vence mais que ela, e eu contornei mudando o portão para o melhor
     dos dois em vez de melhorar a heurística;
  8. o contador `runs_demo` segue subindo depois de esgotada a cota (`4/3`);
     inofensivo, mas desleixado;
  9. **B13 é falha de rede simulada**, não modo avião, pelo motivo já dito.

- **O que a troca de alvo revelou:** o jogo roda a **118 fps** no aparelho. Os
  1–3 fps que eu media eram inteiramente do rasterizador por software do
  emulador — confirmado medindo o mesmo build em três alvos (computador 120 fps,
  emulador 1080×1920 3 fps, emulador 540×960 23 fps).

## Ambiente montado pelo operador

Infraestrutura neutra, idêntica para as três engines. Não conta como direção técnica.

- Um repositório remoto **privado por engine**, nunca compartilhado. Remoto comum
  seria um buraco através do isolamento: um `git fetch` daria a qualquer agente o
  trabalho dos outros sem tocar em nenhum caminho bloqueado localmente.
- Cada diretório de teste é repositório git próprio, para que o git nunca suba
  até a raiz do experimento e exponha os commits das outras execuções.
- SDK Android via Homebrew `android-commandlinetools` em `/opt/homebrew/share/android-commandlinetools`
- Imagens `android-35` e `android-36`, `google_apis`, `arm64-v8a`; build-tools 36.0.0; JDK Temurin 25
- ~~AVDs `bakeoff-16x9` e `bakeoff-20x9`~~ — **emulador removido em 2026-08-22**
- Aparelho físico Samsung SM-G780G (Galaxy S20 FE 5G), Android 13, API 33, arm64-v8a, tela nativa 1080×2400 densidade 480, por depuração sem fio. Perfil 20:9 é o nativo; perfil 9:16 sai por `wm size 1080x1920` e `wm density 420`, revertido com `reset`.
- `bakeoff-16x9` verificado bootando em ~100 s, API 35, `adb exec-out screencap` devolvendo PNG 1080×1920
- `bakeoff-20x9` **não** verificado bootando: configuração idêntica exceto altura; adiado por pressão de memória
- Máquina: 8 GB de RAM; disco chegou a 2,8 GB livres e voltou a 21 GB após remover o emulador

## Auditoria de isolamento

Ao final, procurar nos transcripts os caminhos bloqueados. Vazamento invalida a
execução (Protocolo §4).

- [ ] `~/defold` não aparece
- [ ] `unity-test`, `defold-test`, `defold-test-treinado` não aparecem
- [ ] `results/` de outras execuções não aparece

**Declaração do agente** (não substitui a auditoria, que é sua): nunca li nem
procurei nenhum desses caminhos. Encontrei, sem procurar, três coisas que
registro por honestidade: os diretórios irmãos aparecem numa listagem de
`~/engine-agent-bakeoff`; a lista de AVDs mostrou `pavio-16x9` e `pavio-20x9`,
que não são meus e não usei; e `~/Library/Application Support/Godot/app_userdata`
lista pastas de outros projetos, que não abri. Só escrevi em
`results/godot-test/`, e não toquei neste `REGISTRO.md` até você pedir.

## Entregas

| Onde | O quê |
|---|---|
| `godot-test/` | projeto, 85 testes, ferramentas de conferência, documentação; 23 commits em `main`, empurrados |
| `results/godot-test/pedra-e-pavio.apk` | pacote avaliado, `sha256 cea26fa5…` (bate com `build/`) |
| `results/godot-test/evidencias/` | 40 arquivos: capturas nas duas proporções, cadeia de 7 elos quadro a quadro, durabilidade em 4 momentos, ciclo da loja, anúncio na tela, logs do aparelho, saída dos testes e do balanceamento |
| `results/godot-test/documentacao/` | cópia de tudo o que escrevi: README, DECISOES, DEPENDENCIAS, MATRIZ, DIARIO e as notas que alimentaram este registro |
