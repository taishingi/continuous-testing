use std::{
    env::current_dir,
    path::Path,
    process::{exit, Command, ExitCode},
};

use eywa::touch_with_content;

fn configure_rust() -> Result<String, String> {
    touch_with_content!(".git/hooks/post-commit", "");
    assert!(Command::new("chmod")
        .arg("+x")
        .arg(".git/hooks/post-commit")
        .spawn()
        .expect("failed to run chmod")
        .wait()
        .expect("msg")
        .success());
    println!(
        "The {} rust project is now tracked by continuous testing",
        current_dir().expect("faied to get current dir").display()
    );
    Ok(String::from("success"))
}

fn configure_go() -> Result<String, String> {
    assert!(Command::new("chmod")
        .arg("+x")
        .arg(".git/hooks/post-commit")
        .spawn()
        .expect("failed to run chmod")
        .wait()
        .expect("msg")
        .success());
    println!(
        "The {} go project is now tracked by continuous testing",
        current_dir().expect("failed to get current dir").display()
    );
    Ok(String::from("success"))
}

fn help(args: Vec<String>) -> i32 {
    println!(
        "{} init         : Init the repository for rust by default",
        args[0]
    );
    println!("{} init rust    : Init the repository for rust", args[0]);
    println!("{} init go      : Init the repository for go", args[0]);
    0
}
fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    if args.is_empty() {
        exit(help(args));
    }
    if args.len() == 2 {
        if args[1].eq("init") {
            if Path::new(".git").is_dir() {
                if Path::new(".git/hooks/post-commit").is_file() {
                    println!("Already initialized");
                    exit(1);
                } else {
                    configure_rust().expect("failed to configure rust");
                    exit(0);
                }
            } else {
                println!("not .git directory founded");
                exit(1);
            }
        }
        exit(help(args));
    }
    if args.len() == 3 {
        if args[1].eq("init") && args[2].eq("rust") {
            configure_rust().expect("failed to configire");
            exit(0);
        } else if args[1].eq("init") && args[2].eq("go") {
            configure_go().expect("failed to configure");
            exit(0);
        }
    }
    exit(help(args));
}
