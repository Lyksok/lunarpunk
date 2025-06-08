use bevy::prelude::*;

use bevy::winit::WinitSettings;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WinitSettings::game());
    }
}
