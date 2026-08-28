# ADR 0013 - Manter a Sara privada ao fim do Marco 6

**Status:** Aceita
**Data:** 28 de agosto de 2026
**Decisor:** proprietário do Sara
**Escopo:** desfecho do portão do Marco 6

## Contexto

O Marco 6 tinha critério de conclusão — dez mudanças reais em projetos Godot — e critério
de parada — 20 de setembro de 2026. **A contagem fechou em 28/08/2026, com treze mudanças,
vinte e três dias antes da data.** O registro está em [`USO-PESSOAL.md`](../USO-PESSOAL.md).

Onze das treze não eram trabalho novo: estavam registradas só no diário do porte e subiram
para o registro do marco no mesmo dia, depois de a leitura do
[caso da aranha](../CASO-DA-ARANHA.md) expor que o critério escrito já as abrangia.

### A evidência que o portão julga

| medida | valor |
|---|---|
| mudanças reais | 13, **todas do porte do BomberBoom** |
| instrumento | 12 das 13 com o mesmo binário, `f1f4d5f` |
| avisos | **1 útil, 0 falsos** em treze usos |
| bloqueios falsos | 0 |
| defeitos que só a Sara achou | 1 — toque e mouse no mesmo `_dedo`, cuja versão em Defold **chegou ao jogador** |
| casos que ainda exigiram inspeção humana | **11 de 13** |
| capacidades ausentes nomeadas | 7, todas estáticas e todas de Godot |

O [caso da aranha](../CASO-DA-ARANHA.md) é a medição mais dura da série: nove defeitos numa
peça, **portão pegou um**, e os dois que decidiam se a peça existia foram achados pelo autor
jogando.

### A previsão que isto confirma

Em 25/08/2026, com duas mudanças registradas, ficou escrita uma previsão datada:

> **O recorte é pequeno demais para ser produto e grande o bastante para ser portão
> privado.** [...] Se isso se confirmar, o desfecho do portão será manter privado.

Ela listou duas coisas que a derrubariam: uma das mudanças restantes produzir um verdadeiro
positivo que teria chegado ao jogador, ou a medição Android mostrar a classe de posse mais
comum no aparelho. **Nenhuma das duas aconteceu.** Onze mudanças depois, a previsão está de
pé, e é por ela ter sido datada antes que esta decisão não é raciocínio circular.

## Opções consideradas

São as três que o [`RESULTADO-0.1.0.md`](../RESULTADO-0.1.0.md) já autorizava. Nenhuma
quarta foi inventada agora.

1. **Manter privado.** O binário continua existindo, continua rodando no porte, e continua
   recebendo capacidade pelo método da ADR 0012. Sem publicação, sem preço, sem marca.
2. **Congelar.** Mantém só o kit AI-first e para o desenvolvimento da ferramenta.
3. **Propor nova ADR de expansão.** Trata as sete capacidades ausentes como roteiro e
   abre frente nova.

A opção 2 tem o argumento mais forte da série contra a ferramenta: **onze dos treze casos
ainda exigiram inspeção humana.** A atenção da pessoa é o recurso escasso que este projeto
inteiro existe para economizar, e em 85% dos casos ela continuou sendo gasta. Um aviso útil
em treze usos é colheita magra.

O que a derruba é o custo do outro lado. A Sara é um binário offline de 3,9 MB, sem serviço,
sem rede e sem manutenção fora do corpus; congelar economiza quase nada e joga fora sete
capacidades nomeadas com o contexto fresco. E o defeito que ela achou não era qualquer um:
**é o mesmo que, na versão Defold, chegou ao jogador com duas bombas por toque**, invisível
no desktop onde o Estágio A verificou tudo.

A opção 3 contraria a [ADR 0001](0001-validar-mecanismos-antes-da-engine-completa.md) e a
própria previsão de 25/08. Expandir agora exigiria discordar da previsão, e discordar dela
exigiria evidência que treze usos não produziram.

## Decisão

Nós **mantemos a Sara privada**. O Marco 6 encerra por conclusão, com o desfecho que a
previsão datada de 25/08 antecipou.

### 1. O que isto autoriza

O binário continua em uso no porte; o registro continua sendo preenchido; capacidade nova
continua entrando pelo método da ADR 0012 — dor local, hipótese, confronto com o corpus,
incorporação ou recusa.

### 2. O que isto não autoriza

