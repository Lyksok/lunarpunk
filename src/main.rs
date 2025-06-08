use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy::winit::WinitSettings;
use lunarpunk::components::GameState;

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
        .insert_resource(WinitSettings::game())
        .init_state::<GameState>()
        .add_plugins((lunarpunk::menu::MenuPlugin, 
                     lunarpunk::game::GamePlugin))
        .run();
}
