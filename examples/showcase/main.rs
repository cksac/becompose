//! BECOMPOSE Showcase
//!
//! A comprehensive showcase of BECOMPOSE UI components,
//! demonstrating both basic components and Material UI components.

#![allow(non_snake_case)] // Composable functions use PascalCase like Jetpack Compose

use becompose::bevy_integration::material_ui::{
    Checkbox, CheckboxState, FilledButton, FilledTextField, Slider, Switch,
};
use becompose::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
fn main() {
    run();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run() {
    console_error_panic_hook::set_once();
    run_app("BECOMPOSE - Showcase", || {
        ShowcaseApp();
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    run_app("BECOMPOSE - Showcase", || {
        ShowcaseApp();
    });
}

/// Main showcase app
fn ShowcaseApp() {
    // Set up material theme
    // let theme = MaterialTheme::default();
    // set_material_theme(theme);

    Surface(
        Modifiers::new().then(BackgroundModifier::new(Color::srgb(0.05, 0.05, 0.08))),
        || {
            Column(
                Modifiers::new()
                    .fill_max_size()
                    .padding(24.0)
                    .vertical_arrangement(VerticalArrangement::Top)
                    .horizontal_alignment(HorizontalAlignment::Start)
                    .row_gap(32.0),
                || {
                    // Header
                    ShowcaseHeader();

                    // Basic Components Section
                    BasicComponentsSection();

                    // Material UI Components Section
                    MaterialUISection();
                },
            );
        },
    );
    /// Material UI Components Section
    fn MaterialUISection() {
        Section(
            "Material UI Components",
            "Showcase of bevy_material_ui composables integrated with BECOMPOSE.",
            || {
                ComponentDemo("Filled Button", || {
                    FilledButton("Filled Button", || println!("Filled button clicked!"));
                });

                ComponentDemo("Slider", || {
                    Slider(0.5, 0.0, 1.0, |value| println!("Slider value: {}", value));
                });

                ComponentDemo("Switch", || {
                    Switch("Enable notifications", false, |selected| {
                        println!("Switch is now: {}", selected)
                    });
                });

                ComponentDemo("Checkbox", || {
                    Checkbox("Accept terms", CheckboxState::Unchecked, |new_state| {
                        println!("Checkbox state changed to: {:?}", new_state)
                    });
                });

                ComponentDemo("Filled Text Field", || {
                    FilledTextField("Username", "", |value| println!("Text changed: {}", value));
                });
            },
        );
    }
}

/// Header section
fn ShowcaseHeader() {
    Column(
        Modifiers::new()
            .fill_max_width()
            .horizontal_alignment(HorizontalAlignment::Center)
            .row_gap(8.0),
        || {
            Text(
                "BECOMPOSE Showcase 🎨",
                TextStyle::title().with_color(Color::WHITE),
            );
            Text(
                "A comprehensive demonstration of BECOMPOSE UI components",
                TextStyle::body().with_color(Color::srgb(0.7, 0.7, 0.7)),
            );
        },
    );
}

/// Basic Components Section
fn BasicComponentsSection() {
    Section(
        "Basic Components",
        "Fundamental building blocks of BECOMPOSE UI",
        || {
            // Text examples
            ComponentDemo("Text", || {
                Column(Modifiers::new().row_gap(8.0), || {
                    Text(
                        "Title Text".to_string(),
                        TextStyle::title().with_color(Color::WHITE),
                    );
                    Text(
                        "Headline Text".to_string(),
                        TextStyle::headline().with_color(Color::srgb(0.8, 0.8, 0.8)),
                    );
                    Text(
                        "Body Text".to_string(),
                        TextStyle::body().with_color(Color::srgb(0.6, 0.6, 0.6)),
                    );
                });
            });

            // Button examples
            ComponentDemo("Buttons", || {
                Row(Modifiers::new().column_gap(16.0), || {
                    Button("Basic Button".to_string(), Modifiers::new(), || {
                        println!("Basic button clicked!");
                    });
                });
            });
        },
    );
}

/// Section wrapper
fn Section<F>(title: &'static str, description: &'static str, content: F)
where
    F: Fn() + Send + Sync + 'static,
{
    Column(Modifiers::new().fill_max_width().row_gap(16.0), move || {
        // Section header
        Column(Modifiers::new().row_gap(4.0), move || {
            Text(
                title.to_string(),
                TextStyle::headline().with_color(Color::WHITE),
            );
            Text(
                description.to_string(),
                TextStyle::body().with_color(Color::srgb(0.6, 0.6, 0.6)),
            );
        });

        // Section content
        content();
    });
}

/// Component demo wrapper
fn ComponentDemo<F>(title: &'static str, content: F)
where
    F: Fn() + Send + Sync + 'static,
{
    Column(
        Modifiers::new()
            .fill_max_width()
            .padding(16.0)
            .background(Color::srgb(0.08, 0.08, 0.12))
            .row_gap(12.0),
        move || {
            Text(
                title.to_string(),
                TextStyle::title().with_color(Color::srgb(0.9, 0.9, 0.9)),
            );
            content();
        },
    );
}
