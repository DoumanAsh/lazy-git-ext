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

    /// Amend changes to current HEAD.
    ///
    /// Params:
    ///
    /// * msg - Commit's message.
    fn amend_to_head(&self) -> Result<git2::Oid, git2::Error>;
    /// Adds commit to current HEAD.
    ///
    /// Params:
    ///
    /// * msg - Commit's message.
    fn commit_to_head(&self, msg: &str) -> Result<git2::Oid, git2::Error>;
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
}
