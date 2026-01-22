//! Input Bridge
//!
//! Handles input events and dispatches them to composables.

use crate::components::Clickable;
use bevy::prelude::*;

/// Handles button click interactions
#[allow(clippy::type_complexity)]
pub fn handle_button_interactions(
    interaction_query: Query<(&Interaction, &Clickable), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, clickable) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            (clickable.on_click)();
        }
    }
}

/// Handles general node interactions for clickable elements
pub fn handle_node_interactions(
    interaction_query: Query<(&Interaction, &Clickable), Changed<Interaction>>,
) {
    for (interaction, clickable) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            (clickable.on_click)();
        }
    }
}
