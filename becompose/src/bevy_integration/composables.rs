//! Composable Functions for BECOMPOSE
//!
//! This module provides Jetpack Compose-style composable functions for building UIs.
//!
//! The key principles are:
//! 1. **Composable functions** emit UI by calling other composable functions
//! 2. **Closures for content** - similar to Kotlin's trailing lambdas
//! 3. **Data flows down, events flow up** - functions receive data and emit callbacks
//! 4. **Implicit composition context** - the context is stored in thread-local storage
//!
//! # Example
//! ```ignore
//! fn MyApp() {
//!     Column(|| {
//!         Text("Hello, BECOMPOSE!");
//!         Button("Click me", || println!("Clicked!"));
//!     });
//! }
//! ```

// Allow PascalCase function names to match Jetpack Compose conventions
#![allow(non_snake_case)]

use bevy::prelude::*;
use generational_box::{AnyStorage, GenerationalBox, Owner, SyncStorage};
use std::cell::RefCell;
use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

use crate::components::TextStyle;
use crate::modifier::Modifiers;

pub use super::app::CompositionRoot;

// ============================================================================
// Scope-based Dirty Tracking
// ============================================================================

/// Unique identifier for a composition scope
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId(pub u64);

impl ScopeId {
    pub fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::SeqCst))
    }

    /// Root scope ID (always 0)
    pub fn root() -> Self {
        Self(0)
    }
}

impl Default for ScopeId {
    fn default() -> Self {
        Self::new()
    }
}

/// Stored content function for a scope
pub type ScopedContentFn = Arc<dyn Fn() + Send + Sync>;

/// Registry of scope content functions for granular recomposition
static SCOPE_REGISTRY: RwLock<Option<std::collections::HashMap<ScopeId, ScopeInfo>>> =
    RwLock::new(None);

/// Information about a registered scope
#[derive(Clone)]
pub struct ScopeInfo {
    /// The content function to call when recomposing this scope
    pub content_fn: ScopedContentFn,
    /// Parent scope (for hierarchy)
    pub parent_scope: Option<ScopeId>,
    /// Root entity of this scope's subtree
    pub root_entity: Option<Entity>,
}

/// Registry of scope owners for state lifetime management
/// Each scope has its own Owner - when the scope is destroyed, all states created
/// within that scope are automatically freed.
static SCOPE_OWNERS: RwLock<Option<std::collections::HashMap<ScopeId, Owner<SyncStorage>>>> =
    RwLock::new(None);

/// Get or create an owner for a scope
fn get_or_create_scope_owner(scope_id: ScopeId) -> Owner<SyncStorage> {
    {
        let guard = SCOPE_OWNERS.read().unwrap();
        if let Some(map) = guard.as_ref() {
            if let Some(owner) = map.get(&scope_id) {
                return owner.clone();
            }
        }
    }
    let mut guard = SCOPE_OWNERS.write().unwrap();
    if guard.is_none() {
        *guard = Some(std::collections::HashMap::new());
    }
    let map = guard.as_mut().unwrap();
    map.entry(scope_id)
        .or_insert_with(SyncStorage::owner)
        .clone()
}

/// Drop the owner for a scope, freeing all states created within it
fn drop_scope_owner(scope_id: ScopeId) {
    let mut guard = SCOPE_OWNERS.write().unwrap();
    if let Some(map) = guard.as_mut() {
        map.remove(&scope_id);
    }
}

/// Register a scope with its content function
pub fn register_scope(
    scope_id: ScopeId,
    content_fn: ScopedContentFn,
    parent_scope: Option<ScopeId>,
) {
    let mut guard = SCOPE_REGISTRY.write().unwrap();
    if guard.is_none() {
        *guard = Some(std::collections::HashMap::new());
    }
    if let Some(map) = guard.as_mut() {
        map.insert(
            scope_id,
            ScopeInfo {
                content_fn,
                parent_scope,
                root_entity: None,
            },
        );
    }
}

/// Update the root entity for a scope
pub fn set_scope_root_entity(scope_id: ScopeId, entity: Entity) {
    let mut guard = SCOPE_REGISTRY.write().unwrap();
    if let Some(map) = guard.as_mut() {
        if let Some(info) = map.get_mut(&scope_id) {
            info.root_entity = Some(entity);
        }
    }
}

