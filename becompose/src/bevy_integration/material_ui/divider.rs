//! Material Divider Composable
//!
//! Wraps bevy_material_ui Divider component as a BECOMPOSE composable.

use bevy::prelude::*;

use crate::bevy_integration::composables::with_implicit_scope;
use crate::bevy_integration::material_ui::spawn_material_child;

/// Material Design horizontal divider composable
///
/// # Example
/// ```ignore
/// Column(Modifiers::new(), || {
///     Text("Above divider", TextStyle::body());
///     MaterialDivider();
///     Text("Below divider", TextStyle::body());
/// });
/// ```
pub fn MaterialDivider() {
    with_implicit_scope(|| {
        spawn_material_child(move |commands, theme| {
            commands
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(1.0),
                        ..default()
                    },
                    BackgroundColor(theme.outline_variant),
                ))
                .id()
        });
    });
}

/// Material Design vertical divider composable
///
/// # Example
/// ```ignore
/// Row(Modifiers::new(), || {
///     Text("Left", TextStyle::body());
///     MaterialVerticalDivider();
///     Text("Right", TextStyle::body());
/// });
/// ```
pub fn MaterialVerticalDivider() {
    with_implicit_scope(|| {
        spawn_material_child(move |commands, theme| {
            commands
                .spawn((
                    Node {
                        width: Val::Px(1.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(theme.outline_variant),
                ))
                .id()
        });
    });
}

/// Material Design divider composable with inset
///
/// # Example
/// ```ignore
/// MaterialDividerWithInset(16.0); // 16px inset on both sides
/// ```
pub fn MaterialDividerWithInset(inset: f32) {
    with_implicit_scope(|| {
        spawn_material_child(move |commands, theme| {
            commands
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(1.0),
                        margin: UiRect::horizontal(Val::Px(inset)),
                        ..default()
                    },
                    BackgroundColor(theme.outline_variant),
                ))
                .id()
        });
    });
}

/// Material Design divider composable with configuration
pub fn MaterialDividerConfigured(config: MaterialDividerConfig) {
    with_implicit_scope(|| {
        spawn_material_child(move |commands, theme| {
            let (width, height) = if config.vertical {
                (Val::Px(config.thickness), Val::Percent(100.0))
            } else {
                (Val::Percent(100.0), Val::Px(config.thickness))
            };

            let margin = if config.vertical {
                UiRect::vertical(Val::Px(config.inset))
            } else {
                UiRect::horizontal(Val::Px(config.inset))
            };

            commands
                .spawn((
                    Node {
                        width,
                        height,
                        margin,
                        ..default()
                    },
                    BackgroundColor(config.color.unwrap_or(theme.outline_variant)),
                ))
                .id()
        });
    });
}

/// Configuration for a Material divider
#[derive(Clone)]
pub struct MaterialDividerConfig {
    pub vertical: bool,
    pub inset: f32,
    pub thickness: f32,
    pub color: Option<Color>,
}

impl MaterialDividerConfig {
    pub fn new() -> Self {
        Self {
            vertical: false,
            inset: 0.0,
            thickness: 1.0,
            color: None,
        }
    }

    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }

    pub fn horizontal(mut self) -> Self {
        self.vertical = false;
        self
    }

    pub fn inset(mut self, inset: f32) -> Self {
        self.inset = inset;
        self
    }

    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

impl Default for MaterialDividerConfig {
    fn default() -> Self {
        Self::new()
    }
}
