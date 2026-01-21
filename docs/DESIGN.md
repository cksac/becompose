# BECOMPOSE - Technical Design Document

## 1. Overview

This document outlines the technical architecture and design decisions for BECOMPOSE, a declarative UI framework that brings Jetpack Compose-style APIs to the Bevy game engine.

---

## 2. Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                        User Application                          │
├─────────────────────────────────────────────────────────────────┤
│                      Composable Functions                        │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────────────┐ │
│  │   Text   │  │  Button  │  │   Row    │  │ Custom Composable│ │
│  └──────────┘  └──────────┘  └──────────┘  └──────────────────┘ │
├─────────────────────────────────────────────────────────────────┤
│                       BECOMPOSE Core                             │
│  ┌────────────────┐  ┌─────────────────┐  ┌──────────────────┐  │
│  │ Composition    │  │ State           │  │ Modifier         │  │
│  │ Runtime        │  │ Management      │  │ System           │  │
│  └────────────────┘  └─────────────────┘  └──────────────────┘  │
│  ┌────────────────┐  ┌─────────────────┐  ┌──────────────────┐  │
│  │ Layout         │  │ Theming         │  │ Animation        │  │
│  │ Engine         │  │ Engine          │  │ System           │  │
│  └────────────────┘  └─────────────────┘  └──────────────────┘  │
├─────────────────────────────────────────────────────────────────┤
│                     Bevy Integration Layer                       │
│  ┌────────────────┐  ┌─────────────────┐  ┌──────────────────┐  │
│  │ Entity/Component│ │ Rendering       │  │ Input            │  │
│  │ Bridge         │  │ Bridge          │  │ Bridge           │  │
│  └────────────────┘  └─────────────────┘  └──────────────────┘  │
├─────────────────────────────────────────────────────────────────┤
│                         Bevy Engine                              │
│  ┌────────────────┐  ┌─────────────────┐  ┌──────────────────┐  │
│  │ ECS World      │  │ Renderer        │  │ Input System     │  │
│  └────────────────┘  └─────────────────┘  └──────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 3. Core Components

### 3.1 Composition Runtime

The Composition Runtime is the heart of BECOMPOSE, responsible for managing the composable tree, tracking state changes, and triggering recomposition.

#### 3.1.1 Composition Tree

```rust
/// Represents a node in the composition tree
pub struct CompositionNode {
    /// Unique identifier for this node
    pub id: CompositionId,
    /// The composable type that created this node
    pub composable_type: TypeId,
    /// Key for list reconciliation
    pub key: Option<CompositionKey>,
    /// Parent node reference
    pub parent: Option<CompositionId>,
    /// Child nodes
    pub children: Vec<CompositionId>,
    /// Associated Bevy entity (if materialized)
    pub entity: Option<Entity>,
    /// State slots for this composition
    pub state_slots: Vec<StateSlot>,
    /// Applied modifiers
    pub modifiers: ModifierChain,
}

/// The composition tree manager
pub struct CompositionTree {
    nodes: HashMap<CompositionId, CompositionNode>,
    root: Option<CompositionId>,
    pending_recomposition: HashSet<CompositionId>,
}
```

#### 3.1.2 Recomposition Strategy

```
┌─────────────────────────────────────────────────────────────┐
│                    Recomposition Flow                        │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  State Change Detected                                       │
│         │                                                    │
│         ▼                                                    │
│  ┌─────────────────┐                                        │
│  │ Mark Dirty      │  (Mark affected composables)           │
│  └────────┬────────┘                                        │
│           │                                                  │
│           ▼                                                  │
│  ┌─────────────────┐                                        │
│  │ Batch Changes   │  (Collect all changes in frame)        │
│  └────────┬────────┘                                        │
│           │                                                  │
│           ▼                                                  │
│  ┌─────────────────┐                                        │
│  │ Sort by Depth   │  (Process parents before children)     │
│  └────────┬────────┘                                        │
│           │                                                  │
│           ▼                                                  │
│  ┌─────────────────┐                                        │
│  │ Re-execute      │  (Run composable functions)            │
│  └────────┬────────┘                                        │
│           │                                                  │
│           ▼                                                  │
│  ┌─────────────────┐                                        │
│  │ Reconcile Tree  │  (Diff old vs new children)            │
│  └────────┬────────┘                                        │
│           │                                                  │
│           ▼                                                  │
│  ┌─────────────────┐                                        │
│  │ Update Entities │  (Sync with Bevy ECS)                  │
│  └─────────────────┘                                        │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### 3.2 State Management

#### 3.2.1 State Types

```rust
/// Mutable state holder with change tracking
pub struct MutableState<T> {
    value: T,
    version: u64,
    subscribers: Vec<CompositionId>,
}