/// Get scope info
pub fn get_scope_info(scope_id: ScopeId) -> Option<ScopeInfo> {
    let guard = SCOPE_REGISTRY.read().unwrap();
    guard.as_ref().and_then(|map| map.get(&scope_id).cloned())
}

/// Unregister a scope (for cleanup)
/// This also drops the scope's Owner, freeing all states created within the scope.
pub fn unregister_scope(scope_id: ScopeId) {
    // First drop the scope's owner to free all states created in this scope
    drop_scope_owner(scope_id);

    // Then remove the scope from the registry
    let mut guard = SCOPE_REGISTRY.write().unwrap();
    if let Some(map) = guard.as_mut() {
        map.remove(&scope_id);
    }
}

/// Global dirty scope registry
static DIRTY_SCOPES: RwLock<Option<HashSet<ScopeId>>> = RwLock::new(None);

/// Mark a specific scope as dirty (needs recomposition)
pub fn mark_scope_dirty(scope_id: ScopeId) {
    let mut guard = DIRTY_SCOPES.write().unwrap();
    if guard.is_none() {
        *guard = Some(HashSet::new());
    }
    if let Some(set) = guard.as_mut() {
        set.insert(scope_id);
    }
}

/// Check if any scopes are dirty
pub fn has_dirty_scopes() -> bool {
    let guard = DIRTY_SCOPES.read().unwrap();
    guard.as_ref().map(|s| !s.is_empty()).unwrap_or(false)
}

/// Take all dirty scopes (clears the set)
pub fn take_dirty_scopes() -> HashSet<ScopeId> {
    let mut guard = DIRTY_SCOPES.write().unwrap();
    guard.take().unwrap_or_default()
}

/// Legacy invalidate function - marks the root scope dirty for full recomposition
/// Consider using State<T> which automatically tracks scopes for granular updates
pub fn invalidate() {
    // For backward compatibility, mark scope 0 as dirty (root)
    mark_scope_dirty(ScopeId(0));
}

// ============================================================================
// Thread-Local Composition Context
// ============================================================================

/// Internal composition context stored in thread-local
pub struct CompositionContext {
    pub parent_stack: Vec<Entity>,
    pub commands: *mut Commands<'static, 'static>,
    /// Stack of scope IDs for tracking which scope we're in
    pub scope_stack: Vec<ScopeId>,
    /// Map of scope ID to its root entity (for selective rebuilding)
    pub scope_root_entities: std::collections::HashMap<ScopeId, Entity>,
    /// Map of scope ID to ALL entities in that scope (for cleanup)
    pub scope_all_entities: std::collections::HashMap<ScopeId, Vec<Entity>>,
    /// Map of entity to its scope (for cleanup)
    pub entity_scopes: std::collections::HashMap<Entity, ScopeId>,
}

impl CompositionContext {
    fn new() -> Self {
        Self {
            parent_stack: Vec::new(),
            commands: std::ptr::null_mut(),
            scope_stack: Vec::new(),
            scope_root_entities: std::collections::HashMap::new(),
            scope_all_entities: std::collections::HashMap::new(),
            entity_scopes: std::collections::HashMap::new(),
        }
    }
}

thread_local! {
    pub static COMPOSITION_CTX: RefCell<CompositionContext> = RefCell::new(CompositionContext::new());
}

/// Initialize the composition context for this frame
/// Called by the framework - users don't need to call this
pub fn begin_composition(commands: &mut Commands) {
    COMPOSITION_CTX.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        ctx.parent_stack.clear();
        // Note: Keep scope_entities and entity_scopes for incremental updates
        // SAFETY: We ensure this pointer is only valid during composition
        #[allow(clippy::unnecessary_cast)]
        {
            ctx.commands = commands as *mut Commands as *mut Commands<'static, 'static>;
        }
    });
}

/// Begin composition for incremental updates (preserves scope mappings)
pub fn begin_incremental_composition(commands: &mut Commands) {
    COMPOSITION_CTX.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        ctx.parent_stack.clear();
        ctx.scope_stack.clear();
        #[allow(clippy::unnecessary_cast)]
        {
            ctx.commands = commands as *mut Commands as *mut Commands<'static, 'static>;
        }
    });
}

/// End the composition context for this frame
pub fn end_composition() {
    COMPOSITION_CTX.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        ctx.parent_stack.clear();
        ctx.scope_stack.clear();
        ctx.commands = std::ptr::null_mut();
    });
}

