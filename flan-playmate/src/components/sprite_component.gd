class_name SpriteComponent
extends AnimatedSprite2D

@export var hp: HealthComponent
@export var duration: float = 0.1
@export var repeat: int = 1
var current = 0
var tween

func _ready() -> void:
    if hp:
        tween = create_tween()
        tween.tween_method(_tweening, 0.0, 1.0, duration)
        tween.stop()
        tween.finished.connect(_done)
        hp.hp_change.connect(_hit)
        hp.dead.connect(_dead)

func _dead():
    _play()
    current = repeat

func _hit(_val: int):
    _play()
    current = repeat

func _play():
    tween.play()

func _done():
    tween.stop()
    if current > 0:
        _play()

    material.set("shader_parameter/interpolation", 0.0)
    current -= 1

func _tweening(x):
    material.set("shader_parameter/interpolation", x)
