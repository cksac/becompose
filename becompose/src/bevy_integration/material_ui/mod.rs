//! Material UI Composables
//!
//! This module wraps bevy_material_ui components as BECOMPOSE composables,
//! enabling declarative Material Design 3 UI development.
//!
//! # Example
//! ```ignore
//! use becompose::prelude::*;
//! use becompose::material_ui::*;
//!
//! fn MyApp() {
//!     MaterialColumn(Modifiers::new().padding(16.0), || {
//!         MaterialFilledButton("Click Me", || println!("Clicked!"));
//!         MaterialCheckbox("Accept terms", CheckboxState::Unchecked, |state| println!("State: {:?}", state));
//!         MaterialSwitch("Enable notifications", false, |selected| println!("Selected: {}", selected));
//!     });
//! }
//! ```

// Allow PascalCase function names to match Jetpack Compose conventions
#![allow(non_snake_case)]
#![allow(ambiguous_glob_reexports)]

use bevy::prelude::*;
use std::cell::RefCell;

mod button;
mod card;
mod checkbox;
mod chip;
mod dialog;
mod divider;
mod fab;
mod icon_button;
mod list;
mod menu;
mod progress;
mod radio;
mod select;
mod slider;
mod snackbar;
mod switch;
mod tabs;
mod text_field;
mod tooltip;

pub use button::*;
pub use card::*;
pub use checkbox::*;
pub use chip::*;
pub use dialog::*;
pub use divider::*;
pub use fab::*;
pub use icon_button::*;
pub use list::*;
pub use menu::*;
pub use progress::*;
pub use radio::*;
pub use select::*;
pub use slider::*;
pub use snackbar::*;
pub use switch::*;
pub use tabs::*;
pub use text_field::*;
pub use tooltip::*;

// Re-export bevy_material_ui types for convenience
pub use bevy_material_ui::prelude::*;

// Import MaterialTheme for local use (this is used internally, the glob reexport handles public access)
#[allow(hidden_glob_reexports)]
use bevy_material_ui::prelude::MaterialTheme;

// ============================================================================
// Material Theme Context
// ============================================================================

// Thread-local storage for the material theme during composition
thread_local! {
    static MATERIAL_THEME: RefCell<Option<MaterialTheme>> = const { RefCell::new(None) };
}

/// Set the material theme for the current composition
pub fn set_material_theme(theme: MaterialTheme) {
    MATERIAL_THEME.with(|t| {
        *t.borrow_mut() = Some(theme);
    });
}

/// Get the current material theme
pub fn get_material_theme() -> Option<MaterialTheme> {
    MATERIAL_THEME.with(|t| t.borrow().clone())
}

/// Clear the material theme after composition
pub fn clear_material_theme() {
    MATERIAL_THEME.with(|t| {
        *t.borrow_mut() = None;
    });
}

// ============================================================================
// Material Spawn Helpers
// ============================================================================

/// Spawn a material UI child entity with access to the theme
///
/// This function provides access to both Commands and MaterialTheme for spawning
/// material UI components within a composition context.
pub fn spawn_material_child<F>(f: F) -> Entity
where
    F: FnOnce(&mut Commands, &MaterialTheme) -> Entity,
{
    use crate::bevy_integration::composables::{
        register_entity_scope, CompositionRoot, COMPOSITION_CTX,
    };
    use std::cell::RefCell;

    COMPOSITION_CTX.with(
        |ctx: &RefCell<crate::bevy_integration::composables::CompositionContext>| {
            let ctx_ref = ctx.borrow();
            let commands = unsafe { &mut *ctx_ref.commands };

            // Get the theme - use default if not set
            let theme = get_material_theme().unwrap_or_default();

            // Spawn the entity using the provided function
            let entity = f(commands, &theme);

            // Track which scope this entity belongs to
            let scope_id = ctx_ref.scope_stack.last().copied();
            let parent = ctx_ref.parent_stack.last().copied();

            drop(ctx_ref);

            if let Some(scope_id) = scope_id {
                register_entity_scope(entity, scope_id);
            }

            COMPOSITION_CTX.with(
                |ctx: &RefCell<crate::bevy_integration::composables::CompositionContext>| {
                    let ctx_ref = ctx.borrow();
                    let commands = unsafe { &mut *ctx_ref.commands };

                    if let Some(parent) = parent {
                        commands.entity(parent).add_child(entity);
                    } else {
                        commands.entity(entity).insert(CompositionRoot);
                    }
                },
            );

            entity
        },
    )
}

/// Spawn a material UI child with children content
pub fn spawn_material_child_with_children<F, C>(spawn_fn: F, content: C) -> Entity
where
    F: FnOnce(&mut Commands, &MaterialTheme) -> Entity,
    C: FnOnce(),
{
    use crate::bevy_integration::composables::{
        enter_scope, exit_scope, pop_parent, push_parent, ScopeId,
    };

    let entity = spawn_material_child(spawn_fn);

    // Create a scope for children
    let scope_id = ScopeId::new();
    push_parent(entity);
    enter_scope(scope_id);

    content();

    exit_scope();
    pop_parent();

    entity
}
