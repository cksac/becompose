//! Input Modifiers
//!
//! Modifiers that handle user interaction: clickable, draggable.

use std::sync::Arc;
use super::{Modifier, ModifierType};
use bevy::prelude::*;

/// Click handler type
pub type ClickHandler = Arc<dyn Fn() + Send + Sync>;

/// Clickable modifier
#[derive(Clone)]
pub struct ClickableModifier {
    pub on_click: ClickHandler,
}

impl ClickableModifier {
    pub fn new<F: Fn() + Send + Sync + 'static>(on_click: F) -> Self {
        Self {
            on_click: Arc::new(on_click),
        }
    }
}

impl std::fmt::Debug for ClickableModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClickableModifier").finish()
    }
}

impl Modifier for ClickableModifier {
    fn apply_to_node(&self, _node: &mut Node) {
        // Clickable doesn't affect style directly
    }

    fn modifier_type(&self) -> ModifierType {
        ModifierType::Pointer
    }
}

/// Hover modifier
#[derive(Clone)]
pub struct HoverModifier {
    pub on_enter: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_exit: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl HoverModifier {
    pub fn new() -> Self {
        Self {
            on_enter: None,
            on_exit: None,
        }
    }

    pub fn on_enter<F: Fn() + Send + Sync + 'static>(mut self, handler: F) -> Self {
        self.on_enter = Some(Arc::new(handler));
        self
    }

    pub fn on_exit<F: Fn() + Send + Sync + 'static>(mut self, handler: F) -> Self {
        self.on_exit = Some(Arc::new(handler));
        self
    }
}

impl Default for HoverModifier {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for HoverModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HoverModifier").finish()
    }
}

impl Modifier for HoverModifier {
    fn apply_to_node(&self, _node: &mut Node) {}

    fn modifier_type(&self) -> ModifierType {
        ModifierType::Pointer
    }
}

/// Focus modifier for keyboard navigation
#[derive(Clone)]
pub struct FocusableModifier {
    pub on_focus: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_blur: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl FocusableModifier {
    pub fn new() -> Self {
        Self {
            on_focus: None,
            on_blur: None,
        }
    }
}

impl Default for FocusableModifier {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for FocusableModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FocusableModifier").finish()
    }
}

impl Modifier for FocusableModifier {
    fn apply_to_node(&self, _node: &mut Node) {}

    fn modifier_type(&self) -> ModifierType {
        ModifierType::Pointer
    }
}
