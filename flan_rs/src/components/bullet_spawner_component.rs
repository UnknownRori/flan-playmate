use godot::prelude::*;

use crate::{FlanExtension, autoload::GameState, pools::*, resources::BulletResource};

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct BulletSpawnerComponent {
    #[export]
    target: EntityCollision,
    #[export]
    bullet_type: BulletType,
    #[export]
    damage: i64,
    #[export]
    bullet: Option<Gd<BulletResource>>,

    base: Base<Node>,
}

#[godot_api]
impl BulletSpawnerComponent {
    #[func]
    fn spawn(&mut self, position: Vector2, velocity: Vector2, rotation: f32) {
        let mut gm = FlanExtension::get_singleton::<GameState>().unwrap();
        let bl = self.bullet.clone().unwrap();
        let texture = bl.bind().texture;
        let radius = bl.bind().collision_circle as f32;
        let collision = self.target.clone();
        let bullet_type = self.bullet_type.clone();
        let damage = self.damage;
        gm.bind_mut().spawn_bullet(
            position,
            velocity,
            rotation,
            radius,
            texture,
            collision,
            bullet_type,
            damage,
        );
    }
}
