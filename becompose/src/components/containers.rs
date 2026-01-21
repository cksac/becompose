//! Container Components
//!
//! Card and other container composables.

use bevy::prelude::*;
use crate::modifier::ModifierChain;

/// Configuration for a Card container
#[derive(Debug, Clone)]
pub struct CardConfig {
    pub elevation: f32,
    pub corner_radius: f32,
    pub background_color: Color,
    pub modifier: ModifierChain,
}

impl CardConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_elevation(mut self, elevation: f32) -> Self {
        self.elevation = elevation;
        self
    }

    pub fn with_corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }

    pub fn with_background(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    pub fn with_modifier(mut self, modifier: ModifierChain) -> Self {
        self.modifier = modifier;
        self
    }
}

impl Default for CardConfig {
    fn default() -> Self {
        Self {
            elevation: 2.0,
            corner_radius: 8.0,
            background_color: Color::srgb(0.15, 0.15, 0.15),
            modifier: ModifierChain::default(),
        }
    }
}

/// Marker component for Card nodes
#[derive(Component)]
pub struct CardNode {
    pub elevation: f32,
    pub corner_radius: f32,
}

impl Default for CardNode {
    fn default() -> Self {
        Self {
            elevation: 2.0,
            corner_radius: 8.0,
        }
    }
}

/// Configuration for a Surface container
#[derive(Debug, Clone)]
pub struct SurfaceConfig {
    pub color: Color,
    pub modifier: ModifierChain,
}

impl SurfaceConfig {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            modifier: ModifierChain::default(),
        }
    }

    pub fn with_modifier(mut self, modifier: ModifierChain) -> Self {
        self.modifier = modifier;
        self
    }
}

/// Marker component for Surface nodes
#[derive(Component)]
pub struct SurfaceNode {
    pub color: Color,
}
