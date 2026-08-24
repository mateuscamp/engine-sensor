extends Node2D

# Os dois canais sao tratados, mas cada um chega a um efeito DIFERENTE. Tratar
# toque e mouse no mesmo callback nao e conflito: conflito e os dois caindo no
# mesmo efeito.

func _unhandled_input(evento: InputEvent) -> void:
	if evento is InputEventScreenTouch:
		var na_tela: InputEventScreenTouch = evento
		_dedo(na_tela.position, na_tela.pressed)
	elif evento is InputEventMouseButton:
		var do_mouse: InputEventMouseButton = evento
		_ponteiro(do_mouse.position, do_mouse.pressed)


func _dedo(onde: Vector2, pressionado: bool) -> void:
	print(onde, pressionado)


func _ponteiro(onde: Vector2, pressionado: bool) -> void:
	print(onde, pressionado)
