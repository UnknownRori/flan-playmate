extends NodeState

# Symbol "Danmaku Paranoia" 

@export var entity: CharacterBody2D
@export var hp: HealthComponent
@export var bullet_1: BulletSpawnerComponent

@export_group("Attack Information")
@export var timeout: float
@export var hp_max: int
var spell_name = "Symbol: Danmaku Paranoia"

func _on_enter() -> void:
    hp.set_hp(hp_max)
    hp.set_hp_max(hp_max)
    _start_attack()
    print("Danmaku")

# TODO: Add additional texture for the bullet for variation
func _start_attack():
    while true:
        var parent_pos = entity.global_position
        var dir = parent_pos.direction_to(GameState.get_player_position())

        await BulletHelper.spread_times(bullet_1, parent_pos, 4, 0.2, 100, dir, 4, 10)
        await get_tree().create_timer(0.5).timeout
        await BulletHelper.spread_times(bullet_1, parent_pos, 4, 0.2, 100, dir, 4, 4)
        await BulletHelper.ripple(bullet_1, parent_pos, 4, 64, 50, 20)
        await get_tree().create_timer(2.).timeout