/// Get the current scope ID (for state tracking)
pub fn current_scope_id() -> Option<ScopeId> {
    COMPOSITION_CTX.with(|ctx| ctx.borrow().scope_stack.last().copied())
}

/// Enter a new scope for composition tracking
pub fn enter_scope(scope_id: ScopeId) {
    COMPOSITION_CTX.with(|ctx| {
        ctx.borrow_mut().scope_stack.push(scope_id);
    });
}

/// Exit the current scope
pub fn exit_scope() {
    COMPOSITION_CTX.with(|ctx| {
        ctx.borrow_mut().scope_stack.pop();
    });
}

/// Register an entity with the current scope
pub fn register_entity_scope(entity: Entity, scope_id: ScopeId) {
    COMPOSITION_CTX.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        ctx.entity_scopes.insert(entity, scope_id);
        // Track all entities in this scope
        ctx.scope_all_entities
            .entry(scope_id)
            .or_default()
            .push(entity);
        // Only set scope root if not already set
        ctx.scope_root_entities.entry(scope_id).or_insert(entity);
    });
}

/// Get the root entity for a scope
pub fn get_scope_root_entity(scope_id: ScopeId) -> Option<Entity> {
    COMPOSITION_CTX.with(|ctx| ctx.borrow().scope_root_entities.get(&scope_id).copied())
}

/// Get all entities belonging to a scope
pub fn get_scope_entities(scope_id: ScopeId) -> Vec<Entity> {
    COMPOSITION_CTX.with(|ctx| {
        ctx.borrow()
            .scope_all_entities
            .get(&scope_id)
            .cloned()
            .unwrap_or_default()
    })
}

/// Clear scope mapping for a specific scope (for recomposition)
pub fn clear_scope_mapping(scope_id: ScopeId) {
    COMPOSITION_CTX.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        ctx.scope_root_entities.remove(&scope_id);
        if let Some(entities) = ctx.scope_all_entities.remove(&scope_id) {
            for entity in entities {
                ctx.entity_scopes.remove(&entity);
            }
        }
    });
}

/// Get the parent entity for inserting scope content
pub fn get_current_parent() -> Option<Entity> {
    COMPOSITION_CTX.with(|ctx| ctx.borrow().parent_stack.last().copied())
}

/// Set parent for scope recomposition
pub fn set_parent_for_scope(entity: Entity) {
    COMPOSITION_CTX.with(|ctx| {
        ctx.borrow_mut().parent_stack.push(entity);
    });
}

/// Clear parent stack
pub fn clear_parent_stack() {
    COMPOSITION_CTX.with(|ctx| {
        ctx.borrow_mut().parent_stack.clear();
    });
}

/// Push a new parent onto the stack
pub fn push_parent(entity: Entity) {
    COMPOSITION_CTX.with(|ctx| {
        ctx.borrow_mut().parent_stack.push(entity);
    });
}

/// Pop the current parent from the stack
pub fn pop_parent() {
    COMPOSITION_CTX.with(|ctx| {
        ctx.borrow_mut().parent_stack.pop();
    });
}

/// Spawn an entity and add it as a child of the current parent
fn spawn_child(bundle: impl Bundle) -> Entity {
    COMPOSITION_CTX.with(|ctx| {
        let ctx = ctx.borrow();
        // SAFETY: We ensure commands is valid during composition
        let commands = unsafe { &mut *ctx.commands };
        let entity = commands.spawn(bundle).id();

        // Track which scope this entity belongs to
        if let Some(&scope_id) = ctx.scope_stack.last() {
            // Release borrow before calling register_entity_scope
            drop(ctx);
            register_entity_scope(entity, scope_id);
            // Re-borrow to continue
            let ctx = COMPOSITION_CTX.with(|c| c.borrow().parent_stack.last().copied());
            if let Some(parent) = ctx {
                let ctx_ref = COMPOSITION_CTX.with(|c| c.borrow().commands);
                let commands = unsafe { &mut *ctx_ref };
                commands.entity(parent).add_child(entity);
            } else {
                let ctx_ref = COMPOSITION_CTX.with(|c| c.borrow().commands);
                let commands = unsafe { &mut *ctx_ref };
                commands.entity(entity).insert(CompositionRoot);
            }
            return entity;
        }

        if let Some(parent) = ctx.parent_stack.last() {
            commands.entity(*parent).add_child(entity);
        } else {
            // Root level - mark for recomposition cleanup
            commands.entity(entity).insert(CompositionRoot);
        }
        entity
    })
}

