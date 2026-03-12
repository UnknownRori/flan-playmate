use godot::classes::{INode, Node};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct HealthComponent {
    #[export]
    current_hp: i64,
    #[export]
    max_hp: i64,
    base: Base<Node>,
}

#[godot_api]
impl INode for HealthComponent {
    fn init(base: Base<Node>) -> Self {
        Self {
            current_hp: 0,
            max_hp: 0,
            base,
        }
    }
}

#[godot_api]
impl HealthComponent {
    #[func]
    pub fn take_damage(&mut self, value: i64) {
        self.current_hp -= value;
        let hp = self.current_hp;
        if hp < 0 {
            self.signals().dead().emit();
        } else {
            self.signals().hp_change().emit(hp);
        }
    }

    #[func]
    pub fn set_hp(&mut self, value: i64) {
        self.current_hp = value;
        self.signals().hp_change().emit(value);
    }

    #[func]
    pub fn set_hp_max(&mut self, value: i64) {
        self.max_hp = value;
        let current = self.current_hp;
        self.signals().hp_change().emit(current);
    }

    #[signal]
    fn hp_change(new_value: i64);

    #[signal]
    fn dead();
}
