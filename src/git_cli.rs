use std::fs::create_dir;
use std::io::{
    Error as IOError,
    ErrorKind,
    stdout,
    Write,
};
use std::process::Command;

fn create_directory() -> Result<(), IOError> {
    create_dir(format!(
        "{}/{}",
        std::env::current_dir().unwrap().display(),
        std::env::args().nth(1).unwrap()
    ))
}

pub fn init_git_cmds(url: String) {
    match create_directory() {
        Ok(()) => {
            println!("Created directory.");
        },
        Err(e) => {
            if e.kind() != ErrorKind::AlreadyExists {
                println!("Error: {e}");
                return
            }

            println!("Directory already exists, initializing git on it...");
        }
    };

    std::env::set_current_dir(format!(
        "{}/{}",
        std::env::current_dir().unwrap().display(),
        std::env::args().nth(1).unwrap()
    )).unwrap();

    println!("Executing git commands...");

    exec_commands(url);
}

fn exec_commands(url: String) {
    println!("$git init");

    let output = Command::new("git")
        .arg("init")
        .output()
        .expect("Failed to init git repository");

    stdout().write_all(&output.stdout).unwrap();

    println!("$git remote add origin {}", &url[..]);

    let output = Command::new("git")
        .args(["remote", "add", "origin", &url[..]])
        .output()
        .expect("Failed to add remote to git repository");

    stdout().write_all(&output.stdout).unwrap();

    println!("$git add .");

    let output = Command::new("git")
        .args(["add", "."])
        .output()
        .expect("Failed to init files to git commit");

    stdout().write_all(&output.stdout).unwrap();

    println!("$git commit -m \"First commit\"");

    let output = Command::new("git")
        .args(["commit", "-m", "\"First commit\""])
        .output()
        .expect("Failed to add remote to git repository");

    stdout().write_all(&output.stdout).unwrap();

    println!("$git push origin master");

    let output = Command::new("git")
        .args(["push", "origin", "master"])
        .output()
        .expect("Failed to add remote to git repository");

    stdout().write_all(&output.stdout).unwrap();
}
