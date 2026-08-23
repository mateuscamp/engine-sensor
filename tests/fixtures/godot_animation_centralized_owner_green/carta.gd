extends Node2D

# Dono centralizado: todo escritor encerra o Tween guardado antes de criar o seu.
# E o padrao que a propria remediacao do SAR-OWN-001 recomenda, e o cancelamento
# acontece por metodo auxiliar, nao por `variavel.kill()` na mesma funcao.

const ELEVACAO_PX := 24.0

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
	subida.tween_property(self, "position", _pos_base + Vector2(0.0, -ELEVACAO_PX), 0.14)
	_tween_ativo = subida


func repousar() -> void:
	_encerrar_tween_ativo()
	var descida := create_tween()
	descida.tween_property(self, "position", _pos_base, 0.18)
	_tween_ativo = descida
