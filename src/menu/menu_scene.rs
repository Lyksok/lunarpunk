use bevy::prelude::*;

fn button(text: &str) -> impl Bundle + use<> {
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

fn text(text: &str) -> impl Bundle + use<> {
    (
        Text::new(text),
        TextFont {
            font_size: 42.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
        TextShadow::default(),
    )
}

pub fn setup(mut commands: Commands) {
    commands
        .spawn((
            Node {
                //align_items: AlignItems::Center,
                //justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect {
                    left: Val::Px(50.0),
                    right: Val::Px(50.0),
                    top: Val::Px(50.0),
                    bottom: Val::Px(50.0),
                },
                ..default()
            },
            BackgroundColor(Color::srgb_u8(42, 42, 42)),
        ))
        .with_children(|builder| {
            // ############# Main title #############
            builder
                .spawn(Node {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(50.0),
                    ..default()
                })
                .with_child(text("Lunarpunk"));
            // ############# Quit button #############
            builder.spawn((
                Node {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    width: Val::Percent(50.0),
                    ..default()
                },
                children![
                    button("Play"),
                    button("Settings"),
                    button("Credits"),
                    button("Quit"),
                ],
            ));
        });
}
