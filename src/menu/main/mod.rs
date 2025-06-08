pub mod components;
pub mod systems;

use super::components::*;
use crate::utils::despawn_screen;
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(MenuState::Main), systems::setup)
            .add_systems(Update, systems::button_interaction)
            .add_systems(
                OnExit(MenuState::Main),
                despawn_screen::<components::OnMainMenu>,
            );
    }
}
