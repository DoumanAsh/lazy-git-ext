pub extern crate git2;

pub use git2::Repository;

#[inline(always)]
/// Opens repository.
pub fn open_repo(path: &str) -> Repository {
    Repository::discover(path).expect("Not a git repository")
}

fn remove<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<()> {
    let path = path.as_ref();

    if path.is_file() {
        std::fs::remove_file(path)
    }
    else if path.is_dir() {
        std::fs::remove_dir_all(path)
    }
    else {
        panic!("Unknown type in path");
    }
}

///Lazy Git Trait.
///
///It provides additional functionality to git2 `Repository`.
pub trait LazyGit {
    /// Adds changes or untracked files by pathspecs.
    ///
    /// Params:
    ///
    /// * `pathspecs` - List of patterns.

    fn add<P, I>(&self, pathspecs: P) -> Result<(), git2::Error> where I: git2::IntoCString, P: IntoIterator<Item=I>;

    /// Adds all changes and untracked files to Repository's index.
    fn add_all(&self) -> Result<(), git2::Error>;

    /// Writes current index to tree and retrieves it.
    fn write_tree(&self) -> Result<git2::Tree, git2::Error>;

    /// Checks if there are changes to commit.
    fn is_to_commit(&self) -> bool;

    /// Checks whether repository is dirty.
    ///
    /// Dirty means that that there are:
    /// * Untracked files.
    /// * Staged changes.
    /// * Not staged changes.
    fn is_dirty(&self) -> bool;

    /// Cleans repository.
    ///
    /// Params:
    ///
    /// * include_ignored - Whether ignored files are included.
    /// * verbose - Whether to print removed files to stdout.
    fn clean(&self, include_ignored: bool, verbose: bool);

    /// Amend changes to the current HEAD commit.
    ///
    /// Params:
    ///
    /// * msg - Commit's message.
    fn amend_to_head(&self) -> Result<git2::Oid, git2::Error>;

    /// Creates new commit on top of HEAD.
    ///
    /// Params:
    ///
    /// * msg - Commit's message.
    fn commit_to_head(&self, msg: &str) -> Result<git2::Oid, git2::Error>;

    /// Fetchs changes from remote.
    ///
    /// Params:
    ///
    /// * name - Remote's name.
    /// * opts - Fetch's options. See `git2::FetchOptions`.
    fn fetch_remote(&self, name: &str) -> Result<(), git2::Error>;

    /// Retrieves creditals for git2 callback.
    fn get_creditals(&self, url: &str, username: Option<&str>, allowed: git2::CredentialType) -> Result<git2::Cred, git2::Error>;
}

impl LazyGit for Repository {
    #[inline]
    fn add<P, I>(&self, pathspecs: P) -> Result<(), git2::Error> where I: git2::IntoCString, P: IntoIterator<Item=I> {
        let mut index = try!(self.index());
        try!(index.add_all(pathspecs, git2::ADD_DEFAULT, None));
        try!(index.write());
        Ok(())
    }

    #[inline]
    fn add_all(&self) -> Result<(), git2::Error> {
        self.add(["."].iter())
    }

    #[inline]
    fn write_tree(&self) -> Result<git2::Tree, git2::Error> {
        let mut index = try!(self.index());
        let oid = try!(index.write_tree());
        self.find_tree(oid)
    }

    fn is_to_commit(&self) -> bool {
        let filter = git2::STATUS_INDEX_NEW
                   | git2::STATUS_INDEX_MODIFIED
                   | git2::STATUS_INDEX_DELETED
                   | git2::STATUS_INDEX_RENAMED
                   | git2::STATUS_INDEX_TYPECHANGE;

        let mut options = git2::StatusOptions::new();
        options.include_untracked(false).include_ignored(false).include_unmodified(false);

        let statuses = self.statuses(Some(&mut options)).expect("Cannot retrieve status");
        let statuses = statuses.iter()
                               .filter(|e| e.status().intersects(filter));

        statuses.count() != 0
    }

