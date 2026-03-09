use godot::prelude::*;

use crate::{FlanExtension, autoload::GameState, pools::bullet::*};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct BulletManager {
    #[export]
    pool_size: i64,
    pool: BulletPool,
    base: Base<Node>,
}

#[godot_api]
impl INode for BulletManager {
    fn init(base: Base<Node>) -> Self {
        let pool_size = 1024 as i64;
        Self {
            pool: BulletPool::new(pool_size as usize),
            pool_size,
            base,
        }
    }

    fn ready(&mut self) {
        let mut gm = FlanExtension::get_singleton::<GameState>().unwrap();
        gm.bind_mut().bullet_manager = Some(self.to_gd());
    }

    fn physics_process(&mut self, dt: f64) {
        self.pool.update(dt);
    }
}

#[godot_api]
impl BulletManager {
    pub fn get_size(&self) -> i64 {
        self.pool_size
    }

    pub fn get_bullets(&self) -> &Vec<Bullet> {
        &self.pool.items
    }

    #[func]
    pub fn spawn(&mut self, position: Vector2, velocity: Vector2, texture: Rect2) {
        self.pool.spawn(position, velocity, texture);
    }
}
