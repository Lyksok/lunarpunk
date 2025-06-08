use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn despawn_camera(mut commands: Commands, query: Query<Entity, With<Camera2d>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
