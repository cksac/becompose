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
    Surface(Color::srgb(0.1, 0.1, 0.15), || {
        // Column arranges children vertically
        StyledColumn(
            ModifierChain::new().fill_max_size(),
            VerticalArrangement::Center,
            HorizontalAlignment::Center,
            16.0, // spacing
            || {
                // Composable function calls - just like Jetpack Compose!
                StyledText("Hello, BECOMPOSE! 🎮", TextStyle::title().with_color(Color::WHITE));
                
                StyledText(
                    "Welcome to declarative UI in Bevy",
                    TextStyle::body().with_color(Color::srgb(0.7, 0.7, 0.7))
                );
                
                FixedSpacer(24.0);
                
                // Simple button
                Button("Click me!", || {
                    println!("Button clicked!");
                });
            }
        );
    });
}
