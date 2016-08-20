extern crate lazy_git_ext;

const USAGE: &'static str = "usage: root [options]

Prints root directory of repository.

Options:
-h/--help    - Prints help.
";

fn main() {
    for arg in std::env::args().skip(1) {
        match arg.as_ref() {
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

    let path = repo.workdir().unwrap();

    println!("{}", path.display());
}
