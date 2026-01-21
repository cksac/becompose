//! Layout Components
//!
//! Column, Row, and Box composables.

use bevy::prelude::*;
use crate::layout::{ColumnLayout, RowLayout, BoxLayout};
use crate::modifier::ModifierChain;

/// Configuration for a Column layout
#[derive(Debug, Clone, Default)]
pub struct ColumnConfig {
    pub layout: ColumnLayout,
    pub modifier: ModifierChain,
}

impl ColumnConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_modifier(mut self, modifier: ModifierChain) -> Self {
        self.modifier = modifier;
        self
    }
}

/// Marker component for Column nodes
#[derive(Component, Default)]
pub struct ColumnNode {
    pub layout: ColumnLayout,
}

/// Configuration for a Row layout
#[derive(Debug, Clone, Default)]
pub struct RowConfig {
    pub layout: RowLayout,
    pub modifier: ModifierChain,
}

impl RowConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_modifier(mut self, modifier: ModifierChain) -> Self {
        self.modifier = modifier;
        self
    }
}

/// Marker component for Row nodes
#[derive(Component, Default)]
pub struct RowNode {
    pub layout: RowLayout,
}

/// Configuration for a Box layout
#[derive(Debug, Clone, Default)]
pub struct BoxConfig {
    pub layout: BoxLayout,
    pub modifier: ModifierChain,
}

impl BoxConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_modifier(mut self, modifier: ModifierChain) -> Self {
        self.modifier = modifier;
        self
    }
}

/// Marker component for Box nodes
#[derive(Component, Default)]
pub struct BoxNode {
    pub layout: BoxLayout,
}
