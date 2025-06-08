use crate::menu::components::MenuState;
use bevy::prelude::*;

mod camera;
mod components;
mod main;
mod settings;
mod credits;
mod systems;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>().add_plugins((
            camera::CameraPlugin,
            main::MainMenuPlugin,
            settings::SettingsMenuPlugin,
            credits::CreditsMenuPlugin,
        ));
    }
}
