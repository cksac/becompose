//! # BECOMPOSE
//!
//! A declarative UI framework for the Bevy game engine, inspired by Jetpack Compose.
//!
//! BECOMPOSE brings Compose-style reactive UI development to Rust and Bevy,
//! allowing you to build user interfaces using composable functions and
//! reactive state management.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use bevy::prelude::*;
//! use becompose::prelude::*;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugins(BecomposePlugin)
//!         .add_systems(Startup, setup_ui)
//!         .run();
//! }
//!
//! fn setup_ui(mut commands: Commands) {
//!     commands.spawn(Camera2d::default());
//!     
//!     // Build UI using BECOMPOSE
//!     ColumnElement::new()
//!         .with_modifier(Modifiers::padding(24.0).fill_max_size())
//!         .with_child(text("Hello, BECOMPOSE!"))
//!         .build(&mut commands);
//! }
//! ```

pub mod composition;
pub mod state;
pub mod modifier;
pub mod layout;
pub mod components;
pub mod bevy_integration;

/// Re-export the composable macro
pub use becompose_macros::composable;

/// Prelude module for common imports
pub mod prelude {
    // Re-export bevy essentials
    pub use bevy::prelude::*;

    // Composition
    pub use crate::composition::{
        CompositionContext, CompositionId, CompositionKey, CompositionTree,
    };

    // State management
    pub use crate::state::{
        MutableState, DerivedState, mutable_state_of, derived_state_of,
        remember, remember_mutable_state,
        launched_effect, disposable_effect, side_effect, DisposableEffect,
    };

    // Modifiers
    pub use crate::modifier::{
        Modifier, ModifierChain, ModifierType, Modifiers,
        PaddingModifier, SizeModifier, FillModifier, WeightModifier,
        BackgroundModifier, BorderModifier,
        ClickableModifier,
    };

    // Layout
    pub use crate::layout::{
        Constraints, MeasureResult,
        Arrangement, HorizontalArrangement, VerticalArrangement,
        HorizontalAlignment, VerticalAlignment, Alignment2D,
        ColumnLayout, RowLayout, BoxLayout,
    };

    // Components
    pub use crate::components::{
        TextStyle, TextConfig, TextNode,
        ButtonConfig, ButtonNode, Clickable, OnClick,
        ImageConfig, ImageNode,
        SpacerConfig, SpacerNode,
        ColumnConfig, ColumnNode, RowConfig, RowNode, BoxConfig, BoxNode,
        CardConfig, CardNode,
    };

    // Bevy integration - core
    pub use crate::bevy_integration::{
        BecomposePlugin, UiRoot, CompositionBridge,
        UiBuilder, UiElement, BecomposeCommands,
        TextElement, ButtonElement, ColumnElement, RowElement, BoxElement, SpacerElement,
        // App
        BecomposeApp, WindowConfig, run_app, run_app_with_config, invalidate,
        // Reactive State
        State,
        // Composable functions (Jetpack Compose style)
        Text,
        Button,
        Column,
        Row,
        Box,
        Surface,
        Spacer, FixedSpacer,
        ForEach, If, IfElse,
    };

    // Re-export convenience text/button/etc functions from ui_builder for backwards compat
    pub use crate::bevy_integration::{
        text, text_styled, button, column, row, spacer, spacer_sized,
    };

    // Macro
    pub use crate::composable;
}
