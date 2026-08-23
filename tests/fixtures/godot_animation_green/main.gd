extends Node2D

func animate_bomb() -> void:
	var sequence: Tween = create_tween()
	sequence.tween_property($Bomb, "scale", Vector2.ONE * 1.2, 0.2)
	sequence.tween_property($Bomb, "scale", Vector2.ONE * 0.8, 0.4)
