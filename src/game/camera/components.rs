use bevy::prelude::{Component, Deref, Event};

#[derive(Component)]
pub struct OnCamera;

#[derive(Event, Deref)]
pub struct GrabEvent(pub bool);
