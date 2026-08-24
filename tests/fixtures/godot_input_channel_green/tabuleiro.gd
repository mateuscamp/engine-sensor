extends Node2D

# Mesma estrutura da fixture vermelha, com uma diferenca: o projeto desliga
# emulate_mouse_from_touch. Os canais passam a ser separados de verdade.

func _unhandled_input(evento: InputEvent) -> void:
	if evento is InputEventScreenTouch:
		var na_tela: InputEventScreenTouch = evento
		_dedo(na_tela.position, na_tela.pressed)
	elif evento is InputEventMouseButton:
		var do_mouse: InputEventMouseButton = evento
		if do_mouse.button_index == MOUSE_BUTTON_LEFT:
			_dedo(do_mouse.position, do_mouse.pressed)


func _dedo(onde: Vector2, pressionado: bool) -> void:
	print(onde, pressionado)
