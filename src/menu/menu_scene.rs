use bevy::prelude::*;

fn button(text: &str) -> impl Bundle + use<> {
    (
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Button,
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            children![(
                Text::new(text),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextShadow::default(),
            )]
        )],
    )
}

fn text(text: &str) -> impl Bundle + use<> {
    (
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Text::new(text),
            TextFont {
                font_size: 42.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextShadow::default(),
        )],
    )
}

pub fn setup(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::srgb_u8(42, 42, 42)),
        ))
        .with_children(|builder| {
            // ############# Main title #############
            builder.spawn(text("Lunarpunk"));
            // ############# Quit button #############
            builder.spawn(button("Quit"));
        });
}
