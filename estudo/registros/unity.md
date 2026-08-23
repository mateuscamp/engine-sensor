# Registro de execução — Unity

**Execução 2 de 5** · Protocolo v2 · Premissa v1.1 · Especificação v1.0

> As seções de **Custo** e a **Auditoria de isolamento** são do operador; deixei
> marcado o que não tenho como medir. Todo o resto foi preenchido por mim, o
> agente, ao fim da execução.

## Condições

| Campo | Valor |
|---|---|
| Engine | Unity |
| Versão da engine | **6000.5.9f1** (Apple Silicon), licença Unity Personal |
| Diretório | `~/engine-agent-bakeoff/unity-test` |
| Agente | Claude Opus 5, raciocínio extra high |
| Sistema | macOS (Darwin 25.6.0), 8 GB de RAM |
| Início | **2026-08-22 02:24 -03** |
| Fim | **2026-08-22 16:59 -03** |
| Encerramento | **conclusão** — A1–A19 e B1–B13 fechados, nada pendente |

**Ferramentas usadas:** editor Unity 6000.5.9f1 em batch mode; CLI oficial
`unity` (`test`, `build`, `install-modules`); Unity Hub em modo headless;
Android Build Support com SDK, NDK r27c e OpenJDK 17 do próprio editor;
`adb`/`emulator`/`avdmanager` do `android-commandlinetools`; `ffmpeg` para
recortar quadros; `git`.

## Marcos

| Marco | Quando | Observação |
|---|---|---|
| Primeira execução bem-sucedida do jogo | **03:16** | PlayMode headless: a cena sobe, o toque planta, a bomba resolve e o tabuleiro reenche — antes de existir qualquer pacote |
| Primeiro pacote Android instalado | **03:27** | no emulador; no **aparelho físico**, 09:38 |
| Estágio A fechado | **13:43** | inclui A19 conferido por clone limpo |
| Estágio B iniciado | **~13:50** | logo após a liberação |
| Estágio B fechado | **16:06** | última pendência (APK de clone limpo rodando no aparelho) fechada às **16:27** |

**Linha do tempo por commit** (10 commits, todos empurrados):

```
02:59  núcleo de regras sem UnityEngine + balanceamento medido por simulação
03:09  arte das gemas gerada por código + folha de contato e tabuleiro simulado
03:19  cena, layout, toque; A15/A16 viram teste em 4 proporções
09:18  o jogo passa a registrar no log o que faz, em célula
12:59  varredura de toque das 99 células; conserto do CanvasRenderer
13:36  verificação no aparelho; chama legível; melhor pontuação na derrota
13:43  A19 conferido clonando o repositório
15:57  Estágio B: melhoria, relíquia, barreira, anúncio
16:06  diário do Estágio B e devolução do aparelho
16:27  APK de clone limpo instalado e rodando no aparelho
```

## Custo

| Métrica | Valor |
|---|---|
| Tokens de saída | *(do operador — não tenho como medir)* |
| Custo em dólares | *(do operador)* |
| Wall-clock total | **14 h 35** de 02:24 a 16:59, **incluindo ociosidade**. Há pelo menos um intervalo longo sem nada rodando (03:19 → 09:18, ~6 h). Trabalho ativo ≈ **8 h 30** |
| — raciocínio do agente | *(não separo com confiança; o que sei medir está na linha abaixo)* |
| — espera de ferramenta | **medido por operação:** `unity test` EditMode 34 testes ≈ **21 s**; PlayMode 13 testes ≈ **40 s**; assar a arte inteira ≈ **35 s**; build Android incremental **34–79 s**; build do zero (IL2CPP) **219 s**; build de clone limpo **144 / 145 / 156 s**; download do módulo Android ≈ **12 min**; abrir o jogo no aparelho e ele registrar que subiu **8–20 s**; varredura das 99 células **≈ 4 min por proporção** |
| Ciclos editar→executar→observar→corrigir sem humano | **≈ 36**: regras e balanceamento 6, arte 5, layout e interface 4, emulador 9, aparelho e evidências 8, Estágio B 4 |
| Builds / execuções com falha | **Builds de APK com falha de compilação: 0**, em 13 builds. Um build foi morto pelo tempo limite da minha própria ferramenta, não por erro. **Instalações de módulo com falha: 2** — o NDK, pela CLI `unity`, com a mensagem `1 item(s) failed to install` e nenhum motivo; resolvido pelo Unity Hub em modo headless |

