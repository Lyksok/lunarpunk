use super::components::*;
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
                    (button("Play"), MenuButtonAction::Play),
                    (button("Settings"), MenuButtonAction::Settings),
                    (button("Credits"), MenuButtonAction::Credits),
                    (button("Quit"), MenuButtonAction::Quit),
                ],
            ));
        });
}

pub fn button_interaction(
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, menu_button_action) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // Handle button press
                match menu_button_action {
                    MenuButtonAction::Play => {
                        // Handle play button action
                        println!("Play button pressed");
                    }
                    MenuButtonAction::Settings => {
                        // Handle settings button action
                        println!("Settings button pressed");
                    }
                    MenuButtonAction::Credits => {
                        // Handle credits button action
                        println!("Credits button pressed");
                    }
                    MenuButtonAction::Quit => {
                        // Handle quit button action
                        println!("Quit button pressed");
                        std::process::exit(0);
                    }
                }
            }
            Interaction::Hovered => {
                // Handle button hover
                // println!("Button hovered: {:?}", menu_button_action);
            }
            Interaction::None => {
                // Handle button release or no interaction
                // println!("Button released: {:?}", menu_button_action);
            }
        }
    }
}
