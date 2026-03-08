extends NodeState

@export var sprite: AnimatedSprite2D
@export var entity: CharacterBody2D

@export_group("Movement Info")
@export var accel: float = 50.
@export var max_speed: float = 150
@export var friction: float = 0.85

func _on_process(_dt: float) -> void:
    var dir = GameInputs.update_movement()
    
    if dir.x == 1.0:
        sprite.play("right")
    if dir.x == -1.0:
        sprite.play("left")    
    
    if GameInputs.is_attack():
        pass

    entity.velocity += dir * accel
    entity.velocity *= friction
    entity.velocity = entity.velocity.clampf(-max_speed, max_speed)
    entity.move_and_slide()

func _on_next_transition() -> void:
    if !GameInputs.is_move():
        transition.emit("idle")