impl<T: Clone + PartialEq> MutableState<T> {
    pub fn get(&self) -> T {
        self.value.clone()
    }
    
    pub fn set(&mut self, new_value: T) {
        if self.value != new_value {
            self.value = new_value;
            self.version += 1;
            self.notify_subscribers();
        }
    }
}

/// State that is remembered across recompositions
pub struct RememberedState<T> {
    slot_index: usize,
    _phantom: PhantomData<T>,
}
```

#### 3.2.2 State Slot Management

```rust
/// Manages state slots for a composable
pub struct StateSlotManager {
    slots: Vec<Box<dyn Any>>,
    current_index: usize,
}

impl StateSlotManager {
    /// Called during composition to get or create a state slot
    pub fn remember<T, F>(&mut self, init: F) -> &mut T 
    where 
        T: 'static,
        F: FnOnce() -> T,
    {
        let index = self.current_index;
        self.current_index += 1;
        
        if index >= self.slots.len() {
            // First composition - create the slot
            self.slots.push(Box::new(init()));
        }
        
        self.slots[index].downcast_mut::<T>().unwrap()
    }
    
    /// Reset index at start of recomposition
    pub fn reset(&mut self) {
        self.current_index = 0;
    }
}
```

### 3.3 Modifier System

#### 3.3.1 Modifier Chain Architecture

```rust
/// Base trait for all modifiers
pub trait Modifier: Send + Sync + 'static {
    /// Apply this modifier to a UI node
    fn apply(&self, node: &mut UiNode, context: &LayoutContext);
    
    /// Get the modifier type for conflict resolution
    fn modifier_type(&self) -> ModifierType;
}

/// Chain of modifiers applied to a composable
#[derive(Default, Clone)]
pub struct ModifierChain {
    modifiers: Vec<Arc<dyn Modifier>>,
}

impl ModifierChain {
    pub fn then<M: Modifier>(mut self, modifier: M) -> Self {
        self.modifiers.push(Arc::new(modifier));
        self
    }
    
    /// Convenience methods
    pub fn padding(self, all: f32) -> Self {
        self.then(PaddingModifier::all(all))
    }
    
    pub fn size(self, width: f32, height: f32) -> Self {
        self.then(SizeModifier::new(width, height))
    }
    
    pub fn background(self, color: Color) -> Self {
        self.then(BackgroundModifier::new(color))
    }
    
    pub fn clickable<F: Fn() + Send + Sync + 'static>(self, on_click: F) -> Self {
        self.then(ClickableModifier::new(on_click))
    }
}

/// Public Modifier constructor
pub struct Modifier;

impl Modifier {
    pub fn padding(all: f32) -> ModifierChain {
        ModifierChain::default().padding(all)
    }
    
    pub fn fill_max_width() -> ModifierChain {
        ModifierChain::default().then(FillMaxWidthModifier)
    }
    
    // ... more convenience constructors
}
```

#### 3.3.2 Modifier Types

```rust
/// Categories of modifiers for ordering and conflict resolution
pub enum ModifierType {
    Layout,      // padding, size, fill
    Drawing,     // background, border
    Pointer,     // clickable, draggable  
    Semantics,   // accessibility
    Transform,   // rotate, scale
}

