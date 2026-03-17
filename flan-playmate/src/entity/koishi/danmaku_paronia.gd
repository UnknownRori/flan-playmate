extends BossAttack

@export var bullet_1: BulletSpawnerComponent

# TODO: Add additional texture for the bullet for variation
func _start_attack():
    while !is_done:
        var parent_pos = entity.global_position
        var dir = parent_pos.direction_to(GameState.get_player_position())

        await BulletHelper.spread_times(bullet_1, parent_pos, 4, 0.2, 100, dir, 4, 5)
        await get_tree().create_timer(0.1).timeout
        await BulletHelper.spread_times(bullet_1, parent_pos, 4, 0.2, 100, dir, 4, 10)
        
        await BulletHelper.ripple(bullet_1, parent_pos, 4, 64, 50, 20)
        await get_tree().create_timer(2.).timeout
