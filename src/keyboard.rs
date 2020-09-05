use bevy::prelude::*;
use crate::systems::keyboard::{keyboard_system, robbo_dir_reset, RobboDir};
pub struct KeyboardPlugin;

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(KeyboardPlugin)
            .add_resource(RobboDir::default())
            .add_stage_before(stage::PRE_UPDATE, "keyboard")
            .add_stage_after(stage::POST_UPDATE, "robbo_dir_reset")
            .add_system_to_stage("keyboard", keyboard_system.system())
            .add_system_to_stage("robbo_dir_reset", robbo_dir_reset.system());
    }
}
