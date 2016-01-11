#[macro_use(cmd, git)]
extern crate utils;

fn main() {
    git!("clean", "-xfdq").status().unwrap();
}
