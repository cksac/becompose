//! Mutable State
//!
//! Provides reactive mutable state that triggers recomposition on change.

use std::sync::{Arc, RwLock};

use crate::composition::CompositionId;

/// Callback type for state change notifications
pub type StateChangeCallback = Arc<dyn Fn() + Send + Sync>;

/// Mutable state holder with change tracking
#[derive(Clone)]
pub struct MutableState<T> {
    inner: Arc<RwLock<MutableStateInner<T>>>,
}

struct MutableStateInner<T> {
    value: T,
    version: u64,
    subscribers: Vec<CompositionId>,
    on_change: Option<StateChangeCallback>,
}

impl<T: Clone + PartialEq + Send + Sync + 'static> MutableState<T> {
    pub fn new(initial: T) -> Self {
        Self {
            inner: Arc::new(RwLock::new(MutableStateInner {
                value: initial,
                version: 0,
                subscribers: Vec::new(),
                on_change: None,
            })),
        }
    }

    pub fn get(&self) -> T {
        self.inner.read().unwrap().value.clone()
    }

    pub fn set(&self, new_value: T) {
        let callback = {
            let mut inner = self.inner.write().unwrap();
            if inner.value != new_value {
                inner.value = new_value;
                inner.version += 1;
                inner.on_change.clone()
            } else {
                None
            }
        };

        // Trigger change callback outside of lock
        if let Some(cb) = callback {
            cb();
        }
    }

    pub fn update<F: FnOnce(&T) -> T>(&self, f: F) {
        let new_value = {
            let inner = self.inner.read().unwrap();
            f(&inner.value)
        };
        self.set(new_value);
    }

    pub fn version(&self) -> u64 {
        self.inner.read().unwrap().version
    }

    pub fn subscribe(&self, id: CompositionId) {
        self.inner.write().unwrap().subscribers.push(id);
    }

    pub fn set_on_change(&self, callback: StateChangeCallback) {
        self.inner.write().unwrap().on_change = Some(callback);
    }
}

impl<T: Clone + PartialEq + Send + Sync + Default + 'static> Default for MutableState<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

/// Create mutable state with an initial value
pub fn mutable_state_of<T: Clone + PartialEq + Send + Sync + 'static>(
    initial: T,
) -> MutableState<T> {
    MutableState::new(initial)
}
