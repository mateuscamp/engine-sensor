extends Node2D


func pulsar(no: Node2D) -> Tween:
	var pulso: Tween = no.create_tween().set_loops()
	pulso.tween_property(no, "scale", Vector2.ONE * 1.2, 0.2) \
		.set_trans(Tween.TRANS_SINE).set_ease(Tween.EASE_IN_OUT)
	pulso.tween_property(no, "scale", Vector2.ONE, 0.2) \
		.set_trans(Tween.TRANS_SINE).set_ease(Tween.EASE_IN_OUT)
	return pulso
