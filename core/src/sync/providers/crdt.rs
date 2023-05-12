use std::net::SocketAddr;

use anyhow::Result;
use crdts::{self, List};

use crate::{
    configuration::Configuration,
    sync::{message::Message, status::Status},
    traits::synchronization::Synchronization,
};

use super::config::crdt::CrdtConfig;
struct CRDTSync {
    pub config: CrdtConfig,
    pub peers: Vec<SocketAddr>,
    pub ops: crdts::List<String, String>,
    pub curr_setting: crdts::LWWReg<String, i32>

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
        Self { config, peers: vec![], ops: List::new(), curr_setting: crdts::LWWReg { val: "S".to_string(), marker: 0 }}
    }
}