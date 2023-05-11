use anyhow::Result;
use crdts;

use crate::{
    configuration::Configuration,
    sync::{message::Message, status::Status},
    traits::synchronization::Synchronization,
};
struct CRDTSync {
    key: Option<String>,
}

impl Synchronization for CRDTSync {
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

impl CRDTSync {
    pub fn new() -> Self {
        Self { key: None }
    }
}
