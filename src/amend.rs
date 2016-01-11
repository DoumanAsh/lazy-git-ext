#[macro_use(cmd, git)]
extern crate utils;

fn main() {
    utils::git_add_all();
    match utils::cmd_args().skip(2).next().as_ref().map(|s| &**s) {
        None => { git!("commit", "--amend", "--no-edit").status().unwrap(); },
        Some("edit") => { git!("commit", "--amend").status().unwrap(); },
        arg @ _ => println!("Invalid amend argument: {}", arg.unwrap()),
    }
}
