// publics
pub use self::{
    action_event::OnActionActive, action_event::OnActionBegin, action_event::OnActionEnd,
    action_event::OnActionProgress, axis::GamepadAxis, axis::MouseAxis, bindings::Bindings,
    event_phase::EventPhase, kurinji::Kurinji,
};

// crates
mod action_event;
mod axis;
mod bindings;
mod event_phase;
mod kurinji;
mod stack;
mod util;

mod action;
mod gamepad;
mod keyboard;
mod mouse;
mod serde;

use bevy::app::prelude::*;
use bevy::ecs::IntoSystem;

/// Adds input mapping (via code or json/ron) to an App
#[derive(Default)]
pub struct KurinjiPlugin;

impl Plugin for KurinjiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // input map
            .init_resource::<Kurinji>()
            // events
            .add_event::<OnActionActive>()
            .add_event::<OnActionBegin>()
            .add_event::<OnActionProgress>()
            .add_event::<OnActionEnd>()
            .add_system_to_stage(stage::EVENT, Kurinji::action_event_producer.system())
            // reset
            .add_system_to_stage(stage::PRE_UPDATE, Kurinji::action_reset_system.system())
            // joystick
            .add_system_to_stage(
                stage::UPDATE,
                Kurinji::gamepad_connection_event_system.system(),
            )
            .add_system_to_stage(
                stage::UPDATE,
                Kurinji::gamepad_button_press_input_system.system(),
            )
            .add_system_to_stage(stage::UPDATE, Kurinji::gamepad_axis_system.system())
            // keyboard
            .add_system_to_stage(stage::UPDATE, Kurinji::kb_key_press_input_system.system())
            // mouse
            .add_system_to_stage(
                stage::UPDATE,
                Kurinji::mouse_button_press_input_system.system(),
            )
            .add_system_to_stage(stage::UPDATE, Kurinji::mouse_move_event_system.system());
    }
}
