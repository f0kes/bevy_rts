use bevy::prelude::*;

use crate::{action::Action, input_map::remap_input};

#[derive(Default)]
pub struct InputActionsPlugin;
impl Plugin for InputActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonInput<Action>>();
        app.add_systems(Update, remap_input);
    }
}
