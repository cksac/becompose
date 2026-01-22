//! Material Progress Composables
//!
//! Wraps bevy_material_ui Progress components as BECOMPOSE composables.

use bevy_material_ui::prelude::{CircularProgressBuilder, LinearProgressBuilder};

use crate::bevy_integration::composables::with_implicit_scope;
use crate::bevy_integration::material_ui::spawn_material_child;

/// Material Design linear progress indicator composable (determinate)
///
/// # Example
/// ```ignore
/// MaterialLinearProgress(0.5); // 50% progress
/// ```
pub fn MaterialLinearProgress(progress: f32) {
    with_implicit_scope(|| {
        spawn_material_child(move |commands, theme| {
            let progress_bundle = LinearProgressBuilder::new().progress(progress).build(theme);

            commands.spawn(progress_bundle).id()
        });
    });
}

/// Material Design linear progress indicator composable (indeterminate)
///
/// # Example
/// ```ignore
/// MaterialLinearProgressIndeterminate();
/// ```
pub fn MaterialLinearProgressIndeterminate() {
    with_implicit_scope(|| {
        spawn_material_child(move |commands, theme| {
            let progress_bundle = LinearProgressBuilder::new().indeterminate().build(theme);

            commands.spawn(progress_bundle).id()
        });
    });
}

/// Material Design circular progress indicator composable (determinate)
///
/// # Example
/// ```ignore
/// MaterialCircularProgress(0.75); // 75% progress
/// ```
pub fn MaterialCircularProgress(progress: f32) {
    with_implicit_scope(|| {
        spawn_material_child(move |commands, theme| {
            let progress_bundle = CircularProgressBuilder::new()
                .progress(progress)
                .build(theme);

            commands.spawn(progress_bundle).id()
        });
    });
}

/// Material Design circular progress indicator composable (indeterminate)
///
/// # Example
/// ```ignore
/// MaterialCircularProgressIndeterminate();
/// ```
pub fn MaterialCircularProgressIndeterminate() {
    with_implicit_scope(|| {
        spawn_material_child(move |commands, theme| {
            let progress_bundle = CircularProgressBuilder::new().indeterminate().build(theme);

            commands.spawn(progress_bundle).id()
        });
    });
}

/// Material Design linear progress composable with configuration
pub fn MaterialLinearProgressConfigured(config: MaterialProgressConfig) {
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

/// Material Design circular progress composable with configuration
pub fn MaterialCircularProgressConfigured(config: MaterialProgressConfig) {
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

/// Configuration for a Material progress indicator
#[derive(Clone)]
pub struct MaterialProgressConfig {
    pub progress: f32,
    pub indeterminate: bool,
}

impl MaterialProgressConfig {
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

impl Default for MaterialProgressConfig {
    fn default() -> Self {
        Self::new()
    }
}
