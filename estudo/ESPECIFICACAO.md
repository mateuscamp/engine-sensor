# Especificação e matriz de aceitação

**Versão:** 1.0 · **Protocolo:** v2
**Premissa do produto:** `shared/PREMISSA.md` v1.1

Este documento define **como se verifica** o que a premissa exige. Ele não
descreve implementação.

## Regra de ouro da matriz

**Todo critério é verificável por um observador externo, jogando ou olhando o
jogo, sem ler o código.** Um critério que só se confirma abrindo o fonte está
mal escrito e deve ser reescrito, não contornado.

A régua não muda durante o experimento. Se um critério revelar defeito real,
versiona-se a especificação e registra-se quais execuções precisam ser repetidas
(Protocolo §8).

---

## Estágio A — o jogo e o pacote

Portão do experimento. Só quem fecha **todos** os critérios A entra no Estágio B.

| # | Critério | Como se verifica |
|---|---|---|
| A1 | O jogo roda | Abre e chega à primeira fase jogável, sem passo manual não documentado |
| A2 | Toque planta bomba | Tocar numa célula põe a bomba **naquela** célula; sem arrasto e sem seleção prévia |
| A3 | Chama em cruz | A explosão atinge as quatro direções ortogonais, com alcance visível; nunca a diagonal |
| A4 | Grupo detona inteiro | Chama que alcança gema de grupo de 3+ derruba o grupo todo; gema fora de grupo é atingida sozinha, sem combo |
| A5 | Cadeia e combo | Grupo instável ortogonalmente vizinho ao que saiu acende na onda seguinte; o multiplicador sobe **por elo**, não por gema |
| A6 | Durabilidade | Gema nível 2 exige **duas** explosões e nível 3 exige **três**; ela empobrece visivelmente a cada golpe |
| A7 | Exceção do grupo | Gema rica dentro de um grupo que detona **permanece** no lugar, um nível abaixo, em vez de sair |
| A8 | Ouro | Cada nível removido paga 1 de ouro, **inclusive** no golpe que não destruiu a gema; o ouro é exibido separado da pontuação |
| A9 | Queda e reposição | Terminada a cadeia, o que sobrou cai e peças novas entram por cima; o tabuleiro nunca esvazia |
| A10 | Fase vencível e perdível | Uma sessão de jogo demonstra a vitória por metas e a derrota por bombas esgotadas |
| A11 | Tabuleiro estável | Perder e repetir a fase devolve o mesmo tabuleiro inicial |
| A12 | Grupos não marcados | Nenhum halo, contorno ou destaque indica quais gemas formam grupo |
| A13 | Cinco silhuetas | Numa captura em tamanho de tela, um observador nomeia o tipo de cada célula **pela forma**; a silhueta é a mesma nos três níveis |
| A14 | Três níveis ordenáveis | Dadas três gemas do mesmo tipo, o observador as ordena da mais pobre para a mais rica e diz qual é 1, 2 e 3, guiando-se por lisa → facetada → irisada e pela espessura do aro |
| A15 | Proporção de tela | Tabuleiro e controles corretos em **duas** proporções, uma delas bem mais alta que 16:9 |
| A16 | Toque preciso | Em ambas as proporções, o toque acerta a célula sob o dedo, inclusive nas bordas do tabuleiro |
| A17 | Pacote Android | Um `.apk` gerado pelo agente instala e roda em aparelho ou emulador |
| A18 | Ciclo de vida | Sair do jogo e voltar não corrompe nem perde o que devia persistir |
| A19 | Reprodutibilidade | Outra pessoa reproduz build e execução seguindo apenas o que o agente deixou escrito |

### A arte em duas passadas

A arte das gemas é medida **duas vezes**, e as duas passadas são evidência
separada. O objetivo é comparar como cada engine — e cada agente — lida com uma
direção de arte dada **por texto** e com a mesma direção dada **por imagem**.

**Passada 1 — só texto.** O agente produz a arte a partir da §11 da premissa, sem
nunca ter visto imagem de referência. Os critérios A13 e A14 são julgados aqui.
A captura desta passada é **congelada como evidência antes de qualquer imagem ser
fornecida**.

**Passada 2 — com imagem.** Depois de A13 e A14 fecharem e a captura estar
guardada, o operador fornece as imagens de referência e o agente refaz a arte.
Nova captura, guardada ao lado da primeira.

Regras que mantêm a comparação válida:

- as imagens são fornecidas **no mesmo ponto** das três execuções: depois de A13
  e A14 fecharem por texto, nunca antes;
