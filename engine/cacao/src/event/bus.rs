//! Implementation of an Event Queue.
//!
//! Adapted from: [Implementing an Event Bus using
//! Rust](https://blog.digital-horror.com/blog/event-bus-in-tokio/)

use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use crossbeam_channel::{Receiver, Sender, bounded};
use serde::Serialize;

use crate::event::ron_serialise;

/// Data to send to the [EventBroker]. Includes the name of the [Module] to send the event to and
/// the [Payload] to deliver to it.
///
/// [module]: super::module::Module
#[derive(Clone, Debug)]
pub struct Event {
    /// Name of the [Module](super::module::Module) to deliver the [Payload] to.
    pub module: String,
    /// [Payload] to deliver to the [Module](super::module::Module).
    pub payload: Payload,
}

impl Event {
    pub fn new(module: String, payload: Payload) -> Self {
        Self { module, payload }
    }
}

/// Data to deliver to a [Module] via an [EventBroker].
///
/// [module]: super::module::Module
#[derive(Clone, Debug)]
pub enum Payload {
    /// An empty payload. No data.
    Empty,
    /// Data as a [String].
    ///
    /// Use [ron_serialise] to encode structs on [EventBroker::publish], and [ron_deserialise][rd]
    /// in the receiving [Module].
    ///
    /// [rd]: super::serial::ron_deserialise
    /// [module]: super::module::Module
    Post(String),
}

impl Payload {
    /// Create a new [Post](Payload::Post) from serialize-able data.
    pub fn new_post<E: Serialize>(data: E) -> Payload {
        let serialised = ron_serialise(data);
        Self::Post(serialised)
    }
}

/// Manages [Event]s, ensuring they reach the correct [Module] for it to be processed.
///
/// [module]: super::module::Module
pub struct EventBroker {
    /// Sends an [Event] to the [channel](crossbeam_channel::bounded) used by the [EventBroker] to
    /// transmit data to [Module]s.
    ///
    /// [module]: super::module::Module
    sender: Sender<Event>,
    /// Receives an [Event] from [EventBroker::sender] to be directed to the correct [Module].
    ///
    /// [module]: super::module::Module
    receiver: Receiver<Event>,
    /// A [BTreeMap] of module names to their [Sender]s, allowing fast distribution of [Event]s.
    modules: Arc<Mutex<BTreeMap<String, Sender<Event>>>>,
    /// The stored [thread handle](JoinHandle) of the [EventBroker] thread.
    handle: Option<JoinHandle<()>>,
}

impl EventBroker {
    /// Create a new [EventBroker].
    pub fn new() -> Self {
        let (tx, rx) = bounded(100);

        Self {
            sender: tx,
            receiver: rx,
            modules: Arc::new(Mutex::new(BTreeMap::new())),
            handle: None,
        }
    }

    /// Initialise this [EventBroker], creating a [thread] that uses a
    /// [receiver](Receiver) to accept [Event]s and distribute them to the correct
    /// [Module], then stores the [handle](JoinHandle) for the thread.
    ///
    /// If there is no registered module to handle the event, the event is discarded and the thread
    /// waits for a new event to be received.
    ///
    /// [module]: super::module::Module
    pub fn init(&mut self) -> () {
        let rx = self.receiver.clone();
        let modules = self.modules.clone();
        let handle = thread::spawn(move || {
            loop {
                if let Ok(event) = rx.recv() {
                    let _modules = modules.lock().unwrap();

                    let Some(module) = _modules.get(&event.module) else {
                        let _ = event;
                        continue;
                    };

                    let _ = module.send(event);
                }
            }
        });

        self.handle = Some(handle);
    }

    /// Register a [Module]'s interest in events, returning the [receiver](Receiver) the module
    /// should listen on to receive events.
    ///
    /// [module]: super::module::Module
    pub fn subscribe(&mut self, name: String) -> Receiver<Event> {
        let (tx, rx) = bounded::<Event>(100);

        let mut modules = self.modules.lock().unwrap();
        modules.insert(name, tx);

        rx
    }

    /// Send an [Event] to the [EventBroker] thread.
    pub fn publish(&self, event: Event) -> () {
        let _ = self.sender.send(event);
    }
}
