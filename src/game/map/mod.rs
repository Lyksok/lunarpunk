mod components;
mod ressources;
mod systems;

use crate::components::GameState;
use crate::game::map::components::OnMap;
use crate::utils::despawn_screen;
use bevy::prelude::*;

#[derive(Component)]
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), systems::setup)
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnMap>);
    }
}
