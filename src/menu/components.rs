use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
pub enum MenuState {
    #[default]
    Main,
    Settings,
    Credits,
}
