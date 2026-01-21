//! Remember Hook
//!
//! Provides state persistence across recompositions.

use crate::composition::CompositionContext;
use crate::state::MutableState;

/// Remember a value across recompositions
pub fn remember<T, F>(init: F) -> T
where
    T: Clone + Send + Sync + 'static,
    F: FnOnce() -> T,
{
    let ctx = CompositionContext::current();
    ctx.state_manager().remember(init)
}

/// Remember a mutable state value
pub fn remember_mutable_state<T>(initial: T) -> MutableState<T>
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    remember(|| MutableState::new(initial))
}