/// Execute a closure with mutable access to commands
#[allow(dead_code)]
fn with_commands<R>(f: impl FnOnce(&mut Commands) -> R) -> R {
    COMPOSITION_CTX.with(|ctx| {
        let ctx = ctx.borrow();
        // SAFETY: We ensure commands is valid during composition
        let commands = unsafe { &mut *ctx.commands };
        f(commands)
    })
}

// ============================================================================
// State Owner Management (generational-box)
// ============================================================================

/// Global owner that manages the lifetime of State values created outside any scope.
/// States created at the app level (before any composable runs) use this owner.
static GLOBAL_STATE_OWNER: RwLock<Option<Owner<SyncStorage>>> = RwLock::new(None);

/// Get the global state owner for app-level states
fn get_global_owner() -> Owner<SyncStorage> {
    {
        let guard = GLOBAL_STATE_OWNER.read().unwrap();
        if let Some(ref owner) = *guard {
            return owner.clone();
        }
    }
    let mut guard = GLOBAL_STATE_OWNER.write().unwrap();
    if guard.is_none() {
        *guard = Some(SyncStorage::owner());
    }
    guard.as_ref().unwrap().clone()
}

/// Create a new State value.
/// - If called inside a composable scope, the state is tied to that scope's lifetime
/// - If called outside any scope (app level), the state lives for the app's lifetime
fn create_state_box<T: Send + Sync + 'static>(value: T) -> GenerationalBox<T, SyncStorage> {
    // Check if we're inside a composition scope
    let scope_id = current_scope_id();

    let owner = if let Some(scope_id) = scope_id {
        // Use the scope's owner - state will be freed when scope is destroyed
        get_or_create_scope_owner(scope_id)
    } else {
        // No scope - use global owner (app-level state)
        get_global_owner()
    };

    owner.insert(value)
}

// ============================================================================
// Reactive State
// ============================================================================

/// Inner state data holding value and subscribers
struct StateInner<T> {
    value: T,
    /// Scopes that have read from this state
    subscribers: HashSet<ScopeId>,
}

/// Reactive state that automatically triggers recomposition when modified.
/// Similar to MutableState in Jetpack Compose.
///
/// State is `Copy` thanks to generational-box, so you don't need to clone it!
/// Just pass it around freely.
///
/// ## Lifetime Management
/// - **Inside a composable**: State is tied to the composable's scope. When the scope
///   is destroyed (e.g., during recomposition), all states created within it are freed.
/// - **Outside composables (app level)**: State lives for the entire application lifetime.
///
/// ## Subscription & Recomposition
/// State tracks which composition scope(s) read from it and only marks those
/// scopes as dirty when the value changes, enabling granular recomposition.
///
/// # Example
/// ```ignore
/// // App-level state (lives forever)
/// let count = State::new(0);
///
/// run_app("My App", move || {
///     // No need for count.clone()! State is Copy!
///     Button("Increment", move || count.set(count.get() + 1));
///     Text(format!("Count: {}", count.get()));
/// });
/// ```
pub struct State<T: 'static> {
    inner: GenerationalBox<RwLock<StateInner<T>>, SyncStorage>,
}

// Manually implement Copy and Clone since GenerationalBox is Copy
// regardless of T, and we want State to be Copy for any T
impl<T: 'static> Copy for State<T> {}

impl<T: 'static> Clone for State<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Clone + Send + Sync + 'static> State<T> {
    /// Create a new reactive state with an initial value
    pub fn new(value: T) -> Self {
        let inner = StateInner {
            value,
            subscribers: HashSet::new(),
        };
        Self {
            inner: create_state_box(RwLock::new(inner)),
        }
    }

    /// Get the current value and subscribe the current scope
    pub fn get(&self) -> T {
        // Subscribe current scope to this state
        if let Some(scope_id) = current_scope_id() {
            if let Ok(inner_guard) = self.inner.try_read() {
                if let Ok(mut inner) = inner_guard.write() {
                    inner.subscribers.insert(scope_id);
                }
            }
        }
        self.inner
            .try_read()
            .ok()
            .and_then(|guard| guard.read().ok().map(|inner| inner.value.clone()))
            .expect("State was dropped")
    }

