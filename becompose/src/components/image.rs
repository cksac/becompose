//! Image Component
//!
//! Image display composable.

use bevy::prelude::*;
use crate::modifier::ModifierChain;

/// Configuration for an Image node
#[derive(Debug, Clone)]
pub struct ImageConfig {
    pub image: Handle<Image>,
    pub modifier: ModifierChain,
}

impl ImageConfig {
    pub fn new(image: Handle<Image>) -> Self {
        Self {
            image,
            modifier: ModifierChain::default(),
        }
    }

    pub fn with_modifier(mut self, modifier: ModifierChain) -> Self {
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
