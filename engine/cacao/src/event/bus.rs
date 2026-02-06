//! Implementation of an Event Queue.
//!
//! [Implementing an Event Bus using
//! Rust](https://blog.digital-horror.com/blog/event-bus-in-tokio/)

use crossbeam_channel::{Receiver, Sender, bounded};

#[derive(Clone, Debug)]
pub struct Event {
    pub module: String,
    pub kind: EventKind,
}

#[derive(Clone, Debug)]
pub enum EventKind {
    Empty,
    Stub(String),
}

pub struct EventBus {
    pub sender: Sender<Event>,
    pub receiver: Receiver<Event>,
}

impl EventBus {
    pub fn new() -> Self {
        let (tx, rx) = bounded(100);

        Self {
            sender: tx,
            receiver: rx,
        }
    }

    pub fn grab(&self) -> Sender<Event> {
        self.sender.clone()
    }

    pub fn subscribe(&self) -> Receiver<Event> {
        self.receiver.clone()
    }

    pub fn publish(&self, event: Event) {
        let _ = self.sender.send(event);
    }
}
