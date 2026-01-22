//! Material Radio Button Composable
//!
//! Wraps bevy_material_ui Radio component as a BECOMPOSE composable.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use std::sync::Arc;

use crate::bevy_integration::composables::with_implicit_scope;
use crate::bevy_integration::material_ui::spawn_material_child;

/// Material Design radio button composable
///
/// # Example
/// ```ignore
/// MaterialRadioComposable("Option A", true, || {
///     println!("Option A selected");
/// });
/// ```
pub fn MaterialRadioComposable<F>(label: impl Into<String>, selected: bool, on_select: F)
where
    F: Fn() + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let label = label.into();
        let on_select = Arc::new(on_select);

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

            // Radio button
            let radio_bundle = RadioBuilder::new().selected(selected).build(theme);

            let radio_entity = commands
                .spawn(radio_bundle)
                .insert(MaterialRadioSelectHandler {
                    on_select: on_select.clone(),
                })
                .id();

            commands.entity(row).add_child(radio_entity);

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

            row
        });
    });
}

/// Material Design radio group composable
///
/// # Example
/// ```ignore
/// let options = vec!["Small", "Medium", "Large"];
/// let selected = 0;
/// MaterialRadioGroup(&options, selected, |index| {
///     println!("Selected option index: {}", index);
/// });
/// ```
pub fn MaterialRadioGroup<F>(options: &[impl AsRef<str>], selected_index: usize, on_select: F)
where
    F: Fn(usize) + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let options: Vec<String> = options.iter().map(|s| s.as_ref().to_string()).collect();
        let on_select = Arc::new(on_select);

        spawn_material_child(move |commands, theme| {
            let column = commands
                .spawn((Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    ..default()
                },))
                .id();

            for (index, label) in options.iter().enumerate() {
                let is_selected = index == selected_index;
                let on_select_clone = on_select.clone();

                let row = commands
                    .spawn((Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(8.0),
                        ..default()
                    },))
                    .id();

                let radio_bundle = RadioBuilder::new().selected(is_selected).build(theme);

                let radio_entity = commands
                    .spawn(radio_bundle)
                    .insert(MaterialRadioGroupItemHandler {
                        index,
                        on_select: on_select_clone,
                    })
                    .id();

                commands.entity(row).add_child(radio_entity);

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
                commands.entity(column).add_child(row);
            }

            column
        });
    });
}

/// Material Design radio button composable with configuration
pub fn MaterialRadioConfigured<F>(config: MaterialRadioConfig, on_select: F)
where
    F: Fn() + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let on_select = Arc::new(on_select);

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

            let mut builder = RadioBuilder::new().selected(config.selected);

            if config.disabled {
                builder = builder.disabled(true);
            }

            let radio_bundle = builder.build(theme);

            let radio_entity = commands
                .spawn(radio_bundle)
                .insert(MaterialRadioSelectHandler {
                    on_select: on_select.clone(),
                })
                .id();

            commands.entity(row).add_child(radio_entity);

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

/// Configuration for a Material radio button
#[derive(Clone)]
pub struct MaterialRadioConfig {
    pub label: Option<String>,
    pub selected: bool,
    pub disabled: bool,
}

impl MaterialRadioConfig {
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

impl Default for MaterialRadioConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Component to handle radio selection events
#[derive(Component)]
pub struct MaterialRadioSelectHandler {
    pub on_select: Arc<dyn Fn() + Send + Sync>,
}

/// Component to handle radio group item selection events
#[derive(Component)]
pub struct MaterialRadioGroupItemHandler {
    pub index: usize,
    pub on_select: Arc<dyn Fn(usize) + Send + Sync>,
}
