use godot::classes::*;
use godot::prelude::*;

use crate::pools::EntityCollision;

#[derive(GodotClass)]
#[class(init, base=Area2D)]
/// This component automatically registering collision box to [`BulletManager`]
/// but it only support circle collision box instead of whatever it registered collision box area
/// so there is radius exported variable
pub struct HitboxComponent {
    #[export]
    pub parent: Option<Gd<Node2D>>,
    #[export]
    pub radius: f64,
    #[export]
    pub mask: EntityCollision,
    base: Base<Area2D>,
}

#[godot_api]
impl HitboxComponent {
    #[signal]
    fn hit(by: Variant);
}
