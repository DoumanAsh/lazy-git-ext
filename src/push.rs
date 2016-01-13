#[macro_use(cmd, git)]
extern crate utils;

fn main() {
    match utils::get_cmd_arg(1).as_ref().map(|s| &**s) {
        None => { git!("push", "origin", "HEAD").status().unwrap(); },
        Some("force") => { git!("push", "--force", "origin", "HEAD").status().unwrap(); },
        arg @ _ => println!("Invalid amend argument: {}", arg.unwrap()),
    }
}
