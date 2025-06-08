use bevy::prelude::*;

#[derive(Component, Deref)]
pub struct MenuState(Menu);

#[derive(Component, Debug)]
pub enum Menu {
    Main,
    Settings,
    Credits,
}
