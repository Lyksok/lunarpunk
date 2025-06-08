use bevy::prelude::*;

pub mod systems;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::spawn_camera);
    }
}
