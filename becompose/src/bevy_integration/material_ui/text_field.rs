//! Text Field Composable
//!
//! Wraps bevy_material_ui TextField component as a BECOMPOSE composable.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use std::sync::Arc;

use crate::bevy_integration::composables::with_implicit_scope;
use crate::bevy_integration::material_ui::spawn_material_child;

/// Design filled text field composable
///
/// # Example
/// ```ignore
/// FilledTextField("Username", "", |value| {
///     println!("Text changed: {}", value);
/// });
/// ```
pub fn FilledTextField<F>(
    label: impl Into<String>,
    initial_value: impl Into<String>,
    on_change: F,
) where
    F: Fn(String) + Send + Sync + 'static,
{
    TextField(label, initial_value, TextFieldVariant::Filled, on_change);
}

/// Design outlined text field composable
///
/// # Example
/// ```ignore
/// OutlinedTextField("Email", "", |value| {
///     println!("Email changed: {}", value);
/// });
/// ```
pub fn OutlinedTextField<F>(
    label: impl Into<String>,
    initial_value: impl Into<String>,
    on_change: F,
) where
    F: Fn(String) + Send + Sync + 'static,
{
    TextField(label, initial_value, TextFieldVariant::Outlined, on_change);
}

/// Design text field composable with variant
///
/// # Example
/// ```ignore
/// TextField("Name", "", TextFieldVariant::Filled, |value| {
///     println!("Name: {}", value);
/// });
/// ```
pub fn TextField<F>(
    label: impl Into<String>,
    initial_value: impl Into<String>,
    variant: TextFieldVariant,
    on_change: F,
) where
    F: Fn(String) + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let label = label.into();
        let initial_value = initial_value.into();
        let on_change = Arc::new(on_change);

        spawn_material_child(move |commands, theme| {
            let text_field_bundle = TextFieldBuilder::new()
                .label(&label)
                .value(&initial_value)
                .variant(variant)
                .build(theme);

            commands
                .spawn(text_field_bundle)
                .insert(TextFieldChangeHandler {
                    on_change: on_change.clone(),
                })
                .id()
        });
    });
}

/// Design text field composable with full configuration
pub fn TextFieldConfigured<F, S>(
    config: TextFieldConfig,
    on_change: F,
    on_submit: S,
) where
    F: Fn(String) + Send + Sync + 'static,
    S: Fn(String) + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let on_change = Arc::new(on_change);
        let on_submit = Arc::new(on_submit);

        spawn_material_child(move |commands, theme| {
            let mut builder = TextFieldBuilder::new().variant(config.variant);

            if let Some(ref label) = config.label {
                builder = builder.label(label);
            }

            if let Some(ref value) = config.value {
                builder = builder.value(value);
            }

            if let Some(ref placeholder) = config.placeholder {
                builder = builder.placeholder(placeholder);
            }

            if let Some(ref helper_text) = config.helper_text {
                builder = builder.supporting_text(helper_text);
            }

            if config.disabled {
                builder = builder.disabled(true);
            }

            if config.error {
                builder = builder.error(true);
            }

            if let Some(ref leading_icon) = config.leading_icon {
                builder = builder.leading_icon(leading_icon);
            }

            let text_field_bundle = builder.build(theme);

            commands
                .spawn(text_field_bundle)
                .insert(TextFieldChangeHandler {
                    on_change: on_change.clone(),
                })
                .insert(TextFieldSubmitHandler {
                    on_submit: on_submit.clone(),
                })
                .id()
        });
    });
}

/// Configuration for a text field
#[derive(Clone)]
pub struct TextFieldConfig {
    pub label: Option<String>,
    pub value: Option<String>,
    pub placeholder: Option<String>,
    pub helper_text: Option<String>,
    pub variant: TextFieldVariant,
    pub disabled: bool,
    pub error: bool,
    pub leading_icon: Option<String>,
    pub trailing_icon: Option<String>,
}

impl TextFieldConfig {
    pub fn new() -> Self {
        Self {
            label: None,
            value: None,
            placeholder: None,
            helper_text: None,
            variant: TextFieldVariant::Filled,
            disabled: false,
            error: false,
            leading_icon: None,
            trailing_icon: None,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn helper_text(mut self, text: impl Into<String>) -> Self {
        self.helper_text = Some(text.into());
        self
    }

    pub fn variant(mut self, variant: TextFieldVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn filled(mut self) -> Self {
        self.variant = TextFieldVariant::Filled;
        self
    }

    pub fn outlined(mut self) -> Self {
        self.variant = TextFieldVariant::Outlined;
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

    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.leading_icon = Some(icon.into());
        self
    }

    pub fn trailing_icon(mut self, icon: impl Into<String>) -> Self {
        self.trailing_icon = Some(icon.into());
        self
    }
}

impl Default for TextFieldConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Component to handle text field change events
#[derive(Component)]
pub struct TextFieldChangeHandler {
    pub on_change: Arc<dyn Fn(String) + Send + Sync>,
}

/// Component to handle text field submit events
#[derive(Component)]
pub struct TextFieldSubmitHandler {
    pub on_submit: Arc<dyn Fn(String) + Send + Sync>,
}
