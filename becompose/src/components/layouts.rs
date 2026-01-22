//! Layout Components
//!
//! Column, Row, and Box composables.

use crate::layout::{BoxLayout, ColumnLayout, RowLayout};
use crate::modifier::Modifiers;
use bevy::prelude::*;

/// Configuration for a Column layout
#[derive(Debug, Clone, Default)]
pub struct ColumnConfig {
    pub layout: ColumnLayout,
    pub modifier: Modifiers,
}

impl ColumnConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_modifier(mut self, modifier: Modifiers) -> Self {
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
    pub modifier: Modifiers,
}

impl RowConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_modifier(mut self, modifier: Modifiers) -> Self {
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
    pub modifier: Modifiers,
}

impl BoxConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_modifier(mut self, modifier: Modifiers) -> Self {
        self.modifier = modifier;
        self
    }
}

/// Marker component for Box nodes
#[derive(Component, Default)]
pub struct BoxNode {
    pub layout: BoxLayout,
}
