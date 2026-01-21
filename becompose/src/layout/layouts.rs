//! Layout Types
//!
//! Column, Row, and Box layout containers.

use bevy::prelude::*;

use super::{
    HorizontalAlignment, VerticalAlignment, HorizontalArrangement, 
    VerticalArrangement, Alignment2D,
};

/// Configuration for Column layout
#[derive(Debug, Clone)]
pub struct ColumnLayout {
    pub vertical_arrangement: VerticalArrangement,
    pub horizontal_alignment: HorizontalAlignment,
    pub spacing: f32,
}

impl ColumnLayout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_arrangement(mut self, arrangement: VerticalArrangement) -> Self {
        self.vertical_arrangement = arrangement;
        self
    }

    pub fn with_alignment(mut self, alignment: HorizontalAlignment) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn apply_to_node(&self, node: &mut Node) {
        node.display = Display::Flex;
        node.flex_direction = FlexDirection::Column;
        node.justify_content = self.vertical_arrangement.to_justify_content();
        node.align_items = self.horizontal_alignment.to_align_items();
        node.row_gap = Val::Px(self.spacing);
    }
}

impl Default for ColumnLayout {
    fn default() -> Self {
        Self {
            vertical_arrangement: VerticalArrangement::Top,
            horizontal_alignment: HorizontalAlignment::Start,
            spacing: 0.0,
        }
    }
}

/// Configuration for Row layout
#[derive(Debug, Clone)]
pub struct RowLayout {
    pub horizontal_arrangement: HorizontalArrangement,
    pub vertical_alignment: VerticalAlignment,
    pub spacing: f32,
}

impl RowLayout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_arrangement(mut self, arrangement: HorizontalArrangement) -> Self {
        self.horizontal_arrangement = arrangement;
        self
    }

    pub fn with_alignment(mut self, alignment: VerticalAlignment) -> Self {
        self.vertical_alignment = alignment;
        self
    }

    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn apply_to_node(&self, node: &mut Node) {
        node.display = Display::Flex;
        node.flex_direction = FlexDirection::Row;
        node.justify_content = self.horizontal_arrangement.to_justify_content();
        node.align_items = self.vertical_alignment.to_align_items();
        node.column_gap = Val::Px(self.spacing);
    }
}

impl Default for RowLayout {
    fn default() -> Self {
        Self {
            horizontal_arrangement: HorizontalArrangement::Start,
            vertical_alignment: VerticalAlignment::Top,
            spacing: 0.0,
        }
    }
}

/// Configuration for Box layout (stacking/overlay)
#[derive(Debug, Clone)]
pub struct BoxLayout {
    pub content_alignment: Alignment2D,
}

impl BoxLayout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_alignment(mut self, alignment: Alignment2D) -> Self {
        self.content_alignment = alignment;
        self
    }

    pub fn apply_to_node(&self, node: &mut Node) {
        node.display = Display::Flex;
        node.justify_content = match self.content_alignment.horizontal {
            HorizontalAlignment::Start => JustifyContent::FlexStart,
            HorizontalAlignment::Center => JustifyContent::Center,
            HorizontalAlignment::End => JustifyContent::FlexEnd,
        };
        node.align_items = self.content_alignment.vertical.to_align_items();
    }
}

impl Default for BoxLayout {
    fn default() -> Self {
        Self {
            content_alignment: Alignment2D::top_start(),
        }
    }
}
