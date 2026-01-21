//! Spacer Component
//!
//! Flexible space composable.

use bevy::prelude::*;
use crate::modifier::ModifierChain;

/// Configuration for a Spacer
#[derive(Debug, Clone, Default)]
pub struct SpacerConfig {
    pub modifier: ModifierChain,
}

impl SpacerConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_modifier(mut self, modifier: ModifierChain) -> Self {
        self.modifier = modifier;
        self
    }
}

/// Marker component for Spacer nodes
#[derive(Component, Default)]
pub struct SpacerNode;
