//! Text Component
//!
//! Text display composable.

use crate::modifier::Modifiers;
use bevy::prelude::*;

/// Text style configuration
#[derive(Debug, Clone)]
pub struct TextStyle {
    pub font_size: f32,
    pub color: Color,
    pub font: Option<Handle<Font>>,
}

impl TextStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn title() -> Self {
        Self {
            font_size: 32.0,
            ..Default::default()
        }
    }

    pub fn headline() -> Self {
        Self {
            font_size: 24.0,
            ..Default::default()
        }
    }

    pub fn body() -> Self {
        Self {
            font_size: 16.0,
            ..Default::default()
        }
    }

    pub fn caption() -> Self {
        Self {
            font_size: 12.0,
            ..Default::default()
        }
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            font_size: 16.0,
            color: Color::WHITE,
            font: None,
        }
    }
}

/// Configuration for a Text node
#[derive(Debug, Clone)]
pub struct TextConfig {
    pub text: String,
    pub style: TextStyle,
    pub modifier: Modifiers,
}

impl TextConfig {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: TextStyle::default(),
            modifier: Modifiers::default(),
        }
    }

    pub fn with_style(mut self, style: TextStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_modifier(mut self, modifier: Modifiers) -> Self {
        self.modifier = modifier;
        self
    }
}

/// Marker component for Text nodes
#[derive(Component)]
pub struct TextNode {
    pub config: TextConfig,
}

impl TextNode {
    pub fn new(config: TextConfig) -> Self {
        Self { config }
    }
}
