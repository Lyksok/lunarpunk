use super::components::*;
use crate::menu::components::MenuState;
use bevy::prelude::*;
use crate::menu::systems::*;

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
            OnCreditsMenu,
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
                .with_child(text("Credits", 42.0));
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
                children![(button("Back"), MenuButtonAction::Back),],
            ));
        });
}

pub fn button_interaction(
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    input: Res<ButtonInput<MouseButton>>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    for (interaction, menu_button_action) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed if input.just_pressed(MouseButton::Left) => {
                // Handle button press
                match menu_button_action {
                    MenuButtonAction::Back => {
                        // Handle back button action
                        #[cfg(debug_assertions)]
                        println!("Back button pressed");
                        
                        menu_state.set(MenuState::Main);
                    }
                }
            }
            Interaction::Hovered => {
                // Handle button hover
            }
            Interaction::None => {
                // Handle button release or no interaction
            }
            _ => {}
        }
    }
}
