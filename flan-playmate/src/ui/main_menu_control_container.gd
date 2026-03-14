extends Control

@export var new_run_scene: String = "res://scenes/DemoScene.tscn"
@onready var buttons = [$"VBoxContainer/New Run", $VBoxContainer/Option, $VBoxContainer/Exit]
@onready var cursor = $Cursor
var current = 0

func _ready() -> void:
    _update_selection()

func _process(_delta: float) -> void:
    if Input.is_action_just_pressed("down"):
        current = (current + 1) % len(buttons)
        _update_selection()
        AudioManager.play_sfx("select")
    if Input.is_action_just_pressed("up"):
        current = (current - 1) % len(buttons)
        _update_selection()
        AudioManager.play_sfx("select")
    if Input.is_action_just_pressed("confirm"):
        _click()
        AudioManager.play_sfx("select")

func _update_selection():
    for i in range(0, len(buttons)):
        var btn = buttons[i] as Button
        btn.release_focus()

    var button = buttons[current] as Button
    cursor.global_position.y = button.global_position.y - 4
    button.grab_focus()

func _click():
    if current == 0:
        _on_new_run_pressed()
    elif current == 1:
        _on_exit_pressed()

func _on_new_run_pressed() -> void:
    SceneManager.go_to(new_run_scene)
    $"VBoxContainer/New Run".disabled = true


func _on_exit_pressed() -> void:
    get_tree().quit()
    $"VBoxContainer/Exit".disabled = true
