use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum RepositoryType {
    #[default]
    Git,
}
