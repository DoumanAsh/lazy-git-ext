pub extern crate git2;

pub use git2::Repository;

#[inline(always)]
/// Opens repository.
pub fn open_repo(path: &str) -> Repository {
    Repository::discover(path).expect("Not a git repository")
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
}
