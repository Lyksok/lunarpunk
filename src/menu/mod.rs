use crate::menu::camera::CameraPlugin;
use crate::menu::main::LayoutPlugin;
use bevy::prelude::*;

pub mod camera;
mod components;
pub mod main;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((LayoutPlugin, CameraPlugin));
    }
}
