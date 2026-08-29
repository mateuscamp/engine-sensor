extends Node2D

## O caso da aranha, reduzido: quem decide a profundidade de desenho.
##
## A Sara declarava alvo, propriedade e dono de cada trajetoria, e nenhuma das tres
## diz quem desenha na FRENTE de quem. Foi o fio de seda com `z_index` errado que
## nao apareceu em quadro nenhum -- e um sprite invisivel passa por todos os portoes.

var _fio: Line2D
var _cartas: Array[Node2D] = []


## Os DOIS mecanismos decidindo o mesmo no, e nenhum deles sabe do outro.
##
## `z_index` vence ordem de arvore, entao a reordenacao abaixo e uma verdade que
## nao vale. Duas fontes de verdade sobre a mesma coordenada e a familia que a Sara
## ja modela -- ela so nao a enxergava neste eixo.
func _pendurar() -> void:
	_fio = Line2D.new()
	add_child(_fio)
	_fio.z_index = -1
	move_child(_fio, 0)


## A forma exata do porte: escrita sem prefixo, no proprio no.
func _montar() -> void:
	# NA FRENTE DA GEMA, sempre: quem desenha por ultimo fica por cima, e uma gema
	# nova nascendo depois dele o cobria.
	z_index = 1


## Os quatro limites, e sao eles que separam esta regra de uma que inventa.
func _limites(outro: Node2D, vizinho: Node2D) -> void:
	# [b]`z_index = -1` nao serve[/b]: ele e relativo ao pai. Esta frase mora num
	# COMENTARIO, e comentario nao e fato do jogo: declarar a partir dela seria
	# afirmar como escrita uma frase SOBRE a escrita.
	var antes: int = outro.z_index
	if outro.z_index == vizinho.z_index:
		antes += 1
	if outro.z_index > vizinho.z_index:
		antes -= 1
	# Alvo dinamico: sem saber QUAL no e, nao ha profundidade a declarar, e inventar
	# um no seria pior que o silencio de antes.
	for i in _cartas.size():
		_cartas[i].z_index = antes - i
	# Mesmo fim de nome, outro ato: `remove_child` nao decide profundidade nenhuma.
	remove_child(vizinho)
