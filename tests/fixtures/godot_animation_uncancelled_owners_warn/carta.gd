extends Node2D

# Um lado encerra o Tween guardado, o outro nao. A disputa e real e o aviso
# precisa continuar de pe: cancelar de um lado so nao serializa nada.

var _tween_ativo: Tween = null
var _pos_base := Vector2.ZERO


func _encerrar_tween_ativo() -> void:
	if _tween_ativo and is_instance_valid(_tween_ativo):
		_tween_ativo.kill()
	_tween_ativo = null


func elevar() -> void:
	_encerrar_tween_ativo()
	_pos_base = position
	var subida := create_tween()
	subida.tween_property(self, "position", _pos_base + Vector2(0.0, -24.0), 0.14)
	_tween_ativo = subida


func sacudir() -> void:
	var tremor := create_tween()
	tremor.tween_property(self, "position", _pos_base + Vector2(6.0, 0.0), 0.1)
