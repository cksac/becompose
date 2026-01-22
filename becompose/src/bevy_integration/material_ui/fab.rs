//! FAB (Floating Action Button) Composable
//!
//! Wraps bevy_material_ui FAB component as a BECOMPOSE composable.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use std::sync::Arc;

use crate::bevy_integration::composables::with_implicit_scope;
use crate::bevy_integration::material_ui::spawn_material_child;

/// Design FAB composable
///
/// # Example
/// ```ignore
/// Fab("add", || {
///     println!("FAB clicked!");
/// });
/// ```
pub fn Fab<F>(icon: impl Into<String>, on_click: F)
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
                .insert(FabClickHandler {
                    on_click: on_click.clone(),
                })
                .id()
        });
    });
}

/// Design small FAB composable
///
/// # Example
/// ```ignore
/// SmallFab("edit", || {
///     println!("Small FAB clicked!");
/// });
/// ```
pub fn SmallFab<F>(icon: impl Into<String>, on_click: F)
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
                .insert(FabClickHandler {
                    on_click: on_click.clone(),
                })
                .id()
        });
    });
}

/// Design large FAB composable
///
/// # Example
/// ```ignore
/// LargeFab("add", || {
///     println!("Large FAB clicked!");
/// });
/// ```
pub fn LargeFab<F>(icon: impl Into<String>, on_click: F)
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
                .insert(FabClickHandler {
                    on_click: on_click.clone(),
                })
                .id()
        });
    });
}

/// Design extended FAB composable
///
/// # Example
/// ```ignore
/// ExtendedFab("add", "Create", || {
///     println!("Extended FAB clicked!");
/// });
/// ```
pub fn ExtendedFab<F>(icon: impl Into<String>, label: impl Into<String>, on_click: F)
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
                .insert(FabClickHandler {
                    on_click: on_click.clone(),
                })
                .id()
        });
    });
}

/// Design FAB composable with full configuration
pub fn FabConfigured<F>(config: FabConfig, on_click: F)
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
                .insert(FabClickHandler {
                    on_click: on_click.clone(),
                })
                .id()
        });
    });
}

/// Configuration for a FAB
#[derive(Clone)]
pub struct FabConfig {
    pub icon: String,
    pub label: Option<String>,
    pub size: FabSize,
    pub color: FabColor,
    pub lowered: bool,
}

impl FabConfig {
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
pub struct FabClickHandler {
    pub on_click: Arc<dyn Fn() + Send + Sync>,
}
