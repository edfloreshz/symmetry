use anyhow::Result;
use git2::{BranchType, MergeOptions, PushOptions, Repository, Signature, StatusOptions};
use git2_credentials::CredentialHandler;
use native_dialog::{MessageDialog, MessageType};

use crate::configuration::Configuration;

pub struct SyncManager {
    repo: Option<Repository>,
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
    RepoNotConfigured,
}

impl SyncManager {
    pub fn new() -> Self {
        let path = Configuration::local_path().unwrap();
        let repo = Repository::open(path).ok();
        Self { repo }
    }

    pub fn sync(&self) -> Result<SyncStatus> {
        if let Some(repo) = self.repo.as_ref() {
            let mut options = StatusOptions::new();
            options.include_untracked(true);
            self.set_upstream_branch()?;

            let status = if !repo.statuses(Some(&mut options))?.is_empty() {
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
        } else {
            Ok(SyncStatus::RepoNotConfigured)
        }
    }

    fn commit(&self) -> Result<()> {
        if let Some(repo) = self.repo.as_ref() {
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
        }

        Ok(())
    }

    fn push(&self) -> Result<()> {
        if let Some(repo) = self.repo.as_ref() {
            let remote_callbacks: git2::RemoteCallbacks = Self::callbacks()?;
            let mut push_options = PushOptions::new();
            push_options.remote_callbacks(remote_callbacks);
            let mut remote = repo.find_remote("origin")?;
            remote.push(
                &["refs/heads/main:refs/heads/main"],
                Some(&mut push_options),
            )?;
        }

        Ok(())
    }

    pub fn set_upstream_branch(&self) -> Result<()> {
        if let Some(repo) = self.repo.as_ref() {
            let branch_name = "main";

            let mut branch = repo.find_branch(branch_name, BranchType::Local)?;

            // Check if the branch already has an upstream branch
            if branch.upstream().is_err() {
                let upstream_name = "origin/main"; // Replace with the desired upstream branch name

                let upstream_branch = repo.find_branch(upstream_name, BranchType::Remote)?;

                branch.set_upstream(upstream_branch.name()?)?;

                println!("Upstream branch set successfully.");
            } else {
                println!("Upstream branch is already set.");
            }
        }

        Ok(())
    }

    pub fn pull(&self) -> Result<()> {
        if let Some(repo) = self.repo.as_ref() {
            let branch_name = "main";
            let local_branch = repo.find_branch(branch_name, BranchType::Local)?;
            let upstream = repo.find_branch("origin/main", BranchType::Remote)?;

            let remote_callbacks = Self::callbacks()?;

            let mut fetch_options = git2::FetchOptions::new();
            fetch_options.remote_callbacks(remote_callbacks);

            let mut remote = repo.find_remote("origin")?;
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
                let local_commit = repo.find_commit(local_oid)?;
                let remote_commit = repo.find_commit(remote_oid)?;

                let remote_annotated_commit =
                    repo.reference_to_annotated_commit(&upstream.into_reference())?;

                let mut merge_options = MergeOptions::new();
                merge_options.fail_on_conflict(false);

                let analysis = repo.merge_analysis(&[&remote_annotated_commit])?;
                if analysis.0.is_up_to_date() {
                    println!("Already up to date, no changes to pull.");
                } else if analysis.0.is_fast_forward() {
                    let refname = format!("refs/heads/{}", branch_name);
                    let mut reference = repo.find_reference(&refname)?;
                    reference.set_target(remote_oid, "Fast-forward")?;
                    repo.set_head(&refname)?;
                    repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
                    println!("Pull completed successfully.");
                } else {
                    let index =
                        repo.merge_commits(&local_commit, &remote_commit, Some(&merge_options))?;
                    if index.has_conflicts() {
                        let yes = MessageDialog::new()
                        .set_type(MessageType::Info)
                        .set_title("Conflict")
                        .set_text("There are two conflicting configuration files, which one would you like to keep?")
                        .set_labels("Latest", "Current")
                        .show_confirm()
                        .unwrap();
                        if yes {
                            // Perform the merge with the "theirs" strategy
                            let reference = repo.find_reference("FETCH_HEAD")?; // TODO: How do I make this work?
                            let fetch_commit = repo.reference_to_annotated_commit(&reference)?;
                            let mut merge_options = MergeOptions::new();
                            repo.merge(&[&fetch_commit], Some(&mut merge_options), None)?;

                            // Commit the merge result
                            let signature = repo.signature()?;
                            let tree_oid = repo.index()?.write_tree()?;
                            let tree = repo.find_tree(tree_oid)?;
                            let parent_commit = repo.find_commit(repo.head()?.target().unwrap())?;
                            let message = "Merge changes from remote branch";
                            repo.commit(
                                Some("HEAD"),
                                &signature,
                                &signature,
                                message,
                                &tree,
                                &[&parent_commit],
                            )?;

                            println!("Merge completed successfully.");
                        } else {
                            // Perform the merge with the "ours" strategy
                            let reference = repo.find_reference("FETCH_HEAD")?;
                            let fetch_commit = repo.reference_to_annotated_commit(&reference)?;
                            let mut merge_options = MergeOptions::new();
                            repo.merge(&[&fetch_commit], Some(&mut merge_options), None)?;

                            // Perform "ours" operation
                            let mut checkout_options = git2::build::CheckoutBuilder::new();
                            checkout_options.force();

                            repo.checkout_head(Some(&mut checkout_options))?;
                            repo.index()?.write()?;

                            // Commit the merge result
                            let signature = repo.signature()?;
                            let tree_oid = repo.index()?.write_tree()?;
                            let tree = repo.find_tree(tree_oid)?;
                            let parent_commit = repo.find_commit(repo.head()?.target().unwrap())?;
                            let message = "Merge changes from remote branch with ours strategy";
                            repo.commit(
                                Some("HEAD"),
                                &signature,
                                &signature,
                                message,
                                &tree,
                                &[&parent_commit],
                            )?;

                            println!("Merge completed successfully with ours strategy.");
                        }
                    }
                }
            } else {
                println!("Already up to date, no changes to pull.");
            }
        }

        Ok(())
    }

    fn updates_pending(&self) -> Result<bool> {
        if let Some(repo) = self.repo.as_ref() {
            let local_branch = repo.find_branch("main", BranchType::Local)?;
            let remote_branch = repo.find_branch("origin/main", BranchType::Remote)?;

            let remote_callbacks = Self::callbacks()?;

            let mut fetch_options = git2::FetchOptions::new();
            fetch_options.remote_callbacks(remote_callbacks);

            let mut remote = repo.find_remote("origin")?;
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
        } else {
            Ok(false)
        }
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
