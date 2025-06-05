use bevy::prelude::*;

pub fn quit_button(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
) {
    for interaction in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // Handle button press
                println!("Quit button pressed");
                std::process::exit(0);
            }
            Interaction::Hovered => {
                // Handle button hover
                println!("Quit button hovered");
            }
            Interaction::None => {
                // Handle button release or no interaction
                println!("Quit button released or not interacted with");
            }
        }
    }
}
