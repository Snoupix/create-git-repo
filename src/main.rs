mod hl_chrome;
mod git_cli;

use crate::hl_chrome::ChromeTab;
use crate::git_cli::init_git_cmds;

fn main() {
    if std::env::args().len() < 2 {
        println!("Missing repo name arg");
        std::process::exit(1);
    }

    println!("Creating \"{}\" git repository...", std::env::args().nth(1).unwrap());
    println!("First, sign-in to github...");

    let git_url = ChromeTab::build()
        .git_login()
        .create_repo();

    init_git_cmds(git_url);

    println!("Finished successfully!");
}

pub fn format_input(input: String) -> String {
    input.replace("\r", "").replace("\n", "")
}
