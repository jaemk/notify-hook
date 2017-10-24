use git2;
use chrono::{self, TimeZone};
use {Config};
use errors::*;


#[derive(Debug, Serialize)]
pub struct Payload {
    #[serde(rename = "ref")]
    pub ref_: String,
    pub before: String,
    pub after: String,
    pub commits: Vec<Commit>,
    pub head_commit: Commit,
    pub repository: Repo,
    pub pusher: User,
}
impl Payload {
    pub fn from(repo: &git2::Repository,
                config: &Config,
                head_commit: &git2::Commit,
                commits: &[git2::Commit],
                before: &git2::Oid,
                after: &git2::Oid,
                ref_: &str) -> Self {
        Self {
            ref_: ref_.to_string(),
            before: format!("{}", before),
            after: format!("{}", after),
            commits: commits.iter().map(|c| Commit::from(repo, c)).collect(),
            head_commit: Commit::from(repo, head_commit),
            repository: Repo::from(config),
            pusher: User::from(&head_commit.committer()),
        }
    }
}


#[derive(Debug, Serialize)]
pub struct Commit {
    pub id: String,
    pub tree_id: String,
    pub message: String,
    pub timestamp: String,
    pub author: User,
    pub committer: User,
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub modified: Vec<String>,
}
impl Commit {
    pub fn from(repo: &git2::Repository, commit: &git2::Commit) -> Self {
        let dt = chrono::Local.timestamp(commit.time().seconds(), 0);
        let mut commit_info = Self {
            id: format!("{}", commit.id()),
            tree_id: format!("{}", commit.tree_id()),
            message: commit.message().map(String::from).unwrap_or_else(|| String::new()),
            timestamp: dt.to_rfc2822(),
            author: User::from(&commit.author()),
            committer: User::from(&commit.committer()),
            added: vec![],
            removed: vec![],
            modified: vec![],
        };
        commit_info.try_add_modified(repo, commit).ok();
        commit_info
    }

    /// Try to add Added, Removed, Modified files to the `added`, `removed`, `modified` Vecs
    ///
    /// Fails if the `Commit`'s parent can't be found, any commits cannot be converted to `Tree`s,
    /// or the diff fails.
    fn try_add_modified(&mut self, repo: &git2::Repository, commit: &git2::Commit) -> Result<()> {
        let parent = commit.parent(0)?;
        let from_tree = parent.as_object().peel(git2::ObjectType::Tree)?;
        let from_tree = from_tree.as_tree().ok_or_else(|| "not a tree")?;

        let to_tree = commit.as_object().peel(git2::ObjectType::Tree)?;
        let to_tree = to_tree.as_tree().ok_or_else(|| "not a tree")?;

        let diff = repo.diff_tree_to_tree(Some(&from_tree), Some(&to_tree), None)?;
        for delta in diff.deltas() {
            let path = delta.new_file().path().and_then(::std::path::Path::to_str).map(String::from).unwrap_or_else(|| String::new());
            use git2::Delta::*;
            match delta.status() {
                Added => self.added.push(path),
                Deleted => self.removed.push(path),
                Modified | Renamed => self.modified.push(path),
                _ => continue,
            }
        }
        Ok(())
    }
}


#[derive(Debug, Serialize)]
pub struct User {
    pub name: String,
    pub email: String,
}
impl User {
    pub fn from(sig: &git2::Signature) -> Self {
        Self {
            name: sig.name().map(String::from).unwrap_or_else(|| String::new()),
            email: sig.email().map(String::from).unwrap_or_else(|| String::new()),
        }
    }
}


#[derive(Debug, Serialize)]
pub struct Repo {
    pub name: String,
}
impl Repo {
    pub fn from(config: &Config) -> Self {
        Self {
            name: config.repo_name.clone(),
        }
    }
}

