pub mod components;
pub mod systems;

use bevy::prelude::*;

pub struct LayoutPlugin;

impl Plugin for LayoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::setup)
            .add_systems(Update, systems::button_interaction);
    }
}
