extends Player


func _on_health_component_dead() -> void:
    print("Dead")


func _on_health_component_hp_change(new_value: int) -> void:
    # TODO : Reset the player
    pass