    /// Get the value without subscribing (useful for event handlers)
    pub fn get_untracked(&self) -> T {
        self.inner
            .try_read()
            .ok()
            .and_then(|guard| guard.read().ok().map(|inner| inner.value.clone()))
            .expect("State was dropped")
    }

    /// Set a new value and trigger recomposition of subscribed scopes
    pub fn set(&self, value: T) {
        let subscribers = {
            let inner_guard = self.inner.try_read().expect("State was dropped");
            let mut inner = inner_guard.write().unwrap();
            inner.value = value;
            inner.subscribers.clone()
        };
        Self::notify_subscribers_static(&subscribers);
    }

    /// Update the value using a function and trigger recomposition
    pub fn update(&self, f: impl FnOnce(&mut T)) {
        let subscribers = {
            let inner_guard = self.inner.try_read().expect("State was dropped");
            let mut inner = inner_guard.write().unwrap();
            f(&mut inner.value);
            inner.subscribers.clone()
        };
        Self::notify_subscribers_static(&subscribers);
    }

    /// Modify without triggering recomposition (for batched updates)
    pub fn set_silent(&self, value: T) {
        if let Ok(inner_guard) = self.inner.try_read() {
            if let Ok(mut inner) = inner_guard.write() {
                inner.value = value;
            }
        }
    }

    /// Notify all subscribed scopes that this state changed
    fn notify_subscribers_static(subscribers: &HashSet<ScopeId>) {
        if subscribers.is_empty() {
            // No subscribers, fall back to global invalidation
            mark_scope_dirty(ScopeId(0));
        } else {
            for scope_id in subscribers.iter() {
                mark_scope_dirty(*scope_id);
            }
        }
    }

    /// Clear subscriber list (called during recomposition)
    pub fn clear_subscribers(&self) {
        if let Ok(inner_guard) = self.inner.try_read() {
            if let Ok(mut inner) = inner_guard.write() {
                inner.subscribers.clear();
            }
        }
    }
}

impl<T: Clone + Send + Sync + Default + 'static> Default for State<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

// Convenience for numeric types
impl State<i32> {
    pub fn increment(&self) {
        self.update(|v| *v += 1);
    }

    pub fn decrement(&self) {
        self.update(|v| *v -= 1);
    }
}

// ============================================================================
// Core Composable Functions
// ============================================================================

/// Helper to create an implicit scope for a composable.
/// Every composable automatically becomes a recomposition boundary.
pub fn with_implicit_scope<F, R>(content: F) -> R
where
    F: FnOnce() -> R,
{
    let scope_id = ScopeId::new();
    let parent_scope = current_scope_id();

    // Enter this scope
    enter_scope(scope_id);

    // Execute content
    let result = content();

    // Exit scope
    exit_scope();

    // Note: We don't register the scope here since leaf composables
    // don't have stored content functions. For granular updates,
    // the parent scope will be marked dirty instead.
    let _ = parent_scope; // silence unused warning

    result
}

/// Helper to create a scoped container composable with stored content function.
/// This enables granular recomposition - only this subtree rebuilds when its state changes.
fn scoped_container<F>(container_entity: Entity, content: F)
where
    F: Fn() + Send + Sync + 'static,
{
    let scope_id = ScopeId::new();
    let parent_scope = current_scope_id();

    // Add scope marker to the container
    COMPOSITION_CTX.with(|ctx| {
        let ctx = ctx.borrow();
        let commands = unsafe { &mut *ctx.commands };
        commands
            .entity(container_entity)
            .insert(ScopeMarker(scope_id));
    });

    // Register the scope with its content function for later recomposition
    let content_fn: ScopedContentFn = Arc::new(content);
    register_scope(scope_id, content_fn.clone(), parent_scope);
    set_scope_root_entity(scope_id, container_entity);

    // Enter scope and compose content
    push_parent(container_entity);
    enter_scope(scope_id);

    content_fn();

    exit_scope();
    pop_parent();
}

// Removed unstyled `Text` composable. Use the styled `Text(content, style: TextStyle)` instead.

/// Text composable with styling
///
/// Text is automatically scoped - state reads inside will only trigger
/// recomposition of this text element's parent scope.
///
/// # Example
/// ```ignore
/// Text("Hello!", TextStyle::title().with_color(Color::WHITE));
/// ```
pub fn Text(content: impl Into<String>, style: TextStyle) {
    with_implicit_scope(|| {
        let content = content.into();
        spawn_child((
            bevy::prelude::Text::new(content),
            TextFont {
                font_size: style.font_size,
                ..default()
            },
            TextColor(style.color),
        ));
    });
}

