extends BaseResourceDatabase

func _ready():
    _load_all("res://resources/audio/")

func get_item(id: String) -> AudioResource:
    return items.get(id, null)