/// Specific modifier implementations
pub struct PaddingModifier {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

pub struct SizeModifier {
    pub width: Option<Dimension>,
    pub height: Option<Dimension>,
}

pub struct BackgroundModifier {
    pub color: Color,
    pub shape: Option<Shape>,
}

pub struct ClickableModifier {
    pub on_click: Arc<dyn Fn() + Send + Sync>,
    pub on_long_press: Option<Arc<dyn Fn() + Send + Sync>>,
}
```

### 3.4 Layout Engine

#### 3.4.1 Layout Model

BECOMPOSE uses a constraint-based layout system inspired by Compose's intrinsic measurements.

```rust
/// Constraints passed down during layout
#[derive(Clone, Copy)]
pub struct Constraints {
    pub min_width: f32,
    pub max_width: f32,
    pub min_height: f32,
    pub max_height: f32,
}

impl Constraints {
    pub fn unbounded() -> Self {
        Self {
            min_width: 0.0,
            max_width: f32::INFINITY,
            min_height: 0.0,
            max_height: f32::INFINITY,
        }
    }
    
    pub fn fixed(width: f32, height: f32) -> Self {
        Self {
            min_width: width,
            max_width: width,
            min_height: height,
            max_height: height,
        }
    }
}

/// Result of measuring a composable
pub struct MeasureResult {
    pub width: f32,
    pub height: f32,
}

/// Layout trait for composables
pub trait Layout {
    fn measure(&self, constraints: Constraints, children: &[&dyn Measurable]) -> MeasureResult;
    fn place(&self, size: MeasureResult, children: &mut [&mut dyn Placeable]);
}
```

#### 3.4.2 Standard Layouts

```rust
/// Vertical arrangement
pub struct ColumnLayout {
    pub vertical_arrangement: Arrangement,
    pub horizontal_alignment: Alignment,
}

impl Layout for ColumnLayout {
    fn measure(&self, constraints: Constraints, children: &[&dyn Measurable]) -> MeasureResult {
        let mut total_height = 0.0;
        let mut max_width = 0.0;
        
        for child in children {
            let child_constraints = Constraints {
                min_width: 0.0,
                max_width: constraints.max_width,
                min_height: 0.0,
                max_height: constraints.max_height - total_height,
            };
            
            let result = child.measure(child_constraints);
            total_height += result.height;
            max_width = max_width.max(result.width);
        }
        
        MeasureResult {
            width: max_width.clamp(constraints.min_width, constraints.max_width),
            height: total_height.clamp(constraints.min_height, constraints.max_height),
        }
    }
    
    fn place(&self, size: MeasureResult, children: &mut [&mut dyn Placeable]) {
        let mut y = self.vertical_arrangement.start_offset(size.height, children);
        
        for child in children {
            let x = self.horizontal_alignment.align(child.width(), size.width);
            child.place(x, y);
            y += child.height() + self.vertical_arrangement.spacing;
        }
    }
}

/// Horizontal arrangement
pub struct RowLayout {
    pub horizontal_arrangement: Arrangement,
    pub vertical_alignment: Alignment,
}

/// Stack/overlay arrangement
pub struct BoxLayout {
    pub content_alignment: Alignment2D,
}
```

### 3.5 Bevy Integration Layer

#### 3.5.1 Plugin Architecture

```rust
/// Main plugin for BECOMPOSE
pub struct BecomposePlugin;

impl Plugin for BecomposePlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<CompositionTree>()
            .init_resource::<ThemeProvider>()
            .init_resource::<FocusManager>()
            
            // Systems
            .add_systems(PreUpdate, (
                process_input_events,
                update_focus_state,
            ))
            .add_systems(Update, (
                process_state_changes,
                run_recomposition,
                sync_composition_to_entities,
            ).chain())
            .add_systems(PostUpdate, (
                run_layout_pass,
                apply_modifiers,
                update_render_data,
            ).chain());
    }
}
```

#### 3.5.2 Entity Bridge

```rust
/// Component marking a Bevy entity as a BECOMPOSE node
#[derive(Component)]
pub struct CompositionBridge {
    pub composition_id: CompositionId,
}

