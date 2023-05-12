use anyhow::Result;
use crdts;

use crate::{
    sync::{message::Message, status::Status},
    traits::synchronization::Synchronization,
};
pub struct CrdtSync {
    key: Option<String>,
}

impl Synchronization for CrdtSync {
    type Status = Status;
    type Message = Message;

    fn sync(&self) -> Result<Self::Status> {
        todo!()
    }

    fn handle(&self, message: Self::Message) -> Result<()> {
        match message {
            Message::Update => todo!(),
        }
    }
}

impl CrdtSync {
    pub fn new() -> Self {
        Self { key: None }
    }
}
