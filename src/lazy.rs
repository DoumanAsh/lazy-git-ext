const USAGE: &'static str = "Git Lazy Extensions:

add-all   - Adds everything to index.
root      - Prints path to root of repository.
clean-all - Cleans repository.
commit    - Performs commit.
amend     - Amend changes to the current HEAD's commit.
";

fn main() {
    println!("{}", USAGE);
}
