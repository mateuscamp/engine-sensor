extends Node2D

func animate_bomb() -> void:
	var grow: Tween = create_tween()
	var shrink: Tween = create_tween()
	grow.tween_property($Bomb, "scale", Vector2.ONE * 1.2, 0.2)
	shrink.tween_property($Bomb, "scale", Vector2.ONE * 0.8, 0.4)
