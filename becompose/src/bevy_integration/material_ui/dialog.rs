//! Material Dialog Composable
//!
//! Wraps bevy_material_ui Dialog component as a BECOMPOSE composable.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use std::sync::Arc;

use crate::bevy_integration::composables::with_implicit_scope;
use crate::bevy_integration::material_ui::spawn_material_child_with_children;

/// Material Design dialog composable
///
/// # Example
/// ```ignore
/// MaterialDialogComposable(
///     "Confirm Action",
///     "Are you sure you want to proceed?",
///     || println!("Confirmed!"),
///     || println!("Cancelled"),
/// );
/// ```
pub fn MaterialDialogComposable<F1, F2>(
    title: impl Into<String>,
    content: impl Into<String>,
    on_confirm: F1,
    on_cancel: F2,
) where
    F1: Fn() + Send + Sync + 'static,
    F2: Fn() + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let _title = title.into();
        let _content = content.into();
        let on_confirm = Arc::new(on_confirm);
        let on_cancel = Arc::new(on_cancel);

        spawn_material_child_with_children(
            move |commands, theme| {
                let dialog = MaterialDialog::new();

                commands
                    .spawn((
                        dialog,
                        Node {
                            position_type: PositionType::Absolute,
                            width: Val::Auto,
                            min_width: Val::Px(280.0),
                            max_width: Val::Px(560.0),
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(24.0)),
                            ..default()
                        },
                        BackgroundColor(theme.surface_container_high),
                        BorderRadius::all(Val::Px(28.0)),
                    ))
                    .insert(MaterialDialogHandlers {
                        on_confirm: on_confirm.clone(),
                        on_cancel: on_cancel.clone(),
                    })
                    .id()
            },
            move || {
                // Dialog content will be added here by the caller
            },
        );
    });
}

/// Material Design dialog composable with custom content
///
/// # Example
/// ```ignore
/// MaterialDialogWithContent(
///     MaterialDialogConfig::new()
///         .title("Settings")
///         .on_confirm(|| save_settings())
///         .on_cancel(|| discard_changes()),
///     || {
///         // Custom dialog content
///         Column(Modifiers::new(), || {
///             MaterialSwitchComposable("Enable notifications", true, |_| {});
///         });
///     }
/// );
/// ```
pub fn MaterialDialogWithContent<C>(config: MaterialDialogConfig, content: C)
where
    C: FnOnce(),
{
    with_implicit_scope(|| {
        let on_confirm = config.on_confirm.clone();
        let on_cancel = config.on_cancel.clone();
        let title = config.title.clone();

        spawn_material_child_with_children(
            move |commands, theme| {
                let mut dialog = MaterialDialog::new();

                if config.modal {
                    dialog.modal = true;
                }

                let entity = commands
                    .spawn((
                        dialog,
                        Node {
                            position_type: PositionType::Absolute,
                            width: Val::Auto,
                            min_width: Val::Px(280.0),
                            max_width: Val::Px(560.0),
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(24.0)),
                            row_gap: Val::Px(16.0),
                            ..default()
                        },
                        BackgroundColor(theme.surface_container_high),
                        BorderRadius::all(Val::Px(28.0)),
                    ))
                    .id();

                // Add title if provided
                if let Some(ref title_text) = title {
                    let title_entity = commands
                        .spawn((
                            DialogHeadline,
                            Text::new(title_text.clone()),
                            TextFont {
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(theme.on_surface),
                        ))
                        .id();
                    commands.entity(entity).add_child(title_entity);
                }

                // Insert handlers
                if on_confirm.is_some() || on_cancel.is_some() {
                    commands.entity(entity).insert(MaterialDialogHandlers {
                        on_confirm: on_confirm.unwrap_or_else(|| Arc::new(|| {})),
                        on_cancel: on_cancel.unwrap_or_else(|| Arc::new(|| {})),
                    });
                }

                entity
            },
            content,
        );
    });
}

/// Configuration for a Material dialog
#[derive(Clone)]
pub struct MaterialDialogConfig {
    pub title: Option<String>,
    pub modal: bool,
    pub on_confirm: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_cancel: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl MaterialDialogConfig {
    pub fn new() -> Self {
        Self {
            title: None,
            modal: true,
            on_confirm: None,
            on_cancel: None,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = modal;
        self
    }

    pub fn on_confirm<F: Fn() + Send + Sync + 'static>(mut self, on_confirm: F) -> Self {
        self.on_confirm = Some(Arc::new(on_confirm));
        self
    }

    pub fn on_cancel<F: Fn() + Send + Sync + 'static>(mut self, on_cancel: F) -> Self {
        self.on_cancel = Some(Arc::new(on_cancel));
        self
    }
}

impl Default for MaterialDialogConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Component to handle dialog events
#[derive(Component)]
pub struct MaterialDialogHandlers {
    pub on_confirm: Arc<dyn Fn() + Send + Sync>,
    pub on_cancel: Arc<dyn Fn() + Send + Sync>,
}

/// Marker component for dialog headline
#[derive(Component)]
pub struct DialogHeadline;