/// Syncs composition tree changes to Bevy entities
pub fn sync_composition_to_entities(
    mut commands: Commands,
    mut tree: ResMut<CompositionTree>,
    mut query: Query<(Entity, &CompositionBridge, &mut Style)>,
    mut removed: RemovedComponents<CompositionBridge>,
) {
    // Handle new compositions - spawn entities
    for node_id in tree.new_nodes.drain(..) {
        let node = tree.get(node_id);
        let entity = commands.spawn((
            CompositionBridge { composition_id: node_id },
            NodeBundle::default(),
        )).id();
        tree.set_entity(node_id, entity);
    }
    
    // Handle updates - sync properties
    for (entity, bridge, mut style) in query.iter_mut() {
        if let Some(node) = tree.get(bridge.composition_id) {
            if node.is_dirty() {
                apply_layout_to_style(node, &mut style);
            }
        }
    }
    
    // Handle removals - despawn entities
    for node_id in tree.removed_nodes.drain(..) {
        if let Some(entity) = tree.get_entity(node_id) {
            commands.entity(entity).despawn_recursive();
        }
    }
}
```

---

## 4. Composable Function Design

### 4.1 The `#[composable]` macro

```rust
/// User writes:
#[composable]
fn greeting(name: &str) {
    text(format!("Hello, {}!", name))
}

/// Macro expands to:
fn greeting(name: &str) {
    let __composition_context = CompositionContext::current();
    let __node_id = __composition_context.start_group(
        std::any::TypeId::of::<fn(&str)>(), // Type identity
        None, // Key
    );
    
    // User's code
    text(format!("Hello, {}!", name));
    
    __composition_context.end_group(__node_id);
}
```

### 4.2 Built-in Composables

```rust
/// Text display composable
#[composable]
pub fn text(
    text: impl Into<String>,
    #[modifier] modifier: ModifierChain = ModifierChain::default(),
    style: TextStyle = TextStyle::default(),
) {
    let context = CompositionContext::current();
    context.emit_leaf(TextNode {
        text: text.into(),
        style,
        modifier,
    });
}

/// Clickable button composable
#[composable]
pub fn button(
    on_click: impl Fn() + Send + Sync + 'static,
    #[modifier] modifier: ModifierChain = ModifierChain::default(),
    enabled: bool = true,
    #[slot] content: impl Composable,
) {
    let context = CompositionContext::current();
    
    box_layout(
        modifier
            .clickable(on_click)
            .background(if enabled { 
                context.theme().primary 
            } else { 
                context.theme().disabled 
            })
            .padding(16.0)
    ) {
        content.compose();
    }
}

/// Vertical layout composable
#[composable]
pub fn column(
    #[modifier] modifier: ModifierChain = ModifierChain::default(),
    vertical_arrangement: Arrangement = Arrangement::Top,
    horizontal_alignment: Alignment = Alignment::Start,
    #[slot] content: impl Composable,
) {
    let context = CompositionContext::current();
    context.emit_layout(
        ColumnLayout { vertical_arrangement, horizontal_alignment },
        modifier,
        || content.compose(),
    );
}
```

### 4.3 State Hooks

```rust
/// Remember a value across recompositions
pub fn remember<T, F>(init: F) -> Remembered<T>
where
    T: 'static,
    F: FnOnce() -> T,
{
    let context = CompositionContext::current();
    context.state_manager().remember(init)
}

/// Create mutable state
pub fn mutable_state_of<T>(initial: T) -> MutableState<T>
where
    T: Clone + PartialEq + 'static,
{
    remember(|| MutableState::new(initial))
}

/// Derive state from other state
pub fn derived_state_of<T, F>(calculation: F) -> DerivedState<T>
where
    T: Clone + PartialEq + 'static,
    F: Fn() -> T + 'static,
{
    remember(|| DerivedState::new(calculation))
}

/// Side effect that runs when keys change
pub fn launched_effect<K, F>(key: K, effect: F)
where
    K: PartialEq + 'static,
    F: Future<Output = ()> + Send + 'static,
{
    let context = CompositionContext::current();
    let prev_key = remember(|| None::<K>);
    
    if prev_key.as_ref() != Some(&key) {
        *prev_key = Some(key);
        context.launch_effect(effect);
    }
}
```

