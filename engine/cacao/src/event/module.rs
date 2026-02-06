use std::thread::Result;

use crossbeam_channel::{Receiver, Sender};

use super::bus::{Event, EventBus};

pub trait Module {
    type Response;
    fn new(ctx: ModuleCtx) -> Self;
    async fn run(&mut self) -> Result<Self::Response>;
}

#[derive(Debug)]
pub struct ModuleCtx {
    pub name: String,
    pub sender: Sender<Event>,
    pub receiver: Receiver<Event>,
}

impl ModuleCtx {
    pub fn new(name: &str, bus: &EventBus) -> Self {
        let sender = bus.grab();
        let receiver = bus.subscribe();

        Self {
            name: name.to_string(),
            sender,
            receiver,
        }
    }
}
