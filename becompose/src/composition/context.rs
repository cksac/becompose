//! Composition Context
//!
//! Provides the runtime context for composable functions.

use std::cell::RefCell;
use std::sync::Arc;

use crate::composition::{CompositionId, CompositionKey, CompositionNode, ComposableType};
use crate::state::StateSlotManager;

thread_local! {
    static CURRENT_CONTEXT: RefCell<Option<CompositionContext>> = RefCell::new(None);
}

/// The composition context manages the current composition state
#[derive(Clone)]
pub struct CompositionContext {
    inner: Arc<RefCell<CompositionContextInner>>,
}

struct CompositionContextInner {
    /// Stack of current composition nodes
    node_stack: Vec<CompositionId>,
    /// State manager for the current composition
    state_manager: StateSlotManager,
    /// Pending nodes to be added to the tree
    pending_nodes: Vec<CompositionNode>,
    /// Whether we're currently in batch mode
    batch_mode: bool,
    /// Whether composition is active
    active: bool,
}

impl CompositionContext {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RefCell::new(CompositionContextInner {
                node_stack: Vec::new(),
                state_manager: StateSlotManager::new(),
                pending_nodes: Vec::new(),
                batch_mode: false,
                active: false,
            })),
        }
    }

    /// Get the current composition context
    pub fn current() -> Self {
        CURRENT_CONTEXT.with(|ctx| {
            ctx.borrow()
                .clone()
                .unwrap_or_else(|| {
                    // Create a default context if none exists
                    let new_ctx = CompositionContext::new();
                    *ctx.borrow_mut() = Some(new_ctx.clone());
                    new_ctx
                })
        })
    }

    /// Set the current composition context
    pub fn set_current(ctx: CompositionContext) {
        CURRENT_CONTEXT.with(|current| {
            *current.borrow_mut() = Some(ctx);
        });
    }

    /// Clear the current composition context
    pub fn clear_current() {
        CURRENT_CONTEXT.with(|ctx| {
            *ctx.borrow_mut() = None;
        });
    }

    /// Start a new composition group
    pub fn start_group(&self, type_id: &str, key: Option<CompositionKey>) -> CompositionId {
        let mut inner = self.inner.borrow_mut();
        inner.active = true;
        
        let mut node = CompositionNode::new(ComposableType::Custom(type_id.to_string()));
        if let Some(k) = key {
            node.key = Some(k);
        }
        
        let id = node.id;
        inner.pending_nodes.push(node);
        inner.node_stack.push(id);
        
        id
    }

    /// End the current composition group
    pub fn end_group(&self, _id: CompositionId) {
        let mut inner = self.inner.borrow_mut();
        inner.node_stack.pop();
        
        if inner.node_stack.is_empty() {
            inner.active = false;
        }
    }

    /// Get the current parent node ID
    pub fn current_parent(&self) -> Option<CompositionId> {
        self.inner.borrow().node_stack.last().copied()
    }

    /// Access the state manager
    pub fn state_manager(&self) -> StateSlotManager {
        self.inner.borrow().state_manager.clone()
    }

    /// Start batch mode for multiple state updates
    pub fn begin_batch(&self) {
        self.inner.borrow_mut().batch_mode = true;
    }

    /// End batch mode and trigger recomposition
    pub fn end_batch(&self) {
        self.inner.borrow_mut().batch_mode = false;
    }

    /// Check if currently in batch mode
    pub fn is_batching(&self) -> bool {
        self.inner.borrow().batch_mode
    }

    /// Check if composition is active
    pub fn is_active(&self) -> bool {
        self.inner.borrow().active
    }

    /// Take pending nodes for processing
    pub fn take_pending_nodes(&self) -> Vec<CompositionNode> {
        std::mem::take(&mut self.inner.borrow_mut().pending_nodes)
    }

    /// Skip to end of current group (for optimization)
    pub fn skip_to_end_group(&self) {
        // Used when skipping recomposition of unchanged subtrees
        let mut inner = self.inner.borrow_mut();
        inner.node_stack.pop();
    }
}

impl Default for CompositionContext {
    fn default() -> Self {
        Self::new()
    }
}
