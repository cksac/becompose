//! Material Card Composable
//!
//! Wraps bevy_material_ui Card component as a BECOMPOSE composable.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use std::sync::Arc;

use crate::bevy_integration::composables::with_implicit_scope;
use crate::bevy_integration::material_ui::spawn_material_child_with_children;

/// Material Design elevated card composable
///
/// # Example
/// ```ignore
/// MaterialElevatedCard(|| {
///     Text("Card content", TextStyle::body());
/// });
/// ```
pub fn MaterialElevatedCard<F>(content: F)
where
    F: FnOnce(),
{
    MaterialCardComposable(CardVariant::Elevated, content);
}

/// Material Design filled card composable
///
/// # Example
/// ```ignore
/// MaterialFilledCard(|| {
///     Text("Card content", TextStyle::body());
/// });
/// ```
pub fn MaterialFilledCard<F>(content: F)
where
    F: FnOnce(),
{
    MaterialCardComposable(CardVariant::Filled, content);
}

/// Material Design outlined card composable
///
/// # Example
/// ```ignore
/// MaterialOutlinedCard(|| {
///     Text("Card content", TextStyle::body());
/// });
/// ```
pub fn MaterialOutlinedCard<F>(content: F)
where
    F: FnOnce(),
{
    MaterialCardComposable(CardVariant::Outlined, content);
}

/// Material Design card composable with variant
///
/// # Example
/// ```ignore
/// MaterialCardComposable(CardVariant::Elevated, || {
///     Column(Modifiers::new().padding(16.0), || {
///         Text("Title", TextStyle::title());
///         Text("Description", TextStyle::body());
///     });
/// });
/// ```
pub fn MaterialCardComposable<F>(variant: CardVariant, content: F)
where
    F: FnOnce(),
{
    with_implicit_scope(|| {
        spawn_material_child_with_children(
            move |commands, theme| {
                let card_bundle = CardBuilder::new().variant(variant).build(theme);

                commands.spawn(card_bundle).id()
            },
            content,
        );
    });
}

/// Material Design clickable card composable
///
/// # Example
/// ```ignore
/// MaterialClickableCard(CardVariant::Elevated, || {
///     println!("Card clicked!");
/// }, || {
///     Text("Click me!", TextStyle::body());
/// });
/// ```
pub fn MaterialClickableCard<F, C>(variant: CardVariant, on_click: F, content: C)
where
    F: Fn() + Send + Sync + 'static,
    C: FnOnce(),
{
    with_implicit_scope(|| {
        let on_click = Arc::new(on_click);

        spawn_material_child_with_children(
            move |commands, theme| {
                let card_bundle = CardBuilder::new().variant(variant).clickable().build(theme);

                commands
                    .spawn(card_bundle)
                    .insert(MaterialCardClickHandler {
                        on_click: on_click.clone(),
                    })
                    .id()
            },
            content,
        );
    });
}

/// Material Design card composable with full configuration
pub fn MaterialCardConfigured<C>(config: MaterialCardConfig, content: C)
where
    C: FnOnce(),
{
    with_implicit_scope(|| {
        let on_click = config.on_click.clone();

        spawn_material_child_with_children(
            move |commands, theme| {
                let mut builder = CardBuilder::new().variant(config.variant);

                if config.clickable {
                    builder = builder.clickable();
                }

                if config.draggable {
                    builder = builder.draggable();
                }

                let card_bundle = builder.build(theme);

                let mut entity_commands = commands.spawn(card_bundle);

                if let Some(on_click) = on_click {
                    entity_commands.insert(MaterialCardClickHandler { on_click });
                }

                entity_commands.id()
            },
            content,
        );
    });
}

/// Configuration for a Material card
#[derive(Clone)]
pub struct MaterialCardConfig {
    pub variant: CardVariant,
    pub clickable: bool,
    pub draggable: bool,
    pub on_click: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl MaterialCardConfig {
    pub fn new() -> Self {
        Self {
            variant: CardVariant::Elevated,
            clickable: false,
            draggable: false,
            on_click: None,
        }
    }

    pub fn variant(mut self, variant: CardVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn elevated(mut self) -> Self {
        self.variant = CardVariant::Elevated;
        self
    }

    pub fn filled(mut self) -> Self {
        self.variant = CardVariant::Filled;
        self
    }

    pub fn outlined(mut self) -> Self {
        self.variant = CardVariant::Outlined;
        self
    }

    pub fn clickable(mut self) -> Self {
        self.clickable = true;
        self
    }

    pub fn draggable(mut self) -> Self {
        self.draggable = true;
        self
    }

    pub fn on_click<F: Fn() + Send + Sync + 'static>(mut self, on_click: F) -> Self {
        self.on_click = Some(Arc::new(on_click));
        self.clickable = true;
        self
    }
}

impl Default for MaterialCardConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Component to handle card click events
#[derive(Component)]
pub struct MaterialCardClickHandler {
    pub on_click: Arc<dyn Fn() + Send + Sync>,
}
