//! Implementation of an Event Queue.
//!
//! [Implementing an Event Bus using
//! Rust](https://blog.digital-horror.com/blog/event-bus-in-tokio/)

use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use crossbeam_channel::{Receiver, Sender, bounded};
use serde::Serialize;

use crate::event::ron_serialise;

#[derive(Clone, Debug)]
pub struct Event {
    pub module: String,
    pub kind: EventKind,
}

impl Event {
    pub fn new(module: String, kind: EventKind) -> Self {
        Self { module, kind }
    }
}

#[derive(Clone, Debug)]
pub enum EventKind {
    Empty,
    Post(String),
}

impl EventKind {
    pub fn new_post<E: Serialize>(data: E) -> EventKind {
        let serialised = ron_serialise(data);
        Self::Post(serialised)
    }
}

pub struct EventBus {
    sender: Sender<Event>,
    receiver: Receiver<Event>,
    modules: Arc<Mutex<BTreeMap<String, Sender<Event>>>>,
    handle: Option<JoinHandle<()>>,
}

impl EventBus {
    pub fn new() -> Self {
        let (tx, rx) = bounded(100);

        Self {
            sender: tx,
            receiver: rx,
            modules: Arc::new(Mutex::new(BTreeMap::new())),
            handle: None,
        }
    }

    pub fn init(&mut self) -> () {
        let rx = self.receiver.clone();
        let modules = self.modules.clone();
        let handle = thread::spawn(move || {
            loop {
                if let Ok(event) = rx.recv() {
                    let _modules = modules.lock().unwrap();

                    let Some(module) = _modules.get(&event.module) else {
                        continue;
                    };

                    let _ = module.send(event);
                }
            }
        });

        self.handle = Some(handle);
    }

    pub fn subscribe(&mut self, name: String) -> Receiver<Event> {
        let (tx, rx) = bounded::<Event>(100);

        let mut modules = self.modules.lock().unwrap();
        modules.insert(name, tx);

        rx
    }

    pub fn publish(&self, event: Event) -> () {
        let _ = self.sender.send(event);
    }
}
