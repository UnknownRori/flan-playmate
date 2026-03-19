class_name BossAttack
extends NodeState

@export var entity: CharacterBody2D
@export var hp: HealthComponent
@export var timeout_timer: Timer

@export_group("Attack Information")
@export var timeout: float
@export var hp_max: int
@export var invulnerable: bool
@export var spell_name = "Unknown"
@export var is_spell_card: bool = false
@export var next_state: BossAttack

@export_group("Other Information")
@export var delay_before_attack_start: float = 0.5

var is_done: bool = false

func _on_enter() -> void:
    is_done = false
    hp.set_hp(hp_max)
    hp.set_hp_max(hp_max)
    hp.invulnerable = invulnerable

    timeout_timer.start(timeout)
    timeout_timer.paused = false

    hp.dead.connect(_on_dead)
    timeout_timer.timeout.connect(_on_dead)
    if is_spell_card:
        # Emit Spell card warning
        pass
    _prepare_attack()

func _prepare_attack():
    await get_tree().create_timer(delay_before_attack_start).timeout
    _start_attack()

func _disconnect_event() -> void:
    hp.dead.disconnect(_on_dead)
    timeout_timer.timeout.disconnect(_on_dead)

func _on_dead() -> void:
    is_done = true
    if next_state != null:
        transition.emit(next_state.name)
    

func _on_leave() -> void:
    _disconnect_event()

func _start_attack() -> void:
    # INFO: Dev should overwrite this function
    pass