---

## 5. Rendering Pipeline

### 5.1 Render Flow

```
┌─────────────────────────────────────────────────────────────┐
│                      Render Pipeline                         │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  1. Composition Phase                                        │
│     ├── Execute composable functions                         │
│     ├── Build/update composition tree                        │
│     └── Identify changed subtrees                            │
│                                                              │
│  2. Layout Phase                                             │
│     ├── Measure pass (top-down constraints)                  │
│     ├── Layout pass (determine sizes)                        │
│     └── Placement pass (assign positions)                    │
│                                                              │
│  3. Modifier Application                                     │
│     ├── Apply drawing modifiers (background, border)         │
│     ├── Apply transform modifiers (rotate, scale)            │
│     └── Apply semantic modifiers (accessibility)             │
│                                                              │
│  4. Entity Sync                                              │
│     ├── Create/update Bevy entities                          │
│     ├── Update Style components                              │
│     └── Update render-specific components                    │
│                                                              │
│  5. Bevy Render (handled by Bevy)                           │
│     ├── UI extraction                                        │
│     ├── Batching                                             │
│     └── GPU rendering                                        │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### 5.2 Incremental Updates

```rust
/// Tracks what needs updating
#[derive(Default)]
pub struct DirtyFlags {
    pub needs_recomposition: HashSet<CompositionId>,
    pub needs_layout: HashSet<CompositionId>,
    pub needs_paint: HashSet<CompositionId>,
}

impl DirtyFlags {
    pub fn mark_recomposition(&mut self, id: CompositionId) {
        self.needs_recomposition.insert(id);
        // Recomposition implies layout and paint
        self.needs_layout.insert(id);
        self.needs_paint.insert(id);
    }
    
    pub fn mark_layout(&mut self, id: CompositionId) {
        self.needs_layout.insert(id);
        self.needs_paint.insert(id);
    }
}
```

---

## 6. Input Handling

### 6.1 Event Propagation

```rust
/// Input event types
pub enum UiInputEvent {
    PointerDown { position: Vec2, pointer_id: u64 },
    PointerUp { position: Vec2, pointer_id: u64 },
    PointerMove { position: Vec2, pointer_id: u64 },
    KeyDown { key: KeyCode, modifiers: Modifiers },
    KeyUp { key: KeyCode, modifiers: Modifiers },
    TextInput { text: String },
    Scroll { delta: Vec2, position: Vec2 },
}

/// Event propagation phases
pub enum PropagationPhase {
    Capture,  // Root to target
    Target,   // At target
    Bubble,   // Target to root
}

/// Process input and dispatch to composables
pub fn process_input_events(
    mut events: EventReader<UiInputEvent>,
    tree: Res<CompositionTree>,
    mut focus: ResMut<FocusManager>,
) {
    for event in events.read() {
        // Hit testing
        let targets = tree.hit_test(event.position());
        
        // Capture phase (root to target)
        for node in targets.iter().rev() {
            if let Some(handler) = node.get_capture_handler(event.type()) {
                if handler.handle(event) == Consumed {
                    break;
                }
            }
        }
        
        // Bubble phase (target to root)
        for node in targets.iter() {
            if let Some(handler) = node.get_bubble_handler(event.type()) {
                if handler.handle(event) == Consumed {
                    break;
                }
            }
        }
    }
}
```

### 6.2 Focus Management

```rust
/// Manages keyboard focus
#[derive(Resource)]
pub struct FocusManager {
    focused: Option<CompositionId>,
    focus_order: Vec<CompositionId>,
}

impl FocusManager {
    pub fn request_focus(&mut self, id: CompositionId) {
        if self.focused != Some(id) {
            if let Some(old) = self.focused {
                // Notify old focused item
            }
            self.focused = Some(id);
            // Notify new focused item
        }
    }
    
