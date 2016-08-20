extern crate lazy_git_ext;

use lazy_git_ext::LazyGit;

const USAGE: &'static str = "usage: sync [options] [<remote>]

Fetch remote changes.

Options:
-a/--all     - Fetch all existing remotes.
-h/--help    - Prints help.
";

fn main() {
    let repo = lazy_git_ext::open_repo(".");

    if repo.is_none() {
        println!("Not a git repository (or any of the parent directories)");
        return;
    }

    let repo = repo.unwrap();

    let mut remote: Option<String> = None;
    let mut is_all = false;

    for arg in std::env::args().skip(1) {
        match arg.as_ref() {
            "-a" | "--all" => is_all = true,
            "-h" | "--help" => {
                println!("{}", USAGE);
                return;
            },
            arg @ _ => remote = Some(arg.to_string()),
        }
    }

    if is_all {
        let remotes = repo.remotes().expect("Cannot list all remotes");

        for remote in remotes.iter() {
            let remote = remote.unwrap();
            println!("Fetching {}", remote);
            repo.fetch_remote(&remote).expect("Cannot fetch");
        }
    }
    else {
        if remote == None {
            remote = Some("origin".to_string());
        }

        let remote = remote.unwrap();

        println!("Fetching {}", remote);

        repo.fetch_remote(&remote).expect("Cannot fetch");
    }
}
