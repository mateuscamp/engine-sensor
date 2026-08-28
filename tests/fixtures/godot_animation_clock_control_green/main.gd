extends Node2D

## O caso da aranha, reduzido: o Tween da faisca nasce numa funcao e tem o relogio
## mexido em OUTRA. Alvo, propriedade e dono nao mudam -- e o que acontece na tela
## muda inteiro.

@onready var _faisca: Node2D = $Faisca
@onready var _alto_falante: AudioStreamPlayer = $Passos

var _caminhada: Tween
var _panico: Tween


func acender() -> void:
	_caminhada = create_tween().set_loops()
	_caminhada.tween_property(_faisca, "position", Vector2(0, -48), 2.2)
	_panico = create_tween()
	_panico.tween_property(_faisca, "modulate", Color.RED, 0.4)


## A teia nao para o pavio: ela o lentifica. Esta e a regra inteira da peca, e ela
## mora aqui -- numa chamada que nao toca a propriedade animada.
func mostrar_teia() -> void:
	_caminhada.set_speed_scale(0.5)


func prender() -> void:
	_caminhada.pause()


func soltar() -> void:
	_caminhada.play()
	# Mesmo nome de metodo, objeto que nao e Tween: nao ha Tween para declarar, e
	# inventar um seria pior que o silencio de antes.
	_alto_falante.play()


## A forma em que o caso de origem estava escrito: o relogio mexido por uma variavel
## de laco sobre uma lista de Tweens. Uma capacidade que nao ve o proprio caso de
## origem nao foi construida, foi anunciada.
func lentificar_o_pavio(fator: float) -> void:
	for animacao: Tween in [_caminhada, _panico]:
		if animacao != null and animacao.is_valid():
			animacao.set_speed_scale(1.0 / fator)
