extern crate lazy_git_ext;

use lazy_git_ext::LazyGit;

const USAGE: &'static str = "usage: commit [options] <title> [--subj <description>]

Creates new commit with title and optional description.
Title is the first line of the commit's message.
Description is separated from the title by empty line.

Options:
-a/-all      - Adds all changes and untracked files before to commit.
-h/--help    - Prints help.
";

fn main() {
    let mut is_add = false;

    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() == 0 {
        println!("{}", USAGE);
        return;
    }

    let mut commit_msg: Option<String> = None;

    for idx in 0..args.len() {
        match args[idx].as_ref() {
            "-a" | "--all" => is_add = true,
            "-h" | "--help" => {
                println!("{}", USAGE);
                return;
            }
            _ => {
                commit_msg = Some(args.iter()
                                      .skip(idx)
                                      .fold(String::new(), |acc, line| acc + line.trim() + "\n")
                                      .replace("\\n", "\n")
                                      .replace("--subj", ""));

                break;
            }
        }
    }

    if commit_msg == None {
        println!("Empty commit message.");
        println!("{}", USAGE);
        return;
    }

    let commit_msg = commit_msg.unwrap();

    let repo = lazy_git_ext::open_repo(".");
    let signature = repo.signature().expect("Cannot find signature");

    if is_add {
        repo.add_all().expect("Cannot add changes");
    }

    let tree = repo.write_tree().expect("Failed to retrieve tree to commit");

    let head = repo.head().expect("Cannot retrieve head");
    let head_oid = head.target().expect("HEAD does not point to a direct commit");
    let head_commit = repo.find_commit(head_oid).expect("Cannot retrieve HEAD's commit");

    repo.commit(Some("HEAD"), &signature, &signature, &commit_msg, &tree, &[&head_commit]).expect("Failed to commit");
}
