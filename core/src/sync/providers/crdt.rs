use anyhow::Result;

use crate::{configuration::Configuration, traits::synchronization::Synchronization};

struct CRDTSync {
    
}

impl Synchronization for CRDTSync {
    type Status = Status;
    type Message = SyncMessage;

    fn sync(&self) -> Result<Self::Status> {
        todo!()
    }

    fn handle(&self, message: Self::Message) -> Result<()> {
        match message {
            SyncMessage::Update => self.pull(),
        }
    }
}

impl CRDTSync {
    pub fn new() -> Self {
        Self {  }
    }
}