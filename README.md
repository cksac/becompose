# BECOMPOSE

A declarative UI framework with Jetpack Composse's Composable like API using [bevy](https://bevy.org/) as render.

# Example
```rust
#![allow(non_snake_case)] // Composable functions use PascalCase like Jetpack Compose

use becompose::prelude::*;

fn main() {
    let counter_a = State::new(0i32);
    let counter_b = State::new(0i32);
    run_app("BECOMPOSE - Auto-Scoped Counter", move || {
        CounterApp(counter_a, counter_b);
    });
}

fn CounterApp(counter_a: State<i32>, counter_b: State<i32>) {
    Surface(
        Modifiers::new().then(BackgroundModifier::new(Color::srgb(0.1, 0.1, 0.15))),
        move || {
            Column(
                Modifiers::new()
                    .fill_max_size()
                    .vertical_arrangement(VerticalArrangement::Center)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .row_gap(32.0),
                move || {
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
                    Counter("Counter A", Color::srgb(0.8, 0.3, 0.3), counter_a);
                    FixedSpacer(16.0);
                    Counter("Counter B", Color::srgb(0.3, 0.5, 0.8), counter_b);
                    FixedSpacer(16.0);
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


fn Counter(label: &'static str, accent_color: Color, counter: State<i32>) {
    Column(
        Modifiers::new()
            .horizontal_alignment(HorizontalAlignment::Center)
            .row_gap(12.0),
        move || {
            let current_value = counter.get();
            println!("🔄 {} scope rebuilding... value={}", label, current_value);
            Text(label, TextStyle::headline().with_color(accent_color));
            Text(
                format!("{}", current_value),
                TextStyle::title().with_color(Color::WHITE),
            );
            Row(
                Modifiers::new()
                    .horizontal_arrangement(HorizontalArrangement::Center)
                    .column_gap(16.0),
                move || {
                    Button(
                        "  −  ",
                        Modifiers::new().then(BackgroundModifier::new(accent_color)),
                        move || {
                            counter.decrement();
                        },
                    );

                    Button(
                        "  +  ",
                        Modifiers::new().then(BackgroundModifier::new(accent_color)),
                        move || {
                            counter.increment();
                        },
                    );
                },
            );
        },
    );
}
```
