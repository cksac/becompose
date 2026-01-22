//! Material Menu Composable
//!
//! Wraps bevy_material_ui Menu component as a BECOMPOSE composable.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use std::sync::Arc;

use crate::bevy_integration::composables::with_implicit_scope;
use crate::bevy_integration::material_ui::{
    spawn_material_child, spawn_material_child_with_children,
};

/// Material Design menu composable
///
/// # Example
/// ```ignore
/// MaterialMenuComposable(|| {
///     MaterialMenuItem("Cut", || cut());
///     MaterialMenuItem("Copy", || copy());
///     MaterialMenuItem("Paste", || paste());
/// });
/// ```
pub fn MaterialMenuComposable<F>(content: F)
where
    F: FnOnce(),
{
    with_implicit_scope(|| {
        spawn_material_child_with_children(
            move |commands, theme| {
                commands
                    .spawn((
                        MaterialMenu::new(),
                        Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            min_width: Val::Px(112.0),
                            max_width: Val::Px(280.0),
                            padding: UiRect::vertical(Val::Px(8.0)),
                            ..default()
                        },
                        BackgroundColor(theme.surface_container),
                        BorderRadius::all(Val::Px(4.0)),
                    ))
                    .id()
            },
            content,
        );
    });
}

/// Material Design menu item composable
///
/// # Example
/// ```ignore
/// MaterialMenuItem("Settings", || open_settings());
/// ```
pub fn MaterialMenuItem<F>(label: impl Into<String>, on_select: F)
where
    F: Fn() + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let label = label.into();
        let on_select = Arc::new(on_select);

        spawn_material_child(move |commands, theme| {
            let menu_item = MenuItemBuilder::new(&label).build(theme);

            commands
                .spawn(menu_item)
                .insert(MaterialMenuItemSelectHandler {
                    on_select: on_select.clone(),
                })
                .id()
        });
    });
}

/// Material Design menu item composable with icon
///
/// # Example
/// ```ignore
/// MaterialMenuItemWithIcon("settings", "Settings", || open_settings());
/// ```
pub fn MaterialMenuItemWithIcon<F>(icon: impl Into<String>, label: impl Into<String>, on_select: F)
where
    F: Fn() + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let icon = icon.into();
        let label = label.into();
        let on_select = Arc::new(on_select);

        spawn_material_child(move |commands, theme| {
            let menu_item = MenuItemBuilder::new(&label)
                .leading_icon(&icon)
                .build(theme);

            commands
                .spawn(menu_item)
                .insert(MaterialMenuItemSelectHandler {
                    on_select: on_select.clone(),
                })
                .id()
        });
    });
}

/// Material Design menu item composable with configuration
pub fn MaterialMenuItemConfigured<F>(config: MaterialMenuItemConfig, on_select: F)
where
    F: Fn() + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let on_select = Arc::new(on_select);

        spawn_material_child(move |commands, theme| {
            let mut builder = MenuItemBuilder::new(&config.label);

            if let Some(ref icon) = config.leading_icon {
                builder = builder.leading_icon(icon);
            }

            if let Some(ref trailing) = config.trailing_icon {
                builder = builder.trailing_icon(trailing);
            }

            if config.disabled {
                builder = builder.disabled(true);
            }

            let menu_item = builder.build(theme);

            commands
                .spawn(menu_item)
                .insert(MaterialMenuItemSelectHandler {
                    on_select: on_select.clone(),
                })
                .id()
        });
    });
}

/// Material Design menu divider composable
pub fn MaterialMenuDivider() {
    with_implicit_scope(|| {
        spawn_material_child(move |commands, theme| {
            commands
                .spawn((
                    MenuDivider,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(1.0),
                        margin: UiRect::vertical(Val::Px(8.0)),
                        ..default()
                    },
                    BackgroundColor(theme.outline_variant),
                ))
                .id()
        });
    });
}

/// Configuration for a Material menu item
#[derive(Clone)]
pub struct MaterialMenuItemConfig {
    pub label: String,
    pub leading_icon: Option<String>,
    pub trailing_icon: Option<String>,
    pub disabled: bool,
}

impl MaterialMenuItemConfig {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            leading_icon: None,
            trailing_icon: None,
            disabled: false,
        }
    }

    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.leading_icon = Some(icon.into());
        self
    }

    pub fn trailing_icon(mut self, icon: impl Into<String>) -> Self {
        self.trailing_icon = Some(icon.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Component to handle menu item selection
#[derive(Component)]
pub struct MaterialMenuItemSelectHandler {
    pub on_select: Arc<dyn Fn() + Send + Sync>,
}
