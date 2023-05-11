use serde::{Deserialize, Serialize};

pub mod git;

pub enum Status {
    UpToDate,
    ChangesUploaded,
    NewChangesDetected,
    RepoNotConfigured,
}

pub enum SyncMessage {
    Update,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Repository {
    #[default]
    Git,
}
