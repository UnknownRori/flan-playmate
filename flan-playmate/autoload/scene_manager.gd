extends Node

const FADE_DURATION: float = 1.0

var _overlay: ColorRect

func _ready() -> void:
    _setup_overlay()

func _setup_overlay() -> void:
    var canvas := CanvasLayer.new()
    canvas.layer = 100
    add_child(canvas)
    
    _overlay = ColorRect.new()
    _overlay.color = Color.BLACK
    _overlay.set_anchors_preset(Control.PRESET_FULL_RECT)
    _overlay.mouse_filter = Control.MOUSE_FILTER_IGNORE
    _overlay.modulate.a = 0.0
    canvas.add_child(_overlay)

func go_to(path: String) -> void:
    await _fade_out()
    get_tree().change_scene_to_file(path)
    await get_tree().process_frame
    await _fade_in()
    
    
func _fade_out() -> void:
    var tween := create_tween()
    tween.set_ease(Tween.EASE_IN)
    tween.set_trans(Tween.TRANS_CUBIC)
    tween.tween_property(_overlay, "modulate:a", 1.0, FADE_DURATION)
    await tween.finished

func _fade_in() -> void:
    var tween := create_tween()
    tween.set_ease(Tween.EASE_OUT)
    tween.set_trans(Tween.TRANS_CUBIC)
    tween.tween_property(_overlay, "modulate:a", 0.0, FADE_DURATION)
    await tween.finished
