use std::thread::Result;

use async_trait::async_trait;
use crossbeam_channel::Receiver;

use super::bus::{Event, EventBroker};

#[async_trait]
pub trait Module {
    type Response;
    fn new(ctx: ModuleCtx) -> Self;
    async fn run(&mut self) -> Result<Self::Response>;
}

#[derive(Debug)]
pub struct ModuleCtx {
    pub name: String,
    pub receiver: Receiver<Event>,
}

impl ModuleCtx {
    pub fn new(name: &str, bus: &mut EventBroker) -> Self {
        let receiver = bus.subscribe(name.to_string());

        Self {
            name: name.to_string(),
            receiver,
        }
    }
}
