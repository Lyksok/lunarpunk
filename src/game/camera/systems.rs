use crate::game::camera::components::OnCamera;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::f32::consts::PI;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        OnCamera,
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
pub fn move_mouse(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Camera3d>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    if mouse_motion_events.is_empty() {
        return;
    }

    if let Ok(window) = window.single() {
        if !window.cursor_options.visible {
            return;
        }

        if let Ok(mut transform) = query.single_mut() {
            let player_orient = PI / 2.0;
            let pos = transform.translation.truncate();

            for event in mouse_motion_events.read() {
                let target = Vec3::new(event.delta.x - window.width(), event.delta.y, 0.0);
                // Rotate the camera based on mouse movement
                transform.rotate(Quat::from_rotation_y(-event.delta.x * 0.002));
                transform.rotate(Quat::from_rotation_x(-event.delta.y * 0.002));
                transform.rotation.z = 0.0; // Prevent roll
            }
        }
    }
}
