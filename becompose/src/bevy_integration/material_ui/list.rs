//! List Composable
//!
//! Wraps bevy_material_ui List component as a BECOMPOSE composable.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use std::sync::Arc;

use crate::bevy_integration::composables::with_implicit_scope;
use crate::bevy_integration::material_ui::{
    spawn_material_child, spawn_material_child_with_children,
};

/// Design list composable
///
/// # Example
/// ```ignore
/// List(|| {
///     ListItem("Item 1", || println!("Item 1 clicked"));
///     ListItem("Item 2", || println!("Item 2 clicked"));
/// });
/// ```
pub fn List<F>(content: F)
where
    F: FnOnce(),
{
    with_implicit_scope(|| {
        spawn_material_child_with_children(
            move |commands, _theme| {
                commands
                    .spawn((
                        MaterialList::new(),
                        Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            width: Val::Percent(100.0),
                            ..default()
                        },
                    ))
                    .id()
            },
            content,
        );
    });
}

/// Design list item composable
///
/// # Example
/// ```ignore
/// ListItem("Settings", || open_settings());
/// ```
pub fn ListItem<F>(headline: impl Into<String>, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let headline = headline.into();
        let on_click = Arc::new(on_click);

        spawn_material_child(move |commands, theme| {
            let list_item = ListItemBuilder::new(&headline).build(theme);

            commands
                .spawn(list_item)
                .insert(ListItemClickHandler {
                    on_click: on_click.clone(),
                })
                .id()
        });
    });
}

/// Design list item composable with supporting text
///
/// # Example
/// ```ignore
/// ListItemWithSupporting("Wi-Fi", "Connected to Home Network", || open_wifi_settings());
/// ```
pub fn ListItemWithSupporting<F>(
    headline: impl Into<String>,
    supporting: impl Into<String>,
    on_click: F,
) where
    F: Fn() + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let headline = headline.into();
        let supporting = supporting.into();
        let on_click = Arc::new(on_click);

        spawn_material_child(move |commands, theme| {
            let list_item = ListItemBuilder::new(&headline)
                .supporting_text(&supporting)
                .build(theme);

            commands
                .spawn(list_item)
                .insert(ListItemClickHandler {
                    on_click: on_click.clone(),
                })
                .id()
        });
    });
}

/// Design list item composable with configuration
pub fn ListItemConfigured<F>(config: ListItemConfig, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let on_click = Arc::new(on_click);

        spawn_material_child(move |commands, theme| {
            let mut builder = ListItemBuilder::new(&config.headline);

            if let Some(ref supporting) = config.supporting_text {
                builder = builder.supporting_text(supporting);
            }

            if let Some(ref trailing) = config.trailing_text {
                builder = builder.supporting_text(trailing);
            }

            if let Some(ref leading_icon) = config.leading_icon {
                builder = builder.leading_icon(leading_icon);
            }

            if let Some(ref trailing_icon) = config.trailing_icon {
                builder = builder.trailing_icon(trailing_icon);
            }

            if config.disabled {
                builder = builder.disabled(true);
            }

            let list_item = builder.build(theme);

            commands
                .spawn(list_item)
                .insert(ListItemClickHandler {
                    on_click: on_click.clone(),
                })
                .id()
        });
    });
}

/// Configuration for a list item
#[derive(Clone)]
pub struct ListItemConfig {
    pub headline: String,
    pub supporting_text: Option<String>,
    pub trailing_text: Option<String>,
    pub leading_icon: Option<String>,
    pub trailing_icon: Option<String>,
    pub disabled: bool,
}

impl ListItemConfig {
    pub fn new(headline: impl Into<String>) -> Self {
        Self {
            headline: headline.into(),
            supporting_text: None,
            trailing_text: None,
            leading_icon: None,
            trailing_icon: None,
            disabled: false,
        }
    }

    pub fn supporting_text(mut self, text: impl Into<String>) -> Self {
        self.supporting_text = Some(text.into());
        self
    }

    pub fn trailing_text(mut self, text: impl Into<String>) -> Self {
        self.trailing_text = Some(text.into());
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

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Component to handle list item click events
#[derive(Component)]
pub struct ListItemClickHandler {
    pub on_click: Arc<dyn Fn() + Send + Sync>,
}
