use anyhow::Result;
use git2::{BranchType, PushOptions, Repository, Signature, StatusOptions};
use git2_credentials::CredentialHandler;

use crate::configuration::Configuration;

pub struct SyncManager {
    repo: Repository,
}

impl Default for SyncManager {
    fn default() -> Self {
        Self::new()
    }
}

pub enum SyncStatus {
    UpToDate,
    ChangesUploaded,
    NewChangesDetected,
}

impl SyncManager {
    pub fn new() -> Self {
        let path = Configuration::local_path().unwrap();
        let repo = Repository::open(path).unwrap();
        Self { repo }
    }

    pub fn sync(&self) -> Result<SyncStatus> {
        let mut options = StatusOptions::new();
        options.include_untracked(true);

        let status = if !self.repo.statuses(Some(&mut options))?.is_empty() {
            self.pull()?;
            self.commit()?;
            self.push()?;
            SyncStatus::ChangesUploaded
        } else if self.updates_pending()? {
            SyncStatus::NewChangesDetected
        } else {
            SyncStatus::UpToDate
        };

        Ok(status)
    }

    fn commit(&self) -> Result<()> {
        let author_name = "Symmetry";
        let author_email = "symmetry@proton.me";
        let commit_message = "Update configuration.";

        // Create the commit
        let signature = Signature::now(author_name, author_email)?;

        // Add all files to the index
        let mut index = self.repo.index()?;
        index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;

        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;

        if let Ok(head) = self.repo.head() {
            let head_commit = head.peel_to_commit()?;
            let parent_commit = head_commit.clone();

            self.repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                commit_message,
                &tree,
                &[&parent_commit],
            )?;
        } else {
            self.repo.commit(
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

    fn push(&self) -> Result<()> {
        let remote_callbacks: git2::RemoteCallbacks = Self::callbacks()?;
        let mut push_options = PushOptions::new();
        push_options.remote_callbacks(remote_callbacks);
        let mut remote = self.repo.find_remote("origin")?;
        remote.push(
            &["refs/heads/main:refs/heads/main"],
            Some(&mut push_options),
        )?;
        Ok(())
    }

    pub fn pull(&self) -> Result<()> {
        let branch_name = "main";
        let local_branch = self.repo.find_branch(branch_name, BranchType::Local)?;
        let upstream = self.repo.find_branch("origin/main", BranchType::Remote)?;

        let remote_callbacks = Self::callbacks()?;

        let mut fetch_options = git2::FetchOptions::new();
        fetch_options.remote_callbacks(remote_callbacks);

        let mut remote = self.repo.find_remote("origin")?;
        remote.fetch(
            &[&upstream.name()?.unwrap()],
            Some(&mut fetch_options),
            None,
        )?;

        let local_oid = local_branch
            .get()
            .target()
            .ok_or_else(|| git2::Error::from_str("Local branch has no target (commit)"))?;

        let remote_oid = upstream
            .get()
            .target()
            .ok_or_else(|| git2::Error::from_str("Remote branch has no target (commit)"))?;

        if local_oid != remote_oid {
            let local_commit = self.repo.find_commit(local_oid)?;
            let remote_commit = self.repo.find_commit(remote_oid)?;

            let remote_annotated_commit = self
                .repo
                .reference_to_annotated_commit(&upstream.into_reference())?;

            let analysis = self.repo.merge_analysis(&[&remote_annotated_commit])?;
            if analysis.0.is_up_to_date() {
                println!("Already up to date, no changes to pull.");
            } else if analysis.0.is_fast_forward() {
                let refname = format!("refs/heads/{}", branch_name);
                let mut reference = self.repo.find_reference(&refname)?;
                reference.set_target(remote_oid, "Fast-forward")?;
                self.repo.set_head(&refname)?;
                self.repo
                    .checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
                println!("Pull completed successfully.");
            } else {
                let merge_commit = self
                    .repo
                    .merge_commits(&local_commit, &remote_commit, None)?;
                // Handle merge conflicts if necessary
                println!("Pull completed successfully.");
            }
        } else {
            println!("Already up to date, no changes to pull.");
        }
        Ok(())
    }

    fn updates_pending(&self) -> Result<bool> {
        let local_branch = self.repo.find_branch("main", BranchType::Local)?;
        let remote_branch = self.repo.find_branch("origin/main", BranchType::Remote)?;

        let remote_callbacks = Self::callbacks()?;

        let mut fetch_options = git2::FetchOptions::new();
        fetch_options.remote_callbacks(remote_callbacks);

        let mut remote = self.repo.find_remote("origin")?;
        remote.fetch(
            &[&remote_branch.name()?.unwrap()],
            Some(&mut fetch_options),
            None,
        )?;

        let local_oid = local_branch
            .get()
            .target()
            .ok_or_else(|| git2::Error::from_str("Local branch has no target (commit)"))?;

        let remote_oid = remote_branch
            .get()
            .target()
            .ok_or_else(|| git2::Error::from_str("Remote branch has no target (commit)"))?;

        Ok(local_oid != remote_oid)
    }

    fn callbacks<'a>() -> Result<git2::RemoteCallbacks<'a>, anyhow::Error> {
        let git_config = git2::Config::open_default()?;
        let mut credential_handler = CredentialHandler::new(git_config);
        let mut remote_callbacks = git2::RemoteCallbacks::new();
        remote_callbacks.credentials(move |url, username, allowed| {
            credential_handler.try_next_credential(url, username, allowed)
        });
        Ok(remote_callbacks)
    }
}
