use godot::classes::object::ConnectFlags;
use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

use crate::FlanExtension;
use crate::autoload::GameState;
use crate::components::*;

#[derive(GodotClass)]
#[class(init, base=CharacterBody2D)]
pub struct Player {
    #[export]
    hp: Option<Gd<HealthComponent>>,
    #[export]
    hitbox: Option<Gd<HitboxComponent>>,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn ready(&mut self) {
        let mut gm = FlanExtension::get_singleton::<GameState>().unwrap();
        if let Some(hp) = &self.hp {
            let hp = hp.clone();
            gm.bind_mut().player_hp = Some(hp);
            gm.bind_mut().player = Some(self.to_gd().clone());
        }
        gm.bind_mut().register_entity(self.to_gd().clone().upcast());

        let bm = gm.bind().bullet_manager.clone().unwrap();
        bm.signals()
            .hit_event()
            .builder()
            .flags(ConnectFlags::DEFERRED)
            .connect_other_mut(&*self, Self::hit_info);
    }
}

#[godot_api]
impl Player {
    #[func]
    fn hit_info(&mut self, e: Gd<Node2D>) {
        let s = self.to_gd();
        let sid = s.instance_id();
        let eid = e.instance_id();
        if sid == eid {
            // TODO : Apply bullet damage properly
            let mut hp = self.hp.clone().unwrap();
            hp.bind_mut().take_damage(1.0);
        }
    }
}
