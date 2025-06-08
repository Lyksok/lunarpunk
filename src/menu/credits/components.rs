use bevy::prelude::Component;

#[derive(Component, Debug)]
pub enum MenuButtonAction {
    Back,
}

#[derive(Component)]
pub struct OnCreditsMenu;
