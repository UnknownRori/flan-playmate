extends VBoxContainer

@onready var progress: ProgressBar = $MarginContainer/ProgressBar

func _ready() -> void:
    GameState.boss_hp.hp_change.connect(_hp_change)
    progress.max_value = GameState.boss_hp.max_hp
    progress.value = GameState.boss_hp.current_hp

func _hp_change(new_value: int):
    print(new_value)
    progress.max_value = float(GameState.boss_hp.max_hp)
    progress.value = float(new_value)
