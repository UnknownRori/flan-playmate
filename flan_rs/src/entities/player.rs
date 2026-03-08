use godot::classes::{CharacterBody2D, Engine, ICharacterBody2D};
use godot::prelude::*;

use crate::components::health_component::HealthComponent;
use crate::game_state::GameState;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    #[export]
    hp: Option<Gd<HealthComponent>>,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self { base, hp: None }
    }

    fn ready(&mut self) {
        let mut state = Engine::singleton()
            .get_singleton(&GameState::class_id().to_string_name())
            .unwrap()
            .try_cast::<GameState>()
            .unwrap();
        if let Some(hp) = &self.hp {
            let hp = hp.clone();
            state.set("player_hp", &hp.to_variant());
        }
    }
}
