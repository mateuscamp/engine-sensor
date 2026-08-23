# Resultado do estudo

**Período:** 21 a 23 de agosto de 2026
**Agente:** Claude Opus 5, raciocínio extra high, uma execução por engine
**Aparelho de verificação:** Samsung Galaxy S20 FE (SM-G780G), Android 13, API 33, arm64
**Protocolo:** v2 · **Premissa:** v1.1 · **Especificação:** v1.0

Um agente construiu o mesmo jogo mobile em Godot, Unity e Defold, a partir da mesma
premissa, sem contexto entre as execuções e sem que ninguém lhe dissesse uma API. Este
documento é o resultado. Os registros completos estão em `estudo/registros/`.

---

## 1. O resultado de fundo

**As três fecharam os dois estágios. As três verificaram no mesmo aparelho físico. Nenhuma
consumiu uma única direção técnica** — o teto era três por execução, e o placar final é 0/3
nas três.

Um jogo de grade com cascata, gravidade, reposição, durabilidade em três níveis, progressão
em runs, pacote Android e anúncio integrado, em três engines diferentes, sem ninguém dizer
que API usar.

**A escolha de engine não é entre possível e impossível.** É entre formas de atrito.

---

## 2. Os números

| Medida | Godot | Unity | Defold |
|---|---|---|---|
| Versão | 4.7.2.stable | 6000.5.9f1 | 1.13.1 |
| **Até a primeira execução do jogo** | **36 min** | 52 min | **37 min** |
| **Direções técnicas consumidas** | 0 / 3 | 0 / 3 | 0 / 3 |
| Intervenções humanas | 8 | 6 | 2 |
| Ciclos fechados sem humano | 40+ | ~36 | 32+ |
| Builds com falha de compilação | 0 de 12 | 0 de 13 | 2 de 4 |
| Editor gráfico usado | sim | sim | **não** |
| Android SDK necessário | sim | sim | **não** |
| Custo medido pelo próprio agente | não mediu | não mediu | **US$ 98,91 · 319 mil tokens** |

### Repositório, mesmo jogo

| | Defold | Godot | Unity |
|---|---|---|---|
| Arquivos versionados | 82 | 83 | **365** |
| **Arquivos-sombra** | **0** | 24 `.uid` | **176 `.meta`** |
| Superfície declarativa da engine | 214 linhas / 6 arq | 57 linhas / 2 arq | 16 prefab + 22 asset |
| Código de jogo | 2 339 linhas Lua | 2 930 linhas GDScript | 49 arquivos C# |
| Média de linhas por arquivo de código | 123 | 122 | — |

### Monetização

| | Defold | Godot | Unity |
|---|---|---|---|
| Caminho | extensão oficial | plugin próprio em Java | plugin oficial do Google |
| Linhas escritas à mão | 278 | **856** | ~0 |
| Depende de serviço remoto para compilar | **sim** | não | não |
| Quando o remoto caiu | **o Estágio B parou** | — | — |

---

## 3. O que o estudo concluiu

### 3.1 O empate que mais informa

36, 37 e 52 minutos até o jogo rodar pela primeira vez. Se houvesse uma diferença
estrutural de acessibilidade entre as engines para um agente, ela apareceria aí. Não
apareceu.

### 3.2 O ciclo fechado é o eixo que decide

Godot tem `--headless` de fábrica; Defold precisou de display virtual e escreveu o dobro de
ferramenta de verificação (4 479 linhas contra 2 201). Unity é a única com framework de
teste oficial pela linha de comando — 34 testes em 21 segundos.

**Este é o único eixo com duas medições independentes concordando.** O porte
Defold→Godot feito meses antes, em `~/Godot/boomlitude`, chegou à mesma conclusão por
outro caminho e a enunciou melhor:

> É o único dos dois em que eu fecho o ciclo sozinho. No Defold, conferir qualquer coisa de
> cena termina em "compila, roda, e alguém olha" — e esse alguém é você. É aí que mora o
> prejuízo: não no código que eu escrevo, mas no que só você consegue verificar.

### 3.3 As três falham de jeitos diferentes, e o jeito importa mais que a contagem

- **Godot** — armadilhas de **versão e plataforma**. Aparecem na atualização seguinte e são
  pesquisáveis. Falha alto e opaco: `signal 11` sem mensagem, `VkResult error 5`.
- **Unity** — armadilhas de **ferramenta e ambiente**. Caras (≈3 h em emulador) mas externas
  ao código. Falha mudo: `1 item(s) failed to install` sem motivo, app congelando sem log.
- **Defold** — armadilhas de **semântica silenciosa**. Sobrevivem a todos os testes verdes e
  chegam ao jogador. As falhas de build são as mais legíveis das três (3 de 3 resolvidas em
  um ciclo), e as duas piores não produziram erro nenhum.

**Para um agente, mudo é pior que opaco.** Diante de uma mensagem ruim ele investiga; diante
de silêncio, conclui que funcionou.

### 3.4 O defeito mais caro da medição

No Defold, `MOUSE_BUTTON_LEFT` e `TOUCH_MULTI` chegavam ao mesmo tratador. No desktop só
existe o primeiro, e foi no desktop que todo o Estágio A verificou o toque. No Android o
mesmo dedo chega pelos dois: cada toque gastava duas bombas, e os cartões de fim de run
apareciam e sumiam no mesmo quadro.

