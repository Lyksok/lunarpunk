mod systems;
mod components;

use bevy::prelude::*;
use crate::components::GameState;

pub struct Simple3DPlugin;

impl Plugin for Simple3DPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Game), systems::setup);
    }
}