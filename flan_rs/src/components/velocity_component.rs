use godot::{classes::CharacterBody2D, prelude::*};

use crate::vec2f;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct VelocityComponent {
    #[export]
    entity: Option<Gd<CharacterBody2D>>,

    #[export]
    acceleration: f64,
    #[export]
    max_speed: f64,
    #[export]
    friction: f64,
    #[export]
    focus: bool,

    #[var]
    velocity: Vector2,
    #[var]
    direction: Vector2,

    base: Base<Node>,
}

#[godot_api]
impl INode for VelocityComponent {
    fn process(&mut self, dt: f64) {
        let max_speed = if self.focus {
            self.max_speed * 0.5
        } else {
            self.max_speed
        };
        self.velocity += self.direction * self.acceleration as f32;
        self.velocity = self
            .velocity
            .clamp(vec2f!(-max_speed as f32), vec2f!(max_speed as f32));
        self.velocity *= self.friction as f32;

        let mut entity = self.entity.clone().unwrap();
        let mut pos = entity.get_position();
        pos += self.velocity * dt as f32;
        entity.set_position(pos);
        entity.move_and_slide();
    }
}

#[godot_api]
impl VelocityComponent {
    #[func]
    pub fn set_dir(&mut self, direction: Vector2) {
        self.direction = direction;
    }

    #[func]
    pub fn clear_velocity(&mut self) {
        self.velocity = vec2f!(0.);
    }
}
