use std::thread::JoinHandle;

use crossbeam_channel::Receiver;

use super::bus::{Event, EventBroker};

/// Trait representing a Module that will listen for [Event]s.
pub trait Module {
    /// Type that the [Module::run] function should return.
    type Response;
    /// Create a new [Module], storing the [ModuleCtx].
    fn new(ctx: ModuleCtx) -> Self;
    /// Start a thread to listen for and handle [Event]s sent to this Module.
    ///
    /// Use a [ModuleCtx::receiver] to listen for events, use [ron_deserialise] to extract data
    /// from the [Payload], then perform some action based on that data.
    ///
    /// See [EventBroker::init] for an example of how this could work.
    ///
    /// [ron_deserialise]: super::serial::ron_deserialise
    /// [payload]: super::bus::Payload
    fn run(&mut self) -> JoinHandle<()>;
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
