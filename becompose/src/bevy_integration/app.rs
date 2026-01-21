//! BECOMPOSE Application
//!
//! Provides a high-level API for creating BECOMPOSE applications
//! that hides the complexity of Bevy setup.

use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use super::BecomposePlugin;

/// Configuration for a BECOMPOSE application window
#[derive(Clone)]
pub struct WindowConfig {
    pub title: String,
    pub width: f32,
    pub height: f32,
    pub resizable: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "BECOMPOSE App".to_string(),
            width: 800.0,
            height: 600.0,
            resizable: true,
        }
    }
}

impl WindowConfig {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..default()
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }
}

/// Marker component for UI root entities that should be cleared on recomposition
#[derive(Component)]
pub struct CompositionRoot;

/// Global recomposition trigger - call this to trigger UI rebuild
static NEEDS_RECOMPOSE: AtomicBool = AtomicBool::new(true);

/// Trigger a recomposition of the UI
/// Call this when your state changes and you need the UI to update
pub fn invalidate() {
    NEEDS_RECOMPOSE.store(true, Ordering::SeqCst);
}

/// Resource to store the content function for recomposition
#[derive(Resource)]
struct ContentFn {
    compose_fn: Arc<Mutex<Box<dyn Fn() + Send + Sync>>>,
}

/// Builder for creating a BECOMPOSE application
pub struct BecomposeApp {
    window_config: WindowConfig,
    content: Option<Box<dyn Fn() + Send + Sync>>,
}

impl Default for BecomposeApp {
    fn default() -> Self {
        Self::new()
    }
}

impl BecomposeApp {
    /// Create a new BECOMPOSE application builder
    pub fn new() -> Self {
        Self {
            window_config: WindowConfig::default(),
            content: None,
        }
    }

    /// Set the window title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.window_config.title = title.into();
        self
    }

    /// Set the window size
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.window_config.width = width;
        self.window_config.height = height;
        self
    }

    /// Configure the window
    pub fn window(mut self, config: WindowConfig) -> Self {
        self.window_config = config;
        self
    }

    /// Set the content composable function
    /// This function will be called on recomposition to rebuild the UI
    pub fn content<F>(mut self, content_fn: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.content = Some(Box::new(content_fn));
        self
    }

    /// Run the application
    pub fn run(self) {
        let mut app = App::new();

        // Configure window
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: self.window_config.title,
                resolution: (self.window_config.width, self.window_config.height).into(),
                resizable: self.window_config.resizable,
                ..default()
            }),
            ..default()
        }));

        // Add BECOMPOSE plugin
        app.add_plugins(BecomposePlugin);

        // Store content as a resource for continuous recomposition
        if let Some(content) = self.content {
            let content_fn = ContentFn {
                compose_fn: Arc::new(Mutex::new(content)),
            };
            app.insert_resource(content_fn);
        }

        // Setup camera on startup
        app.add_systems(Startup, setup_camera);
        
        // Recompose UI every frame
        app.add_systems(Update, recompose_ui);

        app.run();
    }
}

/// System that sets up the camera
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

/// System that clears and rebuilds UI only when invalidated
fn recompose_ui(
    mut commands: Commands,
    content: Option<Res<ContentFn>>,
    roots: Query<Entity, With<CompositionRoot>>,
) {
    use super::composables::{begin_composition, end_composition};
    
    // Only recompose if flagged dirty
    if !NEEDS_RECOMPOSE.swap(false, Ordering::SeqCst) {
        return;
    }
    
    // Only recompose if we have content
    let Some(content) = content else { return };
    
    // Clone the Arc to avoid lifetime issues with the Res
    let compose_fn = content.compose_fn.clone();
    
    // Clear previous UI tree
    for entity in roots.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    // Initialize thread-local composition context
    begin_composition(&mut commands);
    
    // Recompose UI
    if let Ok(guard) = compose_fn.lock() {
        guard();
    };
    
    // Clean up composition context
    end_composition();
}

/// Create and run a simple BECOMPOSE app with just a content function
/// The content function is called on recomposition to rebuild the UI
pub fn run_app<F>(title: impl Into<String>, content: F)
where
    F: Fn() + Send + Sync + 'static,
{
    BecomposeApp::new()
        .title(title)
        .content(content)
        .run()
}

/// Create and run a BECOMPOSE app with window configuration
pub fn run_app_with_config<F>(config: WindowConfig, content: F)
where
    F: Fn() + Send + Sync + 'static,
{
    BecomposeApp::new()
        .window(config)
        .content(content)
        .run()
}
