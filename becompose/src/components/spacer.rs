//! Spacer Component
//!
//! Flexible space composable.

use crate::modifier::Modifiers;
use bevy::prelude::*;

/// Configuration for a Spacer
#[derive(Debug, Clone, Default)]
pub struct SpacerConfig {
    pub modifier: Modifiers,
}

impl SpacerConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_modifier(mut self, modifier: Modifiers) -> Self {
        self.modifier = modifier;
        self
    }
}

/// Marker component for Spacer nodes
#[derive(Component, Default)]
pub struct SpacerNode;
