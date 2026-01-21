//! Arrangement and Alignment
//!
//! Types for controlling how children are arranged in layouts.

use bevy::prelude::*;

/// Horizontal arrangement for Row layouts
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum HorizontalArrangement {
    #[default]
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl HorizontalArrangement {
    pub fn to_justify_content(&self) -> JustifyContent {
        match self {
            Self::Start => JustifyContent::FlexStart,
            Self::End => JustifyContent::FlexEnd,
            Self::Center => JustifyContent::Center,
            Self::SpaceBetween => JustifyContent::SpaceBetween,
            Self::SpaceAround => JustifyContent::SpaceAround,
            Self::SpaceEvenly => JustifyContent::SpaceEvenly,
        }
    }
}

/// Vertical arrangement for Column layouts
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum VerticalArrangement {
    #[default]
    Top,
    Bottom,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl VerticalArrangement {
    pub fn to_justify_content(&self) -> JustifyContent {
        match self {
            Self::Top => JustifyContent::FlexStart,
            Self::Bottom => JustifyContent::FlexEnd,
            Self::Center => JustifyContent::Center,
            Self::SpaceBetween => JustifyContent::SpaceBetween,
            Self::SpaceAround => JustifyContent::SpaceAround,
            Self::SpaceEvenly => JustifyContent::SpaceEvenly,
        }
    }
}

/// Generic arrangement with optional spacing
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Arrangement {
    pub justify: JustifyContent,
    pub spacing: f32,
}

impl Arrangement {
    pub fn start() -> Self {
        Self {
            justify: JustifyContent::FlexStart,
            spacing: 0.0,
        }
    }

    pub fn end() -> Self {
        Self {
            justify: JustifyContent::FlexEnd,
            spacing: 0.0,
        }
    }

    pub fn center() -> Self {
        Self {
            justify: JustifyContent::Center,
            spacing: 0.0,
        }
    }

    pub fn space_between() -> Self {
        Self {
            justify: JustifyContent::SpaceBetween,
            spacing: 0.0,
        }
    }

    pub fn space_around() -> Self {
        Self {
            justify: JustifyContent::SpaceAround,
            spacing: 0.0,
        }
    }

    pub fn space_evenly() -> Self {
        Self {
            justify: JustifyContent::SpaceEvenly,
            spacing: 0.0,
        }
    }

    pub fn spaced_by(spacing: f32) -> Self {
        Self {
            justify: JustifyContent::FlexStart,
            spacing,
        }
    }
}

impl Default for Arrangement {
    fn default() -> Self {
        Self::start()
    }
}

/// Horizontal alignment for items
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum HorizontalAlignment {
    #[default]
    Start,
    End,
    Center,
}

impl HorizontalAlignment {
    pub fn to_align_items(&self) -> AlignItems {
        match self {
            Self::Start => AlignItems::FlexStart,
            Self::End => AlignItems::FlexEnd,
            Self::Center => AlignItems::Center,
        }
    }
}

/// Vertical alignment for items
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum VerticalAlignment {
    #[default]
    Top,
    Bottom,
    Center,
}

impl VerticalAlignment {
    pub fn to_align_items(&self) -> AlignItems {
        match self {
            Self::Top => AlignItems::FlexStart,
            Self::Bottom => AlignItems::FlexEnd,
            Self::Center => AlignItems::Center,
        }
    }
}

/// 2D alignment for Box layouts
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Alignment2D {
    pub horizontal: HorizontalAlignment,
    pub vertical: VerticalAlignment,
}

impl Alignment2D {
    pub fn top_start() -> Self {
        Self {
            horizontal: HorizontalAlignment::Start,
            vertical: VerticalAlignment::Top,
        }
    }

    pub fn top_center() -> Self {
        Self {
            horizontal: HorizontalAlignment::Center,
            vertical: VerticalAlignment::Top,
        }
    }

    pub fn top_end() -> Self {
        Self {
            horizontal: HorizontalAlignment::End,
            vertical: VerticalAlignment::Top,
        }
    }

    pub fn center_start() -> Self {
        Self {
            horizontal: HorizontalAlignment::Start,
            vertical: VerticalAlignment::Center,
        }
    }

    pub fn center() -> Self {
        Self {
            horizontal: HorizontalAlignment::Center,
            vertical: VerticalAlignment::Center,
        }
    }

    pub fn center_end() -> Self {
        Self {
            horizontal: HorizontalAlignment::End,
            vertical: VerticalAlignment::Center,
        }
    }

    pub fn bottom_start() -> Self {
        Self {
            horizontal: HorizontalAlignment::Start,
            vertical: VerticalAlignment::Bottom,
        }
    }

    pub fn bottom_center() -> Self {
        Self {
            horizontal: HorizontalAlignment::Center,
            vertical: VerticalAlignment::Bottom,
        }
    }

    pub fn bottom_end() -> Self {
        Self {
            horizontal: HorizontalAlignment::End,
            vertical: VerticalAlignment::Bottom,
        }
    }
}
