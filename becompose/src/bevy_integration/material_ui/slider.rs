//! Material Slider Composable
//!
//! Wraps bevy_material_ui Slider component as a BECOMPOSE composable.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use std::sync::Arc;

use crate::bevy_integration::composables::with_implicit_scope;
use crate::bevy_integration::material_ui::spawn_material_child;

/// Material Design slider composable
///
/// # Example
/// ```ignore
/// MaterialSliderComposable(0.5, 0.0, 1.0, |value| {
///     println!("Slider value: {}", value);
/// });
/// ```
pub fn MaterialSliderComposable<F>(value: f32, min: f32, max: f32, on_change: F)
where
    F: Fn(f32) + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let on_change = Arc::new(on_change);

        spawn_material_child(move |commands, theme| {
            let slider_bundle = SliderBuilder::new(min, max).value(value).build(theme);

            commands
                .spawn(slider_bundle)
                .insert(MaterialSliderChangeHandler {
                    on_change: on_change.clone(),
                })
                .id()
        });
    });
}

/// Material Design slider composable with label
///
/// # Example
/// ```ignore
/// MaterialSliderWithLabel("Volume", 0.5, 0.0, 1.0, |value| {
///     println!("Volume: {}", value);
/// });
/// ```
pub fn MaterialSliderWithLabel<F>(
    label: impl Into<String>,
    value: f32,
    min: f32,
    max: f32,
    on_change: F,
) where
    F: Fn(f32) + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let label = label.into();
        let on_change = Arc::new(on_change);

        spawn_material_child(move |commands, theme| {
            let column = commands
                .spawn((Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(4.0),
                    width: Val::Percent(100.0),
                    ..default()
                },))
                .id();

            // Label
            let label_entity = commands
                .spawn((
                    Text::new(label.clone()),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(theme.on_surface_variant),
                ))
                .id();

            commands.entity(column).add_child(label_entity);

            // Slider
            let slider_bundle = SliderBuilder::new(min, max).value(value).build(theme);

            let slider_entity = commands
                .spawn(slider_bundle)
                .insert(MaterialSliderChangeHandler {
                    on_change: on_change.clone(),
                })
                .id();

            commands.entity(column).add_child(slider_entity);

            column
        });
    });
}

/// Material Design slider composable with configuration
pub fn MaterialSliderConfigured<F>(config: MaterialSliderConfig, on_change: F)
where
    F: Fn(f32) + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let on_change = Arc::new(on_change);

        spawn_material_child(move |commands, theme| {
            let column = commands
                .spawn((Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(4.0),
                    width: Val::Percent(100.0),
                    ..default()
                },))
                .id();

            if let Some(ref label) = config.label {
                let label_entity = commands
                    .spawn((
                        Text::new(label.clone()),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(if config.disabled {
                            theme.on_surface_variant.with_alpha(0.38)
                        } else {
                            theme.on_surface_variant
                        }),
                    ))
                    .id();

                commands.entity(column).add_child(label_entity);
            }

            let mut builder = SliderBuilder::new(config.min, config.max).value(config.value);

            if let Some(step) = config.step {
                builder = builder.step(step);
            }

            if config.disabled {
                builder = builder.disabled(true);
            }

            if config.show_ticks {
                builder = builder.ticks();
            }

            let slider_bundle = builder.build(theme);

            let slider_entity = commands
                .spawn(slider_bundle)
                .insert(MaterialSliderChangeHandler {
                    on_change: on_change.clone(),
                })
                .id();

            commands.entity(column).add_child(slider_entity);

            column
        });
    });
}

/// Configuration for a Material slider
#[derive(Clone)]
pub struct MaterialSliderConfig {
    pub label: Option<String>,
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub step: Option<f32>,
    pub disabled: bool,
    pub show_ticks: bool,
}

impl MaterialSliderConfig {
    pub fn new(value: f32, min: f32, max: f32) -> Self {
        Self {
            label: None,
            value,
            min,
            max,
            step: None,
            disabled: false,
            show_ticks: false,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn show_ticks(mut self, show: bool) -> Self {
        self.show_ticks = show;
        self
    }
}

/// Component to handle slider change events
#[derive(Component)]
pub struct MaterialSliderChangeHandler {
    pub on_change: Arc<dyn Fn(f32) + Send + Sync>,
}
