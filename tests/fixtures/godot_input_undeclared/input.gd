extends Node

func _input(event: InputEvent) -> void:
	if event.is_action_pressed("bomb"):
		plant_bomb()

func _unhandled_input(event: InputEvent) -> void:
	if event.is_action_pressed("bomb"):
		plant_bomb()

func plant_bomb() -> void:
	pass
