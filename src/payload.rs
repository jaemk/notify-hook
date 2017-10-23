use git2;
use {Config};


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
    pub fn from(config: &Config,
                head_commit: &git2::Commit,
                commits: &[git2::Commit],
                before: &git2::Oid,
                after: &git2::Oid,
                reph: &str) -> Self {
        Self {
            ref_: reph.to_string(),
            before: format!("{}", before),
            after: format!("{}", after),
            commits: commits.iter().map(Commit::from).collect(),
            head_commit: Commit::from(head_commit),
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
    pub fn from(commit: &git2::Commit) -> Self {
        Self {
            id: format!("{}", commit.id()),
            tree_id: format!("{}", commit.tree_id()),
            message: commit.message().map(String::from).unwrap_or_else(|| String::new()),
            timestamp: commit.time().seconds().to_string(),
            author: User::from(&commit.author()),
            committer: User::from(&commit.committer()),
            // TODO: Populate these diff lists
            added: vec![],
            removed: vec![],
            modified: vec![],
        }
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

