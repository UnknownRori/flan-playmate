class_name BaseResourceDatabase
extends Node

var items: Dictionary = {}

func _ready():
    _load_all("res://data/items/")

func _load_all(directory: String):
    var dir = DirAccess.open(directory)
    dir.list_dir_begin()
    var file = dir.get_next()
    while file != "":
        if file.ends_with(".tres"):
            var item = load(directory + file) as BaseResource
            items[item.id] = item
        file = dir.get_next()

func get_item(id: String) -> Resource:
    return items.get(id, null)