## Intervenções humanas

Literais, com categoria do Protocolo §7. **Teto de três direções técnicas
(categoria 5).** Direções usadas: **0 / 3**.

**Gatilho da mensagem de ambiente:** a informação sobre o emulador só é dada
quando o agente perguntar, igual ao que aconteceu no Godot. Não antecipar.

Nenhuma intervenção disse que API, nó, pacote ou técnica usar. Todas foram de
ambiente ou de processo.

| # | Momento | Categoria | O que o agente pediu | O que foi respondido |
|---|---|---|---|---|
| 1 | 02:2x, logo no começo | **2 — Ambiente** | nada; foi enviada de ofício | "Seu projeto tem repositório remoto: `git@github.com:mateuscamp/unity-engine-agent-bakeoff.git` … Configure o `origin`, decida o que deve e o que não deve ser versionado, e comece a commitar." |
| 2 | ~12:4x, durante a coleta de evidências | **2 — Ambiente** | eu tinha perguntado, por `AskUserQuestion`, como seguir depois de achar o pacote de outra execução nos AVDs compartilhados | "Crio AVDs próprios (recomendado)" e "Só registrar e seguir" |
| 3 | ~13:5x | **2 — Ambiente** | nada; foi enviada de ofício | "Mudança de alvo de verificação. O emulador Android saiu; a verificação passa a ser num aparelho físico conectado por adb… force as duas [proporções] por software antes de capturar… Sempre reverta ao terminar — o aparelho é de uso pessoal e não pode ficar com a resolução trocada." |
| 4 | ~13:5x | **2 — Ambiente** | eu tinha pedido, de forma específica, que o aparelho fosse conectado e a depuração USB autorizada | "vai ser por Wi-Fi (adb pair)" e, depois de eu oferecer parear, "Prefiro parear eu mesmo" → "pareado" |
| 5 | ~14:0x | **processo** | eu havia pedido o Estágio B ao fechar o A | "Estágio B liberado, pode começar" |
| 6 | 16:3x | **processo** | — | "terminou?" e "manda o resumo final pro REGISTRO.md" |

**Uma observação de isolamento que eu mesmo levantei:** no começo eu li
`results/unity-test/REGISTRO.md` — este arquivo — porque a especificação o nomeia
como evidência da minha execução. Ele continha a seção "Ambiente montado pelo
operador", com o caminho do SDK e os nomes dos AVDs, que o próprio arquivo diz
serem informação a dar só quando o agente perguntar. Avisei na hora. Não li mais
nada sob `results/`.

## Matriz de aceitação — Estágio A

Tentativas até fechar cada critério. Aplicada por observador externo, sem ler código.
Evidência por critério em `EVIDENCIAS.md`.

