use godot::prelude::*;

use crate::components::health_component::HealthComponent;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct GameState {
    #[var]
    pub player_hp: Option<Gd<HealthComponent>>,
    base: Base<Node>,
}
