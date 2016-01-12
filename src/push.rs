#[macro_use(cmd, git)]
extern crate utils;

fn main() {
    match utils::cmd_args().skip(1).next().as_ref().map(|s| &**s) {
        None => { git!("push", "origin", "HEAD").status().unwrap(); },
        Some("force") => { git!("push", "--force", "origin", "HEAD").status().unwrap(); },
        arg @ _ => println!("Invalid amend argument: {}", arg.unwrap()),
    }
}
