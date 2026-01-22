//! Material Tooltip Composable
//!
//! Wraps bevy_material_ui Tooltip component as a BECOMPOSE composable.

use bevy::prelude::*;

use crate::bevy_integration::composables::with_implicit_scope;
use crate::bevy_integration::material_ui::spawn_material_child_with_children;

/// Material Design tooltip composable wrapping content
///
/// # Example
/// ```ignore
/// MaterialTooltipComposable("Click to submit", || {
///     Button(|| {
///         Text("Submit");
///     });
/// });
/// ```
pub fn MaterialTooltipComposable<C>(text: impl AsRef<str>, content: C)
where
    C: FnOnce() + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let text = text.as_ref().to_string();

        spawn_material_child_with_children(
            move |commands, theme| {
                commands
                    .spawn((
                        MaterialTooltipWrapper { text: text.clone() },
                        Node {
                            display: Display::Flex,
                            ..default()
                        },
                    ))
                    .insert(MaterialTooltipConfig {
                        text,
                        delay_ms: 500,
                        position: TooltipPosition::Bottom,
                        container_color: theme.inverse_surface,
                        text_color: theme.inverse_on_surface,
                    })
                    .id()
            },
            content,
        );
    });
}

/// Material Design tooltip with position configuration
pub fn MaterialTooltipPositioned<C>(text: impl AsRef<str>, position: TooltipPosition, content: C)
where
    C: FnOnce() + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let text = text.as_ref().to_string();

        spawn_material_child_with_children(
            move |commands, theme| {
                commands
                    .spawn((
                        MaterialTooltipWrapper { text: text.clone() },
                        Node {
                            display: Display::Flex,
                            ..default()
                        },
                    ))
                    .insert(MaterialTooltipConfig {
                        text,
                        delay_ms: 500,
                        position,
                        container_color: theme.inverse_surface,
                        text_color: theme.inverse_on_surface,
                    })
                    .id()
            },
            content,
        );
    });
}

/// Material Design rich tooltip with title
pub fn MaterialRichTooltip<C>(title: impl AsRef<str>, text: impl AsRef<str>, content: C)
where
    C: FnOnce() + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let title = title.as_ref().to_string();
        let text = text.as_ref().to_string();

        spawn_material_child_with_children(
            move |commands, theme| {
                commands
                    .spawn((
                        MaterialRichTooltipWrapper {
                            title: title.clone(),
                            text: text.clone(),
                        },
                        Node {
                            display: Display::Flex,
                            ..default()
                        },
                    ))
                    .insert(MaterialRichTooltipConfig {
                        title,
                        text,
                        delay_ms: 500,
                        position: TooltipPosition::Bottom,
                        container_color: theme.surface_container,
                        title_color: theme.on_surface,
                        text_color: theme.on_surface_variant,
                    })
                    .id()
            },
            content,
        );
    });
}

/// Material Design tooltip with full configuration
pub fn MaterialTooltipConfigured<C>(config: MaterialTooltipComposableConfig, content: C)
where
    C: FnOnce() + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        spawn_material_child_with_children(
            move |commands, theme| {
                commands
                    .spawn((
                        MaterialTooltipWrapper {
                            text: config.text.clone(),
                        },
                        Node {
                            display: Display::Flex,
                            ..default()
                        },
                    ))
                    .insert(MaterialTooltipConfig {
                        text: config.text,
                        delay_ms: config.delay_ms,
                        position: config.position,
                        container_color: config.container_color.unwrap_or(theme.inverse_surface),
                        text_color: config.text_color.unwrap_or(theme.inverse_on_surface),
                    })
                    .id()
            },
            content,
        );
    });
}

/// Position for tooltip display
#[derive(Clone, Copy, Default)]
pub enum TooltipPosition {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

/// Wrapper component for tooltip
#[derive(Component)]
pub struct MaterialTooltipWrapper {
    pub text: String,
}

/// Wrapper component for rich tooltip
#[derive(Component)]
pub struct MaterialRichTooltipWrapper {
    pub title: String,
    pub text: String,
}

/// Configuration for tooltip display
#[derive(Component, Clone)]
pub struct MaterialTooltipConfig {
    pub text: String,
    pub delay_ms: u32,
    pub position: TooltipPosition,
    pub container_color: Color,
    pub text_color: Color,
}

/// Configuration for rich tooltip display
#[derive(Component, Clone)]
pub struct MaterialRichTooltipConfig {
    pub title: String,
    pub text: String,
    pub delay_ms: u32,
    pub position: TooltipPosition,
    pub container_color: Color,
    pub title_color: Color,
    pub text_color: Color,
}

/// Composable configuration for tooltip
#[derive(Clone)]
pub struct MaterialTooltipComposableConfig {
    pub text: String,
    pub delay_ms: u32,
    pub position: TooltipPosition,
    pub container_color: Option<Color>,
    pub text_color: Option<Color>,
}

impl MaterialTooltipComposableConfig {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            delay_ms: 500,
            position: TooltipPosition::default(),
            container_color: None,
            text_color: None,
        }
    }

    pub fn delay_ms(mut self, ms: u32) -> Self {
        self.delay_ms = ms;
        self
    }

    pub fn position(mut self, position: TooltipPosition) -> Self {
        self.position = position;
        self
    }

    pub fn container_color(mut self, color: Color) -> Self {
        self.container_color = Some(color);
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
        self
    }
}