    pub fn move_focus(&mut self, direction: FocusDirection) {
        // Tab navigation logic
    }
}
```

---

## 7. Theming System

### 7.1 Theme Structure

```rust
/// Complete theme definition
#[derive(Clone)]
pub struct Theme {
    pub colors: ColorScheme,
    pub typography: Typography,
    pub shapes: Shapes,
    pub spacing: Spacing,
}

#[derive(Clone)]
pub struct ColorScheme {
    pub primary: Color,
    pub on_primary: Color,
    pub secondary: Color,
    pub on_secondary: Color,
    pub background: Color,
    pub on_background: Color,
    pub surface: Color,
    pub on_surface: Color,
    pub error: Color,
    pub on_error: Color,
}

#[derive(Clone)]
pub struct Typography {
    pub display_large: TextStyle,
    pub display_medium: TextStyle,
    pub title_large: TextStyle,
    pub title_medium: TextStyle,
    pub body_large: TextStyle,
    pub body_medium: TextStyle,
    pub label_large: TextStyle,
    pub label_medium: TextStyle,
}
```

### 7.2 Theme Provider

```rust
/// Provides theme to composition tree
#[composable]
pub fn material_theme(
    color_scheme: ColorScheme,
    typography: Typography,
    #[slot] content: impl Composable,
) {
    let theme = Theme {
        colors: color_scheme,
        typography,
        ..Default::default()
    };
    
    composition_local_provider(
        LocalTheme provides theme,
    ) {
        content.compose();
    }
}

/// Access current theme
pub fn theme() -> &Theme {
    CompositionContext::current()
        .get_local::<Theme>(LocalTheme)
        .unwrap_or(&DEFAULT_THEME)
}
```

---

## 8. Performance Optimizations

### 8.1 Skipping Recomposition

```rust
/// Skip recomposition if inputs haven't changed
#[composable]
pub fn skippable_composable<T, C>(
    input: T,
    content: C,
) where
    T: PartialEq + 'static,
    C: Composable,
{
    let prev_input = remember(|| None::<T>);
    
    if prev_input.as_ref() == Some(&input) {
        // Skip - reuse previous composition
        CompositionContext::current().skip_to_end_group();
    } else {
        *prev_input = Some(input);
        content.compose();
    }
}
```

### 8.2 Lazy Composition

```rust
/// Virtualized list that only composes visible items
#[composable]
pub fn lazy_column<T, I>(
    items: I,
    #[modifier] modifier: ModifierChain = ModifierChain::default(),
    item_content: impl Fn(usize, &T) + 'static,
) where
    I: IntoIterator<Item = T>,
    T: 'static,
{
    let items: Vec<T> = items.into_iter().collect();
    let scroll_state = remember(|| ScrollState::default());
    let visible_range = calculate_visible_range(&scroll_state, items.len());
    
    scroll_container(scroll_state, modifier) {
        // Only compose visible items
        for index in visible_range {
            key(index) {
                item_content(index, &items[index]);
            }
        }
    }
}
```

### 8.3 Batched Updates

```rust
/// Batch multiple state changes into single recomposition
pub fn batch<F: FnOnce()>(updates: F) {
    let context = CompositionContext::current();
    context.begin_batch();
    updates();
    context.end_batch(); // Triggers single recomposition
}
```

---

## 9. Error Handling

### 9.1 Composition Errors

```rust
/// Errors that can occur during composition
#[derive(Debug)]
pub enum CompositionError {
    /// State accessed outside of composition
    StateAccessOutsideComposition,
    /// Composable called outside of composition context
    NoCompositionContext,
    /// Too many recompositions (infinite loop detection)
    RecompositionLimit { limit: u32 },
    /// Layout constraint violation
    LayoutConstraintViolation { constraint: String },
}