// Removed unstyled `Button`. Use the styled `Button(label, modifier, on_click)` with a `ModifierChain` instead.

/// Button composable with modifier
///
/// Button is automatically scoped - changes to state it reads only rebuild this button.
///
/// # Example
/// ```ignore
/// Button("Submit", Modifier().background(Color::BLUE), || submit());
/// ```
pub fn Button<F>(label: impl Into<String>, modifier: Modifiers, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    with_implicit_scope(|| {
        let label = label.into();
        let on_click = Arc::new(on_click);

        let mut node = Node {
            padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };
        modifier.apply_to_node(&mut node);

        let mut bg = BackgroundColor(Color::srgb(0.25, 0.25, 0.3));
        modifier.apply_to_background(&mut bg);

        let button = spawn_child((
            bevy::prelude::Button,
            node,
            bg,
            BorderRadius::all(Val::Px(4.0)),
            crate::components::Clickable { on_click },
        ));

        push_parent(button);
        // Use body style by default for button labels - no scope needed, it's inside button's scope
        let label_content = label;
        spawn_child((
            bevy::prelude::Text::new(label_content),
            TextFont {
                font_size: TextStyle::body().font_size,
                ..default()
            },
            TextColor(TextStyle::body().color),
        ));
        pop_parent();
    });
}

/// Spacer composable - flexible space that expands
///
/// # Example
/// ```ignore
/// Column(|| {
///     Text("Top");
///     Spacer();
///     Text("Bottom");
/// });
/// ```
pub fn Spacer() {
    spawn_child((Node {
        flex_grow: 1.0,
        ..default()
    },));
}

/// Fixed-size spacer
pub fn FixedSpacer(size: f32) {
    spawn_child((Node {
        width: Val::Px(size),
        height: Val::Px(size),
        ..default()
    },));
}

// ============================================================================
// Layout Composables
// ============================================================================

// Column now relies on `ModifierChain` for visual/layout properties

/// Column layout composable with automatic scoping.
///
/// Column is automatically a recomposition boundary - state reads inside
/// only trigger rebuilds of this column's subtree.
///
/// # Example
/// ```ignore
/// Column(
///     Modifiers::new()
///         .padding(16.0)
///         .vertical_arrangement(VerticalArrangement::Center)
///         .horizontal_alignment(HorizontalAlignment::Center)
///         .row_gap(16.0),
///     || {
///         Text("Centered content", TextStyle::body());
///     }
/// );
/// ```
pub fn Column<F>(modifier: Modifiers, content: F)
where
    F: Fn() + Send + Sync + 'static,
{
    let mut node = Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        ..default()
    };
    modifier.apply_to_node(&mut node);

    let mut bg = BackgroundColor(Color::NONE);
    modifier.apply_to_background(&mut bg);

    let column = spawn_child((node, bg));

    scoped_container(column, content);
}

// Row now relies on `ModifierChain` for visual/layout properties

/// Row layout composable with automatic scoping.
///
/// Row is automatically a recomposition boundary.
pub fn Row<F>(modifier: Modifiers, content: F)
where
    F: Fn() + Send + Sync + 'static,
{
    let mut node = Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Row,
        ..default()
    };
    modifier.apply_to_node(&mut node);

    let mut bg = BackgroundColor(Color::NONE);
    modifier.apply_to_background(&mut bg);

    let row = spawn_child((node, bg));

    scoped_container(row, content);
}

// Removed unstyled `Box`. Use the styled `Box(modifier, content)` instead.

/// Box layout composable with automatic scoping.
///
/// Box is automatically a recomposition boundary.
pub fn Box<F>(modifier: Modifiers, content: F)
where
    F: Fn() + Send + Sync + 'static,
{
    let mut node = Node {
        display: Display::Flex,
        ..default()
    };
    modifier.apply_to_node(&mut node);

    let mut bg = BackgroundColor(Color::NONE);
    modifier.apply_to_background(&mut bg);

    let box_node = spawn_child((node, bg));

    scoped_container(box_node, content);
}

// ============================================================================
// Root Composable
// ============================================================================

