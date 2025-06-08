use bevy::prelude::*;

pub fn move_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction -= Vec3::Z;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::Z;
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
