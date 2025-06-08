use crate::game::camera::components::*;
use crate::game::components::Player;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow, WindowFocused};
use std::f32::consts::PI;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        OnCamera,
        Player,
        Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
pub fn rotate_camera(
    mut player: Single<&mut Transform, With<Player>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    if !window.focused {
        return;
    }
    let base_sensitivity = 5.0;
    let sensitivity = base_sensitivity / window.width().min(window.height());

    let (mut yaw, mut pitch, _) = player.rotation.to_euler(EulerRot::YXZ);
    pitch -= mouse_motion.delta.y * sensitivity;
    yaw -= mouse_motion.delta.x * sensitivity;
    pitch = pitch.clamp(-PI / 2.0, PI / 2.0);

    player.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
}

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut delta = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        delta.z += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        delta.z -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        delta.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        delta.x += 1.0;
    }

    let forward = player.forward().as_vec3() * delta.z;
    let right = player.right().as_vec3() * delta.x;
    let to_move = (forward + right).normalize_or_zero();

    player.translation += to_move * time.delta_secs() * 30.0;
}

pub fn apply_grab(
    // tells bevy what event to watch for with this observer
    grab: Trigger<GrabEvent>,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
) {
    if **grab {
        window.cursor_options.visible = false;
        window.cursor_options.grab_mode = CursorGrabMode::Locked
    } else {
        window.cursor_options.visible = true;
        window.cursor_options.grab_mode = CursorGrabMode::None;
    }
}

pub fn focus_events(mut events: EventReader<WindowFocused>, mut commands: Commands) {
    if let Some(event) = events.read().last() {
        commands.trigger(GrabEvent(event.focused));
    }
}

pub fn toggle_grab(
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    window.focused = !window.focused;
    commands.trigger(GrabEvent(window.focused));
}
