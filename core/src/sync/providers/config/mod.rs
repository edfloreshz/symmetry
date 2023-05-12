use serde::{Deserialize, Serialize};

use self::{crdt::CrdtConfig, git::GitConfig};

pub mod crdt;
pub mod git;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Services {
    pub git: GitConfig,
    pub crdt: CrdtConfig,
}
