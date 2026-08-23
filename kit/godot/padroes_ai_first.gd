class_name PadroesAiFirst
extends RefCounted

## Modelos pequenos: adapte os nomes ao domínio e mantenha a engine neste adapter.


class RngDeterministico:
	var _estado: int


	func _init(semente: int) -> void:
		_estado = semente & 0x7fffffff


	func proximo(limite_exclusivo: int) -> int:
		assert(limite_exclusivo > 0)
		_estado = (1103515245 * _estado + 12345) & 0x7fffffff
		return _estado % limite_exclusivo


static func normalizar_entrada(evento: InputEvent) -> Dictionary:
	if evento.is_action_pressed(&"acao_primaria"):
		return {"acao": "primaria", "pressionada": true}
	return {}


static func log_estruturado(evento: String, dados: Dictionary) -> void:
	var registro: Dictionary = dados.duplicate(true)
	registro["evento"] = evento
	print(JSON.stringify(registro))
