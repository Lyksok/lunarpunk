use bevy::prelude::*;
use crate::menu::components::MenuState;

mod camera;
mod components;
mod main;
mod settings;
mod systems;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>()
            .add_plugins((
            camera::CameraPlugin,
            main::MainMenuPlugin,
            settings::SettingsMenuPlugin,
        ));
    }
}
