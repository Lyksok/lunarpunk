mod components;
mod systems;

use super::components::*;
use crate::utils::despawn_screen;
use bevy::prelude::*;

pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuState::Settings), systems::setup)
            .add_systems(
                Update,
                systems::button_interaction.run_if(in_state(MenuState::Settings)),
            )
            .add_systems(
                OnExit(MenuState::Settings),
                despawn_screen::<components::OnSettingsMenu>,
            );
    }
}
