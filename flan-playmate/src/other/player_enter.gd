extends AnimationPlayer

@export var player: Player
var p: Player

signal done_enter

func _ready() -> void:
    player.hp.hp_change.connect(_player_hp_lost, ConnectFlags.CONNECT_DEFERRED)
    p = player
    

func _player_hp_lost(_val: int):
    AudioManager.play_sfx("player_death")
    p.hp.invulnerable = true
    play("enter")
    await animation_finished
    await get_tree().create_timer(1.).timeout
    done_enter.emit()
    p.hp.invulnerable = false
