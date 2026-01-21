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
use std::cell::RefCell;
use std::sync::Arc;

use crate::modifier::ModifierChain;
use crate::components::TextStyle;
use crate::layout::{VerticalArrangement, HorizontalArrangement, HorizontalAlignment, VerticalAlignment};

pub use super::app::CompositionRoot;
pub use super::app::invalidate;

// ============================================================================
// Thread-Local Composition Context
// ============================================================================

/// Internal composition context stored in thread-local
struct CompositionContext {
    parent_stack: Vec<Entity>,
    commands: *mut Commands<'static, 'static>,
}

impl CompositionContext {
    fn new() -> Self {
        Self {
            parent_stack: Vec::new(),
            commands: std::ptr::null_mut(),
        }
    }
}

thread_local! {
    static COMPOSITION_CTX: RefCell<CompositionContext> = RefCell::new(CompositionContext::new());
}

/// Initialize the composition context for this frame
/// Called by the framework - users don't need to call this
pub fn begin_composition(commands: &mut Commands) {
    COMPOSITION_CTX.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        ctx.parent_stack.clear();
        // SAFETY: We ensure this pointer is only valid during composition
        ctx.commands = commands as *mut Commands as *mut Commands<'static, 'static>;
    });
}

/// End the composition context for this frame
pub fn end_composition() {
    COMPOSITION_CTX.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        ctx.parent_stack.clear();
        ctx.commands = std::ptr::null_mut();
    });
}

/// Push a new parent onto the stack
fn push_parent(entity: Entity) {
    COMPOSITION_CTX.with(|ctx| {
        ctx.borrow_mut().parent_stack.push(entity);
    });
}

/// Pop the current parent from the stack
fn pop_parent() {
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
// Reactive State
// ============================================================================

/// Reactive state that automatically triggers recomposition when modified.
/// Similar to MutableState in Jetpack Compose.
/// 
/// # Example
/// ```ignore
/// let count = State::new(0);
/// 
/// Button("Increment", {
///     let count = count.clone();
///     move || count.set(count.get() + 1)
/// });
/// 
/// Text(format!("Count: {}", count.get()));
/// ```
#[derive(Clone)]
pub struct State<T: Clone + Send + Sync + 'static> {
    inner: Arc<std::sync::RwLock<T>>,
}

impl<T: Clone + Send + Sync + 'static> State<T> {
    /// Create a new reactive state with an initial value
    pub fn new(value: T) -> Self {
        Self {
            inner: Arc::new(std::sync::RwLock::new(value)),
        }
    }
    
    /// Get the current value
    pub fn get(&self) -> T {
        self.inner.read().unwrap().clone()
    }
    
    /// Set a new value and trigger recomposition
    pub fn set(&self, value: T) {
        *self.inner.write().unwrap() = value;
        invalidate();
    }
    
    /// Update the value using a function and trigger recomposition
    pub fn update(&self, f: impl FnOnce(&mut T)) {
        {
            let mut guard = self.inner.write().unwrap();
            f(&mut *guard);
        }
        invalidate();
    }
    
    /// Modify without triggering recomposition (for batched updates)
    pub fn set_silent(&self, value: T) {
        *self.inner.write().unwrap() = value;
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

/// Text composable - displays text
/// 
/// # Example
/// ```ignore
/// Text("Hello, World!");
/// ```
pub fn Text(content: impl Into<String>) {
    let content = content.into();
    spawn_child((
        bevy::prelude::Text::new(content),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::WHITE),
    ));
}

/// Styled text composable
/// 
/// # Example
/// ```ignore
/// StyledText("Hello!", TextStyle::title().with_color(Color::WHITE));
/// ```
pub fn StyledText(content: impl Into<String>, style: TextStyle) {
    let content = content.into();
    spawn_child((
        bevy::prelude::Text::new(content),
        TextFont {
            font_size: style.font_size,
            ..default()
        },
        TextColor(style.color),
    ));
}

/// Button composable - a clickable button with text
/// 
/// # Example
/// ```ignore
/// Button("Click me", || println!("Clicked!"));
/// ```
pub fn Button<F>(label: impl Into<String>, on_click: F)
where
    F: Fn() + Send + Sync + 'static,
{
    let label = label.into();
    let on_click = Arc::new(on_click);
    
    let button = spawn_child((
        bevy::prelude::Button,
        Node {
            padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.25, 0.25, 0.3)),
        BorderRadius::all(Val::Px(4.0)),
        crate::components::Clickable { on_click },
    ));
    
    push_parent(button);
    Text(label);
    pop_parent();
}

/// Styled button composable
/// 
/// # Example
/// ```ignore
/// StyledButton("Submit", Modifier::background(Color::BLUE), || submit());
/// ```
pub fn StyledButton<F>(
    label: impl Into<String>,
    modifier: ModifierChain,
    on_click: F,
)
where
    F: Fn() + Send + Sync + 'static,
{
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
    Text(label);
    pop_parent();
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
    spawn_child((
        Node {
            flex_grow: 1.0,
            ..default()
        },
    ));
}

/// Fixed-size spacer
pub fn FixedSpacer(size: f32) {
    spawn_child((
        Node {
            width: Val::Px(size),
            height: Val::Px(size),
            ..default()
        },
    ));
}

// ============================================================================
// Layout Composables
// ============================================================================

/// Column layout composable - arranges children vertically
/// 
/// # Example
/// ```ignore
/// Column(|| {
///     Text("First");
///     Text("Second");
///     Text("Third");
/// });
/// ```
pub fn Column<F>(content: F)
where
    F: FnOnce(),
{
    let column = spawn_child((
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::NONE),
    ));
    
    push_parent(column);
    content();
    pop_parent();
}

