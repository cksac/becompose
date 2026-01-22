//! Material Select Composable
//!
//! Wraps bevy_material_ui Select component as a BECOMPOSE composable.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use std::sync::Arc;

use crate::bevy_integration::composables::with_implicit_scope;
use crate::bevy_integration::material_ui::spawn_material_child;

/// Material Design select (dropdown) composable
///
/// # Example
/// ```ignore
/// let options = vec!["Small", "Medium", "Large"];
/// MaterialSelectComposable("Size", &options, 0, |index| {
///     println!("Selected index: {}", index);
/// });
/// ```
pub fn MaterialSelectComposable<F>(
    label: impl Into<String>,
    options: &[impl AsRef<str>],
    selected_index: usize,
    on_select: F,
) where
    F: Fn(usize) + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let label = label.into();
        let options: Vec<String> = options.iter().map(|s| s.as_ref().to_string()).collect();
        let on_select = Arc::new(on_select);

        spawn_material_child(move |commands, theme| {
            let select_options: Vec<SelectOption> = options.iter().map(SelectOption::new).collect();

            let mut select = MaterialSelect::new(select_options);
            select.label = Some(label.clone());
            select.selected_index = Some(selected_index);

            commands
                .spawn((
                    select,
                    Node {
                        width: Val::Px(200.0),
                        ..default()
                    },
                    BackgroundColor(theme.surface_container_highest),
                ))
                .insert(MaterialSelectChangeHandler {
                    on_select: on_select.clone(),
                })
                .id()
        });
    });
}

/// Material Design select composable with configuration
pub fn MaterialSelectConfigured<F>(config: MaterialSelectConfig, on_select: F)
where
    F: Fn(usize) + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let on_select = Arc::new(on_select);

        spawn_material_child(move |commands, theme| {
            let select_options: Vec<SelectOption> =
                config.options.iter().map(SelectOption::new).collect();

            let mut select = MaterialSelect::new(select_options);
            select.label = Some(config.label.clone());
            select.selected_index = Some(config.selected_index);

            select.variant = config.variant;
            select.disabled = config.disabled;

            commands
                .spawn((
                    select,
                    Node {
                        width: Val::Px(config.width),
                        ..default()
                    },
                    BackgroundColor(theme.surface_container_highest),
                ))
                .insert(MaterialSelectChangeHandler {
                    on_select: on_select.clone(),
                })
                .id()
        });
    });
}

/// Configuration for a Material select
#[derive(Clone)]
pub struct MaterialSelectConfig {
    pub label: String,
    pub options: Vec<String>,
    pub selected_index: usize,
    pub variant: SelectVariant,
    pub disabled: bool,
    pub width: f32,
}

impl MaterialSelectConfig {
    pub fn new(label: impl Into<String>, options: Vec<impl Into<String>>) -> Self {
        Self {
            label: label.into(),
            options: options.into_iter().map(|s| s.into()).collect(),
            selected_index: 0,
            variant: SelectVariant::Filled,
            disabled: false,
            width: 200.0,
        }
    }

    pub fn selected_index(mut self, index: usize) -> Self {
        self.selected_index = index;
        self
    }

    pub fn variant(mut self, variant: SelectVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn filled(mut self) -> Self {
        self.variant = SelectVariant::Filled;
        self
    }

    pub fn outlined(mut self) -> Self {
        self.variant = SelectVariant::Outlined;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}

/// Component to handle select change events
#[derive(Component)]
pub struct MaterialSelectChangeHandler {
    pub on_select: Arc<dyn Fn(usize) + Send + Sync>,
}
