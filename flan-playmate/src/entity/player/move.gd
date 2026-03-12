extends NodeState

@export var sprite: AnimatedSprite2D
@export var entity: CharacterBody2D
@export var basic: BulletSpawnerComponent
@export var velocity: VelocityComponent
@export var attack_cooldown: Timer

func _on_process(_dt: float) -> void:
    var dir = GameInputs.update_movement()
    velocity.set_dir(dir)
    velocity.set_focus(GameInputs.is_focus())
    
    if dir.x == 1.0:
        sprite.play("right")
    if dir.x == -1.0:
        sprite.play("left")    
    
    if GameInputs.is_attack() and attack_cooldown.is_stopped():
        basic.spawn(entity.global_position, Vector2(0, -100), 0.)
        attack_cooldown.start()
        pass

func _on_next_transition() -> void:
    if !GameInputs.is_move():
        transition.emit("idle")