/// Styled column with modifier and layout options
/// 
/// # Example
/// ```ignore
/// StyledColumn(
///     Modifier::padding(16.0).background(Color::BLACK),
///     VerticalArrangement::Center,
///     HorizontalAlignment::Center,
///     16.0, // spacing
///     || {
///         Text("Centered content");
///     }
/// );
/// ```
pub fn StyledColumn<F>(
    modifier: ModifierChain,
    arrangement: VerticalArrangement,
    alignment: HorizontalAlignment,
    spacing: f32,
    content: F,
)
where
    F: FnOnce(),
{
    let mut node = Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        justify_content: arrangement.to_justify_content(),
        align_items: alignment.to_align_items(),
        row_gap: Val::Px(spacing),
        ..default()
    };
    modifier.apply_to_node(&mut node);
    
    let mut bg = BackgroundColor(Color::NONE);
    modifier.apply_to_background(&mut bg);
    
    let column = spawn_child((node, bg));
    
    push_parent(column);
    content();
    pop_parent();
}

/// Row layout composable - arranges children horizontally
/// 
/// # Example
/// ```ignore
/// Row(|| {
///     Text("Left");
///     Spacer();
///     Text("Right");
/// });
/// ```
pub fn Row<F>(content: F)
where
    F: FnOnce(),
{
    let row = spawn_child((
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        BackgroundColor(Color::NONE),
    ));
    
    push_parent(row);
    content();
    pop_parent();
}

/// Styled row with modifier and layout options
pub fn StyledRow<F>(
    modifier: ModifierChain,
    arrangement: HorizontalArrangement,
    alignment: VerticalAlignment,
    spacing: f32,
    content: F,
)
where
    F: FnOnce(),
{
    let mut node = Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Row,
        justify_content: arrangement.to_justify_content(),
        align_items: alignment.to_align_items(),
        column_gap: Val::Px(spacing),
        ..default()
    };
    modifier.apply_to_node(&mut node);
    
    let mut bg = BackgroundColor(Color::NONE);
    modifier.apply_to_background(&mut bg);
    
    let row = spawn_child((node, bg));
    
    push_parent(row);
    content();
    pop_parent();
}

/// Box layout composable - stacks children on top of each other
/// 
/// # Example
/// ```ignore
/// Box(|| {
///     Image("background.png");
///     Text("Overlay text");
/// });
/// ```
pub fn Box<F>(content: F)
where
    F: FnOnce(),
{
    let box_node = spawn_child((
        Node {
            display: Display::Flex,
            ..default()
        },
        BackgroundColor(Color::NONE),
    ));
    
    push_parent(box_node);
    content();
    pop_parent();
}

/// Styled box with modifier
pub fn StyledBox<F>(
    modifier: ModifierChain,
    content: F,
)
where
    F: FnOnce(),
{
    let mut node = Node {
        display: Display::Flex,
        ..default()
    };
    modifier.apply_to_node(&mut node);
    
    let mut bg = BackgroundColor(Color::NONE);
    modifier.apply_to_background(&mut bg);
    
    let box_node = spawn_child((node, bg));
    
    push_parent(box_node);
    content();
    pop_parent();
}

// ============================================================================
// Root Composable
// ============================================================================

/// Surface composable - a root container that fills the screen
/// Use this as the top-level composable for your app.
/// 
/// # Example
/// ```ignore
/// Surface(Color::srgb(0.1, 0.1, 0.15), || {
///     Column(|| {
///         Text("Hello, World!");
///     });
/// });
/// ```
pub fn Surface<F>(background: Color, content: F)
where
    F: FnOnce(),
{
    let surface = spawn_child((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(background),
    ));
    
    push_parent(surface);
    content();
    pop_parent();
}

/// Styled surface with full modifier support
pub fn StyledSurface<F>(
    modifier: ModifierChain,
    content: F,
)
where
    F: FnOnce(),
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
    
    push_parent(surface);
    content();
    pop_parent();
}

// ============================================================================
// List Composables
// ============================================================================

/// Iterates over items and composes content for each
/// 
/// # Example
/// ```ignore
/// let names = vec!["Alice", "Bob", "Charlie"];
/// Column(|| {
///     ForEach(&names, |name| {
///         Text(format!("Hello, {}!", name));
///     });
/// });
/// ```
pub fn ForEach<T, F>(items: &[T], content: F)
where
    F: Fn(&T),
{
    for item in items {
        content(item);
    }
}

/// Conditional composition
/// 
/// # Example
/// ```ignore
/// If(show_greeting, || {
///     Text("Hello!");
/// });
/// ```
pub fn If<F>(condition: bool, content: F)
where
    F: FnOnce(),
{
    if condition {
        content();
    }
}

/// Conditional composition with else branch
/// 
/// # Example
/// ```ignore
/// IfElse(is_logged_in,
///     || Text("Welcome back!"),
///     || Text("Please log in"),
/// );
/// ```
pub fn IfElse<F1, F2>(condition: bool, if_true: F1, if_false: F2)
where
    F1: FnOnce(),
    F2: FnOnce(),
{
    if condition {
        if_true();
    } else {
        if_false();
    }
}

// ============================================================================
// Convenience Modifier Functions
// ============================================================================

/// Create a modifier chain starting with padding
pub fn Modifier() -> ModifierChain {
    ModifierChain::new()
}
