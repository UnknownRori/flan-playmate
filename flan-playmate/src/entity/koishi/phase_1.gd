extends NodeState

@export var entity: CharacterBody2D
@export var hp: HealthComponent
@export var bullet_1: BulletSpawnerComponent

@export_group("Attack Information")
@export var timeout: float
@export var hp_max: int

func _process(_delta: float) -> void:
    entity.move_and_slide()

func _on_enter() -> void:
    hp.set_hp(hp_max)
    hp.set_hp_max(hp_max)
    _start_attack()

# INFO: Dummy attack
func _start_attack():
    while true:
        var dir = entity.global_position.direction_to(GameState.get_player_position())
        spread_times(4, 0.2, 100, dir, 4, 10)
        await get_tree().create_timer(5.).timeout

func spread_times(time: int, delay: float, speed: float, dir: Vector2, num_side: float, spread_angle: float):
    for _i in range(0, time):
        spread(speed, dir, num_side, spread_angle)
        await get_tree().create_timer(delay).timeout
        AudioManager.play_sfx("shot_1")
        
func spread(speed: float, dir: Vector2, num_side: float, spread_angle: float):
    var offset_rotation = deg_to_rad(90)
    bullet_1.spawn(entity.global_position, dir * speed, dir.angle() + offset_rotation)
    for i in range(1, num_side + 1):
        var offset = deg_to_rad(spread_angle * i)
        bullet_1.spawn(
            entity.global_position, 
            dir.rotated(offset) * speed, 
            dir.angle() + offset_rotation + offset
        )
        bullet_1.spawn(
            entity.global_position, 
            dir.rotated(-offset) * speed, 
            dir.angle() + offset_rotation + -offset
        )
