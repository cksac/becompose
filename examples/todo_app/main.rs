//! Todo App Example
//!
//! A todo list application demonstrating Jetpack Compose-style
//! composable functions with lists and state management.
//!
//! Notice how clean the API is:
//! - No cx, commands passing
//! - State<T> auto-invalidates when modified

#![allow(non_snake_case)] // Composable functions use PascalCase like Jetpack Compose

use becompose::prelude::*;

/// A todo item
#[derive(Clone, Debug)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

/// Application state using reactive State<T>
#[derive(Clone)]
struct AppState {
    todos: State<Vec<Todo>>,
    next_id: State<u32>,
}

impl AppState {
    fn new() -> Self {
        Self {
            todos: State::new(vec![
                Todo { id: 1, title: "Learn BECOMPOSE".to_string(), completed: false },
                Todo { id: 2, title: "Build awesome UIs".to_string(), completed: false },
                Todo { id: 3, title: "Have fun with Bevy".to_string(), completed: true },
            ]),
            next_id: State::new(4),
        }
    }

    fn add_todo(&self) {
        let id = self.next_id.get();
        self.next_id.set(id + 1);
        self.todos.update(|todos| {
            todos.push(Todo {
                id,
                title: format!("New Todo #{}", id),
                completed: false,
            });
        });
        println!("Added new todo!");
    }

    fn toggle_todo(&self, id: u32) {
        self.todos.update(|todos| {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                todo.completed = !todo.completed;
            }
        });
        println!("Toggled todo {}", id);
    }

    fn delete_todo(&self, id: u32) {
        self.todos.update(|todos| {
            todos.retain(|t| t.id != id);
        });
        println!("Deleted todo {}", id);
    }
}

fn main() {
    let state = AppState::new();
    
    run_app("BECOMPOSE - Todo App", move || {
        // Call the main app composable
        TodoApp(state.clone());
    });
}

/// Main TodoApp composable
/// 
/// This function composes the entire todo application UI.
/// Like in Jetpack Compose, it's a function that takes state
/// and emits UI by calling other composable functions.
/// No cx/commands needed!
fn TodoApp(state: AppState) {
    let state_for_add = state.clone();
    
    Surface(ModifierChain::new().background(Color::srgb(0.1, 0.1, 0.15)), || {
        Column(
            ModifierChain::new()
                .fill_max_size()
                .padding(24.0),
            VerticalArrangement::Top,
            HorizontalAlignment::Start,
            16.0,
            || {
                // Title
                Text("📝 Todo List", TextStyle::title().with_color(Color::WHITE));
                
                // Add button
                Button(
                    "+ Add New Todo",
                    ModifierChain::new().background(Color::srgb(0.3, 0.6, 0.9)),
                    move || {
                        state_for_add.add_todo();
                    }
                );
                
                FixedSpacer(8.0);
                
                // Todo list
                TodoList(state);
            }
        );
    });
}

/// TodoList composable - renders the list of todos
fn TodoList(state: AppState) {
    let todos = state.todos.get();
    
    Column(ModifierChain::new(), VerticalArrangement::Top, HorizontalAlignment::Start, 0.0, || {
        // ForEach iterates and composes content for each item
        ForEach(&todos, |todo| {
            TodoItem(todo, state.clone());
        });
    });
}

/// TodoItem composable - renders a single todo item
/// 
/// This is a reusable composable that can be composed anywhere.
fn TodoItem(todo: &Todo, state: AppState) {
    let todo_id = todo.id;
    let is_completed = todo.completed;
    let title = todo.title.clone();
    
    let state_toggle = state.clone();
    let state_delete = state;
    
    let bg_color = if is_completed {
        Color::srgb(0.15, 0.2, 0.15)
    } else {
        Color::srgb(0.2, 0.2, 0.25)
    };
    
    let text_color = if is_completed {
        Color::srgb(0.5, 0.5, 0.5)
    } else {
        Color::WHITE
    };
    
    Row(
        ModifierChain::new()
            .fill_max_width()
            .padding(12.0)
            .background(bg_color),
        HorizontalArrangement::SpaceBetween,
        VerticalAlignment::Center,
        12.0,
        || {
            // Left side: checkbox + text
            Row(ModifierChain::new(), HorizontalArrangement::Start, VerticalAlignment::Center, 0.0, || {
                // Checkbox button
                Button(
                    if is_completed { "✓" } else { "○" },
                    ModifierChain::new()
                        .size(28.0, 28.0)
                        .background(if is_completed {
                            Color::srgb(0.3, 0.7, 0.4)
                        } else {
                            Color::srgb(0.3, 0.3, 0.35)
                        }),
                    move || {
                        state_toggle.toggle_todo(todo_id);
                    }
                );
                
                FixedSpacer(12.0);
                
                // Todo text
                let display_text = if is_completed {
                    format!("~{}~", title)
                } else {
                    title.clone()
                };
                Text(display_text, TextStyle::body().with_color(text_color));
            });
            
            // Delete button
            Button(
                "×",
                ModifierChain::new().background(Color::srgb(0.7, 0.3, 0.3)),
                move || {
                    state_delete.delete_todo(todo_id);
                }
            );
        }
    );
}
