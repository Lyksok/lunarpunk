use bevy::prelude::Component;

#[derive(Component, Debug)]
pub enum MenuButtonAction {
    Play,
    Settings,
    Credits,
    Quit,
}

#[derive(Component)]
pub struct OnMainMenu;
