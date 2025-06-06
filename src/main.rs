use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy::winit::WinitSettings;
use lunarpunk::{
    camera::menu_camera::spawn_camera, menu::button_interactions::quit_button,
    menu::menu_scene::setup,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Lunarpunk".to_string(),
                resolution: WindowResolution::new(1280.0, 720.0).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, (setup, spawn_camera))
        .add_systems(Update, quit_button)
        .run();
}
