use std::{
    path::Path,
    process::{exit, Command, ExitCode},
};

fn help(args: Vec<String>) -> i32 {
    println!("{} init         : Init the repository", args[0]);
    println!(
        "{} new          : Init the repository with the gh cli command",
        args[0]
    );
    0
}

fn gh(args: Vec<String>) -> ExitCode {
    if args.is_empty() {
        exit(help(args));
    }

    assert!(Command::new("gh")
        .arg("repo")
        .arg("create")
        .current_dir(".")
        .spawn()
        .expect("gh not found")
        .wait()
        .expect("")
        .success());
    exit(0);
}

fn again(args: Vec<String>) -> ExitCode {
    if args.is_empty() {
        exit(help(args));
    }

    assert!(Command::new("wget")
        .arg("-q")
        .arg("https://raw.githubusercontent.com/taishingi/continuous-testing/master/post-commit")
        .current_dir("/tmp")
        .spawn()
        .expect("wget not found")
        .wait()
        .expect("")
        .success());
    assert!(Command::new("chmod")
        .arg("+x")
        .arg("post-commit")
        .current_dir("/tmp")
        .spawn()
        .expect("failed to run chmod")
        .wait()
        .expect("")
        .success());
    assert!(Command::new("mv")
        .arg("/tmp/post-commit")
        .arg(".git/hooks")
        .spawn()
        .expect("")
        .wait()
        .expect("")
        .success());
    assert!(Command::new("bash")
        .arg(".git/hooks/post-commit")
        .current_dir(".")
        .spawn()
        .expect("failed")
        .wait()
        .expect("")
        .success());
    exit(0);
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 && args[1].eq("new") {
        return gh(args);
    }
    if args.len() == 2 {
        if args[1].eq("init") {
            if Path::new(".git").is_dir() {
                if Path::new(".git/hooks/post-commit").is_file() {
                    println!("Already initialized");
                    exit(0);
                } else {
                    return again(args);
                }
            } else {
                assert!(Command::new("git")
                    .arg("init")
                    .spawn()
                    .expect("Git not found")
                    .wait()
                    .expect("")
                    .success());

                return again(args);
            }
        }
        exit(help(args));
    }
    exit(help(args));
}
