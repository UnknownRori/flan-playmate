use godot::prelude::*;

use crate::{FlanExtension, autoload::GameState, pools::*};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct BulletManager {
    #[export]
    pool_size: i64,
    pool: BulletPool,
    entities: EntityPool,
    base: Base<Node>,
}

#[godot_api]
impl INode for BulletManager {
    fn init(base: Base<Node>) -> Self {
        let pool_size = 1024 as i64;
        Self {
            pool: BulletPool::new(pool_size as usize),
            entities: EntityPool::new(pool_size as usize),
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
        self.entities.prepare();
        let events = self.pool.resolve_collision(&self.entities.items).clone();
        for e in events {
            let damage = e.damage;
            let obj = self.entities.get_entity(e.id).unwrap();
            self.signals().hit_event().emit(&obj, damage as i64);
        }
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
    pub fn spawn(
        &mut self,
        position: Vector2,
        velocity: Vector2,
        rotation: f32,
        radius: f32,
        texture: Rect2,
        collision: EntityCollision,
        bullet_type: BulletType,
        damage: i64,
    ) {
        let bullet = Bullet::new(
            position,
            velocity,
            rotation,
            radius,
            texture,
            collision,
            bullet_type,
            damage as i32,
        );
        self.pool.spawn(bullet);
    }

    #[func]
    pub fn register(&mut self, entity: Gd<Node2D>) {
        self.entities.register(entity);
    }

    #[signal]
    pub fn hit_event(target: Gd<Node2D>, damage: i64);

    //#[func]
    //pub fn add_entity(&mut self, position: Vector2, radius: f32, collision: EntityCollision) {
    //    self.entities
    //        .insert(Entity::new(position, radius, collision));
    //}
}
