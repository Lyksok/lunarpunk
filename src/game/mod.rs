mod simple_3d;
mod movement;
mod map;
mod camera;

use bevy::prelude::*;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(simple_3d::Simple3DPlugin);
    }
}
