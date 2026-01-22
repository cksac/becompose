//! Switch Composable
//!
//! Wraps bevy_material_ui Switch component as a BECOMPOSE composable.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use std::sync::Arc;

use crate::bevy_integration::composables::with_implicit_scope;
use crate::bevy_integration::material_ui::spawn_material_child;

/// Design switch composable
///
/// # Example
/// ```ignore
/// Switch("Enable notifications", false, |selected| {
///     println!("Switch is now: {}", if selected { "ON" } else { "OFF" });
/// });
/// ```
pub fn Switch<F>(label: impl Into<String>, initial_selected: bool, on_change: F)
where
    F: Fn(bool) + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let label = label.into();
        let on_change = Arc::new(on_change);

        spawn_material_child(move |commands, theme| {
            let row = commands
                .spawn((Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    column_gap: Val::Px(16.0),
                    ..default()
                },))
                .id();

            // Label
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

            // Switch
            let switch_bundle = SwitchBuilder::new().selected(initial_selected).build(theme);

            let switch_entity = commands
                .spawn(switch_bundle)
                .insert(SwitchChangeHandler {
                    on_change: on_change.clone(),
                })
                .id();

            commands.entity(row).add_child(switch_entity);

            row
        });
    });
}

/// Design switch composable with configuration
pub fn SwitchConfigured<F>(config: SwitchConfig, on_change: F)
where
    F: Fn(bool) + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let on_change = Arc::new(on_change);

        spawn_material_child(move |commands, theme| {
            let row = commands
                .spawn((Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    column_gap: Val::Px(16.0),
                    ..default()
                },))
                .id();

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

            let mut builder = SwitchBuilder::new().selected(config.selected);

            if config.disabled {
                builder = builder.disabled(true);
            }

            let switch_bundle = builder.build(theme);

            let switch_entity = commands
                .spawn(switch_bundle)
                .insert(SwitchChangeHandler {
                    on_change: on_change.clone(),
                })
                .id();

            commands.entity(row).add_child(switch_entity);

            row
        });
    });
}

#[derive(Clone)]
pub struct SwitchConfig {
    pub label: Option<String>,
    pub selected: bool,
    pub disabled: bool,
}

impl SwitchConfig {
    pub fn new() -> Self {
        Self {
            label: None,
            selected: false,
            disabled: false,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Default for SwitchConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Component to handle switch change events
#[derive(Component)]
pub struct SwitchChangeHandler {
    pub on_change: Arc<dyn Fn(bool) + Send + Sync>,
}
