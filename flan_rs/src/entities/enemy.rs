use godot::classes::object::ConnectFlags;
use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

use crate::autoload::GameState;
use crate::{FlanExtension, components::*};

#[derive(GodotClass)]
#[class(init, base=CharacterBody2D)]
pub struct Enemy {
    #[export]
    hitbox: Option<Gd<HitboxComponent>>,
    #[export]
    hp: Option<Gd<HealthComponent>>,
    #[export]
    is_boss: bool,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Enemy {
    fn ready(&mut self) {
        let mut gm = FlanExtension::get_singleton::<GameState>().unwrap();
        gm.bind_mut().register_entity(self.to_gd().clone().upcast());

        let bm = gm.bind().bullet_manager.clone().unwrap();
        bm.signals()
            .hit_event()
            .builder()
            .flags(ConnectFlags::DEFERRED)
            .connect_other_mut(&*self, Self::hit_info);

        if self.is_boss {
            gm.bind_mut().boss_hp = self.hp.clone();
        }
    }
}

#[godot_api]
impl Enemy {
    #[func]
    fn hit_info(&mut self, e: Gd<Node2D>, d: i64) {
        let s = self.to_gd();
        let sid = s.instance_id();
        let eid = e.instance_id();
        if sid == eid {
            // TODO : Apply bullet damage properly
            let mut hp = self.hp.clone().unwrap();
            hp.bind_mut().take_damage(d);
        }
    }
}