/// Error boundary composable
#[composable]
pub fn error_boundary<F, E>(
    fallback: F,
    #[slot] content: impl Composable,
) where
    F: Fn(&CompositionError) + 'static,
{
    let error = remember(|| None::<CompositionError>);
    
    if let Some(err) = error.as_ref() {
        fallback(err);
    } else {
        // Try to compose content, catch errors
        match std::panic::catch_unwind(|| content.compose()) {
            Ok(_) => {}
            Err(e) => {
                *error = Some(CompositionError::from_panic(e));
            }
        }
    }
}
```

---

## 10. Testing Strategy

### 10.1 Unit Testing Composables

```rust
#[cfg(test)]
mod tests {
    use becompose::testing::*;
    
    #[test]
    fn test_button_click() {
        let clicked = Arc::new(AtomicBool::new(false));
        let clicked_clone = clicked.clone();
        
        compose_test(|| {
            button(
                on_click = move || clicked_clone.store(true, Ordering::SeqCst),
            ) {
                text("Click me")
            }
        })
        .find_by_text("Click me")
        .click();
        
        assert!(clicked.load(Ordering::SeqCst));
    }
    
    #[test]
    fn test_state_updates() {
        let tree = compose_test(|| {
            let count = mutable_state_of(0);
            
            column {
                text(format!("Count: {}", count.get()))
                button(on_click = || count.set(count.get() + 1)) {
                    text("Increment")
                }
            }
        });
        
        tree.assert_text_exists("Count: 0");
        tree.find_by_text("Increment").click();
        tree.assert_text_exists("Count: 1");
    }
}
```

### 10.2 Snapshot Testing

```rust
#[test]
fn test_layout_snapshot() {
    let snapshot = compose_snapshot(|| {
        card(modifier = Modifier::padding(16)) {
            column {
                text("Title", style = TextStyle::title())
                text("Description", style = TextStyle::body())
            }
        }
    });
    
    assert_snapshot!(snapshot, "card_layout");
}
```

---

## 11. Project Structure

```
becompose/
├── Cargo.toml
├── src/
│   ├── lib.rs                  # Main library entry
│   ├── composition/
│   │   ├── mod.rs
│   │   ├── tree.rs             # Composition tree
│   │   ├── context.rs          # Composition context
│   │   ├── reconciler.rs       # Tree diffing
│   │   └── recomposition.rs    # Recomposition logic
│   ├── state/
│   │   ├── mod.rs
│   │   ├── mutable_state.rs    # MutableState<T>
│   │   ├── derived_state.rs    # DerivedState<T>
│   │   ├── remember.rs         # remember() hook
│   │   └── effects.rs          # launched_effect, etc.
│   ├── layout/
│   │   ├── mod.rs
│   │   ├── constraints.rs      # Layout constraints
│   │   ├── measure.rs          # Measurement
│   │   ├── arrangement.rs      # Arrangement/Alignment
│   │   └── layouts/
│   │       ├── column.rs
│   │       ├── row.rs
│   │       └── box.rs
│   ├── modifier/
│   │   ├── mod.rs
│   │   ├── chain.rs            # ModifierChain
│   │   ├── layout_modifiers.rs # padding, size, etc.
│   │   ├── draw_modifiers.rs   # background, border
│   │   └── input_modifiers.rs  # clickable, draggable
│   ├── components/
│   │   ├── mod.rs
│   │   ├── text.rs
│   │   ├── button.rs
│   │   ├── image.rs
│   │   ├── text_field.rs
│   │   └── ...
│   ├── theme/
│   │   ├── mod.rs
│   │   ├── color_scheme.rs
│   │   ├── typography.rs
│   │   └── material_theme.rs
│   ├── animation/
│   │   ├── mod.rs
│   │   ├── animate_as_state.rs
│   │   └── transitions.rs
│   ├── bevy_integration/
│   │   ├── mod.rs
│   │   ├── plugin.rs           # BecomposePlugin
│   │   ├── entity_bridge.rs    # Entity sync
│   │   └── input_bridge.rs     # Input handling
│   └── testing/
│       ├── mod.rs
│       └── test_utils.rs
├── becompose_macros/           # Proc macro crate
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       └── composable.rs       # #[composable] macro
└── examples/
    ├── hello_world.rs
    ├── counter.rs
    ├── todo_app.rs
    └── theme_showcase.rs