| # | Critério | Fechou? | Tentativas | Nota |
|---|---|---|---|---|
| A1 | O jogo roda | sim | 1 | abre direto na fase 1, sem menu |
| A2 | Toque planta bomba na célula tocada | sim | 2 | fechou na 2ª: a 1ª tabela de toque supunha área segura = tela inteira, e errava 60 px. O jogo estava certo desde o começo |
| A3 | Chama em cruz | sim | 2 | a 1ª versão da chama era uma estrela por célula e a cruz lia como bolhas soltas; virou quadrado arredondado que encosta no vizinho |
| A4 | Grupo de 3+ detona inteiro | sim | 1 | conferido por teste e pelo log em célula |
| A5 | Cadeia e combo por elo | sim | 1 | onda 1 com 13 golpes a x1, onda 3 com 4 golpes a x3 |
| A6 | Nível 2 exige 2 explosões, nível 3 exige 3 | sim | 1 | tira de quadros da mesma célula: irisada → facetada → lisa → vazia |
| A7 | Gema rica sobrevive dentro do grupo | sim | 1 | o **teste** estava errado duas vezes (eu conferia a célula depois da queda); o código, não |
| A8 | Ouro por nível removido, inclusive sem destruir | sim | 1 | 7 golpes = 7 de ouro numa explosão em que 3 gemas não morreram |
| A9 | Queda e reposição | sim | 1 | `caiu=29 nasceu=12` |
| A10 | Fase vencível e perdível | sim | 4 | 3 rodadas de rebalanceamento: na 1ª o jogador do canto **vencia** 1 em 40 |
| A11 | Tabuleiro inicial estável | sim | 1 | idêntico entre tentativas, entre emulador e aparelho, e ao desenho gerado fora do jogo |
| A12 | Grupos não marcados | sim | 1 | não existe código de marcação |
| A13 | Cinco silhuetas distinguíveis pela forma | sim | 4 | 3 defeitos corrigidos olhando a folha de contato |
| A14 | Três níveis ordenáveis | sim | 4 | idem; o pior era a dispersão do nível 3 comendo a cor base |
| A15 | Duas proporções de tela | sim | 2 | a 1ª tinha vão morto na barra de cima e fichas dimensionadas para 3 metas quando só 2 aparecem |
| A16 | Toque acerta a célula sob o dedo | sim | 2 | **99/99 em cada proporção**, mais cantos e pontos fora |
| A17 | Pacote Android instala e roda | sim | 1 | build passou de primeira; o que falhou foi instalar o NDK |
| A18 | Ciclo de vida preserva o que persiste | sim | 1 | pontos, ouro, metas e o tabuleiro inteiro voltam |
| A19 | Reprodutibilidade pela documentação | sim | 3 | dois clones limpos construídos, e o binário do terceiro instalado e rodando no aparelho |

## Matriz de aceitação — Estágio B

| # | Critério | Fechou? | Tentativas | Nota |
|---|---|---|---|---|
| B1 | A run encerra | sim | 1 | |
| B2 | Algo persiste | sim | 1 | o jogador **escolhe** qual melhoria guardar |
| B3 | A run seguinte difere | sim | 1 | fase 1 abre com 11 bombas na run 1 e 14 na run 2 |
| B4 | A diferença é percebida | sim | 1 | a fileira "GUARDADO DE RUNS ANTERIORES" só existe quando há relíquia |
| B5 | Barreira em ponto seguro | sim | 2 | fechou na 2ª: perder o foco depois da run acabar ressuscitava a run e a barreira não aparecia |
| B6 | Sem pressão repetida | sim | 1 | nenhuma menção a compra durante a demonstração |
| B7 | Derrota precoce não queima a demo | sim | 1 | conta a run que pegou melhoria **e** jogou uma fase depois |
| B8 | Titularidade persiste | sim | 1 | |
| B9 | Progresso preservado | sim | 1 | |
| B10 | Comunicação transparente | sim | 1 | o que é grátis vem antes do que custa |
| B11 | Anúncio integrado | sim | 1 | AdMob, unidade de teste oficial, no aparelho |
| B12 | Retorno do anúncio | sim | 2 | o anúncio de teste premia rápido demais; precisei fechar em ~2 s para produzir o caso "fechado antes" |
| B13 | Falha de rede | sim | 2 | 13 s até desistir, sem travar |

## Dependências adicionadas

