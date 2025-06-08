use super::components::*;
use crate::components::GameState;
use crate::menu::components::MenuState;
use crate::menu::systems::*;
use bevy::app::AppExit;
use bevy::prelude::*;

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
            OnMainMenu,
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
                .with_child(text("Lunarpunk", 42.0));
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
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
    input: Res<ButtonInput<MouseButton>>,
) {
    for (interaction, menu_button_action) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // Handle button press
                match menu_button_action {
                    MenuButtonAction::Play if input.just_pressed(MouseButton::Left) => {
                        // Handle play button action
                        #[cfg(debug_assertions)]
                        println!("Play button pressed");
                        
                        menu_state.set(MenuState::Disabled);
                        game_state.set(GameState::Game);
                    }
                    MenuButtonAction::Settings if input.just_pressed(MouseButton::Left) => {
                        // Handle settings button action
                        #[cfg(debug_assertions)]
                        println!("Settings button pressed");

                        menu_state.set(MenuState::Settings);
                    }
                    MenuButtonAction::Credits if input.just_pressed(MouseButton::Left) => {
                        // Handle credits button action
                        #[cfg(debug_assertions)]
                        println!("Credits button pressed");

                        menu_state.set(MenuState::Credits);
                    }
                    MenuButtonAction::Quit if input.just_pressed(MouseButton::Left) => {
                        // Handle quit button action
                        #[cfg(debug_assertions)]
                        println!("Quit button pressed");

                        app_exit_events.write(AppExit::Success);
                    }
                    _ => {}
                }
            }
            Interaction::Hovered => {
                // Handle button hover
            }
            Interaction::None => {
                // Handle button release or no interaction
            }
        }
    }
}