**As 107 asserções continuaram verdes o tempo todo**, porque testavam a lógica, e a lógica
estava certa.

Junto com as duas `gui.animate` que se cancelavam em silêncio, formam a mesma classe:
**dois donos para um recurso, e o motor escolhendo um sem contar.**

### 3.5 Convergências — onde a lacuna é da ferramenta, não do dia

Três agentes independentes, sem contexto um do outro, fizeram a mesma coisa:

1. **Reimplementaram o gerador de números aleatórios**, porque nenhuma das três garante
   reprodutibilidade entre plataformas.
2. **Construíram um canal de narração** — o jogo publicando o próprio estado no log — porque
   nenhuma oferece introspecção do jogo rodando.
3. **Separaram as regras do motor**, para poder testá-las sem subir engine nenhuma.
4. **Não usaram o editor visual** para montar a interface. O Defold não o abriu uma vez.

Onde três agentes independentes constroem a mesma peça improvisada, ela pertence ao motor.

### 3.6 Nem tudo que o agente constrói à mão é perda

Nove reimplementações no total, três por agente, todas com justificativa escrita. A
distinção que a medição sustenta:

- **A engine ganha quando sabe algo que você não pode conferir** — ajuste de tela, escala de
  entrada, área segura, assinatura. Você não verifica sua versão contra um telefone que não
  possui.
- **A mão ganha quando você precisa de uma garantia que a engine não dá** — determinismo,
  estado serializável, tempo de vida explícito, testabilidade.
- **A duplicação deliberada é a mais sofisticada**, e parece a pior. Godot reimplementou as
  regras em Python para conferir o GDScript; Defold escreveu a detecção de grade separada da
  fórmula do jogo *de propósito*. Compartilhar código ali destruiria a verificação.

**A assimetria Godot ↔ Defold não é "um oferece mais".** O Defold entrega mais pronto e
entrega com coleira; o Godot entrega menos e o que você constrói é seu. As 856 linhas de
Java do Godot são passivo por qualquer métrica de esforço — e ativo pela única que importou
no dia em que o servidor de build não respondeu.

---

## 4. O que o estudo NÃO concluiu

**Não há nota, vencedor ou recomendação de migração.** O protocolo proíbe pontuar antes que
todas as evidências comparáveis existam, e proíbe fixar pesos depois de ler os resultados —
que é onde o estudo parou.

**Cada engine correu uma vez, com o mesmo modelo.** Um defeito visto uma vez pode ser
propriedade da ferramenta ou do dia. Só as convergências do §3.5 escapam dessa ressalva.

**Os tempos estão contaminados**, por quatro motivos registrados:

1. Godot e Unity rodaram **ao mesmo tempo**, na mesma máquina de 8 GB.
2. Só o Godot pagou o retrabalho de arte — a premissa mudou no meio da execução dele.
3. Só Godot e Unity pagaram a migração do emulador para o aparelho.
4. O Defold correu em duas máquinas, e ~3h30 da segunda sessão foram espera pelo telefone.

---

## 5. O que ficou aberto

| Pendência | Por que importa |
|---|---|
| **Execução treinada** (Defold com acesso ao histórico acumulado) | Mede se migrar custa reescrever um jogo ou **descartar um ativo**. É o número que mais pode mudar a conta, e não existe. |
| **Repetição de confirmação** | Com n=1, um dia ruim do agente é indistinguível de propriedade da engine. |
| Tokens e custo de Godot e Unity | Só o Defold mediu. Recuperável do histórico das sessões. |
| Auditoria de isolamento do Unity | O Godot foi auditado e passou. |
| Pesos e limiar de decisão | Devem ser fixados **por escrito**, e o estudo já foi lido. |

---

## 6. O gargalo real

Os quatro projetos anteriores — `gods`, `bomberboom`, `boomlitude`, `mineboom` — são
explorações paralelas de **uma pessoa só**, que alterna entre elas conforme a vontade.
Nenhum parou por dificuldade de ferramenta.

Se o limite é a atenção de uma pessoa, então o que vale mais não é o que reduz o trabalho do
agente — é **o que reduz o que só o humano consegue verificar**. Toda ida e volta para
"roda aí e me diz se ficou bom" consome o recurso mais escasso do projeto inteiro.

Isso reordena tudo: um motor que faz o agente escrever menos código vale pouco para quem tem
tempo de agente de sobra e atenção humana escassa. Um motor que faz o agente **provar mais
coisas sozinho** devolve exatamente o recurso que falta.

---

## 7. Para onde isso aponta

Os três artigos em `artigos/` desenvolvem as consequências:

1. **O Agente Não Vê** — a versão de desenvolvimento de jogos do Clean Code para agentes, com
   o `CLAUDE.md` que sai dela.
2. **O Motor que Narra** — o projeto de uma engine cujo usuário principal é um agente, com
   API, arquitetura, riscos e roteiro.
3. **O Que Só Aparece Depois** — as lições dos quatro projetos com história, que uma sessão
   não pode medir.

O primeiro passo do roteiro **não é uma engine**: é um verificador de posse que roda nos
projetos Defold e Godot que já existem, lê os arquivos, monta a tabela de quem anima o quê e
de quem trata qual entrada, e reclama quando há dois. Custa dias, e teria pego os dois
defeitos mais caros de toda a medição antes de qualquer um chegar ao aparelho.
