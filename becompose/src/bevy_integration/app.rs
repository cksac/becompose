//! BECOMPOSE Application
//!
//! Provides a high-level API for creating BECOMPOSE applications
//! that hides the complexity of Bevy setup.

use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};
use std::sync::{Arc, Mutex};

use super::BecomposePlugin;
use super::composables::{
    ScopeId, ScopeMarker, has_dirty_scopes, take_dirty_scopes, 
    begin_incremental_composition, end_composition, enter_scope, exit_scope, 
    clear_scope_mapping, get_scope_info, get_scope_entities, 
    set_parent_for_scope, clear_parent_stack,
};

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

/// Resource to store the content function for recomposition
#[derive(Resource)]
struct ContentFn {
    compose_fn: Arc<Mutex<Box<dyn Fn() + Send + Sync>>>,
}

/// Resource to track scope-to-entity mappings for incremental updates
#[derive(Resource, Default)]
struct ScopeRegistry {
    /// Tracks if initial composition has happened
    initial_composition_done: bool,
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
        
        // Initialize scope registry
        app.init_resource::<ScopeRegistry>();

        // Store content as a resource for continuous recomposition
        if let Some(content) = self.content {
            let content_fn = ContentFn {
                compose_fn: Arc::new(Mutex::new(content)),
            };
            app.insert_resource(content_fn);
        }

        // Setup camera on startup
        app.add_systems(Startup, setup_camera);
        
        // Initial composition on first frame
        app.add_systems(Startup, initial_composition.after(setup_camera));
        
        // Incremental recompose UI when scopes are dirty
        app.add_systems(Update, incremental_recompose_ui);

        app.run();
    }
}

/// System that sets up the camera
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

/// System that performs the initial full composition
fn initial_composition(
    mut commands: Commands,
    content: Option<Res<ContentFn>>,
    mut registry: ResMut<ScopeRegistry>,
) {
    use super::composables::begin_composition;
    
    let Some(content) = content else { return };
    
    let compose_fn = content.compose_fn.clone();
    
    // Initialize thread-local composition context
    begin_composition(&mut commands);
    
    // Enter root scope (ScopeId(0)) for initial composition
    enter_scope(ScopeId(0));
    
    // Compose UI
    if let Ok(guard) = compose_fn.lock() {
        guard();
    };
    
    exit_scope();
    
    // Clean up composition context
    end_composition();
    
    registry.initial_composition_done = true;
}

/// System that performs incremental recomposition for dirty scopes only
fn incremental_recompose_ui(
    mut commands: Commands,
    content: Option<Res<ContentFn>>,
    roots: Query<Entity, With<CompositionRoot>>,
    scope_markers: Query<(Entity, &ScopeMarker, Option<&Parent>)>,
    registry: Res<ScopeRegistry>,
) {
    // Only proceed if there are dirty scopes
    if !has_dirty_scopes() {
        return;
    }
    
    // Only recompose if we have content and initial composition is done
    let Some(content) = content else { return };
    if !registry.initial_composition_done {
        return;
    }
    
    let dirty_scopes = take_dirty_scopes();
    
    // Clone the Arc to avoid lifetime issues with the Res
    let compose_fn = content.compose_fn.clone();
    
    // Check if root scope (0) is dirty - means full recomposition
    let full_recompose = dirty_scopes.contains(&ScopeId(0)) || dirty_scopes.contains(&ScopeId::root());
    
    if full_recompose {
        // Full recomposition: clear everything and rebuild
        for entity in roots.iter() {
            commands.entity(entity).despawn_recursive();
        }
        
        // Clear all scope mappings
        for scope_id in dirty_scopes.iter() {
            clear_scope_mapping(*scope_id);
        }
        
        // Initialize thread-local composition context  
        begin_incremental_composition(&mut commands);
        
        // Enter root scope for full recomposition
        enter_scope(ScopeId(0));
        
        // Recompose UI
        if let Ok(guard) = compose_fn.lock() {
            guard();
        };
        
        exit_scope();
        
        // Clean up composition context
        end_composition();
    } else {
        // Granular recomposition: only rebuild dirty scope subtrees
        
        // Find scope entities that need rebuilding
        let mut scopes_to_rebuild: Vec<(ScopeId, Entity, Option<Entity>)> = Vec::new();
        
        for (entity, marker, parent) in scope_markers.iter() {
            if dirty_scopes.contains(&marker.0) {
                let parent_entity = parent.map(|p| p.get());
                scopes_to_rebuild.push((marker.0, entity, parent_entity));
            }
        }
        
        // Rebuild each dirty scope
        for (scope_id, scope_entity, _parent_entity) in scopes_to_rebuild {
            // Get the scope's content function
            if let Some(scope_info) = get_scope_info(scope_id) {
                // Despawn only the children of the scope container (preserve the container)
                // First collect children to despawn
                let entities_to_despawn = get_scope_entities(scope_id);
                for entity in entities_to_despawn {
                    // Don't despawn the scope container itself
                    if entity != scope_entity {
                        if let Some(entity_commands) = commands.get_entity(entity) {
                            entity_commands.despawn_recursive();
                        }
                    }
                }
                
                // Also despawn direct children of the scope container
                commands.entity(scope_entity).despawn_descendants();
                
                // Clear scope mapping for this scope
                clear_scope_mapping(scope_id);
                
                // Set up composition context for this scope
                begin_incremental_composition(&mut commands);
                
                // Rebuild inside the scope container
                set_parent_for_scope(scope_entity);
                
                // Enter the scope and recompose
                enter_scope(scope_id);
                
                // Call the scope's content function
                (scope_info.content_fn)();
                
                exit_scope();
                clear_parent_stack();
                
                end_composition();
            }
        }
    }
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
