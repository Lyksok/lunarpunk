use bevy::prelude::*;
use crate::menu::menu_scene::MenuButtonAction;

pub fn main_menu_button_interaction(
    mut interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
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
                println!("Button hovered: {:?}", menu_button_action);
            }
            Interaction::None => {
                // Handle button release or no interaction
                println!("Button interaction ended: {:?}", menu_button_action);
            }
        }
    }
}
