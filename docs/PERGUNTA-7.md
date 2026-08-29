# A pergunta 7 — imagem mais estado diagnostica melhor que imagem isolada?

**Data:** 29 de agosto de 2026
**Estado:** contrafactual registrado, **não** fitness function cumprida
**Autorizado por:** [ADR 0012 §4](decisoes/0012-sara-e-corpus-coevoluem.md) — contrafactual é
permitido, é datado, e **nunca reescreve o resultado original do caso**

A [ADR 0004](decisoes/0004-spike-de-visao-instrumentada-em-godot.md) tinha sete fitness
functions. Seis eram infraestrutura, e o Marco 7 as entregou sem ter sido planejado — a
Sentinela e o portão de cena do porte, pelo adendo da
[ADR 0014](decisoes/0014-comparacao-do-marco-7-com-as-ferramentas-existentes.md).

A sétima é a única que produz conhecimento em vez de aparelho:

> 7. imagem mais estado produz diagnóstico mais preciso que imagem isolada em pelo menos
>    um caso previamente definido.

Este documento a responde com a única medição disponível, e diz com a mesma clareza o que
ela **não** responde.

---

## Por que isto é contrafactual e não a fitness function cumprida

**A cláusula "previamente definido" não foi satisfeita, e é o ponto que decide o rótulo.**
O [caso da aranha](CASO-DA-ARANHA.md) não foi montado para responder esta pergunta: ele foi
uma tarefa de jogo, de 28/08/2026, e a pergunta só foi reconhecida nele depois. Caso
escolhido depois de a evidência existir é caso escolhido pelo resultado, e é exatamente o
que a cláusula existe para impedir.

Chamar isso de "fitness function 7 cumprida" seria a forma mais barata de fingir que um marco
fechou bem. Ela fica **contrafactual**, com data, e o que ela vale é o que estiver escrito
abaixo — nem mais.

## O caso teve quatro instrumentos, e ninguém os montou para comparar

A aranha que rouba a bomba foi construída, mergeada e **não funcionava em condição nenhuma**.
Quatro instrumentos olharam para ela, e por acaso eles cobrem exatamente as categorias que a
comparação da ADR 0004 pedia.

| | instrumento | o que é | o que ele disse |
|---|---|---|---|
| A | **Sentinela** | imagem **com referência** — 25 telas gravadas, `--fixed-fps 60`, assinatura dos arquivos que desenham | **verde: 0 de 25 telas mudaram** |
| B | **sonda de tira** | imagem **crua**, sem referência — 10 quadros do momento, lado a lado, 4797×583 px | **inconclusivo** |
| C | **medição de domínio** | **estado puro**, com o pavio de produção, em tiques de 1/60 | **0 de 36 roubos completam** |
| D | **`PresencaNaCena`** | **imagem contra estado** — o que o estado tem está na tela, e o que está na tela o estado conhece | **`nós=1, carregadoras=1, órfãos=1` para sempre** |

## Achado 1 — no defeito que decidia se a peça existia, a imagem não foi menos precisa: foi cega

O mecanismo era uma corrida entre dois relógios. Pavio de 2,20 s; andar até a bomba até
0,50; tecer 0,50; a carregadora descer 0,35; subir 0,20 por célula, e sete células até a
borda são 1,40. Sobravam 0,85 s. **Não era balanceamento apertado, era impossível**, e a
conta cabe em duas linhas.

- **A (imagem com referência) foi verde, e não podia ser outra coisa.** Não há entre as 25
  telas nenhum instante de aranha-com-bomba; e se houvesse, a referência teria sido gravada
  já contendo o defeito. Referência pega tela que estava certa e ficou errada. Esta nasceu
  errada.
- **B (imagem crua) não decidiu.** A frase escrita na hora, antes de a resposta existir, é o
  registro mais limpo deste achado: *"Pequeno demais para julgar por imagem. Vou medir no
  domínio, que decide."*
- **C (estado puro) decidiu na hora.** `de 36 encontros: 0 ROUBADAS, 36 estouraram` — 24 na
  altura 2 e 12 na altura 4, todas com raio 0, todas inúteis. E o mesmo instrumento mediu o
  conserto: com teto de 3 degraus, **12 de 36 roubadas e zero bombas inúteis**.

**Isto é mais forte do que a pergunta 7 perguntou, e é uma resposta diferente da esperada.**
A pergunta supõe que a imagem contribui e que o estado a torna mais precisa. Aqui a imagem
não contribuiu com nada: quem respondeu foi o **estado sozinho**. A combinação não era
necessária — era dispensável do lado errado.

