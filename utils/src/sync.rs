use anyhow::Result;
use git2::{PushOptions, Repository};
use git2_credentials::CredentialHandler;

use crate::configuration::Configuration;

pub struct SyncManager;

impl SyncManager {
    pub fn sync() -> Result<()> {
        let path = Configuration::local_path()?;
        let repo = Repository::open(path)?;

        let git_config = git2::Config::open_default()?;
        let mut credential_handler = CredentialHandler::new(git_config);
        // Set up authentication for the remote repository (if needed)
        let mut callbacks = git2::RemoteCallbacks::new();
        callbacks.credentials(move |url, username, allowed| {
            credential_handler.try_next_credential(url, username, allowed)
        });

        let mut push_options = PushOptions::new();
        push_options.remote_callbacks(callbacks);

        // Perform the push
        let mut remote = repo.find_remote("origin")?;
        remote.push(&["refs/heads/master"], Some(&mut push_options))?;

        Ok(())
    }
}
