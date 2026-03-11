extends NodeState

@export var sprite: AnimatedSprite2D
@export var entity: CharacterBody2D
@export var basic: BulletSpawnerComponent
@export var velocity: VelocityComponent

func _on_process(_dt: float) -> void:
    var dir = GameInputs.update_movement()
    velocity.set_dir(dir)
    velocity.set_focus(GameInputs.is_focus())
    
    if dir.x == 1.0:
        sprite.play("right")
    if dir.x == -1.0:
        sprite.play("left")    
    
    if GameInputs.is_attack():
        basic.spawn(entity.global_position, Vector2(0, -100), 0)
        pass

func _on_next_transition() -> void:
    if !GameInputs.is_move():
        transition.emit("idle")
