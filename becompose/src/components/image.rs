//! Image Component
//!
//! Image display composable.

use crate::modifier::Modifiers;
use bevy::prelude::*;

/// Configuration for an Image node
#[derive(Debug, Clone)]
pub struct ImageConfig {
    pub image: Handle<Image>,
    pub modifier: Modifiers,
}

impl ImageConfig {
    pub fn new(image: Handle<Image>) -> Self {
        Self {
            image,
            modifier: Modifiers::default(),
        }
    }

    pub fn with_modifier(mut self, modifier: Modifiers) -> Self {
        self.modifier = modifier;
        self
    }
}

/// Marker component for Image nodes
#[derive(Component)]
pub struct ImageNode {
    pub config: ImageConfig,
}

impl ImageNode {
    pub fn new(config: ImageConfig) -> Self {
        Self { config }
    }
}
