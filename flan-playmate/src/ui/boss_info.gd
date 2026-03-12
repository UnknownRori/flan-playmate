extends VBoxContainer

@onready var progress: ProgressBar = $MarginContainer/ProgressBar
var tween: Tween

func _ready() -> void:
    
    GameState.boss_hp.hp_change.connect(_hp_change)
    progress.max_value = GameState.boss_hp.max_hp
    progress.value = GameState.boss_hp.current_hp

func _hp_change(new_value: int):
    if tween:
        tween.kill()
    tween = get_tree().create_tween()
    tween.set_trans(Tween.TRANS_SINE)
    tween.set_ease(Tween.EASE_IN_OUT)
    tween.tween_property(progress, "value", float(new_value), 0.1)
    tween.play()
    progress.max_value = float(GameState.boss_hp.max_hp)
