#[macro_use(cmd, git)]
extern crate utils;

fn main() {
    if utils::cmd_args().skip(2).len() == 0 {
        println!("Empty commit message!");
    }
    else {
        let args: Vec<String> = utils::cmd_args().skip(2).collect();
        let mut args_split = args.split(|elem| elem.to_lowercase() == "--subj");

        //If no --subj split will have only one element.
        //Consider this case as the whole message.
        let title: String = args_split.next().unwrap().join(" ");
        if let Some(subj_array) = args_split.next() {
            let mut subj: String = subj_array.join(" ").replace("\\n", "\n").lines().fold(String::new(), |acc, line| acc + line.trim() + "\n");
            subj.pop();
            git!("commit", "-m", format!("{}\n\n{}", title, subj));
        }
        else {
            let mut title = title.replace("\\n", "\n").lines().fold(String::new(), |acc, line| acc + line.trim() + "\n");
            title.pop();
            git!("commit", "-m", title);
        }

    }
}
