//! Progress Composables
//!
//! Wraps bevy_material_ui Progress components as BECOMPOSE composables.

use bevy_material_ui::prelude::{CircularProgressBuilder, LinearProgressBuilder};

use crate::bevy_integration::composables::with_implicit_scope;
use crate::bevy_integration::material_ui::spawn_material_child;

/// Design linear progress indicator composable (determinate)
///
/// # Example
/// ```ignore
/// LinearProgress(0.5); // 50% progress
/// ```
pub fn LinearProgress(progress: f32) {
    with_implicit_scope(|| {
        spawn_material_child(move |commands, theme| {
            let progress_bundle = LinearProgressBuilder::new().progress(progress).build(theme);

            commands.spawn(progress_bundle).id()
        });
    });
}

/// Design linear progress indicator composable (indeterminate)
///
/// # Example
/// ```ignore
/// LinearProgressIndeterminate();
/// ```
pub fn LinearProgressIndeterminate() {
    with_implicit_scope(|| {
        spawn_material_child(move |commands, theme| {
            let progress_bundle = LinearProgressBuilder::new().indeterminate().build(theme);

            commands.spawn(progress_bundle).id()
        });
    });
}

/// Design circular progress indicator composable (determinate)
///
/// # Example
/// ```ignore
/// CircularProgress(0.75); // 75% progress
/// ```
pub fn CircularProgress(progress: f32) {
    with_implicit_scope(|| {
        spawn_material_child(move |commands, theme| {
            let progress_bundle = CircularProgressBuilder::new()
                .progress(progress)
                .build(theme);

            commands.spawn(progress_bundle).id()
        });
    });
}

/// Design circular progress indicator composable (indeterminate)
///
/// # Example
/// ```ignore
/// CircularProgressIndeterminate();
/// ```
pub fn CircularProgressIndeterminate() {
    with_implicit_scope(|| {
        spawn_material_child(move |commands, theme| {
            let progress_bundle = CircularProgressBuilder::new().indeterminate().build(theme);

            commands.spawn(progress_bundle).id()
        });
    });
}

/// Design linear progress composable with configuration
pub fn LinearProgressConfigured(config: ProgressConfig) {
    with_implicit_scope(|| {
        spawn_material_child(move |commands, theme| {
            let mut builder = LinearProgressBuilder::new();

            if config.indeterminate {
                builder = builder.indeterminate();
            } else {
                builder = builder.progress(config.progress);
            }

            let progress_bundle = builder.build(theme);

            commands.spawn(progress_bundle).id()
        });
    });
}

/// Design circular progress composable with configuration
pub fn CircularProgressConfigured(config: ProgressConfig) {
    with_implicit_scope(|| {
        spawn_material_child(move |commands, theme| {
            let mut builder = CircularProgressBuilder::new();

            if config.indeterminate {
                builder = builder.indeterminate();
            } else {
                builder = builder.progress(config.progress);
            }

            let progress_bundle = builder.build(theme);

            commands.spawn(progress_bundle).id()
        });
    });
}

/// Configuration for a progress indicator
#[derive(Clone)]
pub struct ProgressConfig {
    pub progress: f32,
    pub indeterminate: bool,
}

impl ProgressConfig {
    pub fn new() -> Self {
        Self {
            progress: 0.0,
            indeterminate: false,
        }
    }

    pub fn progress(mut self, progress: f32) -> Self {
        self.progress = progress.clamp(0.0, 1.0);
        self.indeterminate = false;
        self
    }

    pub fn indeterminate(mut self) -> Self {
        self.indeterminate = true;
        self
    }
}

impl Default for ProgressConfig {
    fn default() -> Self {
        Self::new()
    }
}