    fn is_dirty(&self) -> bool {
        let mut options = git2::StatusOptions::new();
        options.include_untracked(true).include_ignored(true).include_unmodified(false);

        let statuses = self.statuses(Some(&mut options)).expect("Cannot retrieve status");
        statuses.len() != 0
    }

    fn clean(&self, include_ignored: bool, verbose: bool) {
        let mut filter = git2::STATUS_WT_NEW;

        if include_ignored {
            filter = filter | git2::STATUS_IGNORED;
        }

        let mut options = git2::StatusOptions::new();
        options.include_untracked(true).include_ignored(true).include_unmodified(false);

        let statuses = self.statuses(Some(&mut options)).expect("Cannot retrieve status");
        let statuses = statuses.iter()
                               .filter(|e| e.status().intersects(filter));


        let repo_path = self.workdir().unwrap();

        for status in statuses {
            let path = repo_path.join(status.path().unwrap());

            if verbose {
                println!("Removing '{}'", path.display());
            }

            if let Err(error) = remove(&path) {
                println!("Cannot remove '{}'. Error: '{}'", path.display(), error);
            }
        }
    }

    fn amend_to_head(&self) -> Result<git2::Oid, git2::Error> {
        let tree = try!(self.write_tree());

        let head = try!(self.head());
        let head_oid = head.target().unwrap();
        let head_commit = try!(self.find_commit(head_oid));

        head_commit.amend(Some("HEAD"), None, None, None, None, Some(&tree))
    }

    fn commit_to_head(&self, msg: &str) -> Result<git2::Oid, git2::Error> {
        let signature = try!(self.signature());

        let tree = try!(self.write_tree());

        let head = try!(self.head());
        let head_oid = head.target().unwrap();
        let head_commit = try!(self.find_commit(head_oid));

        self.commit(Some("HEAD"), &signature, &signature, &msg, &tree, &[&head_commit])
    }

    fn get_creditals(&self, url: &str, username: Option<&str>, allowed: git2::CredentialType) -> Result<git2::Cred, git2::Error> {
        let cfg = try!(self.config());

        let mut cred_helper = git2::CredentialHelper::new(url);
        cred_helper.config(&cfg);

        if allowed.contains(git2::USERNAME) {
            return git2::Cred::username(username.unwrap_or("git"));
        }

        if allowed.contains(git2::DEFAULT) {
            return git2::Cred::default();
        }

        if allowed.contains(git2::SSH_KEY) {
            let name = username.map(|s| s.to_string())
                               .or_else(|| cred_helper.username.clone())
                               .or_else(|| std::env::var("USER").ok())
                               .or_else(|| std::env::var("USERNAME").ok())
                               .or_else(|| Some("git".to_string())).unwrap();

            let result = git2::Cred::ssh_key_from_agent(&name);

            if result.is_ok() {
                return result
            }
        }

        if allowed.contains(git2::USER_PASS_PLAINTEXT) {
            if let Ok(token) = std::env::var("GH_TOKEN") {
                return git2::Cred::userpass_plaintext(&token, "");
            }
            else if let Ok(cred_helper) = git2::Cred::credential_helper(&cfg, url, username) {
                return Ok(cred_helper);
            }
        }

        Err(git2::Error::from_str("no authentication available"))
    }

    fn fetch_remote(&self, name: &str) -> Result<(), git2::Error> {
        let mut remote = try!(self.find_remote(name).or_else(|_| {
            self.remote_anonymous(name)
        }));

        let mut cb = git2::RemoteCallbacks::new();
        cb.credentials(|url, username, allowed| {
            let ret = self.get_creditals(url, username, allowed);

            if let Err(ref error) = ret {
                println!("error: {}", error)
            }

            ret
        });
        let mut opts = git2::FetchOptions::new();
        opts.remote_callbacks(cb)
            .download_tags(git2::AutotagOption::All);

        try!(remote.download(&[], Some(&mut opts)));

        remote.disconnect();

        try!(remote.update_tips(None, true, git2::AutotagOption::All, None));

        Ok(())
    }
}