```

---

## 12. API Examples

### 12.1 Hello World

```rust
use becompose::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BecomposePlugin)
        .add_systems(Startup, setup_ui)
        .run();
}

fn setup_ui(mut commands: Commands) {
    commands.spawn_composition(|| {
        greeting("World")
    });
}

#[composable]
fn greeting(name: &str) {
    text(
        format!("Hello, {}!", name),
        modifier = Modifier::padding(16),
        style = TextStyle::title(),
    )
}
```

### 12.2 Counter App

```rust
#[composable]
fn counter_app() {
    let count = mutable_state_of(0);
    
    column(
        modifier = Modifier::fill_max_size().padding(24),
        vertical_arrangement = Arrangement::Center,
        horizontal_alignment = Alignment::CenterHorizontally,
    ) {
        text(
            format!("Count: {}", count.get()),
            style = theme().typography.display_medium,
        )
        
        spacer(modifier = Modifier::height(16))
        
        row(horizontal_arrangement = Arrangement::spaced_by(8)) {
            button(on_click = || count.set(count.get() - 1)) {
                text("-")
            }
            button(on_click = || count.set(count.get() + 1)) {
                text("+")
            }
        }
    }
}
```

### 12.3 Todo List

```rust
#[composable]
fn todo_app() {
    let todos = mutable_state_of(Vec::<Todo>::new());
    let input_text = mutable_state_of(String::new());
    
    column(modifier = Modifier::fill_max_size().padding(16)) {
        // Input row
        row(modifier = Modifier::fill_max_width()) {
            text_field(
                value = input_text.get(),
                on_change = |text| input_text.set(text),
                modifier = Modifier::weight(1.0),
                placeholder = "Add a todo...",
            )
            
            spacer(modifier = Modifier::width(8))
            
            button(on_click = || {
                if !input_text.get().is_empty() {
                    todos.update(|list| {
                        list.push(Todo::new(input_text.get().clone()));
                    });
                    input_text.set(String::new());
                }
            }) {
                text("Add")
            }
        }
        
        spacer(modifier = Modifier::height(16))
        
        // Todo list
        lazy_column(
            items = todos.get(),
            modifier = Modifier::fill_max_width(),
        ) { index, todo |
            todo_item(
                todo = todo,
                on_toggle = || todos.update(|list| list[index].toggle()),
                on_delete = || todos.update(|list| { list.remove(index); }),
            )
        }
    }
}

#[composable]
fn todo_item(
    todo: &Todo,
    on_toggle: impl Fn(),
    on_delete: impl Fn(),
) {
    row(
        modifier = Modifier
            .fill_max_width()
            .padding(12)
            .background(theme().colors.surface),
        vertical_alignment = Alignment::CenterVertically,
    ) {
        checkbox(
            checked = todo.completed,
            on_change = |_| on_toggle(),
        )
        
        text(
            todo.title.clone(),
            modifier = Modifier::weight(1.0).padding_start(12),
            style = if todo.completed {
                TextStyle::body().strikethrough()
            } else {
                TextStyle::body()
            },
        )
        
        icon_button(on_click = on_delete) {
            icon(Icons::Delete)
        }
    }
}
```

---

## 13. Future Considerations

### 13.1 Planned Features
- **Gesture Recognition** - Swipe, pinch, long press
- **Drag and Drop** - Native drag-and-drop support
- **Accessibility** - Screen reader support, semantic properties
- **Localization** - Built-in i18n support

### 13.2 Performance Roadmap
- Compilation caching for composables
- GPU-accelerated layout calculations
- Parallel composition for independent subtrees
- Memory pooling for allocation reduction

---

## 14. References

- [Jetpack Compose Architecture](https://developer.android.com/jetpack/compose/architecture)
- [Bevy UI Documentation](https://bevyengine.org/learn/book/getting-started/ui/)
- [React Reconciliation](https://reactjs.org/docs/reconciliation.html)
- [Flutter Rendering Pipeline](https://flutter.dev/docs/resources/architectural-overview#rendering)
