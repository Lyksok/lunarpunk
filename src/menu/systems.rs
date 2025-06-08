use bevy::prelude::*;

pub fn button(text: &str) -> impl Bundle + use<> {
    (
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Px(250.0),
            padding: UiRect {
                left: Val::Px(20.0),
                right: Val::Px(20.0),
                top: Val::Px(10.0),
                bottom: Val::Px(10.0),
            },
            ..default()
        },
        Button,
        BorderColor(Color::BLACK),
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        children![(
            Text::new(text),
            TextFont {
                font_size: 32.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextShadow::default()
        )],
    )
}

pub fn text(text: &str, size: f32) -> impl Bundle + use<> {
    (
        Text::new(text),
        TextFont {
            font_size: size,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
        TextShadow::default(),
    )
}