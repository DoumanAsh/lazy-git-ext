extern crate lazy_git_ext;

use lazy_git_ext::LazyGit;

const USAGE: &'static str = "usage: amend [options]

Amends changes to the current HEAD.

Options:
-a/--all     - Adds all changes and untracked files before to commit.
-h/--help    - Prints help.
";

fn main() {
    let mut is_add = false;

    for arg in std::env::args().skip(1) {
        match arg.as_ref() {
            "-a" | "--all" => is_add = true,
            "-h" | "--help" => {
                println!("{}", USAGE);
                return;
            },
            _ => {
                println!("Incorrect usage\n{}", USAGE);
                return;
            }
        }
    }

    let repo = lazy_git_ext::open_repo(".");

    if is_add {
        repo.add_all().expect("Cannot add changes");
    }

    if !repo.is_to_commit() {
        println!("No changes to amend");
        return;
    }

    repo.amend_to_head().expect("Failed to amend");
}
