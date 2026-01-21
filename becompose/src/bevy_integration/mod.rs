//! Bevy Integration Layer
//!
//! Connects BECOMPOSE with the Bevy engine.

mod plugin;
mod entity_bridge;
mod input_bridge;
mod ui_builder;
mod app;
mod composables;

pub use plugin::*;
pub use entity_bridge::*;
pub use input_bridge::*;
pub use ui_builder::*;
pub use app::*;
pub use composables::*;
