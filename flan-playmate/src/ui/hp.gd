extends HBoxContainer

@onready var love_icon: PackedScene = preload("res://ui/shared/love_icon.tscn")
var love_icons = []

func _ready() -> void:
    for i in range(0, int(GameState.player_hp.current_hp)):
        var icon = love_icon.instantiate()
        love_icons.append(icon)
        add_child(icon)
    _on_update_hp(GameState.player_hp.current_hp)
    GameState.player_hp.hp_change.connect(_on_update_hp)

func _on_update_hp(new_value: float):
    for i in range(0, new_value):
        var icon = love_icons[i] as TextureRect
        icon.visible = true
    for i in range(new_value, GameState.player_hp.max_hp):
        var icon = love_icons[i] as TextureRect
        icon.visible = false
