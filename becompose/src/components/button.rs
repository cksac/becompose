//! Button Component
//!
//! Clickable button composable.

use bevy::prelude::*;
use std::sync::Arc;
use crate::modifier::Modifiers;

/// Click handler type
pub type OnClick = Arc<dyn Fn() + Send + Sync>;

/// Configuration for a Button
#[derive(Clone)]
pub struct ButtonConfig {
    pub on_click: OnClick,
    pub modifier: Modifiers,
    pub enabled: bool,
}

impl ButtonConfig {
    pub fn new<F: Fn() + Send + Sync + 'static>(on_click: F) -> Self {
        Self {
            on_click: Arc::new(on_click),
            modifier: Modifiers::default(),
            enabled: true,
        }
    }

    pub fn with_modifier(mut self, modifier: Modifiers) -> Self {
        self.modifier = modifier;
        self
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl std::fmt::Debug for ButtonConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ButtonConfig")
            .field("enabled", &self.enabled)
            .finish()
    }
}

/// Marker component for Button nodes
#[derive(Component)]
pub struct ButtonNode {
    pub on_click: OnClick,
    pub enabled: bool,
}

impl ButtonNode {
    pub fn new(config: ButtonConfig) -> Self {
        Self {
            on_click: config.on_click,
            enabled: config.enabled,
        }
    }
}

/// Marker component for interactive elements
#[derive(Component)]
pub struct Clickable {
    pub on_click: OnClick,
}

impl Clickable {
    pub fn new<F: Fn() + Send + Sync + 'static>(on_click: F) -> Self {
        Self {
            on_click: Arc::new(on_click),
        }
    }
}
