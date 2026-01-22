//! Material FAB (Floating Action Button) Composable
//!
//! Wraps bevy_material_ui FAB component as a BECOMPOSE composable.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use std::sync::Arc;

use crate::bevy_integration::composables::with_implicit_scope;
use crate::bevy_integration::material_ui::spawn_material_child;

/// Material Design FAB composable
///
/// # Example
/// ```ignore
/// MaterialFabComposable("add", || {
///     println!("FAB clicked!");
/// });
/// ```
pub fn MaterialFabComposable<F>(icon: impl Into<String>, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let icon = icon.into();
        let on_click = Arc::new(on_click);

        spawn_material_child(move |commands, theme| {
            let fab_bundle = FabBuilder::new(&icon).build(theme);

            commands
                .spawn(fab_bundle)
                .insert(MaterialFabClickHandler {
                    on_click: on_click.clone(),
                })
                .id()
        });
    });
}

/// Material Design small FAB composable
///
/// # Example
/// ```ignore
/// MaterialSmallFab("edit", || {
///     println!("Small FAB clicked!");
/// });
/// ```
pub fn MaterialSmallFab<F>(icon: impl Into<String>, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let icon = icon.into();
        let on_click = Arc::new(on_click);

        spawn_material_child(move |commands, theme| {
            let fab_bundle = FabBuilder::new(&icon).small().build(theme);

            commands
                .spawn(fab_bundle)
                .insert(MaterialFabClickHandler {
                    on_click: on_click.clone(),
                })
                .id()
        });
    });
}

/// Material Design large FAB composable
///
/// # Example
/// ```ignore
/// MaterialLargeFab("add", || {
///     println!("Large FAB clicked!");
/// });
/// ```
pub fn MaterialLargeFab<F>(icon: impl Into<String>, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let icon = icon.into();
        let on_click = Arc::new(on_click);

        spawn_material_child(move |commands, theme| {
            let fab_bundle = FabBuilder::new(&icon).large().build(theme);

            commands
                .spawn(fab_bundle)
                .insert(MaterialFabClickHandler {
                    on_click: on_click.clone(),
                })
                .id()
        });
    });
}

/// Material Design extended FAB composable
///
/// # Example
/// ```ignore
/// MaterialExtendedFab("add", "Create", || {
///     println!("Extended FAB clicked!");
/// });
/// ```
pub fn MaterialExtendedFab<F>(icon: impl Into<String>, label: impl Into<String>, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let icon = icon.into();
        let label = label.into();
        let on_click = Arc::new(on_click);

        spawn_material_child(move |commands, theme| {
            let fab_bundle = FabBuilder::new(&icon).extended(&label).build(theme);

            commands
                .spawn(fab_bundle)
                .insert(MaterialFabClickHandler {
                    on_click: on_click.clone(),
                })
                .id()
        });
    });
}

/// Material Design FAB composable with full configuration
pub fn MaterialFabConfigured<F>(config: MaterialFabConfig, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let on_click = Arc::new(on_click);

        spawn_material_child(move |commands, theme| {
            let mut builder = FabBuilder::new(&config.icon);

            builder = match config.size {
                FabSize::Small => builder.small(),
                FabSize::Regular => builder,
                FabSize::Large => builder.large(),
            };

            // Handle extended FAB with label
            if let Some(ref label) = config.label {
                builder = builder.extended(label);
            }

            builder = builder.color(config.color);

            if config.lowered {
                builder = builder.lowered();
            }

            let fab_bundle = builder.build(theme);

            commands
                .spawn(fab_bundle)
                .insert(MaterialFabClickHandler {
                    on_click: on_click.clone(),
                })
                .id()
        });
    });
}

/// Configuration for a Material FAB
#[derive(Clone)]
pub struct MaterialFabConfig {
    pub icon: String,
    pub label: Option<String>,
    pub size: FabSize,
    pub color: FabColor,
    pub lowered: bool,
}

impl MaterialFabConfig {
    pub fn new(icon: impl Into<String>) -> Self {
        Self {
            icon: icon.into(),
            label: None,
            size: FabSize::Regular,
            color: FabColor::Primary,
            lowered: false,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn size(mut self, size: FabSize) -> Self {
        self.size = size;
        self
    }

    pub fn small(mut self) -> Self {
        self.size = FabSize::Small;
        self
    }

    pub fn large(mut self) -> Self {
        self.size = FabSize::Large;
        self
    }

    pub fn extended(mut self, label: impl Into<String>) -> Self {
        // Note: Extended is handled via label, not via FabSize
        self.label = Some(label.into());
        self
    }

    pub fn color(mut self, color: FabColor) -> Self {
        self.color = color;
        self
    }

    pub fn lowered(mut self, lowered: bool) -> Self {
        self.lowered = lowered;
        self
    }
}

/// Component to handle FAB click events
#[derive(Component)]
pub struct MaterialFabClickHandler {
    pub on_click: Arc<dyn Fn() + Send + Sync>,
}
