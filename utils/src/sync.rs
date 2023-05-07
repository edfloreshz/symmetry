use anyhow::Result;
use git2::{PushOptions, Repository, Signature, StatusOptions};
use git2_credentials::CredentialHandler;

use crate::configuration::Configuration;

pub struct SyncManager;

impl SyncManager {
    pub fn sync() -> Result<()> {
        let path = Configuration::local_path()?;
        let repo = Repository::open(path)?;
        let mut options = StatusOptions::new();
        options.include_untracked(true);

        if !repo.statuses(Some(&mut options))?.is_empty() {
            Self::commit(&repo)?;
            Self::push(&repo)?;
        }

        Ok(())
    }

    fn commit(repo: &Repository) -> Result<()> {
        let author_name = "Symmetry";
        let author_email = "symmetry@proton.me";
        let commit_message = "Update configuration.";

        // Create the commit
        let signature = Signature::now(author_name, author_email)?;

        // Add all files to the index
        let mut index = repo.index()?;
        index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;

        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;

        if let Ok(head) = repo.head() {
            let head_commit = head.peel_to_commit()?;
            let parent_commit = head_commit.clone();

            repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                commit_message,
                &tree,
                &[&parent_commit],
            )?;
        } else {
            repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                commit_message,
                &tree,
                &[],
            )?;
        }

        Ok(())
    }

    fn push(repo: &Repository) -> Result<(), anyhow::Error> {
        let git_config = git2::Config::open_default()?;
        let mut credential_handler = CredentialHandler::new(git_config);
        let mut callbacks = git2::RemoteCallbacks::new();
        callbacks.credentials(move |url, username, allowed| {
            credential_handler.try_next_credential(url, username, allowed)
        });
        let mut push_options = PushOptions::new();
        push_options.remote_callbacks(callbacks);
        let mut remote = repo.find_remote("origin")?;
        remote.push(
            &["refs/heads/main:refs/heads/main"],
            Some(&mut push_options),
        )?;
        Ok(())
    }
}
