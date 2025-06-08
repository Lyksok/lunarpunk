mod camera;
mod components;
mod map;
mod movement;

use bevy::prelude::*;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            map::MapPlugin,
            camera::CameraPlugin,
            movement::MovementPlugin,
        ));
    }
}
