mod components;
mod systems;

use crate::components::GameState;
use crate::game::camera::components::OnCamera;
use crate::utils::despawn_screen;
use bevy::input::common_conditions::input_just_released;
use bevy::prelude::*;

#[derive(Component)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Game),
            (systems::setup, systems::focus_events.after(systems::setup)),
        )
        .add_systems(
            Update,
            (
                systems::rotate_camera.run_if(in_state(GameState::Game)),
                systems::move_player
                    .run_if(in_state(GameState::Game))
                    .after(systems::rotate_camera),
                systems::focus_events.run_if(in_state(GameState::Game)),
                systems::toggle_grab.run_if(input_just_released(KeyCode::Escape)),
            ),
        )
        .add_observer(systems::apply_grab)
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnCamera>);
    }
}
