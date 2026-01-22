//! Material Button Composables
//!
//! Wraps bevy_material_ui Button components as BECOMPOSE composables.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use std::sync::Arc;

use crate::bevy_integration::composables::with_implicit_scope;
use crate::bevy_integration::material_ui::spawn_material_child;

/// Material Design filled button composable
///
/// # Example
/// ```ignore
/// MaterialFilledButton("Submit", || submit_form());
/// ```
pub fn MaterialFilledButton<F>(label: impl Into<String>, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    MaterialButtonComposable(label, ButtonVariant::Filled, on_click);
}

/// Material Design outlined button composable
///
/// # Example
/// ```ignore
/// MaterialOutlinedButton("Cancel", || cancel());
/// ```
pub fn MaterialOutlinedButton<F>(label: impl Into<String>, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    MaterialButtonComposable(label, ButtonVariant::Outlined, on_click);
}

/// Material Design text button composable
///
/// # Example
/// ```ignore
/// MaterialTextButton("Learn More", || show_info());
/// ```
pub fn MaterialTextButton<F>(label: impl Into<String>, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    MaterialButtonComposable(label, ButtonVariant::Text, on_click);
}

/// Material Design elevated button composable
///
/// # Example
/// ```ignore
/// MaterialElevatedButton("Save", || save_data());
/// ```
pub fn MaterialElevatedButton<F>(label: impl Into<String>, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    MaterialButtonComposable(label, ButtonVariant::Elevated, on_click);
}

/// Material Design filled tonal button composable
///
/// # Example
/// ```ignore
/// MaterialTonalButton("Add", || add_item());
/// ```
pub fn MaterialTonalButton<F>(label: impl Into<String>, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    MaterialButtonComposable(label, ButtonVariant::FilledTonal, on_click);
}

/// Material Design button composable with configurable variant
///
/// # Example
/// ```ignore
/// MaterialButtonComposable("Click Me", ButtonVariant::Filled, || handle_click());
/// ```
pub fn MaterialButtonComposable<F>(label: impl Into<String>, variant: ButtonVariant, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let label = label.into();
        let on_click = Arc::new(on_click);

        spawn_material_child(move |commands, theme| {
            let button_bundle = MaterialButtonBuilder::new(&label)
                .variant(variant)
                .build(theme);

            commands
                .spawn(button_bundle)
                .insert(MaterialButtonClickHandler {
                    on_click: on_click.clone(),
                })
                .id()
        });
    });
}

/// Material Design button composable with full configuration
///
/// # Example
/// ```ignore
/// MaterialButtonConfigured(
///     MaterialButtonConfig::new("Save")
///         .variant(ButtonVariant::Filled)
///         .icon("save")
///         .disabled(false),
///     || save_data()
/// );
/// ```
pub fn MaterialButtonConfigured<F>(config: MaterialButtonConfig, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let on_click = Arc::new(on_click);

        spawn_material_child(move |commands, theme| {
            let mut builder = MaterialButtonBuilder::new(&config.label);

            builder = builder.variant(config.variant);

            if config.disabled {
                builder = builder.disabled(true);
            }

            if let Some(ref icon) = config.icon {
                builder = builder.icon(icon);
            }

            // Note: trailing_icon is not supported by the current bevy_material_ui API
            // If config.trailing_icon is set, it will be ignored

            let button_bundle = builder.build(theme);

            commands
                .spawn(button_bundle)
                .insert(MaterialButtonClickHandler {
                    on_click: on_click.clone(),
                })
                .id()
        });
    });
}

/// Configuration for a Material button
#[derive(Clone)]
pub struct MaterialButtonConfig {
    pub label: String,
    pub variant: ButtonVariant,
    pub disabled: bool,
    pub icon: Option<String>,
    pub trailing_icon: Option<String>,
}

impl MaterialButtonConfig {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: ButtonVariant::Filled,
            disabled: false,
            icon: None,
            trailing_icon: None,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn trailing_icon(mut self, icon: impl Into<String>) -> Self {
        self.trailing_icon = Some(icon.into());
        self
    }
}

/// Component to handle button click events and call the user's callback
#[derive(Component)]
pub struct MaterialButtonClickHandler {
    pub on_click: Arc<dyn Fn() + Send + Sync>,
}
