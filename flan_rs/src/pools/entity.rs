use godot::prelude::*;

use crate::components::*;

#[derive(GodotConvert, Var, Export, Default, Clone, Debug, PartialEq)]
#[godot(via = GString)]
pub enum EntityCollision {
    Player,
    #[default]
    Enemy,
}

#[derive(Default, Debug)]
#[repr(align(16))]
pub struct Entity {
    pub position: Vector2,
    pub radius: f32,
    pub collision: EntityCollision,
}

impl Entity {
    pub fn new(position: Vector2, radius: f32, collision: EntityCollision) -> Self {
        Self {
            position,
            radius,
            collision,
        }
    }
}

// TODO : Need to convert into Spatial Hash for efficient collision resolver
pub struct EntityPool {
    pub items: Vec<Entity>,
    pub parents: Vec<Gd<Node2D>>,
}

impl EntityPool {
    pub fn new(pool_size: usize) -> Self {
        let items = Vec::with_capacity(pool_size);
        let parents = Vec::with_capacity(pool_size);
        Self { items, parents }
    }

    pub fn register(&mut self, entity: Gd<Node2D>) {
        self.parents.push(entity);
    }

    pub fn unregister(&mut self, entity: Gd<Node2D>) {
        let mut found = false;
        let mut id = 0;
        for (i, p) in self.parents.iter().enumerate() {
            if *p == entity {
                found = true;
                id = i;
            }
        }

        if found {
            self.parents.remove(id);
        }
    }

    pub fn get_entity(&mut self, i: u32) -> Option<Gd<Node2D>> {
        self.parents.get(i as usize).cloned()
    }

    pub fn prepare(&mut self) {
        self.items.clear();
        for i in &self.parents {
            let hitbox = i.get("hitbox").to::<Gd<HitboxComponent>>();
            let hp = i.get("hp").to::<Gd<HealthComponent>>();

            if hp.bind().invulnerable {
                continue;
            }

            let position = i.get_global_position();
            let collision = hitbox.bind().mask.clone();
            let radius = hitbox.bind().radius as f32;
            self.items.push(Entity {
                position,
                collision,
                radius,
            });
        }
    }

    pub fn insert(&mut self, entity: Entity) {
        self.items.push(entity);
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}
