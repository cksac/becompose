//! BECOMPOSE Plugin
//!
//! Main Bevy plugin for BECOMPOSE.

use bevy::prelude::*;

use crate::composition::{CompositionTree, DirtyFlags};
use super::{sync_composition_to_entities, handle_button_interactions};

/// Main plugin for BECOMPOSE
pub struct BecomposePlugin;

impl Plugin for BecomposePlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<CompositionTree>()
            .init_resource::<DirtyFlags>()
            .init_resource::<UiRoot>()
            // Systems
            .add_systems(Update, (
                sync_composition_to_entities,
                handle_button_interactions,
            ).chain());
    }
}

/// Resource holding the root UI entity
#[derive(Resource, Default)]
pub struct UiRoot {
    pub entity: Option<Entity>,
}
