use godot::{classes::Engine, prelude::*};

use crate::game_state::GameState;

pub mod components;
pub mod entities;
pub mod game_state;
pub mod node_state;
pub mod state_machine;

struct FlanExtension;

#[gdextension]
unsafe impl ExtensionLibrary for FlanExtension {
    fn on_stage_init(stage: InitStage) {
        if stage == InitStage::Scene {
            Engine::singleton()
                .register_singleton(&GameState::class_id().to_string(), &GameState::new_alloc());
        }
    }

    fn on_stage_deinit(stage: InitStage) {
        if stage == InitStage::Scene {
            let mut engine = Engine::singleton();
            let singleton_name = &GameState::class_id().to_string_name();

            if let Some(my_singleton) = engine.get_singleton(singleton_name) {
                // Unregistering from Godot, and freeing from memory is required
                // to avoid memory leaks, warnings, and hot reloading problems.
                engine.unregister_singleton(singleton_name);
                my_singleton.free();
            } else {
                // You can either recover, or panic from here.
                godot_error!("Failed to get singleton");
            }
        }
    }
}
