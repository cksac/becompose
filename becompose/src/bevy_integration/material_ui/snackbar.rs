//! Material Snackbar Composable
//!
//! Wraps bevy_material_ui Snackbar component as a BECOMPOSE composable.

use bevy::prelude::*;
use bevy_material_ui::prelude::{ShowSnackbar, SnackbarPosition};
use std::sync::Arc;

use crate::bevy_integration::composables::with_implicit_scope;
use crate::bevy_integration::material_ui::spawn_material_child;

/// Material Design snackbar composable
///
/// Note: Snackbars are typically triggered via events rather than composed directly.
/// Use this to create a snackbar host that can display snackbar messages.
///
/// # Example
/// ```ignore
/// // In your UI setup
/// MaterialSnackbarHost();
///
/// // To show a snackbar, send an event
/// fn show_message(mut writer: EventWriter<ShowSnackbar>) {
///     writer.send(ShowSnackbar::new("Message saved"));
/// }
/// ```
pub fn MaterialSnackbarHost() {
    with_implicit_scope(|| {
        spawn_material_child(move |commands, _theme| {
            // Note: SnackbarQueue is typically handled via a resource in bevy_material_ui
            // This creates a placeholder node for snackbar positioning
            commands
                .spawn(Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(24.0),
                    left: Val::Percent(50.0),
                    ..default()
                })
                .id()
        });
    });
}

/// Material Design snackbar composable with position
pub fn MaterialSnackbarHostPositioned(position: SnackbarPosition) {
    with_implicit_scope(|| {
        spawn_material_child(move |commands, _theme| {
            let node = match position {
                SnackbarPosition::BottomCenter => Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(24.0),
                    left: Val::Percent(50.0),
                    ..default()
                },
                SnackbarPosition::BottomLeft => Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(24.0),
                    left: Val::Px(24.0),
                    ..default()
                },
                SnackbarPosition::BottomRight => Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(24.0),
                    right: Val::Px(24.0),
                    ..default()
                },
                SnackbarPosition::TopCenter => Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(24.0),
                    left: Val::Percent(50.0),
                    ..default()
                },
                SnackbarPosition::TopLeft => Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(24.0),
                    left: Val::Px(24.0),
                    ..default()
                },
                SnackbarPosition::TopRight => Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(24.0),
                    right: Val::Px(24.0),
                    ..default()
                },
            };

            // Note: SnackbarQueue is typically handled via a resource in bevy_material_ui
            // This creates a placeholder node for snackbar positioning
            commands.spawn(node).id()
        });
    });
}

/// Helper to create a ShowSnackbar event with text
pub fn show_snackbar(message: impl Into<String>) -> ShowSnackbar {
    ShowSnackbar::message(message)
}

/// Helper to create a ShowSnackbar event with action
pub fn show_snackbar_with_action<F>(
    message: impl Into<String>,
    action_label: impl Into<String>,
    _on_action: F,
) -> ShowSnackbar
where
    F: Fn() + Send + Sync + 'static,
{
    ShowSnackbar::with_action(message, action_label)
}

/// Configuration for showing a snackbar
#[derive(Clone)]
pub struct MaterialSnackbarConfig {
    pub message: String,
    pub action_label: Option<String>,
    pub duration: f32,
    pub on_action: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl MaterialSnackbarConfig {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            action_label: None,
            duration: 4.0,
            on_action: None,
        }
    }

    pub fn action<F: Fn() + Send + Sync + 'static>(
        mut self,
        label: impl Into<String>,
        on_action: F,
    ) -> Self {
        self.action_label = Some(label.into());
        self.on_action = Some(Arc::new(on_action));
        self
    }

    pub fn duration(mut self, duration: f32) -> Self {
        self.duration = duration;
        self
    }

    /// Convert to ShowSnackbar event
    pub fn to_event(self) -> ShowSnackbar {
        if let Some(label) = self.action_label {
            ShowSnackbar::with_action(self.message, label)
        } else {
            ShowSnackbar::message(self.message)
        }
    }
}
