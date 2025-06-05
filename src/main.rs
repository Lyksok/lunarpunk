use bevy::prelude::*;
use bevy::winit::WinitSettings;
use lunarpunk::{
    camera::menu_camera::spawn_camera, menu::button_interactions::quit_button,
    menu::menu_scene::setup,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, (setup, spawn_camera))
        .add_systems(Update, quit_button)
        .run();
}
