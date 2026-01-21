//! Layout Modifiers
//!
//! Modifiers that affect layout: padding, size, fill, weight.

use bevy::prelude::*;
use super::{Modifier, ModifierType};

/// Padding modifier
#[derive(Debug, Clone)]
pub struct PaddingModifier {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl PaddingModifier {
    pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self { top, right, bottom, left }
    }

    pub fn all(value: f32) -> Self {
        Self::new(value, value, value, value)
    }

    pub fn horizontal(value: f32) -> Self {
        Self::new(0.0, value, 0.0, value)
    }

    pub fn vertical(value: f32) -> Self {
        Self::new(value, 0.0, value, 0.0)
    }
}

impl Modifier for PaddingModifier {
    fn apply_to_node(&self, node: &mut Node) {
        node.padding = UiRect {
            top: Val::Px(self.top),
            right: Val::Px(self.right),
            bottom: Val::Px(self.bottom),
            left: Val::Px(self.left),
        };
    }

    fn modifier_type(&self) -> ModifierType {
        ModifierType::Layout
    }
}

/// Size modifier
#[derive(Debug, Clone)]
pub struct SizeModifier {
    pub width: Option<f32>,
    pub height: Option<f32>,
}

impl SizeModifier {
    pub fn new(width: Option<f32>, height: Option<f32>) -> Self {
        Self { width, height }
    }

    pub fn fixed(width: f32, height: f32) -> Self {
        Self::new(Some(width), Some(height))
    }

    pub fn width(width: f32) -> Self {
        Self::new(Some(width), None)
    }

    pub fn height(height: f32) -> Self {
        Self::new(None, Some(height))
    }
}

impl Modifier for SizeModifier {
    fn apply_to_node(&self, node: &mut Node) {
        if let Some(w) = self.width {
            node.width = Val::Px(w);
        }
        if let Some(h) = self.height {
            node.height = Val::Px(h);
        }
    }

    fn modifier_type(&self) -> ModifierType {
        ModifierType::Layout
    }
}

/// Fill modifier for max width/height
#[derive(Debug, Clone)]
pub struct FillModifier {
    pub fill_width: bool,
    pub fill_height: bool,
}

impl FillModifier {
    pub fn max_width() -> Self {
        Self { fill_width: true, fill_height: false }
    }

    pub fn max_height() -> Self {
        Self { fill_width: false, fill_height: true }
    }

    pub fn max_size() -> Self {
        Self { fill_width: true, fill_height: true }
    }
}

impl Modifier for FillModifier {
    fn apply_to_node(&self, node: &mut Node) {
        if self.fill_width {
            node.width = Val::Percent(100.0);
        }
        if self.fill_height {
            node.height = Val::Percent(100.0);
        }
    }

    fn modifier_type(&self) -> ModifierType {
        ModifierType::Layout
    }
}

/// Weight modifier for flex layouts
#[derive(Debug, Clone)]
pub struct WeightModifier {
    pub weight: f32,
}

impl WeightModifier {
    pub fn new(weight: f32) -> Self {
        Self { weight }
    }
}

impl Modifier for WeightModifier {
    fn apply_to_node(&self, node: &mut Node) {
        node.flex_grow = self.weight;
    }

    fn modifier_type(&self) -> ModifierType {
        ModifierType::Layout
    }
}

/// Margin modifier
#[derive(Debug, Clone)]
pub struct MarginModifier {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl MarginModifier {
    pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self { top, right, bottom, left }
    }

    pub fn all(value: f32) -> Self {
        Self::new(value, value, value, value)
    }
}

impl Modifier for MarginModifier {
    fn apply_to_node(&self, node: &mut Node) {
        node.margin = UiRect {
            top: Val::Px(self.top),
            right: Val::Px(self.right),
            bottom: Val::Px(self.bottom),
            left: Val::Px(self.left),
        };
    }

    fn modifier_type(&self) -> ModifierType {
        ModifierType::Layout
    }
}

/// Justify content modifier (sets `justify_content` on Node)
#[derive(Debug, Clone)]
pub struct JustifyModifier {
    pub justify: JustifyContent,
}

impl JustifyModifier {
    pub fn new(justify: JustifyContent) -> Self {
        Self { justify }
    }
}

impl Modifier for JustifyModifier {
    fn apply_to_node(&self, node: &mut Node) {
        node.justify_content = self.justify;
    }

    fn modifier_type(&self) -> ModifierType {
        ModifierType::Layout
    }
}

/// Align items modifier (sets `align_items` on Node)
#[derive(Debug, Clone)]
pub struct AlignItemsModifier {
    pub align: AlignItems,
}

impl AlignItemsModifier {
    pub fn new(align: AlignItems) -> Self {
        Self { align }
    }
}

impl Modifier for AlignItemsModifier {
    fn apply_to_node(&self, node: &mut Node) {
        node.align_items = self.align;
    }

    fn modifier_type(&self) -> ModifierType {
        ModifierType::Layout
    }
}

/// Row gap modifier (sets `row_gap` on Node)
#[derive(Debug, Clone)]
pub struct RowGapModifier {
    pub gap: f32,
}

impl RowGapModifier {
    pub fn new(gap: f32) -> Self {
        Self { gap }
    }
}

impl Modifier for RowGapModifier {
    fn apply_to_node(&self, node: &mut Node) {
        node.row_gap = Val::Px(self.gap);
    }

    fn modifier_type(&self) -> ModifierType {
        ModifierType::Layout
    }
}

/// Column gap modifier (sets `column_gap` on Node)
#[derive(Debug, Clone)]
pub struct ColumnGapModifier {
    pub gap: f32,
}

impl ColumnGapModifier {
    pub fn new(gap: f32) -> Self {
        Self { gap }
    }
}

impl Modifier for ColumnGapModifier {
    fn apply_to_node(&self, node: &mut Node) {
        node.column_gap = Val::Px(self.gap);
    }

    fn modifier_type(&self) -> ModifierType {
        ModifierType::Layout
    }
}
