//! Tree Reconciliation
//!
//! Handles diffing and updating the composition tree.

use crate::composition::{CompositionId, CompositionKey, CompositionNode, CompositionTree};

/// Reconciles old children with new children, handling keys for efficient updates
pub fn reconcile_children(
    tree: &mut CompositionTree,
    parent_id: CompositionId,
    old_children: &[CompositionId],
    new_children: Vec<CompositionNode>,
) -> Vec<CompositionId> {
    let mut result = Vec::new();

    // Build a map of keyed old children for quick lookup
    let mut keyed_old: std::collections::HashMap<CompositionKey, CompositionId> =
        std::collections::HashMap::new();
    let mut unkeyed_old: Vec<CompositionId> = Vec::new();

    for &old_id in old_children {
        if let Some(node) = tree.get(old_id) {
            if let Some(key) = &node.key {
                keyed_old.insert(key.clone(), old_id);
            } else {
                unkeyed_old.push(old_id);
            }
        }
    }

    let mut unkeyed_index = 0;

    for new_node in new_children {
        let matched_id = if let Some(key) = &new_node.key {
            // Try to match by key
            keyed_old.remove(key)
        } else {
            // Try to match by position
            if unkeyed_index < unkeyed_old.len() {
                let id = unkeyed_old[unkeyed_index];
                unkeyed_index += 1;
                Some(id)
            } else {
                None
            }
        };

        if let Some(existing_id) = matched_id {
            // Update existing node
            if let Some(node) = tree.get_mut(existing_id) {
                node.modifiers = new_node.modifiers;
                node.mark_dirty();
            }
            result.push(existing_id);
        } else {
            // Insert new node
            let id = tree.insert(new_node);
            tree.add_child(parent_id, id);
            result.push(id);
        }
    }

    // Remove unmatched old children
    for (_, old_id) in keyed_old {
        remove_subtree(tree, old_id);
    }
    for old_id in unkeyed_old.iter().skip(unkeyed_index) {
        remove_subtree(tree, *old_id);
    }

    result
}

/// Recursively remove a subtree from the composition tree
pub fn remove_subtree(tree: &mut CompositionTree, id: CompositionId) {
    // First, collect all descendant IDs
    let children: Vec<CompositionId> = tree.get(id).map(|n| n.children.clone()).unwrap_or_default();

    // Recursively remove children
    for child_id in children {
        remove_subtree(tree, child_id);
    }

    // Remove the node itself
    tree.remove(id);
}
