use crate::components::GameState::Menu;
use crate::menu::camera::systems::despawn_camera;
use bevy::prelude::*;

pub mod systems;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Menu), systems::spawn_camera)
            .add_systems(OnExit(Menu), despawn_camera);
    }
}
