mod components;
mod systems;

use super::components::*;
use crate::utils::despawn_screen;
use bevy::prelude::*;

pub struct CreditsMenuPlugin;

impl Plugin for CreditsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuState::Credits), systems::setup)
            .add_systems(
                Update,
                systems::button_interaction
                    .run_if(in_state(MenuState::Credits)),
            )
            .add_systems(
                OnExit(MenuState::Credits),
                despawn_screen::<components::OnCreditsMenu>,
            );
    }
}