- as três recebem **as mesmas imagens** e o mesmo texto de acompanhamento;
- **o agente não é avisado da passada 2 antes dela começar** — saber que a
  referência visual está a caminho o convidaria a economizar esforço na primeira,
  e a passada 1 deixaria de medir o que existe para medir;
- a passada 2 **não** altera o resultado de A13 e A14, que já foram julgados.

Evidência: `arte-passada-1.png` e `arte-passada-2.png` em `results/<projeto>/`,
mais o custo de cada passada registrado separadamente.

**Fora do escopo do Estágio A**, por decisão de escopo e não por serem
irrelevantes: barra de fúria, bomba secundária de grupo grande, obstáculos de
geometria e múltiplos idiomas. Um agente que os construa por conta própria não é
penalizado, mas o custo disso é registrado.

---

## Estágio B — a monetização

| # | Critério | Como se verifica |
|---|---|---|
| B1 | A run encerra | Perder uma fase encerra a run e devolve o jogador ao começo |
| B2 | Algo persiste | Depois da run, o jogador recebe, escolhe ou usa algo que permanece |
| B3 | A run seguinte difere | A nova run apresenta consequência, possibilidade ou configuração diferente da anterior |
| B4 | A diferença é percebida | Um jogador que não recebeu explicação consegue dizer o que mudou entre as duas runs |
| B5 | Barreira em ponto seguro | A barreira aparece no retorno após uma run ou no fim do conteúdo demonstrativo; **nunca** durante uma partida |
| B6 | Sem pressão repetida | O jogo não reapresenta a oferta insistentemente durante a demonstração |
| B7 | Derrota precoce não queima a demo | Perder antes de conhecer uma melhoria e sua consequência não consome a demonstração |
| B8 | Titularidade persiste | O estado de desbloqueio sobrevive a fechar e reabrir o aplicativo |
| B9 | Progresso preservado | Desbloquear preserva o que foi obtido na demonstração; não recomeça |
| B10 | Comunicação transparente | O jogo diz, antes da barreira, o que é gratuito e o que a compra desbloqueia |
| B11 | Anúncio integrado | Um anúncio de **unidade de teste oficial** da rede escolhida é exibido no aparelho |
| B12 | Retorno do anúncio | O jogo recebe e trata o resultado do anúncio, inclusive quando ele é fechado antes do fim |
| B13 | Falha de rede | Sem conexão, o jogo continua jogável e não trava esperando o anúncio |

Nenhum critério do Estágio B depende de aprovação de loja ou de propagação
externa. Se algo assim aparecer, é medido separadamente (Protocolo §12).

---

## Evidências obrigatórias por execução

Depositadas em `results/<projeto>/`:

1. `arte-passada-1.png` — captura em tamanho real com o tabuleiro cheio, feita a
   partir da direção em texto, usada para julgar A13 e A14;
1b. `arte-passada-2.png` — a mesma captura depois de o agente receber as imagens
   de referência;
2. gravação ou sequência de capturas de uma cadeia com pelo menos 3 elos;
3. gravação ou sequência mostrando uma gema de nível 3 sobrevivendo duas
   explosões e saindo na terceira;
4. capturas nas duas proporções exigidas por A15;
5. o `.apk` avaliado, ou o comando exato que o reconstrói;
6. o log da execução no aparelho;
7. tudo que o agente escreveu como documentação;
8. o registro de execução (`results/<projeto>/REGISTRO.md`).

---

## Registro de execução

Cada execução mantém um `REGISTRO.md` com, no mínimo:

- engine, versão, ferramentas, sistema operacional, data e hora de início e fim;
- versão da premissa, da especificação e do protocolo usadas;
- tempo até a primeira execução bem-sucedida;
- tempo total até o encerramento, e se ele foi por conclusão, bloqueio ou
  estagnação (Protocolo §5.1);
- custo em tokens e em dólares;
- wall-clock separado em **raciocínio do agente** e **espera de ferramenta**
  (compilação, importação, abertura de editor, instalação no aparelho);
- número de ciclos editar → executar → observar → corrigir fechados **sem
  humano no meio**;
- builds e execuções com falha;
- toda intervenção humana, literal, com a categoria do Protocolo §7;
- tentativas até fechar cada critério da matriz;
- dependências adicionadas: nome, versão, origem, justificativa, e se são da
  engine, extensão oficial, plugin de comunidade ou código próprio;
- o que ficou defeituoso ou pendente ao final, **sem corrigir antes do retrato**;
- passos manuais ainda necessários;
- ocasiões em que o agente reimplementou à mão algo que a engine já oferecia.
