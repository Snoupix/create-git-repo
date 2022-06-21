# create-git-repo

A little and simple Rust project using **headless_chrome**, **rpassword** and Rust's standard library to create a project folder, the project repository on your github and link them.

It can also be used with an existing project/directory to link and push a first commit.

Note: at the moment you need to have the github phone app to confirm your auth.

## Documentation

Move the .exe that you can find in the release folder in the parent folder of your future project.

Execute the file in your favorite shell with the proper args.

`./create-git-repo.exe repo_name`

- **repo_name** is literally the repository name of the existing folder or the new project ([respect github repository name rules](https://github.com/bcgov/BC-Policy-Framework-For-GitHub/blob/master/BC-Gov-Org-HowTo/Naming-Repos.md))

## crates.io dependencies

 - [headless_chrome](https://crates.io/crates/headless_chrome)
 - [rpassword](https://crates.io/crates/rpassword)

## Author

- [@Snoupix](https://www.github.com/Snoupix)
