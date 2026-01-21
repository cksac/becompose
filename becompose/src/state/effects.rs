//! Effects
//!
//! Side effects that run during composition.

use crate::composition::CompositionContext;

/// A disposable effect handle
pub struct DisposableEffect {
    pub on_dispose: Option<Box<dyn FnOnce() + Send + Sync>>,
}

impl DisposableEffect {
    pub fn new() -> Self {
        Self { on_dispose: None }
    }

    pub fn with_dispose<F: FnOnce() + Send + Sync + 'static>(mut self, f: F) -> Self {
        self.on_dispose = Some(Box::new(f));
        self
    }
}

impl Default for DisposableEffect {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for DisposableEffect {
    fn drop(&mut self) {
        if let Some(dispose) = self.on_dispose.take() {
            dispose();
        }
    }
}

/// Run an effect when a key changes
pub fn launched_effect<K, F>(key: K, effect: F)
where
    K: PartialEq + Clone + Send + Sync + 'static,
    F: FnOnce() + Send + Sync + 'static,
{
    let ctx = CompositionContext::current();
    let prev_key: Option<K> = ctx.state_manager().remember(|| None);
    
    if prev_key.as_ref() != Some(&key) {
        // Key changed, run effect
        ctx.state_manager().update(ctx.state_manager().current_index() - 1, Some(key));
        effect();
    }
}

/// Run a disposable effect
pub fn disposable_effect<K, F>(key: K, effect: F) -> DisposableEffect
where
    K: PartialEq + Clone + Send + Sync + 'static,
    F: FnOnce() -> DisposableEffect + Send + Sync + 'static,
{
    let ctx = CompositionContext::current();
    let prev_key: Option<K> = ctx.state_manager().remember(|| None);
    
    if prev_key.as_ref() != Some(&key) {
        ctx.state_manager().update(ctx.state_manager().current_index() - 1, Some(key));
        effect()
    } else {
        DisposableEffect::new()
    }
}

/// Side effect that runs after every composition
pub fn side_effect<F: FnOnce() + Send + Sync + 'static>(effect: F) {
    // Run the effect immediately
    effect();
}
