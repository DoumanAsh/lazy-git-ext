extern crate lazy_git_ext;

use lazy_git_ext::LazyGit;

const USAGE: &'static str = "usage: sync [options] [<remote>]

Fetch remote changes and merge it with current repository.

Options:
-a/--all     - Fetch all existing remotes.
-h/--help    - Prints help.
";

fn main() {
    let repo = lazy_git_ext::open_repo(".");

    let mut remote: Option<String> = None;

    for arg in std::env::args().skip(1) {
        match arg.as_ref() {
            "-h" | "--help" => {
                println!("{}", USAGE);
                return;
            }
            arg @ _ => remote = Some(arg.to_string()),
        }
    }

    if remote == None {
        remote = Some("origin".to_string());
    }

    let remote = remote.unwrap();

    println!("Fetching {}", remote);

    repo.fetch_remote(&remote).expect("Cannot fetch");
}
