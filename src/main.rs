use std::{
    fs,
    path::Path,
    process::{exit, Command, ExitCode},
};

fn help(args: Vec<String>) -> i32 {
    println!("{} init         : Init the repository", args[0]);
    0
}

fn again(args: Vec<String>) -> ExitCode {
    if args.is_empty() {
        exit(help(args));
    }

    match Path::new("/tmp/continuous-testing").exists() {
        true => assert!(Command::new("git")
            .arg("pull")
            .arg("--quiet")
            .current_dir("/tmp/continuous-testing")
            .spawn()
            .expect("failed to find git")
            .wait()
            .expect("")
            .success()),
        false => assert!(Command::new("git")
            .arg("clone")
            .arg("--quiet")
            .arg("https://github.com/taishingi/continuous-testing.git")
            .current_dir("/tmp")
            .spawn()
            .expect("git not found")
            .wait()
            .expect("")
            .success()),
    }

    match Path::new(".icon").exists() {
        true => assert!(true),
        false => {
            fs::create_dir(".icon").expect("failed to create the .icon directory");
            assert!(Command::new("cp")
                .arg("/tmp/continuous-testing/.icon/notif.png")
                .arg(".icon/notif.png")
                .current_dir(".")
                .spawn()
                .expect("cp not found")
                .wait()
                .expect("")
                .success());
        }
    }

    assert!(Command::new("cp")
        .arg("/tmp/continuous-testing/post-commit")
        .arg(".git/hooks/post-commit")
        .current_dir(".")
        .spawn()
        .expect("failed to run cp")
        .wait()
        .expect("")
        .success());
    assert!(Command::new("chmod")
        .arg("+x")
        .arg(".git/hooks/post-commit")
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
