extends Node

func spread_times(
    bullet: BulletSpawnerComponent,
    start_pos: Vector2,
    time: int,
    delay: float,
    speed: float,
    dir: Vector2,
    num_side: float,
    spread_angle: float,
    sfx: String = "shot_1"
    ):
    for _i in range(0, time):
        spread(bullet, start_pos, speed, dir, num_side, spread_angle)
        await get_tree().create_timer(delay).timeout
        AudioManager.play_sfx(sfx)
        
func spread(
    bullet: BulletSpawnerComponent,
    start_pos: Vector2,
    speed: float,
    dir: Vector2,
    num_side: float,
    spread_angle: float
    ):
    var offset_rotation = deg_to_rad(90)
    bullet.spawn(start_pos, dir * speed, dir.angle() + offset_rotation)
    for i in range(1, num_side + 1):
        var offset = deg_to_rad(spread_angle * i)
        bullet.spawn(
            start_pos, 
            dir.rotated(offset) * speed, 
            dir.angle() + offset_rotation + offset
        )
        bullet.spawn(
            start_pos, 
            dir.rotated(-offset) * speed, 
            dir.angle() + offset_rotation + -offset
        )

func circle(
    bullet: BulletSpawnerComponent,
    start_pos: Vector2,
    speed: float,
    count: int,
) -> void:
    for i in count:
        var angle = (TAU / count) * i
        var dir = Vector2.from_angle(angle)
        var offset_rotation = deg_to_rad(90)
        bullet.spawn(start_pos, dir * speed, angle + offset_rotation)


func circle_bloom(
    bullet: BulletSpawnerComponent,
    start_pos: Vector2,
    speed: float,
    count: int,
    delay_per_bullet: float = 0.03,
    sfx: String = "shot_1"
) -> void:
    for i in count:
        AudioManager.play_sfx(sfx)
        var angle = (TAU / count) * i
        var dir = Vector2.from_angle(angle)
        bullet.spawn(start_pos, dir * speed, angle + deg_to_rad(90))
        await get_tree().create_timer(delay_per_bullet).timeout

func ripple(
    bullet: BulletSpawnerComponent,
    start_pos: Vector2,
    ring_count: int,
    bullets_per_ring: int,
    base_speed: float = 120.0,
    speed_step: float = 60.0, ## each ring moves faster than the last
    delay_between_rings: float = 0.15,
    ring_rotation_offset: float = 0.0,
    sfx: String = "shot_1"
) -> void:
    for r in ring_count:
        var spd = base_speed + speed_step * r
        for i in bullets_per_ring:
            var angle = (TAU / bullets_per_ring) * i + ring_rotation_offset * r
            var dir = Vector2.from_angle(angle)
            bullet.spawn(start_pos, dir * spd, angle + deg_to_rad(90))
        await get_tree().create_timer(delay_between_rings).timeout
        AudioManager.play_sfx(sfx)
        


func spiral(
    bullet: BulletSpawnerComponent,
    start_pos: Vector2,
    speed: float,
    arms: int = 2,
    duration: float = 4.0,
    tick_rate: float = 0.05,
    rotation_per_tick: float = 0.18,
    sfx: String = "shot_1",
) -> void:
    var angle := 0.0
    var elapsed := 0.0
    while elapsed < duration:
        AudioManager.play_sfx(sfx)
        for a in arms:
            var arm_angle = angle + (TAU / arms) * a
            var dir = Vector2.from_angle(arm_angle)
            bullet.spawn(start_pos, dir * speed, arm_angle + deg_to_rad(90))
        angle += rotation_per_tick
        elapsed += tick_rate
        await get_tree().create_timer(tick_rate).timeout


func flower(
    bullet: BulletSpawnerComponent,
    start_pos: Vector2,
    speed: float,
    petals: int = 6,
    num_side: int = 2,
    spread_angle: float = 20.0,
    delay_per_petal: float = 0.08,
    sfx: String = "shot_1"
) -> void:
    for p in petals:
        AudioManager.play_sfx(sfx)
        var dir = Vector2.from_angle((TAU / petals) * p)
        spread(bullet, start_pos, speed, dir, num_side, spread_angle)
        await get_tree().create_timer(delay_per_petal).timeout
