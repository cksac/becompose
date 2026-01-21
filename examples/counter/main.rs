//! Counter Example
//!
//! A counter app demonstrating state management with
//! Jetpack Compose-style composable functions.
//! 
//! Notice how clean the API is:
//! - No cx, commands passing
//! - State auto-invalidates on change

#![allow(non_snake_case)] // Composable functions use PascalCase like Jetpack Compose

use becompose::prelude::*;

fn main() {
    // Create reactive state - automatically triggers recomposition on change
    let counter = State::new(0i32);
    
    run_app("BECOMPOSE - Counter", move || {
        // Call the counter app composable
        CounterApp(counter.clone());
    });
}

/// Counter App composable
/// 
/// In Jetpack Compose style, this function takes state as a parameter
/// and renders the UI based on that state. No cx/commands needed!
fn CounterApp(counter: State<i32>) {
    let counter_inc = counter.clone();
    let counter_dec = counter.clone();
    let current_value = counter.get();
    
    Surface(Color::srgb(0.1, 0.1, 0.15), || {
        StyledColumn(
            ModifierChain::new().fill_max_size(),
            VerticalArrangement::Center,
            HorizontalAlignment::Center,
            24.0,
            || {
                // Title
                StyledText("Counter App 🔢", TextStyle::title().with_color(Color::WHITE));
                
                // Counter display
                StyledText(
                    format!("Count: {}", current_value),
                    TextStyle::headline().with_color(Color::srgb(0.4, 0.8, 1.0))
                );
                
                FixedSpacer(16.0);
                
                // Button row
                StyledRow(
                    ModifierChain::new(),
                    HorizontalArrangement::Center,
                    VerticalAlignment::Center,
                    16.0,
                    || {
                        // Decrement button - State::decrement auto-invalidates!
                        StyledButton(
                            "  −  ",
                            ModifierChain::new().background(Color::srgb(0.8, 0.3, 0.3)),
                            move || {
                                counter_dec.decrement();
                                println!("Decremented!");
                            }
                        );
                        
                        // Increment button - State::increment auto-invalidates!
                        StyledButton(
                            "  +  ",
                            ModifierChain::new().background(Color::srgb(0.3, 0.7, 0.4)),
                            move || {
                                counter_inc.increment();
                                println!("Incremented!");
                            }
                        );
                    }
                );
            }
        );
    });
}
