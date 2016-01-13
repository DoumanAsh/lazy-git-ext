#[macro_use(cmd, git)]
extern crate utils;

fn main() {
    let upstream = utils::get_cmd_arg(1).unwrap_or("upstream".to_string());
    git!("checkout", "master").status().unwrap();
    if git!("fetch", &upstream).status().unwrap().success() {
        git!("merge", format!("{}/master", &upstream)).status().unwrap();
    }
}
