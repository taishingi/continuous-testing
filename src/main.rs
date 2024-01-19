use notifme::Notification;
use std::env::current_dir;
use std::fs::File;
use std::{
    fs,
    path::Path,
    process::{exit, Command, ExitCode},
};

const HOOK: &str = ".git/hooks/post-commit";
const HOOK_DIR: &str = ".git/hooks";
const TMP_DIR: &str = "/tmp/continuous-testing";
const CONTINUOUS: &str = "continuous";
const ICON_DIR: &str = ".icons";

fn init_hook() {
    assert!(Command::new("wget")
        .arg("-q")
        .arg("https://raw.githubusercontent.com/taishingi/continuous-testing/master/post-commit")
        .current_dir(HOOK_DIR)
        .spawn()
        .expect("Failed to upgrade the script")
        .wait()
        .expect("")
        .success());
    assert!(Command::new("chmod")
        .arg("+x")
        .arg(HOOK)
        .current_dir(".")
        .spawn()
        .expect("failed to run chmod")
        .wait()
        .expect("")
        .success());
}
fn help(args: &[String]) -> i32 {
    println!("{}              : Run the hook", args[0]);
    println!("{} --help       : Display help", args[0]);
    println!("{} init         : Init the repository", args[0]);
    println!("{} upgrade      : Upgrade the hook file", args[0]);
    1
}
fn commit() -> String {
    if Path::new("commit").is_file() {
        fs::remove_file("commit").expect("failed to remove the file");
    }
    let f = File::create("commit").expect("failed to create the file");
    assert!(Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--pretty=%s")
        .stdout(f)
        .current_dir(".")
        .spawn()
        .expect("failed to get last commit")
        .wait()
        .expect("")
        .success());

    fs::read_to_string("commit").expect("failed to read the commit file")
}
fn send(summary: &str, body: &str) {
    let icon = format!("{}/{ICON_DIR}/continuous.png", env!("HOME"));

    assert!(Notification::new()
        .app("Continuous testing")
        .icon(icon.as_str())
        .summary(summary)
        .body(body)
        .send());
}
fn init(args: &[String]) -> i32 {
    if args.is_empty() {
        exit(help(args));
    }
    if Path::new(TMP_DIR).exists() {
        assert!(Command::new("git")
            .arg("pull")
            .arg("--quiet")
            .current_dir(TMP_DIR)
            .spawn()
            .expect("failed to find git")
            .wait()
            .expect("")
            .success());
    } else {
        assert!(Command::new("git")
            .arg("clone")
            .arg("--quiet")
            .arg("https://github.com/taishingi/continuous-testing.git")
            .arg(TMP_DIR)
            .spawn()
            .expect("git not found")
            .wait()
            .expect("")
            .success());
    }
    let icons = format!("{}/{}", env!("HOME"), ICON_DIR);
    if !Path::new(icons.as_str()).exists() {
        fs::create_dir(icons.as_str()).expect("failed to create the icon directory");
    }

    if !Path::new(format!("{icons}/continuous.png").as_str()).exists() {
        assert!(fs::copy(
            "/tmp/continuous-testing/.icon/notif.png",
            format!("{icons}/continuous.png").as_str()
        )
        .is_ok());
    }

    init_hook();

    if !Path::new(CONTINUOUS).exists() {
        assert!(Command::new("bash")
            .arg(HOOK)
            .spawn()
            .expect("Failed to start hook")
            .wait()
            .expect("msg")
            .success());
        return 0;
    }
    println!("run -> again init");
    1
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 && Path::new(HOOK).exists() {
        if Command::new("bash")
            .arg(HOOK)
            .current_dir(".")
            .spawn()
            .expect("failed to execute hook file")
            .wait()
            .expect("")
            .success()
        {
            send(commit().as_str(), "All test passes");
            exit(0);
        } else {
            send(commit().as_str(), "Tests fail");
            exit(1);
        }
    }
    if args.len() == 2 {
        if args.get(1).expect("Failed to get argument").eq("init") {
            if Path::new(".git").is_dir() && Path::new(HOOK).exists() {
                println!("Already initialized");
                exit(0);
            }
            if init(&args).eq(&0) {
                send(
                    current_dir()
                        .expect("")
                        .as_path()
                        .to_str()
                        .expect("failed to get current dir"),
                    "Is now tracked by continuous testing",
                );
                exit(0);
            }
            exit(1);
        } else if !Path::new(".git").exists() {
            assert!(Command::new("git")
                .arg("init")
                .spawn()
                .expect("Git not found")
                .wait()
                .expect("")
                .success());
            if init(&args).eq(&0) {
                send(
                    current_dir()
                        .expect("")
                        .as_path()
                        .to_str()
                        .expect("failed to get current dir"),
                    "Is now tracked by continuous testing",
                );
                exit(0);
            }
            send(
                current_dir()
                    .expect("")
                    .as_path()
                    .to_str()
                    .expect("failed to get current dir"),
                "Failed to track the repository",
            );
            exit(1);
        } else if args.get(1).expect("failed to get argument").eq("upgrade")
            && Path::new(HOOK).exists()
        {
            fs::remove_file(HOOK).expect("failed to remove file");
            init_hook();
            exit(0);
        } else if !Path::new(HOOK).exists()
            && args.get(1).expect("failed to get argument").eq("upgrade")
        {
            println!("run -> again init");
            exit(1);
        } else if args.get(1).expect("failed to get argument").eq("--help") {
            let _ = help(&args);
            exit(0);
        }
    }
    exit(help(&args));
}
