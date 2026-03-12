use godot::prelude::*;

use crate::pools::{Entity, EntityCollision};

#[derive(GodotConvert, Var, Export, Default, Clone, Debug)]
#[godot(via = GString)]
pub enum BulletType {
    #[default]
    Default,
}

#[derive(Default)]
#[repr(align(16))]
pub struct Bullet {
    pub active: bool,
    pub lifetime: f32,
    pub position: Vector2,
    pub velocity: Vector2,
    pub texture: Rect2,
    pub rotation: f32,
    pub radius: f32,
    pub collision: EntityCollision,
    pub bullet_type: BulletType,
}

impl Bullet {
    pub fn new(
        position: Vector2,
        velocity: Vector2,
        rotation: f32,
        radius: f32,
        texture: Rect2,
        collision: EntityCollision,
        bullet_type: BulletType,
    ) -> Self {
        Self {
            position,
            velocity,
            lifetime: 2.,
            rotation,
            radius,
            texture,
            collision,
            bullet_type,
            active: true,
        }
    }
}

pub struct BulletPool {
    pub items: Vec<Bullet>,
    pending_remove: Vec<u32>,
    events: Vec<u32>,
    next_id: u32,
}

impl BulletPool {
    pub fn new(pool_size: usize) -> Self {
        let mut items = Vec::with_capacity(pool_size);
        let pending_remove = Vec::with_capacity(pool_size);
        let events = Vec::with_capacity(pool_size);
        for _ in 0..pool_size {
            items.push(Bullet::default());
        }

        Self {
            items,
            pending_remove,
            events,
            next_id: 0,
        }
    }

    pub fn spawn(&mut self, out: Bullet) {
        let bullet = &mut self.items[self.next_id as usize];
        *bullet = out;

        self.next_id = (self.next_id + 1) % self.items.len() as u32;
    }

    pub fn update(&mut self, dt: f64) {
        for bullet in &mut self.items {
            if !bullet.active {
                continue;
            }
            if bullet.lifetime < 0. {
                bullet.active = false;
                continue;
            }

            bullet.position += bullet.velocity * dt as f32;
            bullet.lifetime -= dt as f32;
        }
    }

    pub fn resolve_collision(&mut self, entities: &Vec<Entity>) -> &Vec<u32> {
        self.pending_remove.clear();
        self.events.clear();
        for (i, bullet) in &mut self.items.iter().enumerate() {
            if !bullet.active {
                continue;
            }
            for (y, e) in entities.iter().enumerate() {
                // TODO : Maybe there are better way to check collision
                if e.collision != bullet.collision {
                    continue;
                }

                let dist = e.position.distance_to(bullet.position);
                if dist <= bullet.radius + e.radius {
                    self.pending_remove.push(i as u32);
                    self.events.push(y as u32);
                }
            }
        }

        for i in &self.pending_remove {
            let item = self.items.get_mut(*i as usize).unwrap();
            item.active = false;
        }

        &self.events
    }
}

impl Default for BulletPool {
    fn default() -> Self {
        BulletPool::new(1024)
    }
}
