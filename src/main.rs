use std::{
    path::Path,
    process::{exit, Command, ExitCode},
};

fn help(args: Vec<String>) -> i32 {
    println!(
        "{} init         : Init the repository for rust by default",
        args[0]
    );
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
                    assert!(Command::new("wget").arg("https://raw.githubusercontent.com/taishingi/continuous-testing/master/post-commit")
                    .current_dir("/tmp").spawn().expect("git not init").wait().expect("").success());
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
            } else {
                println!("not .git directory founded");
                exit(1);
            }
        }
        exit(help(args));
    }
    exit(help(args));
}
