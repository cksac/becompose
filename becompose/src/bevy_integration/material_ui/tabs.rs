//! Tabs Composable
//!
//! Wraps bevy_material_ui Tabs component as a BECOMPOSE composable.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use std::sync::Arc;

use crate::bevy_integration::composables::with_implicit_scope;
use crate::bevy_integration::material_ui::spawn_material_child;

/// Design tabs composable
///
/// # Example
/// ```ignore
/// let tabs = vec!["Home", "Profile", "Settings"];
/// Tabs(&tabs, 0, |index| {
///     println!("Selected tab: {}", index);
/// });
/// ```
pub fn Tabs<F>(tabs: &[impl AsRef<str>], selected_index: usize, on_select: F)
where
    F: Fn(usize) + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let tabs: Vec<String> = tabs.iter().map(|s| s.as_ref().to_string()).collect();
        let on_select = Arc::new(on_select);

        spawn_material_child(move |commands, theme| {
            let tab_row = commands
                .spawn((
                    MaterialTabs::new(),
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(theme.surface),
                ))
                .insert(TabsChangeHandler {
                    on_select: on_select.clone(),
                })
                .id();

            // Add tab items
            for (index, label) in tabs.iter().enumerate() {
                let is_selected = index == selected_index;

                let tab_entity = commands
                    .spawn((
                        MaterialTab {
                            index,
                            label: label.clone(),
                            icon: None,
                            disabled: false,
                            selected: is_selected,
                            pressed: false,
                            hovered: false,
                        },
                        Button,
                        Node {
                            flex_grow: 1.0,
                            padding: UiRect::all(Val::Px(16.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(if is_selected {
                            theme.surface_container_highest
                        } else {
                            Color::NONE
                        }),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new(label.clone()),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(if is_selected {
                                theme.primary
                            } else {
                                theme.on_surface_variant
                            }),
                        ));
                    })
                    .id();

                commands.entity(tab_row).add_child(tab_entity);
            }

            tab_row
        });
    });
}

/// Design tabs composable with icons
///
/// # Example
/// ```ignore
/// let tabs = vec![
///     ("home", "Home"),
///     ("person", "Profile"),
///     ("settings", "Settings"),
/// ];
/// TabsWithIcons(&tabs, 0, |index| {
///     println!("Selected tab: {}", index);
/// });
/// ```
pub fn TabsWithIcons<F>(
    tabs: &[(impl AsRef<str>, impl AsRef<str>)],
    selected_index: usize,
    on_select: F,
) where
    F: Fn(usize) + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let tabs: Vec<(String, String)> = tabs
            .iter()
            .map(|(icon, label)| (icon.as_ref().to_string(), label.as_ref().to_string()))
            .collect();
        let on_select = Arc::new(on_select);

        spawn_material_child(move |commands, theme| {
            let tab_row = commands
                .spawn((
                    MaterialTabs::new(),
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(theme.surface),
                ))
                .insert(TabsChangeHandler {
                    on_select: on_select.clone(),
                })
                .id();

            for (index, (icon, label)) in tabs.iter().enumerate() {
                let is_selected = index == selected_index;

                let tab_entity = commands
                    .spawn((
                        MaterialTab {
                            index,
                            label: label.clone(),
                            icon: Some(icon.clone()),
                            disabled: false,
                            selected: is_selected,
                            pressed: false,
                            hovered: false,
                        },
                        Button,
                        Node {
                            flex_grow: 1.0,
                            padding: UiRect::all(Val::Px(12.0)),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(4.0),
                            ..default()
                        },
                        BackgroundColor(if is_selected {
                            theme.surface_container_highest
                        } else {
                            Color::NONE
                        }),
                    ))
                    .with_children(|parent| {
                        // Icon would go here - using text placeholder
                        parent.spawn((
                            Text::new(label.clone()),
                            TextFont {
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(if is_selected {
                                theme.primary
                            } else {
                                theme.on_surface_variant
                            }),
                        ));
                    })
                    .id();

                commands.entity(tab_row).add_child(tab_entity);
            }

            tab_row
        });
    });
}

/// Design tabs composable with configuration
pub fn TabsConfigured<F>(config: TabsConfig, on_select: F)
where
    F: Fn(usize) + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let on_select = Arc::new(on_select);

        spawn_material_child(move |commands, theme| {
            let tab_row = commands
                .spawn((
                    MaterialTabs::new(),
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(theme.surface),
                ))
                .insert(TabsChangeHandler {
                    on_select: on_select.clone(),
                })
                .id();

            for (index, tab) in config.tabs.iter().enumerate() {
                let is_selected = index == config.selected_index;
                let is_disabled = config.disabled_indices.contains(&index);

                let tab_entity = commands
                    .spawn((
                        MaterialTab {
                            index,
                            label: tab.label.clone(),
                            icon: tab.icon.clone(),
                            disabled: is_disabled,
                            selected: is_selected,
                            pressed: false,
                            hovered: false,
                        },
                        Button,
                        Node {
                            flex_grow: 1.0,
                            padding: UiRect::all(Val::Px(16.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(if is_selected {
                            theme.surface_container_highest
                        } else {
                            Color::NONE
                        }),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new(tab.label.clone()),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(if is_disabled {
                                theme.on_surface.with_alpha(0.38)
                            } else if is_selected {
                                theme.primary
                            } else {
                                theme.on_surface_variant
                            }),
                        ));
                    })
                    .id();

                commands.entity(tab_row).add_child(tab_entity);
            }

            tab_row
        });
    });
}

/// Configuration for a single tab
#[derive(Clone)]
pub struct TabConfig {
    pub label: String,
    pub icon: Option<String>,
}

impl TabConfig {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
        }
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

/// Configuration for Material tabs
#[derive(Clone)]
pub struct TabsConfig {
    pub tabs: Vec<TabConfig>,
    pub selected_index: usize,
    pub disabled_indices: Vec<usize>,
}

impl TabsConfig {
    pub fn new(tabs: Vec<TabConfig>) -> Self {
        Self {
            tabs,
            selected_index: 0,
            disabled_indices: Vec::new(),
        }
    }

    pub fn selected_index(mut self, index: usize) -> Self {
        self.selected_index = index;
        self
    }

    pub fn disable_tab(mut self, index: usize) -> Self {
        self.disabled_indices.push(index);
        self
    }
}

/// Component to handle tab change events
#[derive(Component)]
pub struct TabsChangeHandler {
    pub on_select: Arc<dyn Fn(usize) + Send + Sync>,
}
