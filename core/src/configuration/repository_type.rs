use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Service {
    #[default]
    Git,
    Crdt,
}
