//! Entity Bridge
//!
//! Synchronizes composition tree with Bevy entities.

use bevy::prelude::*;

use crate::composition::{CompositionId, CompositionTree};

/// Component marking a Bevy entity as a BECOMPOSE node
#[derive(Component)]
pub struct CompositionBridge {
    pub composition_id: CompositionId,
}

/// Syncs composition tree changes to Bevy entities
pub fn sync_composition_to_entities(
    mut commands: Commands,
    mut tree: ResMut<CompositionTree>,
    query: Query<(Entity, &CompositionBridge)>,
) {
    // Collect new node IDs first
    let new_node_ids: Vec<_> = tree.new_nodes.drain(..).collect();
    
    // Handle new nodes - spawn entities
    for node_id in new_node_ids {
        commands.spawn((
            CompositionBridge { composition_id: node_id },
            Node::default(),
        ));
    }

    // Collect removed node IDs
    let removed_node_ids: Vec<_> = tree.removed_nodes.drain(..).collect();

    // Handle removed nodes - despawn entities
    for node_id in removed_node_ids {
        for (entity, bridge) in query.iter() {
            if bridge.composition_id == node_id {
                commands.entity(entity).despawn_recursive();
                break;
            }
        }
    }
}
