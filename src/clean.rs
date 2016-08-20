extern crate lazy_git_ext;

use lazy_git_ext::LazyGit;

const USAGE: &'static str = "usage: clean-all [options]

Cleans repository.

Options:
-i/--ignored - Include ignored files.
-v/--verbose - Be verbose.
-h/--help    - Prints help.
";

fn main() {
    let mut verbose = false;
    let mut ignored = false;

    for arg in std::env::args().skip(1) {
        match arg.as_ref() {
            "-i" | "--ignored" => ignored = true,
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

    if repo.is_none() {
        println!("Not a git repository (or any of the parent directories)");
        return;
    }

    let repo = repo.unwrap();

    repo.clean(ignored, verbose);
}
