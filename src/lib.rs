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
}

impl LazyGit for Repository {
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
}