// Removed unstyled `Surface`. Use the styled `Surface(modifier, content)` instead.

/// Surface composable with automatic scoping.
///
/// Surface is typically the root of your UI and is automatically a recomposition boundary.
pub fn Surface<F>(modifier: Modifiers, content: F)
where
    F: Fn() + Send + Sync + 'static,
{
    let mut node = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        ..default()
    };
    modifier.apply_to_node(&mut node);

    let mut bg = BackgroundColor(Color::NONE);
    modifier.apply_to_background(&mut bg);

    let surface = spawn_child((node, bg));

    scoped_container(surface, content);
}

// ============================================================================
// List Composables
// ============================================================================

/// Iterates over items and composes content for each.
///
/// Each item gets its own implicit scope for granular updates.
///
/// # Example
/// ```ignore
/// let names = vec!["Alice", "Bob", "Charlie"];
/// Column(Modifiers::new(), || {
///     ForEach(&names, |name| {
///         Text(format!("Hello, {}!", name), TextStyle::body());
///     });
/// });
/// ```
pub fn ForEach<T, F>(items: &[T], content: F)
where
    F: Fn(&T),
{
    for item in items {
        with_implicit_scope(|| {
            content(item);
        });
    }
}

/// Conditional composition with automatic scoping.
///
/// # Example
/// ```ignore
/// If(show_greeting, || {
///     Text("Hello!", TextStyle::body());
/// });
/// ```
pub fn If<F>(condition: bool, content: F)
where
    F: FnOnce(),
{
    if condition {
        with_implicit_scope(|| {
            content();
        });
    }
}

/// Conditional composition with else branch and automatic scoping.
///
/// # Example
/// ```ignore
/// IfElse(is_logged_in,
///     || Text("Welcome back!", TextStyle::body()),
///     || Text("Please log in", TextStyle::body()),
/// );
/// ```
pub fn IfElse<F1, F2>(condition: bool, if_true: F1, if_false: F2)
where
    F1: FnOnce(),
    F2: FnOnce(),
{
    with_implicit_scope(|| {
        if condition {
            if_true();
        } else {
            if_false();
        }
    });
}

// ============================================================================
// Convenience Modifier Functions
// ============================================================================

/// Create a modifier chain starting with padding
pub fn Modifier() -> Modifiers {
    Modifiers::new()
}

// ============================================================================
// Scoped Composition
// ============================================================================

/// Marker component for scope root entities
#[derive(Component, Clone, Copy)]
pub struct ScopeMarker(pub ScopeId);

/// Explicit scope boundary (for backward compatibility).
///
/// Note: Since all composables (Column, Row, Box, Surface, etc.) are now
/// automatically scoped, you typically don't need to use this directly.
/// Use this only when you want an explicit scope without a container.
///
/// # Example
/// ```ignore
/// Column(Modifiers::new(), || {
///     // Explicit scope - usually not needed since Column is already scoped
///     Scope(|| {
///         let count = counter.get();
///         Text(format!("Count: {}", count), TextStyle::body());
///     });
/// });
/// ```
pub fn Scope<F>(content: F)
where
    F: Fn() + Send + Sync + 'static,
{
    let scope_id = ScopeId::new();
    let parent_scope = current_scope_id();

    // Wrap content in Arc for storage
    let content_fn: ScopedContentFn = Arc::new(content);

    // Register this scope with its content function
    register_scope(scope_id, content_fn.clone(), parent_scope);

    // Create a minimal container node for this scope
    let scope_container = spawn_child((
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ScopeMarker(scope_id),
    ));

    // Update scope registry with root entity
    set_scope_root_entity(scope_id, scope_container);

    // Enter this scope and compose content
    push_parent(scope_container);
    enter_scope(scope_id);

    content_fn();

    exit_scope();
    pop_parent();
}

/// Scoped state wrapper (legacy - prefer using State directly in any composable).
///
/// Since all composables are now automatically scoped, you can simply use
/// State<T> directly and it will automatically subscribe to the current scope.
#[deprecated(note = "All composables are now automatically scoped. Use State<T> directly.")]
pub fn ScopedState<T, F>(initial: T, content: F)
where
    T: Clone + Send + Sync + 'static,
    F: Fn(State<T>) + Send + Sync + 'static,
{
    let state = State::new(initial);

    Scope(move || {
        // State<T> is Copy, no need to clone
        content(state);
    });
}
