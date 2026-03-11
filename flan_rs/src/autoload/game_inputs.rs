use godot::{classes::Input, prelude::*};

#[derive(GodotClass)]
#[class(init, base=Object)]
pub struct GameInputs {
    #[var]
    dir: Vector2,

    base: Base<Object>,
}

#[godot_api]
impl GameInputs {
    #[func]
    fn update_movement(&mut self) -> Vector2 {
        let mut dir = Vector2::ZERO;
        let input = Input::singleton();

        if input.is_action_pressed("left") {
            dir.x = -1.;
        } else if input.is_action_pressed("right") {
            dir.x = 1.;
        }

        if input.is_action_pressed("up") {
            dir.y = -1.;
        } else if input.is_action_pressed("down") {
            dir.y = 1.;
        }

        self.dir = dir;
        dir
    }

    #[func]
    fn is_move(&mut self) -> bool {
        !self.dir.is_zero_approx()
    }

    #[func]
    fn is_focus(&mut self) -> bool {
        Input::singleton().is_action_pressed("focus")
    }

    #[func]
    fn is_attack(&mut self) -> bool {
        Input::singleton().is_action_pressed("confirm")
    }

    #[func]
    fn is_confirm(&mut self) -> bool {
        Input::singleton().is_action_just_pressed("confirm")
    }

    #[func]
    fn is_spell(&mut self) -> bool {
        Input::singleton().is_action_just_pressed("spell")
    }
}
