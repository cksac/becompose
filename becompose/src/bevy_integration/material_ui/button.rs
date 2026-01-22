//! Button Composables
//!
//! Wraps bevy_material_ui Button components as BECOMPOSE composables.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use std::sync::Arc;

use crate::bevy_integration::composables::with_implicit_scope;
use crate::bevy_integration::material_ui::spawn_material_child;

/// Design filled button composable
///
/// # Example
/// ```ignore
/// FilledButton("Submit", || submit_form());
/// ```
pub fn FilledButton<F>(label: impl Into<String>, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    Button(label, ButtonVariant::Filled, on_click);
}

/// Design outlined button composable
///
/// # Example
/// ```ignore
/// OutlinedButton("Cancel", || cancel());
/// ```
pub fn OutlinedButton<F>(label: impl Into<String>, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    Button(label, ButtonVariant::Outlined, on_click);
}

/// Design text button composable
///
/// # Example
/// ```ignore
/// TextButton("Learn More", || show_info());
/// ```
pub fn TextButton<F>(label: impl Into<String>, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    Button(label, ButtonVariant::Text, on_click);
}

/// Design elevated button composable
///
/// # Example
/// ```ignore
/// ElevatedButton("Save", || save_data());
/// ```
pub fn ElevatedButton<F>(label: impl Into<String>, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    Button(label, ButtonVariant::Elevated, on_click);
}

/// Design filled tonal button composable
///
/// # Example
/// ```ignore
/// TonalButton("Add", || add_item());
/// ```
pub fn TonalButton<F>(label: impl Into<String>, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    Button(label, ButtonVariant::FilledTonal, on_click);
}

/// Design button composable with configurable variant
///
/// # Example
/// ```ignore
/// Button("Click Me", ButtonVariant::Filled, || handle_click());
/// ```
pub fn Button<F>(label: impl Into<String>, variant: ButtonVariant, on_click: F)
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
                .insert(ButtonClickHandler {
                    on_click: on_click.clone(),
                })
                .id()
        });
    });
}

/// Design button composable with full configuration
///
/// # Example
/// ```ignore
/// ButtonConfigured(
///     ButtonConfig::new("Save")
///         .variant(ButtonVariant::Filled)
///         .icon("save")
///         .disabled(false),
///     || save_data()
/// );
/// ```
pub fn ButtonConfigured<F>(config: ButtonConfig, on_click: F)
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
                .insert(ButtonClickHandler {
                    on_click: on_click.clone(),
                })
                .id()
        });
    });
}

/// Configuration for a button
#[derive(Clone)]
pub struct ButtonConfig {
    pub label: String,
    pub variant: ButtonVariant,
    pub disabled: bool,
    pub icon: Option<String>,
    pub trailing_icon: Option<String>,
}

impl ButtonConfig {
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
pub struct ButtonClickHandler {
    pub on_click: Arc<dyn Fn() + Send + Sync>,
}
