extern crate lazy_git_ext;

const USAGE: &'static str = "usage: clean-all [options]

Cleans repository.

Options:
-i/--ignored - Include ignored files.
-v/--verbose - Be verbose.
-h/--help    - Prints help.
";

use lazy_git_ext::{
    git2
};

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

fn main() {
    let mut filter = git2::STATUS_WT_NEW;
    let mut verbose = false;

    for arg in std::env::args().skip(1) {
        match arg.as_ref() {
            "-i" | "--ignored" => filter = filter | git2::STATUS_IGNORED,
            "-v" | "--verbose" => verbose = true,
            "-h" | "--help" => {
                println!("{}", USAGE);
                return;
            }
            _ => {
                println!("Incorrect usage\n{}", USAGE);
                return;
            }
        }
    }

    let repo = lazy_git_ext::open_repo(".");

    let mut options = git2::StatusOptions::new();
    options.include_untracked(true).include_ignored(true).include_unmodified(false);

    let statuses = repo.statuses(Some(&mut options)).expect("Cannot retrieve status");
    let statuses = statuses.iter()
                           .filter(|e| e.status().intersects(filter));


    let repo_path = repo.workdir().unwrap();

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
