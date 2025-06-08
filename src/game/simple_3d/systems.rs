use bevy::prelude::*;

pub fn setup(mut commands: Commands,mut meshes: ResMut<Assets<Mesh>>,
             mut materials: ResMut<Assets<StandardMaterial>>,) {
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
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn move_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::Z;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction -= Vec3::Z;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction -= Vec3::X;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::X;
        }
        
        transform.translation += direction.normalize_or_zero() * 5.0 * time.delta_secs();
    }
}