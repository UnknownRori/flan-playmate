extends NodeState

@export var sprite: AnimatedSprite2D
@export var entity: CharacterBody2D

@export_group("Movement Info")
@export var friction: float = 0.85

func _on_process(_dt: float) -> void:
    GameInputs.update_movement()
    
    if GameInputs.is_attack():
        pass
    
    entity.velocity *= friction
    entity.move_and_slide()

func _on_next_transition() -> void:
    if GameInputs.is_move():
        transition.emit("move")

func _on_enter() -> void:
    sprite.play("idle")
