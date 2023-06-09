use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct CrdtConfig {
    pub enabled: bool,
}
