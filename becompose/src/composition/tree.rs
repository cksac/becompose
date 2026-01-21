//! Composition Tree
//!
//! The composition tree represents the hierarchical structure of UI elements.

use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::modifier::ModifierChain;
use crate::state::StateSlot;

/// Unique identifier for composition nodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompositionId(pub u64);

impl CompositionId {
    pub fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}

impl Default for CompositionId {
    fn default() -> Self {
        Self::new()
    }
}

/// Key for list reconciliation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompositionKey(pub String);

impl<T: ToString> From<T> for CompositionKey {
    fn from(value: T) -> Self {
        Self(value.to_string())
    }
}

/// Types of composable nodes
#[derive(Debug, Clone, PartialEq)]
pub enum ComposableType {
    /// A layout container (Row, Column, Box)
    Layout(LayoutType),
    /// A leaf node (Text, Image)
    Leaf(LeafType),
    /// A custom composable function
    Custom(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutType {
    Row,
    Column,
    Box,
    LazyColumn,
    LazyRow,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LeafType {
    Text,
    Image,
    Spacer,
}

/// Represents a node in the composition tree
#[derive(Debug)]
pub struct CompositionNode {
    /// Unique identifier for this node
    pub id: CompositionId,
    /// The composable type that created this node
    pub composable_type: ComposableType,
    /// Key for list reconciliation
    pub key: Option<CompositionKey>,
    /// Parent node reference
    pub parent: Option<CompositionId>,
    /// Child nodes
    pub children: Vec<CompositionId>,
    /// Associated Bevy entity (if materialized)
    pub entity: Option<Entity>,
    /// State slots for this composition
    pub state_slots: Vec<StateSlot>,
    /// Applied modifiers
    pub modifiers: ModifierChain,
    /// Whether this node needs recomposition
    pub dirty: bool,
}

impl CompositionNode {
    pub fn new(composable_type: ComposableType) -> Self {
        Self {
            id: CompositionId::new(),
            composable_type,
            key: None,
            parent: None,
            children: Vec::new(),
            entity: None,
            state_slots: Vec::new(),
            modifiers: ModifierChain::default(),
            dirty: true,
        }
    }

    pub fn with_key(mut self, key: impl Into<CompositionKey>) -> Self {
        self.key = Some(key.into());
        self
    }

    pub fn with_modifiers(mut self, modifiers: ModifierChain) -> Self {
        self.modifiers = modifiers;
        self
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }
}

/// The composition tree manager
#[derive(Resource, Default)]
pub struct CompositionTree {
    nodes: HashMap<CompositionId, CompositionNode>,
    root: Option<CompositionId>,
    /// Nodes pending recomposition
    pub pending_recomposition: HashSet<CompositionId>,
    /// Newly created nodes that need entities
    pub new_nodes: Vec<CompositionId>,
    /// Nodes that were removed and need entity cleanup
    pub removed_nodes: Vec<CompositionId>,
}

impl CompositionTree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn root(&self) -> Option<CompositionId> {
        self.root
    }

    pub fn set_root(&mut self, id: CompositionId) {
        self.root = Some(id);
    }

    pub fn get(&self, id: CompositionId) -> Option<&CompositionNode> {
        self.nodes.get(&id)
    }

    pub fn get_mut(&mut self, id: CompositionId) -> Option<&mut CompositionNode> {
        self.nodes.get_mut(&id)
    }

    pub fn insert(&mut self, node: CompositionNode) -> CompositionId {
        let id = node.id;
        self.new_nodes.push(id);
        self.nodes.insert(id, node);
        id
    }

    pub fn remove(&mut self, id: CompositionId) -> Option<CompositionNode> {
        if let Some(node) = self.nodes.remove(&id) {
            self.removed_nodes.push(id);
            // Remove from parent's children list
            if let Some(parent_id) = node.parent {
                if let Some(parent) = self.nodes.get_mut(&parent_id) {
                    parent.children.retain(|&child_id| child_id != id);
                }
            }
            Some(node)
        } else {
            None
        }
    }

    pub fn add_child(&mut self, parent_id: CompositionId, child_id: CompositionId) {
        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            parent.children.push(child_id);
        }
        if let Some(child) = self.nodes.get_mut(&child_id) {
            child.parent = Some(parent_id);
        }
    }

    pub fn set_entity(&mut self, node_id: CompositionId, entity: Entity) {
        if let Some(node) = self.nodes.get_mut(&node_id) {
            node.entity = entity.into();
        }
    }

    pub fn get_entity(&self, node_id: CompositionId) -> Option<Entity> {
        self.nodes.get(&node_id).and_then(|n| n.entity)
    }

    pub fn mark_dirty(&mut self, id: CompositionId) {
        if let Some(node) = self.nodes.get_mut(&id) {
            node.mark_dirty();
            self.pending_recomposition.insert(id);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&CompositionId, &CompositionNode)> {
        self.nodes.iter()
    }

    pub fn clear(&mut self) {
        // Mark all entities for removal
        for (id, _) in self.nodes.iter() {
            self.removed_nodes.push(*id);
        }
        self.nodes.clear();
        self.root = None;
        self.pending_recomposition.clear();
        self.new_nodes.clear();
    }
}
