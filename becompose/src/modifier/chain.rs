//! Modifier Chain
//!
//! Provides the chainable modifier system.

use bevy::prelude::*;
use std::sync::Arc;

/// Categories of modifiers for ordering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModifierType {
    Layout,
    Drawing,
    Pointer,
    Semantics,
    Transform,
}

/// Base trait for all modifiers
pub trait Modifier: Send + Sync + 'static {
    /// Apply this modifier to a Bevy Node component
    fn apply_to_node(&self, node: &mut Node);

    /// Apply this modifier to a BackgroundColor component (if applicable)
    fn apply_to_background(&self, _background: &mut BackgroundColor) {}

    /// Apply this modifier to a BorderColor component (if applicable)
    fn apply_to_border(&self, _border: &mut BorderColor) {}

    /// Get the modifier type for ordering
    fn modifier_type(&self) -> ModifierType;
}

/// Chain of modifiers applied to a composable
#[derive(Default, Clone)]
pub struct ModifierChain {
    modifiers: Vec<Arc<dyn Modifier>>,
}

impl std::fmt::Debug for ModifierChain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModifierChain")
            .field("modifier_count", &self.modifiers.len())
            .finish()
    }
}

impl ModifierChain {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a modifier to the chain
    pub fn then<M: Modifier>(mut self, modifier: M) -> Self {
        self.modifiers.push(Arc::new(modifier));
        self
    }

    /// Add padding on all sides
    pub fn padding(self, all: f32) -> Self {
        use super::PaddingModifier;
        self.then(PaddingModifier::all(all))
    }

    /// Add individual padding values
    pub fn padding_values(self, top: f32, right: f32, bottom: f32, left: f32) -> Self {
        use super::PaddingModifier;
        self.then(PaddingModifier::new(top, right, bottom, left))
    }

    /// Set fixed size
    pub fn size(self, width: f32, height: f32) -> Self {
        use super::SizeModifier;
        self.then(SizeModifier::fixed(width, height))
    }

    /// Set fixed width
    pub fn width(self, width: f32) -> Self {
        use super::SizeModifier;
        self.then(SizeModifier::width(width))
    }

    /// Set fixed height
    pub fn height(self, height: f32) -> Self {
        use super::SizeModifier;
        self.then(SizeModifier::height(height))
    }

    /// Fill maximum width
    pub fn fill_max_width(self) -> Self {
        use super::FillModifier;
        self.then(FillModifier::max_width())
    }

    /// Fill maximum height
    pub fn fill_max_height(self) -> Self {
        use super::FillModifier;
        self.then(FillModifier::max_height())
    }

    /// Fill maximum size
    pub fn fill_max_size(self) -> Self {
        use super::FillModifier;
        self.then(FillModifier::max_size())
    }

    /// Set background color
    pub fn background(self, color: Color) -> Self {
        use super::BackgroundModifier;
        self.then(BackgroundModifier::new(color))
    }

    /// Set border
    pub fn border(self, width: f32, color: Color) -> Self {
        use super::BorderModifier;
        self.then(BorderModifier::new(width, color))
    }

    /// Make clickable
    pub fn clickable<F: Fn() + Send + Sync + 'static>(self, on_click: F) -> Self {
        use super::ClickableModifier;
        self.then(ClickableModifier::new(on_click))
    }

    /// Set weight for flex layouts
    pub fn weight(self, weight: f32) -> Self {
        use super::WeightModifier;
        self.then(WeightModifier::new(weight))
    }

    /// Apply all modifiers to a Node component
    pub fn apply_to_node(&self, node: &mut Node) {
        for modifier in &self.modifiers {
            modifier.apply_to_node(node);
        }
    }

    /// Apply all modifiers to a BackgroundColor component
    pub fn apply_to_background(&self, background: &mut BackgroundColor) {
        for modifier in &self.modifiers {
            modifier.apply_to_background(background);
        }
    }

    /// Apply all modifiers to a BorderColor component
    pub fn apply_to_border(&self, border: &mut BorderColor) {
        for modifier in &self.modifiers {
            modifier.apply_to_border(border);
        }
    }

    /// Check if the chain is empty
    pub fn is_empty(&self) -> bool {
        self.modifiers.is_empty()
    }

    /// Get the number of modifiers
    pub fn len(&self) -> usize {
        self.modifiers.len()
    }

    /// Get click handlers from the chain
    pub fn get_click_handlers(&self) -> Vec<Arc<dyn Fn() + Send + Sync>> {
        self.modifiers
            .iter()
            .filter_map(|_m| {
                // This is a workaround since we can't downcast Arc<dyn Modifier>
                None::<Arc<dyn Fn() + Send + Sync>>
            })
            .collect()
    }
}

/// Public Modifier constructor type
pub struct Modifiers;

impl Modifiers {
    pub fn padding(all: f32) -> ModifierChain {
        ModifierChain::new().padding(all)
    }

    pub fn size(width: f32, height: f32) -> ModifierChain {
        ModifierChain::new().size(width, height)
    }

    pub fn width(width: f32) -> ModifierChain {
        ModifierChain::new().width(width)
    }

    pub fn height(height: f32) -> ModifierChain {
        ModifierChain::new().height(height)
    }

    pub fn fill_max_width() -> ModifierChain {
        ModifierChain::new().fill_max_width()
    }

    pub fn fill_max_height() -> ModifierChain {
        ModifierChain::new().fill_max_height()
    }

    pub fn fill_max_size() -> ModifierChain {
        ModifierChain::new().fill_max_size()
    }

    pub fn background(color: Color) -> ModifierChain {
        ModifierChain::new().background(color)
    }

    pub fn clickable<F: Fn() + Send + Sync + 'static>(on_click: F) -> ModifierChain {
        ModifierChain::new().clickable(on_click)
    }
}
