extends SceneTree

## Modelo deliberadamente pequeno. Copie para `.sara/godot/` e passe as pastas
## puras depois de `--`.
const PROIBIDOS: PackedStringArray = [
	"Node", "Node2D", "Control", "Tween", "Timer", "Input", "InputEvent",
	"AudioStreamPlayer", "get_tree", "get_node", "queue_free", "randi(",
	"randf(", "randomize(", "Time.",
]


func _initialize() -> void:
	var pastas: PackedStringArray = _pastas()
	var falhas: int = 0
	for pasta: String in pastas:
		falhas += _conferir_pasta(pasta)
	print(JSON.stringify({"portao": "ai-first", "falhas": falhas, "pastas": pastas}))
	quit(0 if falhas == 0 else 1)


func _pastas() -> PackedStringArray:
	var argumentos: PackedStringArray = OS.get_cmdline_user_args()
	return argumentos if not argumentos.is_empty() else PackedStringArray(["game/domain"])


func _conferir_pasta(pasta: String) -> int:
	var falhas: int = 0
	var raiz: String = "res://%s" % pasta.trim_prefix("res://")
	if DirAccess.open(raiz) == null:
		push_error("pasta pura ausente: %s" % raiz)
		return 1
	for caminho: String in _scripts(raiz):
		var fonte: String = FileAccess.get_file_as_string(caminho)
		for proibido: String in PROIBIDOS:
			if _codigo(fonte).contains(proibido):
				push_error("%s usa %s dentro da fronteira pura" % [caminho, proibido])
				falhas += 1
	return falhas


func _scripts(pasta: String) -> PackedStringArray:
	var encontrados: PackedStringArray = []
	var diretorio: DirAccess = DirAccess.open(pasta)
	if diretorio == null:
		return encontrados
	for nome: String in diretorio.get_files():
		if nome.ends_with(".gd"):
			encontrados.append("%s/%s" % [pasta, nome])
	for nome: String in diretorio.get_directories():
		encontrados.append_array(_scripts("%s/%s" % [pasta, nome]))
	return encontrados


func _codigo(fonte: String) -> String:
	var linhas: PackedStringArray = []
	for linha: String in fonte.split("\n"):
		linhas.append(linha.split("#", true, 1)[0])
	return "\n".join(linhas)