## Achado 2 — no defeito de cena, a combinação é que localizou, e ela deu o número

Os nós da bomba e da carregadora ficavam presos na tela para sempre. A causa: a explosão de
uma bomba já levantada tem raio 0 e não toca a própria célula, então **não produz evento
nenhum** — e o redesenho da cena estava preso a *"só quando há evento"*.

- **A (imagem com referência) não viu**, pelo mesmo motivo do achado 1.
- **O autor viu, jogando, e mandou uma captura.** Detecção, não diagnóstico.
- **D (imagem contra estado) localizou e quantificou.** Antes: `nós=1, carregadoras=1,
  órfãos=1`, tique após tique, para sempre. Depois do conserto: **tudo zero**. É o
  discriminador que nem a imagem nem o estado davam sozinhos — a imagem mostra um nó parado,
  o estado diz que não há bomba, e **é o desencontro entre os dois que nomeia o defeito**.

Este é o achado que responde a pergunta 7 nos termos em que ela foi escrita: a evidência
combinada produziu diagnóstico mais preciso que a imagem isolada, num defeito real.

## O que isto responde

**Sim, com n=1 e como contrafactual.** Num caso real, imagem contra estado diagnosticou o
que a imagem com referência não viu, e deu um número que separa o defeito do conserto.

E acrescenta uma coisa que a pergunta não previa: **há defeitos em que a imagem não entra na
conta.** Quando o que está errado é uma relação entre durações, nenhuma quantidade de pixels
decide, e a imagem crua atrapalha por parecer plausível — a tira de dez quadros do roubo é
bonita e mostra um mecanismo que nunca completava.

## O que isto NÃO responde — e é a parte que importa

**1. Nenhum dos quatro instrumentos detectou primeiro. Os dois defeitos foram achados pelo
autor, jogando.** O C só rodou depois de ele dizer que a aranha estava parada; o D só foi
apontado para o lugar certo depois da captura que ele mandou. A pergunta 7 é sobre
**diagnóstico**, e nesse recorte está respondida — mas quem ler isto e concluir que o
aparelho fecha o ciclo terá concluído errado.

O próprio porte já tinha escrito isso, no cabeçalho da família que responde a pergunta:

> as duas nasceram do mesmo jeito: o autor viu o defeito **JOGANDO**, e nenhum portão daqui
> tinha visto.

Três defeitos, três vezes a mesma origem. É a mesma linha da
[ADR 0013](decisoes/0013-manter-a-sara-privada-ao-fim-do-marco-6.md): **onze dos treze casos
do Marco 6 ainda exigiram inspeção humana.**

**2. O lado "estado" é deste jogo, e não é portável.** `PresencaNaCena` sabe o que é uma
gema, o que é um órfão e o que `_sob_tampa()` significa. A medição independente do
`godot-agent`, em 26/08, chegou ao mesmo limite por fora: *"`conferir_buracos` compara
`partida.grade` contra `_pecas` — um lado é DOMÍNIO. O gda **não tem como**, e nem chega
perto"*. Ferramenta genérica alcança a metade nó-contra-nó e não a metade nó-contra-regra.

**3. Um caso, um jogo, um dia.** E escolhido depois. Vale como evidência e não vale como
prova.

**4. Não muda decisão nenhuma.** A [ADR 0004](decisoes/0004-spike-de-visao-instrumentada-em-godot.md)
continua substituída, o Marco 7 continua sem ser construído, e nada aqui autoriza binário,
runtime, addon ou dependência. Este documento acrescenta conhecimento e não escopo.

## O que derrubaria isto

- **Um caso definido antes**, montado para a comparação, em que a imagem contra estado não
  supere a imagem isolada. Ele valeria mais que este, porque teria a cláusula que este não
  tem.
- **Um defeito que a imagem com referência pegue e o estado não** — ele existe por
  construção (mudança de cor, de fonte, de espaçamento; a Sentinela pegou o fio da aranha
  invadindo 48 px do HUD, 0,01% dos pixels) e mostraria que a relação é de complemento e não
  de superioridade. Esta é a leitura que eu esperaria confirmar, e ela **enfraquece** o
  título deste documento em vez de reforçá-lo.
- **Um instrumento que detecte antes da pessoa**, em vez de diagnosticar depois. Se aparecer,
  a conclusão útil deixa de ser esta e passa a ser a dele.
