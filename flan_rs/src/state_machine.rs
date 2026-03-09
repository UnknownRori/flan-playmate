use godot::classes::object::ConnectFlags;
use godot::classes::{INode, Node};
use godot::prelude::*;

use crate::node_state::NodeState;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct StateMachine {
    #[export]
    current: Option<Gd<NodeState>>,
    initial: Option<Gd<NodeState>>,
    lists: VarDictionary,
    base: Base<Node>,
}

#[godot_api]
impl INode for StateMachine {
    fn init(base: Base<Node>) -> Self {
        Self {
            current: None,
            initial: None,
            lists: VarDictionary::new(),
            base,
        }
    }

    fn ready(&mut self) {
        let child = self.base().get_children();
        for i in child.iter_shared() {
            let node = i
                .try_cast::<NodeState>()
                .expect("StateMachine: Wrong type [it should be NodeState]");

            node.signals()
                .transition()
                .builder()
                .flags(ConnectFlags::DEFERRED)
                .connect_other_mut(&*self, Self::transition_to);

            let name = node.get_name().to_lower();
            self.lists.set(&name, &node);
        }

        if let Some(initial) = &mut self.initial {
            self.current = Some(initial.clone());
            let _ = initial
                .try_call("on_enter", &[])
                .expect("fail to call on_enter");
        }
    }

    fn process(&mut self, dt: f64) {
        if let Some(initial) = &mut self.current {
            initial.bind_mut().on_process(dt);
        }
    }

    fn physics_process(&mut self, dt: f64) {
        if let Some(initial) = &mut self.current {
            initial.bind_mut().on_physics_process(dt);
            initial.bind_mut().on_next_transition();
        }
    }
}

#[godot_api]
impl StateMachine {
    fn transition_to(&mut self, name: GString) {
        let name = name.to_lower();
        let value = self.lists.get(&name).expect("No node found!");
        let mut node = value.try_to::<Gd<NodeState>>().expect("Not NodeState");
        if let Some(current) = &mut self.current {
            current.bind_mut().on_leave();
        }
        node.bind_mut().on_enter();
        self.current = Some(node);
    }
}
