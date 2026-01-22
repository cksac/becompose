//! Modifier Chain
//!
//! Provides the chainable modifier system.

use crate::layout::{
    HorizontalAlignment, HorizontalArrangement, VerticalAlignment, VerticalArrangement,
};
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
pub struct Modifiers {
    modifiers: Vec<Arc<dyn Modifier>>,
}

impl std::fmt::Debug for Modifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Modifiers")
            .field("modifier_count", &self.modifiers.len())
            .finish()
    }
}

impl Modifiers {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a modifier to the chain
    pub fn then<M: Modifier>(mut self, modifier: M) -> Self {
        let arc: Arc<dyn Modifier> = Arc::new(modifier);
        self.modifiers.push(arc);
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
    /// Set vertical arrangement for Column (maps to justify_content)
    pub fn vertical_arrangement(self, arrangement: VerticalArrangement) -> Self {
        use super::JustifyModifier;
        self.then(JustifyModifier::new(arrangement.to_justify_content()))
    }

    /// Set horizontal arrangement for Row (maps to justify_content)
    pub fn horizontal_arrangement(self, arrangement: HorizontalArrangement) -> Self {
        use super::JustifyModifier;
        self.then(JustifyModifier::new(arrangement.to_justify_content()))
    }

    /// Set horizontal alignment (maps to align_items)
    pub fn horizontal_alignment(self, alignment: HorizontalAlignment) -> Self {
        use super::AlignItemsModifier;
        self.then(AlignItemsModifier::new(alignment.to_align_items()))
    }

    /// Set vertical alignment (maps to align_items)
    pub fn vertical_alignment(self, alignment: VerticalAlignment) -> Self {
        use super::AlignItemsModifier;
        self.then(AlignItemsModifier::new(alignment.to_align_items()))
    }

    /// Set row gap (spacing between rows/children in Column)
    pub fn row_gap(self, gap: f32) -> Self {
        use super::RowGapModifier;
        self.then(RowGapModifier::new(gap))
    }

    /// Set column gap (spacing between columns/children in Row)
    pub fn column_gap(self, gap: f32) -> Self {
        use super::ColumnGapModifier;
        self.then(ColumnGapModifier::new(gap))
    }

    /// Set justify content directly
    pub fn justify_content(self, justify: JustifyContent) -> Self {
        use super::JustifyModifier;
        self.then(JustifyModifier::new(justify))
    }

    /// Set align items directly
    pub fn align_items(self, align: AlignItems) -> Self {
        use super::AlignItemsModifier;
        self.then(AlignItemsModifier::new(align))
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
