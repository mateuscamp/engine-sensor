extends Node2D

# Toque e mouse chegam ao MESMO efeito. No aparelho um toque entrega os dois
# eventos, porque nada desliga emulate_mouse_from_touch: `_dedo` roda duas vezes.

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
