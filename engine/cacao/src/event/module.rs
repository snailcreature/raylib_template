use std::thread::Result;

use async_trait::async_trait;
use crossbeam_channel::Receiver;

use super::bus::{Event, EventBroker};

/// Trait representing a Module that will listen for [Event]s.
#[async_trait]
pub trait Module {
    /// Type that the [Module::run] function should return.
    type Response;
    /// Create a new [Module], storing the [ModuleCtx].
    fn new(ctx: ModuleCtx) -> Self;
    /// Action to perform when an event is received.
    async fn run(&mut self) -> Result<Self::Response>;
}

/// Context for a [Module].
#[derive(Debug)]
pub struct ModuleCtx {
    /// Name of the [Module] for the [EventBroker] to use to ensure [Event]s reach the correct
    /// location.
    pub name: String,
    /// [Receiver] to poll for [Event]s.
    pub receiver: Receiver<Event>,
}

impl ModuleCtx {
    /// Create a new [ModuleCtx], [subscribing](EventBroker::subscribe) it to the given
    /// [EventBroker].
    pub fn new(name: &str, bus: &mut EventBroker) -> Self {
        let receiver = bus.subscribe(name.to_string());

        Self {
            name: name.to_string(),
            receiver,
        }
    }
}