| Nome | Versão | Origem | Institucional | Justificativa |
|---|---|---|---|---|
| `com.unity.ugui` | 2.5.0 | empacotado com o editor | extensão oficial | a interface inteira |
| `com.unity.test-framework` | 1.7.0 | empacotado com o editor | extensão oficial | rodar 47 testes sem abrir janela |
| `com.unity.modules.imageconversion` | 1.0.0 | módulo embutido | engine | `EncodeToPNG` — não reimplementei codificador de PNG |
| `com.unity.modules.jsonserialize` | 1.0.0 | módulo embutido | engine | `JsonUtility` no save |
| `com.unity.modules.ui` / `uielements` / `imgui` / `androidjni` / `screencapture` / `unitywebrequest` | 1.0.0 | módulos embutidos | engine | requisitos do uGUI, do test framework e da plataforma |
| **Google Mobile Ads Unity Plugin** | **11.4.0** | [release oficial no GitHub](https://github.com/googleads/googleads-mobile-unity/releases/tag/v11.4.0), sha256 `5eef2630…8796f1` | **plugin oficial do anunciante (Google)**, gratuito | B11 exige unidade de teste oficial de uma rede. As do AdMob servem anúncio **sem conta e sem loja** — Unity Ads exigiria Game ID de painel, com autenticação que eu não podia fazer |
| External Dependency Manager | 1.2.188 | dentro do pacote acima | dependência do plugin oficial | resolve as bibliotecas Android do AdMob |

O manifesto foi **enxugado** em relação ao que `-createProject` gera: saíram
física, animação, terreno, vídeo, partículas, XR e `multiplayer.center`.
Detalhe e justificativa de cada uma em `documentacao/DEPENDENCIAS.md`.

## Observações qualitativas

**Capacidade de observar o estado do editor e do jogo.** Alta, e foi o que
decidiu a execução. A CLI oficial dá `test` e `build` em batch, com ciclo de
**21 segundos** para 34 testes. O que faltava era enxergar o jogo *rodando*: a
captura de tela leva 2,4 s e o pavio dura 0,85 s, então não dá para fotografar
um estado transitório. Resolvi fazendo o jogo **narrar o que faz em célula** no
log do sistema — a cruz inteira, cada onda com seu multiplicador, cada golpe com
nível antes e depois. Foi a melhor decisão de verificação da sessão.

**Capacidade de verificar as próprias alterações.** Alta. 47 testes
automatizados (34 EditMode sobre um núcleo sem `UnityEngine`, 13 PlayMode com o
jogo rodando headless). O balanceamento não foi estimado: três políticas jogam
40 vezes cada fase. A arte não foi suposta: o assador emite uma folha de contato
no tamanho real da célula do aparelho e um tabuleiro simulado com o mesmo fundo,
e eu olhei os dois a cada volta. A16 virou varredura das 99 células em vez de
amostragem de quatro cantos.

**Passos manuais ainda necessários.** Para reconstruir e rodar: **nenhum**. Um
comando faz o build a partir de clone limpo, e ele reaplica toda a configuração,
importa nada e resolve as dependências do AdMob sozinho — verificado três vezes.
Para *automatizar captura* num aparelho novo, dois ajustes de aparelho, ambos
documentados no README: desligar o aviso de modo imersivo (que rouba o foco e faz
a Unity pausar) e, se for emulador, usar `-gpu host`.

**Reimplementou à mão algo que a engine já oferecia?** Três coisas, todas com
motivo escrito em `documentacao/DEPENDENCIAS.md`: as interpolações de animação
(quatro lerps curtos, contra trazer um tween engine inteiro); o layout da
interface, feito em aritmética em vez de `LayoutGroup` — de propósito, porque com
layout automático eu não teria como **afirmar em teste** que o pixel *x* pertence
à célula *c*, que é o que A16 mede; e um PRNG `xorshift32`, porque preciso de
estado serializável e reproduzível entre plataformas, o que `System.Random` não
garante. **Não** reimplementei codificador de PNG nem serialização — usei os da
engine.

**Defeitos restantes ao final.** Nenhum critério em aberto. O que sei que está
imperfeito, sem corrigir antes do retrato:

- em 20:9 sobram ~630 px verticais (o tabuleiro é limitado pela largura), que eu
  reparti entre as duas barras; o resultado é um painel folgado, com faixas
  vazias visíveis. Não mexi depois de fechar A15/A16 para não invalidar as duas
  varreduras de 99 células;
- `docs/toques.tsv` é aproximado — supõe área segura igual à tela. O script de
  captura não o usa: lê do log o retângulo que o jogo realmente calculou. Está
  escrito no README;
- `minSdk` sai **26**, não 24; a Unity 6 eleva no Android com IL2CPP e eu não fui
  atrás do motivo;
- a tela de abertura da engine não foi conferida — o build pede
  `SplashScreen.show = false`, a licença Personal pode ignorar, e eu não fui
  atrás de provar que não aparece;
- o registro detalhado de golpes fica no build; num pacote de loja eu tiraria,
  por ser verboso;
- a compra do Estágio B é **simulada** (vira estado no save), porque a
  especificação diz que nenhum critério pode depender de aprovação de loja;
- sem consentimento de privacidade (UMP), sem som, sem menu, um idioma só.

**Uma alteração que deixei no aparelho:** `immersive_mode_confirmations=confirmed`.
É o equivalente a tocar em "Entendi" naquele aviso uma vez. Resolução, densidade,
tempo de tela e DNS foram todos devolvidos ao original e conferidos.

**O que mais custou, e não era o jogo.** Cerca de três horas em armadilhas de
emulador: 3 quadros por segundo que eram o renderizador por software (no aparelho
o mesmo APK dá **59**), `screenrecord` devolvendo vídeo vazio com `-gpu host`, o
aviso de modo imersivo congelando o app sem log, e o adb caindo com dois
emuladores numa máquina de 8 GB. Detalhe em `documentacao/DIARIO.md` §7.

**Tensão entre os dois documentos.** A13 diz "a silhueta é a mesma nos três
níveis"; a §11 da premissa v1.1a diz o oposto com todas as letras — a gema
engorda, e o que identifica o tipo é a orientação e as pontas, não o contorno.
Segui a premissa, que é o documento mais novo. Registrado em
`documentacao/DIARIO.md` §9.

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
- Máquina: 8 GB de RAM; disco chegou a 2,8 GB livres e voltou a 21 GB após remover o emulador

## Auditoria de isolamento

Ao final, procurar nos transcripts os caminhos bloqueados. Vazamento invalida a
execução (Protocolo §4).

- [ ] `~/defold` não aparece
- [ ] `godot-test`, `defold-test`, `defold-test-treinado` não aparecem
- [ ] `results/` de outras execuções não aparece

**Nota do agente para esta auditoria** (não marquei nenhuma caixa — a auditoria é
sua). Dois pontos que você vai encontrar no transcript e é melhor saber de
antemão:

1. **Li este arquivo no início.** A especificação nomeia
   `results/<projeto>/REGISTRO.md` como evidência da minha execução, então eu o
   abri. Ele continha a seção "Ambiente montado pelo operador", com o caminho do
   SDK e os nomes dos AVDs — informação que o próprio arquivo diz para dar só
   quando o agente perguntar. Avisei na hora. Não abri mais nada sob `results/`.
2. **Encontrei o pacote de outra execução nos AVDs compartilhados.** Os dois AVDs
   tinham `com.bakeoff.pedraepavio` instalado, e durante uma gravação ele veio
   para a frente: o vídeo capturou a tela dele e eu vi alguns quadros antes de
   perceber que não era o meu jogo. O que vi foi uma barra escrita "BOMBAS 9" com
   pastilhas laranja. Parei, avisei, apaguei as capturas contaminadas e passei a
   usar AVDs criados por mim. Nada desse pacote foi aberto, decompilado, copiado
   ou lido de outra forma, e nada dele entrou no repositório nem nas evidências.
   O script de captura ganhou uma guarda que aborta se o app em primeiro plano
   não for o meu.
