//! Material Icon Button Composable
//!
//! Wraps bevy_material_ui IconButton component as a BECOMPOSE composable.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use std::sync::Arc;

use crate::bevy_integration::composables::with_implicit_scope;
use crate::bevy_integration::material_ui::spawn_material_child;

/// Material Design standard icon button composable
///
/// # Example
/// ```ignore
/// MaterialIconButtonComposable("favorite", || {
///     println!("Icon button clicked!");
/// });
/// ```
pub fn MaterialIconButtonComposable<F>(icon: impl Into<String>, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    MaterialIconButtonWithVariant(icon, IconButtonVariant::Standard, on_click);
}

/// Material Design filled icon button composable
///
/// # Example
/// ```ignore
/// MaterialFilledIconButton("add", || {
///     println!("Filled icon button clicked!");
/// });
/// ```
pub fn MaterialFilledIconButton<F>(icon: impl Into<String>, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    MaterialIconButtonWithVariant(icon, IconButtonVariant::Filled, on_click);
}

/// Material Design filled tonal icon button composable
///
/// # Example
/// ```ignore
/// MaterialTonalIconButton("edit", || {
///     println!("Tonal icon button clicked!");
/// });
/// ```
pub fn MaterialTonalIconButton<F>(icon: impl Into<String>, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    MaterialIconButtonWithVariant(icon, IconButtonVariant::FilledTonal, on_click);
}

/// Material Design outlined icon button composable
///
/// # Example
/// ```ignore
/// MaterialOutlinedIconButton("delete", || {
///     println!("Outlined icon button clicked!");
/// });
/// ```
pub fn MaterialOutlinedIconButton<F>(icon: impl Into<String>, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    MaterialIconButtonWithVariant(icon, IconButtonVariant::Outlined, on_click);
}

/// Material Design icon button composable with variant
pub fn MaterialIconButtonWithVariant<F>(
    icon: impl Into<String>,
    variant: IconButtonVariant,
    on_click: F,
) where
    F: Fn() + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let icon = icon.into();
        let on_click = Arc::new(on_click);

        spawn_material_child(move |commands, theme| {
            let icon_button_bundle = IconButtonBuilder::new(&icon).variant(variant).build(theme);

            commands
                .spawn(icon_button_bundle)
                .insert(MaterialIconButtonClickHandler {
                    on_click: on_click.clone(),
                })
                .id()
        });
    });
}

/// Material Design icon button composable with full configuration
pub fn MaterialIconButtonConfigured<F>(config: MaterialIconButtonConfig, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let on_click = Arc::new(on_click);

        spawn_material_child(move |commands, theme| {
            let mut builder = IconButtonBuilder::new(&config.icon).variant(config.variant);

            if config.disabled {
                builder = builder.disabled(true);
            }

            if config.selected {
                builder = builder.selected(true);
            }

            let icon_button_bundle = builder.build(theme);

            commands
                .spawn(icon_button_bundle)
                .insert(MaterialIconButtonClickHandler {
                    on_click: on_click.clone(),
                })
                .id()
        });
    });
}

/// Configuration for a Material icon button
#[derive(Clone)]
pub struct MaterialIconButtonConfig {
    pub icon: String,
    pub variant: IconButtonVariant,
    pub disabled: bool,
    pub selected: bool,
}

impl MaterialIconButtonConfig {
    pub fn new(icon: impl Into<String>) -> Self {
        Self {
            icon: icon.into(),
            variant: IconButtonVariant::Standard,
            disabled: false,
            selected: false,
        }
    }

    pub fn variant(mut self, variant: IconButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn standard(mut self) -> Self {
        self.variant = IconButtonVariant::Standard;
        self
    }

    pub fn filled(mut self) -> Self {
        self.variant = IconButtonVariant::Filled;
        self
    }

    pub fn filled_tonal(mut self) -> Self {
        self.variant = IconButtonVariant::FilledTonal;
        self
    }

    pub fn outlined(mut self) -> Self {
        self.variant = IconButtonVariant::Outlined;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

/// Component to handle icon button click events
#[derive(Component)]
pub struct MaterialIconButtonClickHandler {
    pub on_click: Arc<dyn Fn() + Send + Sync>,
}
