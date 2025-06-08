use crate::game::map::components::OnMap;
use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a simple entity with a transform
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        OnMap,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // Add a light source
    commands.spawn((
        PointLight {
            intensity: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        OnMap,
    ));
}
