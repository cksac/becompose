//! Checkbox Composable
//!
//! Wraps bevy_material_ui Checkbox component as a BECOMPOSE composable.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use std::sync::Arc;

use crate::bevy_integration::composables::with_implicit_scope;
use crate::bevy_integration::material_ui::spawn_material_child;

/// Design checkbox composable
///
/// # Example
/// ```ignore
/// Checkbox("Accept terms", CheckboxState::Unchecked, |new_state| {
///     println!("Checkbox state changed to: {:?}", new_state);
/// });
/// ```
pub fn Checkbox<F>(
    label: impl Into<String>,
    initial_state: CheckboxState,
    on_change: F,
) where
    F: Fn(CheckboxState) + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let label = label.into();
        let on_change = Arc::new(on_change);

        spawn_material_child(move |commands, theme| {
            // Create a row to hold checkbox and label
            let row = commands
                .spawn((Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(8.0),
                    ..default()
                },))
                .id();

            // Spawn checkbox
            let checkbox_entity = commands
                .spawn(CheckboxBuilder::new().state(initial_state).build())
                .insert(CheckboxChangeHandler {
                    on_change: on_change.clone(),
                })
                .id();

            commands.entity(row).add_child(checkbox_entity);

            // Spawn label
            let label_entity = commands
                .spawn((
                    Text::new(label.clone()),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(theme.on_surface),
                ))
                .id();

            commands.entity(row).add_child(label_entity);

            row
        });
    });
}

/// Design checkbox composable with configuration
pub fn CheckboxConfigured<F>(config: CheckboxConfig, on_change: F)
where
    F: Fn(CheckboxState) + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let on_change = Arc::new(on_change);

        spawn_material_child(move |commands, theme| {
            let row = commands
                .spawn((Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(8.0),
                    ..default()
                },))
                .id();

            let mut builder = CheckboxBuilder::new().state(config.state);

            if config.disabled {
                builder = builder.disabled(true);
            }

            if config.error {
                builder = builder.error(true);
            }

            let checkbox_entity = commands
                .spawn(builder.build())
                .insert(CheckboxChangeHandler {
                    on_change: on_change.clone(),
                })
                .id();

            commands.entity(row).add_child(checkbox_entity);

            if let Some(ref label) = config.label {
                let label_entity = commands
                    .spawn((
                        Text::new(label.clone()),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(if config.disabled {
                            theme.on_surface.with_alpha(0.38)
                        } else {
                            theme.on_surface
                        }),
                    ))
                    .id();

                commands.entity(row).add_child(label_entity);
            }

            row
        });
    });
}

/// Configuration for a checkbox
#[derive(Clone)]
pub struct CheckboxConfig {
    pub label: Option<String>,
    pub state: CheckboxState,
    pub disabled: bool,
    pub error: bool,
}

impl CheckboxConfig {
    pub fn new() -> Self {
        Self {
            label: None,
            state: CheckboxState::Unchecked,
            disabled: false,
            error: false,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn state(mut self, state: CheckboxState) -> Self {
        self.state = state;
        self
    }

    pub fn checked(mut self) -> Self {
        self.state = CheckboxState::Checked;
        self
    }

    pub fn indeterminate(mut self) -> Self {
        self.state = CheckboxState::Indeterminate;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }
}

impl Default for CheckboxConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Component to handle checkbox change events
#[derive(Component)]
pub struct CheckboxChangeHandler {
    pub on_change: Arc<dyn Fn(CheckboxState) + Send + Sync>,
}
