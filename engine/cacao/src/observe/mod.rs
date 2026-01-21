//! An implementation of the Observer pattern.

pub struct Subject<T> {
    state: T,
    observers: Vec<Box<dyn Observer<T>>>,
}

impl<T> Subject<T> {
    pub fn new(state: T) -> Self {
        Self {
            state,
            observers: Vec::new(),
        }
    }

    pub fn state(&self) -> &T {
        &self.state
    }

    pub fn attach(&mut self, observer: Box<dyn Observer<T>>) -> () {
        self.observers.push(observer);
    }

    pub fn update_state(&mut self, state: T) -> () {
        self.state = state;
        self.notify();
    }

    fn notify(&self) {
        for observer in &self.observers {
            observer.update(&self.state);
        }
    }
}

pub trait Observer<T> {
    fn update(&self, state: &T);
}
