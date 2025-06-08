mod components;
mod systems;

use crate::components::GameState;
use crate::game::camera::components::OnCamera;
use crate::utils::despawn_screen;
use bevy::prelude::*;

#[derive(Component)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), systems::setup)
            .add_systems(
                Update,
                systems::move_mouse.run_if(in_state(GameState::Game)),
            )
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnCamera>);
    }
}
