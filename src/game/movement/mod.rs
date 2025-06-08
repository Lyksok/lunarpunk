mod components;
mod systems;

use crate::components::GameState;
use bevy::prelude::*;
#[derive(Component)]
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            systems::move_camera.run_if(in_state(GameState::Game)),
        );
    }
}
