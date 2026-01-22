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

pub mod bevy_integration;
pub mod components;
pub mod composition;
pub mod layout;
pub mod modifier;
pub mod state;

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
        derived_state_of, disposable_effect, launched_effect, mutable_state_of, remember,
        remember_mutable_state, side_effect, DerivedState, DisposableEffect, MutableState,
    };

    // Modifiers
    pub use crate::modifier::{
        BackgroundModifier, BorderModifier, ClickableModifier, FillModifier, Modifier,
        ModifierType, Modifiers, PaddingModifier, SizeModifier, WeightModifier,
    };

    // Layout
    pub use crate::layout::{
        Alignment2D, Arrangement, BoxLayout, ColumnLayout, Constraints, HorizontalAlignment,
        HorizontalArrangement, MeasureResult, RowLayout, VerticalAlignment, VerticalArrangement,
    };

    // Components
    pub use crate::components::{
        BoxConfig, BoxNode, ButtonConfig, ButtonNode, CardConfig, CardNode, Clickable,
        ColumnConfig, ColumnNode, ImageConfig, ImageNode, OnClick, RowConfig, RowNode,
        SpacerConfig, SpacerNode, TextConfig, TextNode, TextStyle,
    };

    // Bevy integration - core
    pub use crate::bevy_integration::{
        invalidate,
        run_app,
        run_app_with_config,
        // App
        BecomposeApp,
        BecomposeCommands,
        BecomposePlugin,
        Box,
        BoxElement,
        Button,
        ButtonElement,
        Column,
        ColumnElement,
        CompositionBridge,
        FixedSpacer,
        ForEach,
        If,
        IfElse,
        Row,
        RowElement,
        Scope,
        // Scope-based recomposition
        ScopeId,
        Spacer,
        SpacerElement,
        // Reactive State
        State,
        Surface,
        // Composable functions (Jetpack Compose style)
        Text,
        TextElement,
        UiBuilder,
        UiElement,
        UiRoot,
        WindowConfig,
    };

    // Re-export convenience text/button/etc functions from ui_builder for backwards compat
    pub use crate::bevy_integration::{
        button, column, row, spacer, spacer_sized, text, text_styled,
    };

    // Material UI composables
    pub use crate::bevy_integration::material_ui;

    // Macro
    pub use crate::composable;
}
