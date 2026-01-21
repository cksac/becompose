//! Recomposition Logic
//!
//! Handles the recomposition process when state changes.

use bevy::prelude::*;
use crate::composition::{CompositionId, CompositionTree};

/// Dirty flags for tracking what needs updating
#[derive(Default, Resource)]
pub struct DirtyFlags {
    pub needs_recomposition: std::collections::HashSet<CompositionId>,
    pub needs_layout: std::collections::HashSet<CompositionId>,
    pub needs_paint: std::collections::HashSet<CompositionId>,
}

impl DirtyFlags {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mark_recomposition(&mut self, id: CompositionId) {
        self.needs_recomposition.insert(id);
        // Recomposition implies layout and paint
        self.needs_layout.insert(id);
        self.needs_paint.insert(id);
    }

    pub fn mark_layout(&mut self, id: CompositionId) {
        self.needs_layout.insert(id);
        self.needs_paint.insert(id);
    }

    pub fn mark_paint(&mut self, id: CompositionId) {
        self.needs_paint.insert(id);
    }

    pub fn clear(&mut self) {
        self.needs_recomposition.clear();
        self.needs_layout.clear();
        self.needs_paint.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.needs_recomposition.is_empty()
            && self.needs_layout.is_empty()
            && self.needs_paint.is_empty()
    }
}

/// Process pending recompositions
pub fn process_recompositions(tree: &mut CompositionTree, dirty: &mut DirtyFlags) {
    // Sort dirty nodes by depth (process parents before children)
    let mut sorted_ids: Vec<CompositionId> = dirty.needs_recomposition.iter().copied().collect();
    sorted_ids.sort_by(|a, b| {
        let depth_a = get_node_depth(tree, *a);
        let depth_b = get_node_depth(tree, *b);
        depth_a.cmp(&depth_b)
    });

    for id in sorted_ids {
        if let Some(node) = tree.get_mut(id) {
            node.mark_clean();
        }
    }

    dirty.needs_recomposition.clear();
}

/// Get the depth of a node in the tree
fn get_node_depth(tree: &CompositionTree, id: CompositionId) -> usize {
    let mut depth = 0;
    let mut current = id;
    
    while let Some(node) = tree.get(current) {
        if let Some(parent) = node.parent {
            depth += 1;
            current = parent;
        } else {
            break;
        }
    }
    
    depth
}
