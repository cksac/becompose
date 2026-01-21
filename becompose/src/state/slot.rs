//! State Slot Management
//!
//! Manages state slots for composables.

use std::any::Any;
use std::cell::RefCell;
use std::sync::Arc;

/// A slot for storing state
pub type StateSlot = Box<dyn Any + Send + Sync>;

/// Manages state slots for a composable
#[derive(Clone)]
pub struct StateSlotManager {
    inner: Arc<RefCell<StateSlotManagerInner>>,
}

struct StateSlotManagerInner {
    slots: Vec<StateSlot>,
    current_index: usize,
}

impl StateSlotManager {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RefCell::new(StateSlotManagerInner {
                slots: Vec::new(),
                current_index: 0,
            })),
        }
    }

    /// Called during composition to get or create a state slot
    pub fn remember<T, F>(&self, init: F) -> T
    where
        T: Clone + Send + Sync + 'static,
        F: FnOnce() -> T,
    {
        let mut inner = self.inner.borrow_mut();
        let index = inner.current_index;
        inner.current_index += 1;

        if index >= inner.slots.len() {
            // First composition - create the slot
            let value = init();
            inner.slots.push(Box::new(value.clone()));
            value
        } else {
            // Return existing value
            inner.slots[index]
                .downcast_ref::<T>()
                .cloned()
                .unwrap_or_else(|| {
                    let value = init();
                    inner.slots[index] = Box::new(value.clone());
                    value
                })
        }
    }

    /// Update a state value at a given index
    pub fn update<T: Clone + Send + Sync + 'static>(&self, index: usize, value: T) {
        let mut inner = self.inner.borrow_mut();
        if index < inner.slots.len() {
            inner.slots[index] = Box::new(value);
        }
    }

    /// Reset index at start of recomposition
    pub fn reset(&self) {
        self.inner.borrow_mut().current_index = 0;
    }

    /// Get the current slot index
    pub fn current_index(&self) -> usize {
        self.inner.borrow().current_index
    }
}

impl Default for StateSlotManager {
    fn default() -> Self {
        Self::new()
    }
}
