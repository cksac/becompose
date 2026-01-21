//! Hello World Example
//!
//! A simple example demonstrating the BECOMPOSE declarative UI with
//! Jetpack Compose-style composable functions.
//!
//! Notice: No cx, commands passing needed!

#![allow(non_snake_case)] // Composable functions use PascalCase like Jetpack Compose

use becompose::prelude::*;

fn main() {
    // Run the app - no Bevy boilerplate needed!
    run_app("BECOMPOSE - Hello World", || {
        // Just call composable functions directly
        App();
    });
}

/// The root App composable
/// 
/// Like Jetpack Compose, this is just a function that emits UI
/// by calling other composable functions. No cx/commands needed!
fn App() {
    // Surface is the root container (like Scaffold in Compose)
    Surface(ModifierChain::new().background(Color::srgb(0.1, 0.1, 0.15)), || {
        // Column arranges children vertically
        Column(
            ModifierChain::new()
                .fill_max_size()
                .vertical_arrangement(VerticalArrangement::Center)
                .horizontal_alignment(HorizontalAlignment::Center)
                .row_gap(16.0),
            || {
                // Composable function calls - just like Jetpack Compose!
                Text("Hello, BECOMPOSE! 🎮", TextStyle::title().with_color(Color::WHITE));
                
                Text(
                    "Welcome to declarative UI in Bevy",
                    TextStyle::body().with_color(Color::srgb(0.7, 0.7, 0.7))
                );
                
                FixedSpacer(24.0);
                
                // Simple button
                Button("Click me!", ModifierChain::new(), || {
                    println!("Button clicked!");
                });
            }
        );
    });
}
