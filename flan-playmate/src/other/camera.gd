extends Camera2D

@export var max_lean_offset: float = 80.0
@export var lean_speed: float = 6.0
@export var return_speed: float = 3.0
@export var follow_speed: float = 5.0

var current_lean: float = 0.0

func _process(delta: float) -> void:
    if GameState.player == null:
        return
    _update_lean(delta)

func _update_lean(delta: float) -> void:
    var player_vel = GameState.player.velocity_component.velocity
    var lean_input = clampf(player_vel.x / 200.0, -1.0, 1.0)

    var target_lean = lean_input * max_lean_offset
    var speed = lean_speed if abs(lean_input) > 0.01 else return_speed

    current_lean = lerpf(current_lean, target_lean, speed * delta)

    offset.x = current_lean
