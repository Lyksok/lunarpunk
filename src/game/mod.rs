mod camera;
mod components;
mod map;

use bevy::prelude::*;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((map::MapPlugin, camera::CameraPlugin));
    }
}
