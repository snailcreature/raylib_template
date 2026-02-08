//! An implementation of the Observer pattern.

use std::sync::Arc;

/// An object that has its state observed.
pub struct Subject<StateType> {
    state: Arc<StateType>,
    observers: Vec<Box<dyn Observer<StateType>>>,
}

impl<StateType> Subject<StateType> {
    pub fn new(state: StateType) -> Self {
        Self {
            state: Arc::new(state),
            observers: Vec::new(),
        }
    }

    /// Get the current value of the state.
    pub fn state(&self) -> &StateType {
        &self.state
    }

    /// Attach an Observer to this Subject.
    pub fn attach(&mut self, observer: Box<dyn Observer<StateType>>) -> () {
        self.observers.push(observer);
    }

    /// Update the state and notify the Observers.
    pub fn update_state(&mut self, state: StateType) -> () {
        let old_state = Arc::get_mut(&mut self.state).unwrap();
        *old_state = state;
        self.notify();
    }

    /// Notify Observers of new state.
    fn notify(&self) {
        for observer in &self.observers {
            observer.update(&self.state);
        }
    }
}

/// Action to perform when observed state is updated.
pub trait Observer<StateType> {
    /// Action run when observed Subject updates its state.
    fn update(&self, state: &StateType);
}
