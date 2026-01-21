//! Derived State
//!
//! State that is computed from other state values.

use std::cell::RefCell;
use std::sync::Arc;

/// Derived state that computes its value from other state
#[derive(Clone)]
pub struct DerivedState<T> {
    inner: Arc<RefCell<DerivedStateInner<T>>>,
}

struct DerivedStateInner<T> {
    value: Option<T>,
    calculation: Box<dyn Fn() -> T + Send + Sync>,
    dirty: bool,
}

impl<T: Clone + PartialEq + Send + Sync + 'static> DerivedState<T> {
    pub fn new<F: Fn() -> T + Send + Sync + 'static>(calculation: F) -> Self {
        Self {
            inner: Arc::new(RefCell::new(DerivedStateInner {
                value: None,
                calculation: Box::new(calculation),
                dirty: true,
            })),
        }
    }

    pub fn get(&self) -> T {
        let mut inner = self.inner.borrow_mut();
        if inner.dirty || inner.value.is_none() {
            let new_value = (inner.calculation)();
            inner.value = Some(new_value.clone());
            inner.dirty = false;
            new_value
        } else {
            inner.value.clone().unwrap()
        }
    }

    pub fn invalidate(&self) {
        self.inner.borrow_mut().dirty = true;
    }
}

/// Create derived state from a calculation function
pub fn derived_state_of<T, F>(calculation: F) -> DerivedState<T>
where
    T: Clone + PartialEq + Send + Sync + 'static,
    F: Fn() -> T + Send + Sync + 'static,
{
    DerivedState::new(calculation)
}