Publicação, código aberto, itch.io, preço, licença, marca, plugin de editor, SARIF, daemon,
SDK, protocolo público, runtime e engine própria. A lista do
[`ROTEIRO.md`](../ROTEIRO.md#o-que-fica-adiado) continua valendo inteira, e nada aqui a
encurta. A [ADR 0001](0001-validar-mecanismos-antes-da-engine-completa.md) permanece de pé.

### 3. A ADR 0012 continua valendo, e isso é decisão e não omissão

A [ADR 0012](0012-sara-e-corpus-coevoluem.md) declarou governar "um marco em curso, não o
projeto inteiro", e previu encerrar junto se o portão concluísse **congelar ou encerrar**.
O portão concluiu nenhum dos dois.

Nós decidimos que o método dela **continua**, porque a razão que o produziu não terminou com
o marco: o domínio continua novo, e o instrumento continua sendo descoberto junto com o que
precisa medir. O que muda é o que ele serve — deixa de alimentar uma contagem e passa a
alimentar a única pergunta que sobra, a da §5 dela: **se o que nasce num projeto generaliza
para o corpus.**

### 4. O Marco 7 não é afetado

Continua barrado pela [ADR 0011](0011-marco-7-exige-comparacao-com-ferramenta-existente.md),
e ganhou uma segunda pergunta pelo caso da aranha, registrada no
[`ROTEIRO.md`](../ROTEIRO.md). Manter privado não o autoriza nem o adianta.

## Consequências

### Positivas

- O desfecho é o que uma previsão datada antecipou antes da evidência, e não uma conclusão
  moldada por ela. É o resultado mais forte que este método podia produzir sobre si mesmo.
- A [ADR 0001](0001-validar-mecanismos-antes-da-engine-completa.md) é confirmada
  empiricamente: validar mecanismo antes de construir engine estava certo, e agora está
  medido em vez de argumentado.
- As sete capacidades ausentes têm caminho declarado para entrar, sem que nada nelas
  autorize escopo novo.
- O custo de manter é conhecido e pequeno: um binário offline, sem serviço e sem rede.

### Negativas

- **Onze dos treze casos ainda exigiram inspeção humana, e manter privado não conserta
  isso.** A decisão aceita conviver com a lacuna principal em vez de fechá-la. Quem ler esta
  ADR procurando o argumento contra ela deve ler esta linha primeiro.
- **A generalização continua sem medida.** As treze mudanças são de um jogo só; a segunda
  linha de evidência da ADR 0012 §5 tem um ponto e não uma série. O que se mediu foi a Sara
  acompanhando um projeto.
- A medição Android continua faltando e mantém poder de veto sobre qualquer conclusão de
  foco. Nada aqui a substitui.
- Manter é a saída mais confortável das três, e conforto é justamente o que uma decisão de
  portão deve desconfiar. O antídoto é o critério de revisão abaixo, que é datado.

## Conformidade

Fitness function automática, em `tests/governanca.rs`:

- `adr_0013_o_pacote_continua_privado` exige `publish = false` no `Cargo.toml`. Publicar em
  registro é o passo irreversível que separa privado de público, e ele não pode acontecer
  por descuido de manifesto.
- As fitness functions da ADR 0001 (dependências) e da ADR 0007 (binários) continuam sendo
  o freio de escopo, e não são substituídas por esta.

O que **não** é automatizável e fica declarado como manual: nada num teste impede publicar
um binário fora do Cargo. O freio real é esta ADR estar escrita.

## Critério de revisão

- **Data:** 20 de setembro de 2026, o antigo critério de parada. Se até lá a Sara não tiver
  recebido nenhuma capacidade nova nem sido executada num projeto novo, "manter privado"
  virou "congelado sem admitir", e a revisão passa a ser entre congelar de verdade e
  encerrar.
- **Evidência:** se a proporção de casos exigindo inspeção humana não cair abaixo de 11 em
  13 depois das três capacidades do caso da aranha entrarem, o argumento da opção 2 fica
  mais forte que o desta decisão, e ela é revista.
- **Contrário:** se uma capacidade nascida no porte generalizar para os projetos parados
  produzindo verdadeiro positivo — como a ADR 0010 quase fez —, a opção 3 volta à mesa com
  evidência que hoje não existe.

## Notas

- Autor: proprietário do Sara
- Aprovada por: proprietário do Sara
- Substitui: nenhuma. Encerra o Marco 6 e mantém a ADR 0012 em vigor por decisão explícita.
- Última alteração: 28 de agosto de 2026
