//! Draw Modifiers
//!
//! Modifiers that affect visual appearance: background, border.

use super::{Modifier, ModifierType};
use bevy::prelude::*;

/// Background modifier
#[derive(Debug, Clone)]
pub struct BackgroundModifier {
    pub color: Color,
}

impl BackgroundModifier {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Modifier for BackgroundModifier {
    fn apply_to_node(&self, _node: &mut Node) {
        // Background is applied via BackgroundColor component
    }

    fn apply_to_background(&self, background: &mut BackgroundColor) {
        *background = BackgroundColor(self.color);
    }

    fn modifier_type(&self) -> ModifierType {
        ModifierType::Drawing
    }
}

/// Border modifier
#[derive(Debug, Clone)]
pub struct BorderModifier {
    pub width: f32,
    pub color: Color,
}

impl BorderModifier {
    pub fn new(width: f32, color: Color) -> Self {
        Self { width, color }
    }
}

impl Modifier for BorderModifier {
    fn apply_to_node(&self, node: &mut Node) {
        node.border = UiRect::all(Val::Px(self.width));
    }

    fn apply_to_border(&self, border: &mut BorderColor) {
        *border = BorderColor(self.color);
    }

    fn modifier_type(&self) -> ModifierType {
        ModifierType::Drawing
    }
}

/// Corner radius modifier
#[derive(Debug, Clone)]
pub struct BorderRadiusModifier {
    pub radius: f32,
}

impl BorderRadiusModifier {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

impl Modifier for BorderRadiusModifier {
    fn apply_to_node(&self, _node: &mut Node) {
        // Border radius is handled separately in Bevy
    }

    fn modifier_type(&self) -> ModifierType {
        ModifierType::Drawing
    }
}

/// Alpha/transparency modifier
#[derive(Debug, Clone)]
pub struct AlphaModifier {
    pub alpha: f32,
}

impl AlphaModifier {
    pub fn new(alpha: f32) -> Self {
        Self {
            alpha: alpha.clamp(0.0, 1.0),
        }
    }
}

impl Modifier for AlphaModifier {
    fn apply_to_node(&self, _node: &mut Node) {
        // Alpha is applied to colors
    }

    fn apply_to_background(&self, background: &mut BackgroundColor) {
        let mut color = background.0;
        color.set_alpha(self.alpha);
        *background = BackgroundColor(color);
    }

    fn modifier_type(&self) -> ModifierType {
        ModifierType::Drawing
    }
}
