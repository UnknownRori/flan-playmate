use godot::classes::{INode, Node};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct NodeState {
    base: Base<Node>,
}

#[godot_api]
impl INode for NodeState {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }

    fn process(&mut self, _dt: f64) {}
    fn physics_process(&mut self, _dt: f64) {}
}

#[godot_api]
impl NodeState {
    #[func(virtual)]
    pub fn on_process(&mut self, _dt: f64) {}

    #[func(virtual)]
    pub fn on_physics_process(&mut self, _dt: f64) {}

    #[func(virtual)]
    pub fn on_enter(&mut self) {}

    #[func(virtual)]
    pub fn on_leave(&mut self) {}

    #[func(virtual)]
    pub fn on_next_transition(&mut self) {}

    #[signal]
    pub fn transition(name: GString);
}
