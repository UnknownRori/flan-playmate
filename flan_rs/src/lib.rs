use std::error::Error;

use godot::{classes::Engine, obj::NewAlloc, prelude::*};

use crate::autoload::*;

pub mod autoload;
pub mod bullet_manager;
pub mod bullet_renderer;
pub mod components;
pub mod entities;
pub mod node_state;
pub mod pools;
pub mod state_machine;

struct FlanExtension;

#[gdextension]
unsafe impl ExtensionLibrary for FlanExtension {
    fn on_stage_init(stage: InitStage) {
        if stage == InitStage::Scene {
            Self::register_singleton::<GameState>();
            Self::register_singleton::<GameInputs>();
        }
    }

    fn on_stage_deinit(stage: InitStage) {
        if stage == InitStage::Scene {
            Self::unregister_singleton::<GameState>().unwrap();
            Self::unregister_singleton::<GameInputs>().unwrap();
        }
    }
}

impl FlanExtension {
    fn get_singleton<T: GodotClass + NewAlloc + Inherits<Object>>() -> Option<Gd<T>> {
        Engine::singleton()
            .get_singleton(&T::class_id().to_string_name())
            .map(|i| i.cast::<T>())
    }

    fn register_singleton<T: GodotClass + NewAlloc + Inherits<Object>>() {
        Engine::singleton().register_singleton(&T::class_id().to_string_name(), &T::new_alloc());
    }

    fn unregister_singleton<T: GodotClass>() -> Result<(), Box<dyn Error>> {
        let mut engine = Engine::singleton();
        let name = &T::class_id().to_string_name();

        if let Some(singleton) = engine.get_singleton(name) {
            engine.unregister_singleton(name);
            singleton.free();
        } else {
            godot_warn!("Failed to unregister singleton: {}", name);
        }

        Ok(())
    }
}
