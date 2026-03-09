use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

use crate::FlanExtension;
use crate::autoload::GameState;
use crate::components::health_component::HealthComponent;

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
        let mut gm = FlanExtension::get_singleton::<GameState>().unwrap();
        if let Some(hp) = &self.hp {
            let hp = hp.clone();
            gm.bind_mut().player_hp = Some(hp);
        }
    }
}
