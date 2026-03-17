extends BossAttack

@export var bullet_1: BulletSpawnerComponent

# TODO: Make this bullet simple one
func _start_attack():
    AudioManager.play_bgm("hartman")
    while !is_done:
        var parent_pos = entity.global_position
        var dir = parent_pos.direction_to(GameState.get_player_position())

        await BulletHelper.spread_times(bullet_1, parent_pos, 4, 0.2, 100, dir, 4, 10)
        await get_tree().create_timer(4.).timeout
