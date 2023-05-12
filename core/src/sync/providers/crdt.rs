use std::net::SocketAddr;

use anyhow::Result;
use crdts;

use crate::{
    configuration::Configuration,
    sync::{message::Message, status::Status},
    traits::synchronization::Synchronization,
};

use super::config::crdt::CrdtConfig;
struct CRDTSync {
    pub config: CrdtConfig,
    pub peers: Vec<SocketAddr>
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
    pub fn new(config: CrdtConfig) -> Self {
        Self { config: CrdtConfig { key: todo!(), id: todo!(), enabled: true }, peers: vec![]}
    }
}