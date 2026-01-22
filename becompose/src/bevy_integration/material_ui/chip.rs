//! Chip Composable
//!
//! Wraps bevy_material_ui Chip component as a BECOMPOSE composable.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use std::sync::Arc;

use crate::bevy_integration::composables::with_implicit_scope;
use crate::bevy_integration::material_ui::spawn_material_child;

/// Design assist chip composable
///
/// # Example
/// ```ignore
/// AssistChip("Help", || {
///     println!("Assist chip clicked!");
/// });
/// ```
pub fn AssistChip<F>(label: impl Into<String>, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    Chip(label, ChipVariant::Assist, on_click);
}

/// Design filter chip composable
///
/// # Example
/// ```ignore
/// FilterChip("Active", true, |selected| {
///     println!("Filter chip selected: {}", selected);
/// });
/// ```
pub fn FilterChip<F>(label: impl Into<String>, selected: bool, on_select: F)
where
    F: Fn(bool) + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let label = label.into();
        let on_select = Arc::new(on_select);

        spawn_material_child(move |commands, theme| {
            let chip_bundle = ChipBuilder::filter(&label).selected(selected).build(theme);

            commands
                .spawn(chip_bundle)
                .insert(ChipSelectHandler {
                    on_select: on_select.clone(),
                })
                .with_children(|parent| {
                    parent.spawn((
                        ChipLabel,
                        Text::new(label.clone()),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                    ));
                })
                .id()
        });
    });
}

/// Design input chip composable
///
/// # Example
/// ```ignore
/// InputChip("Tag", || {
///     println!("Input chip deleted!");
/// });
/// ```
pub fn InputChip<F>(label: impl Into<String>, on_delete: F)
where
    F: Fn() + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let label = label.into();
        let on_delete = Arc::new(on_delete);

        spawn_material_child(move |commands, theme| {
            let chip_bundle = ChipBuilder::input(&label).build(theme);

            commands
                .spawn(chip_bundle)
                .insert(ChipDeleteHandler {
                    on_delete: on_delete.clone(),
                })
                .with_children(|parent| {
                    parent.spawn((
                        ChipLabel,
                        Text::new(label.clone()),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                    ));
                })
                .id()
        });
    });
}

/// Design suggestion chip composable
///
/// # Example
/// ```ignore
/// SuggestionChip("Suggestion", || {
///     println!("Suggestion chip clicked!");
/// });
/// ```
pub fn SuggestionChip<F>(label: impl Into<String>, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    Chip(label, ChipVariant::Suggestion, on_click);
}

/// Design chip composable with variant
pub fn Chip<F>(label: impl Into<String>, variant: ChipVariant, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let label = label.into();
        let on_click = Arc::new(on_click);

        spawn_material_child(move |commands, theme| {
            let chip_bundle = match variant {
                ChipVariant::Assist => ChipBuilder::assist(&label).build(theme),
                ChipVariant::Filter => ChipBuilder::filter(&label).build(theme),
                ChipVariant::Input => ChipBuilder::input(&label).build(theme),
                ChipVariant::Suggestion => ChipBuilder::suggestion(&label).build(theme),
            };

            commands
                .spawn(chip_bundle)
                .insert(ChipClickHandler {
                    on_click: on_click.clone(),
                })
                .with_children(|parent| {
                    parent.spawn((
                        ChipLabel,
                        Text::new(label.clone()),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                    ));
                })
                .id()
        });
    });
}

/// Design chip composable with full configuration
pub fn ChipConfigured<F>(config: ChipConfig, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let on_click = Arc::new(on_click);

        spawn_material_child(move |commands, theme| {
            let mut builder = match config.variant {
                ChipVariant::Assist => ChipBuilder::assist(&config.label),
                ChipVariant::Filter => ChipBuilder::filter(&config.label),
                ChipVariant::Input => ChipBuilder::input(&config.label),
                ChipVariant::Suggestion => ChipBuilder::suggestion(&config.label),
            };

            if config.selected {
                builder = builder.selected(true);
            }

            if config.disabled {
                builder = builder.disabled(true);
            }

            if config.elevated {
                builder = builder.elevated();
            }

            if let Some(ref icon) = config.leading_icon {
                builder = builder.leading_icon(icon);
            }

            let chip_bundle = builder.build(theme);

            commands
                .spawn(chip_bundle)
                .insert(ChipClickHandler {
                    on_click: on_click.clone(),
                })
                .with_children(|parent| {
                    parent.spawn((
                        ChipLabel,
                        Text::new(config.label.clone()),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                    ));
                })
                .id()
        });
    });
}

/// Configuration for a chip
#[derive(Clone)]
pub struct ChipConfig {
    pub label: String,
    pub variant: ChipVariant,
    pub selected: bool,
    pub disabled: bool,
    pub elevated: bool,
    pub leading_icon: Option<String>,
}

impl ChipConfig {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: ChipVariant::Assist,
            selected: false,
            disabled: false,
            elevated: false,
            leading_icon: None,
        }
    }

    pub fn variant(mut self, variant: ChipVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn assist(mut self) -> Self {
        self.variant = ChipVariant::Assist;
        self
    }

    pub fn filter(mut self) -> Self {
        self.variant = ChipVariant::Filter;
        self
    }

    pub fn input(mut self) -> Self {
        self.variant = ChipVariant::Input;
        self
    }

    pub fn suggestion(mut self) -> Self {
        self.variant = ChipVariant::Suggestion;
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

    pub fn elevated(mut self, elevated: bool) -> Self {
        self.elevated = elevated;
        self
    }

    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.leading_icon = Some(icon.into());
        self
    }
}

/// Component to handle chip click events
#[derive(Component)]
pub struct ChipClickHandler {
    pub on_click: Arc<dyn Fn() + Send + Sync>,
}

/// Component to handle chip select events (for filter chips)
#[derive(Component)]
pub struct ChipSelectHandler {
    pub on_select: Arc<dyn Fn(bool) + Send + Sync>,
}

/// Component to handle chip delete events (for input chips)
#[derive(Component)]
pub struct ChipDeleteHandler {
    pub on_delete: Arc<dyn Fn() + Send + Sync>,
}
