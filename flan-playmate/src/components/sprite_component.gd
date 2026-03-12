class_name SpriteComponent
extends AnimatedSprite2D

@export var hp: HealthComponent
var tween

func _ready() -> void:
    if hp:
        tween = create_tween()
        tween.tween_method(_tweening, 0.0, 1.0, 0.1)
        tween.stop()
        tween.finished.connect(_done)
        hp.hp_change.connect(_hit)
        hp.dead.connect(_dead)

func _dead():
    _play()

func _hit(_val: int):
    _play()

func _play():
    tween.play()

func _done():
    tween.stop()
    material.set("shader_parameter/interpolation", 0.0)

func _tweening(x):
    material.set("shader_parameter/interpolation", x)
