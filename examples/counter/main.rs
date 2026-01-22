//! Counter Example
//!
//! A counter app demonstrating AUTOMATIC SCOPED RECOMPOSITION.
//!
//! Every composable (Column, Row, Box, etc.) is automatically a scope!
//! - State reads inside a composable subscribe that composable to the state
//! - When state changes, only composables that read it will rebuild
//! - Watch the console to see which composables rebuild!

#![allow(non_snake_case)] // Composable functions use PascalCase like Jetpack Compose

use becompose::prelude::*;

fn main() {
    // Create two independent counters
    let counter_a = State::new(0i32);
    let counter_b = State::new(0i32);

    run_app("BECOMPOSE - Auto-Scoped Counter", move || {
        // Call the counter app composable
        CounterApp(counter_a.clone(), counter_b.clone());
    });
}

/// Counter App with AUTOMATIC SCOPED RECOMPOSITION
///
/// Notice: NO explicit Scope() calls needed!
/// Each Column/Row is automatically a scope boundary.
fn CounterApp(counter_a: State<i32>, counter_b: State<i32>) {
    let counter_a_clone = counter_a.clone();
    let counter_b_clone = counter_b.clone();

    Surface(
        Modifiers::new().then(BackgroundModifier::new(Color::srgb(0.1, 0.1, 0.15))),
        move || {
            let counter_a = counter_a_clone.clone();
            let counter_b = counter_b_clone.clone();

            Column(
                Modifiers::new()
                    .fill_max_size()
                    .vertical_arrangement(VerticalArrangement::Center)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .row_gap(32.0),
                move || {
                    let counter_a = counter_a.clone();
                    let counter_b = counter_b.clone();

                    // Title (static - this Column never rebuilds after initial)
                    Column(
                        Modifiers::new().horizontal_alignment(HorizontalAlignment::Center),
                        || {
                            Text(
                                "Auto-Scoped Recomposition 🎯",
                                TextStyle::title().with_color(Color::WHITE),
                            );
                            Text(
                                "Every composable is automatically scoped!",
                                TextStyle::body().with_color(Color::srgb(0.7, 0.7, 0.7)),
                            );
                        },
                    );

                    FixedSpacer(16.0);

                    // Counter A display
                    CounterDisplay("Counter A", Color::srgb(0.8, 0.3, 0.3), counter_a.clone());

                    FixedSpacer(16.0);

                    // Counter B display
                    CounterDisplay("Counter B", Color::srgb(0.3, 0.5, 0.8), counter_b.clone());

                    FixedSpacer(16.0);

                    // Instructions (static)
                    Column(
                        Modifiers::new().horizontal_alignment(HorizontalAlignment::Center),
                        || {
                            Text(
                                "Click buttons and watch the console!",
                                TextStyle::body().with_color(Color::srgb(0.5, 0.5, 0.5)),
                            );
                            Text(
                                "Only the clicked counter's scope rebuilds!",
                                TextStyle::body().with_color(Color::srgb(0.5, 0.5, 0.5)),
                            );
                        },
                    );
                },
            );
        },
    );
}

/// Reusable counter display component
fn CounterDisplay(label: &'static str, accent_color: Color, counter: State<i32>) {
    let counter_for_column = counter.clone();

    Column(
        Modifiers::new()
            .horizontal_alignment(HorizontalAlignment::Center)
            .row_gap(12.0),
        move || {
            // IMPORTANT: Call counter.get() INSIDE the Column closure
            // This subscribes THIS scope (the Column) to the counter state.
            // If we called it outside, we'd subscribe the parent scope instead.
            let current_value = counter_for_column.get();
            let counter_dec = counter_for_column.clone();
            let counter_inc = counter_for_column.clone();

            println!("🔄 {} scope rebuilding... value={}", label, current_value);

            // Label
            Text(label, TextStyle::headline().with_color(accent_color));

            // Value display
            Text(
                format!("{}", current_value),
                TextStyle::title().with_color(Color::WHITE),
            );

            // Button row
            Row(
                Modifiers::new()
                    .horizontal_arrangement(HorizontalArrangement::Center)
                    .column_gap(16.0),
                move || {
                    let dec = counter_dec.clone();
                    Button(
                        "  −  ",
                        Modifiers::new().then(BackgroundModifier::new(accent_color)),
                        move || {
                            dec.decrement();
                        },
                    );

                    let inc = counter_inc.clone();
                    Button(
                        "  +  ",
                        Modifiers::new().then(BackgroundModifier::new(accent_color)),
                        move || {
                            inc.increment();
                        },
                    );
                },
            );
        },
    );
}
