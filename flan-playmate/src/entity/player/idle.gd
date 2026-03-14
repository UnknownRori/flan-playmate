extends NodeState

@export var sprite: AnimatedSprite2D
@export var entity: CharacterBody2D
@export var basic: BulletSpawnerComponent
@export var velocity: VelocityComponent
@export var attack_cooldown: Timer


func _on_process(_dt: float) -> void:
    GameInputs.update_movement()
    velocity.set_dir(Vector2(0, 0))
    
    if GameInputs.is_attack() and attack_cooldown.is_stopped():
        basic.spawn(entity.global_position, Vector2(0, -100), 0.)
        attack_cooldown.start()
        AudioManager.play_sfx("player_shot")
        pass
    
func _on_next_transition() -> void:
    if GameInputs.is_move():
        transition.emit("move")

func _on_enter() -> void:
    sprite.play("idle")
