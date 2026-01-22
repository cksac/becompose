//! Counter Example
//!
//! A counter app demonstrating state management with
//! Jetpack Compose-style composable functions.
//! 
//! This example shows SCOPED RECOMPOSITION:
//! - Each Scope() only rebuilds when the state it reads changes
//! - Other scopes remain untouched
//! - Watch the console to see which scopes rebuild!

#![allow(non_snake_case)] // Composable functions use PascalCase like Jetpack Compose

use becompose::prelude::*;

fn main() {
    // Create two independent counters
    let counter_a = State::new(0i32);
    let counter_b = State::new(0i32);
    
    run_app("BECOMPOSE - Scoped Counter", move || {
        // Call the counter app composable
        CounterApp(counter_a.clone(), counter_b.clone());
    });
}

/// Counter App with SCOPED RECOMPOSITION
/// 
/// Each counter is in its own Scope, so changing one counter
/// only rebuilds that scope - not the entire UI!
fn CounterApp(counter_a: State<i32>, counter_b: State<i32>) {
    Surface(Modifiers::new().then(BackgroundModifier::new(Color::srgb(0.1, 0.1, 0.15))), || {
        Column(
            Modifiers::new()
                .fill_max_size()
                .vertical_arrangement(VerticalArrangement::Center)
                .horizontal_alignment(HorizontalAlignment::Center)
                .row_gap(32.0),
            || {
                // Title (static - never rebuilds)
                Text("Scoped Recomposition Demo 🎯", TextStyle::title().with_color(Color::WHITE));
                Text("Each counter is in its own Scope!", TextStyle::body().with_color(Color::srgb(0.7, 0.7, 0.7)));
                
                FixedSpacer(16.0);
                
                // Counter A - in its own scope
                {
                    let counter = counter_a.clone();
                    Scope(move || {
                        println!("🔴 Scope A rebuilding...");
                        CounterDisplay("Counter A", Color::srgb(0.8, 0.3, 0.3), counter.clone());
                    });
                }
                
                FixedSpacer(16.0);
                
                // Counter B - in its own scope (independent!)
                {
                    let counter = counter_b.clone();
                    Scope(move || {
                        println!("🔵 Scope B rebuilding...");
                        CounterDisplay("Counter B", Color::srgb(0.3, 0.5, 0.8), counter.clone());
                    });
                }
                
                FixedSpacer(16.0);
                
                // Instructions (static)
                Text("Click buttons and watch the console!", TextStyle::body().with_color(Color::srgb(0.5, 0.5, 0.5)));
            }
        );
    });
}

/// Reusable counter display component
fn CounterDisplay(label: &'static str, accent_color: Color, counter: State<i32>) {
    let counter_inc = counter.clone();
    let counter_dec = counter.clone();
    let current_value = counter.get();
    
    Column(
        Modifiers::new()
            .horizontal_alignment(HorizontalAlignment::Center)
            .row_gap(12.0),
        || {
            // Label
            Text(label, TextStyle::headline().with_color(accent_color));
            
            // Value display
            Text(
                format!("{}", current_value),
                TextStyle::title().with_color(Color::WHITE)
            );
            
            // Button row
            Row(
                Modifiers::new()
                    .horizontal_arrangement(HorizontalArrangement::Center)
                    .column_gap(16.0),
                || {
                    Button(
                        "  −  ",
                        Modifiers::new().then(BackgroundModifier::new(accent_color)),
                        move || {
                            counter_dec.decrement();
                        }
                    );
                    
                    Button(
                        "  +  ",
                        Modifiers::new().then(BackgroundModifier::new(accent_color)),
                        move || {
                            counter_inc.increment();
                        }
                    );
                }
            );
        }
    );
}
