use marked_yaml::parse_yaml;
use notifme::Notification;
use std::env::current_dir;
use std::fs::File;
use std::io::Write;
use std::{
    fs,
    path::Path,
    process::{exit, Command, ExitCode},
};
const HOOK: &str = ".git/hooks/post-commit";
const CONTINUOUS: &str = "continuous";
const ICON_DIR: &str = ".icons";
const RELEASE: &str = "1.0.0";
fn init_hook() -> i32 {
    let mut f = File::create(HOOK).expect("");
    f.write_all(b"#!/bin/bash\n\nunset GIT_DIR\n\nagain\n\nexit $?\n\n")
        .expect("failed to write file");
    f.sync_data().expect("failed to write file");
    assert!(Command::new("chmod")
        .arg("+x")
        .arg(HOOK)
        .current_dir(".")
        .spawn()
        .expect("chmod not founded")
        .wait()
        .expect("")
        .success());
    send(project().as_str(), "Is now tracked by continuous testing")
}
fn help(args: &[String]) -> i32 {
    println!("{}              : Run the hook", args[0]);
    println!("{} --help       : Display help", args[0]);
    println!("{} init         : Init the repository", args[0]);
    0
}

fn project() -> String {
    current_dir()
        .expect("")
        .as_path()
        .iter()
        .last()
        .expect("")
        .to_str()
        .expect("failed to get current dir")
        .to_string()
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
fn send(summary: &str, body: &str) -> i32 {
    let icon = format!("{}/{ICON_DIR}/continuous.png", env!("HOME"));

    assert!(Notification::new()
        .app(project().as_str())
        .icon(icon.as_str())
        .summary(summary)
        .body(body)
        .send());
    0
}
fn init() -> i32 {
    if Path::new(".git").is_dir() && Path::new(HOOK).exists() {
        println!("Already initialized");
        exit(0);
    } else if !Path::new(".git").exists() {
        assert!(Command::new("git")
            .arg("init")
            .current_dir(".")
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

    assert_eq!(init_hook(), 0);
    assert_eq!(init_continuous(), 0);
    0
}

fn yaml(key: &str) -> String {
    let binding = parse_yaml(0, include_str!("../again.yaml")).unwrap();
    let k = binding.as_mapping().expect("").get(key);
    k.expect("").as_scalar().expect("").to_string()
}

fn gen_script() -> i32 {
    let dir = format!("{CONTINUOUS}/{}", yaml("language"));
    let repository = yaml("repository");
    let provider = yaml("provider");
    let username = yaml("username");
    assert!(Command::new("bash")
        .arg("scripts-gen")
        .arg(provider.as_str())
        .arg(username.as_str())
        .arg(repository.as_str())
        .current_dir(dir.as_str())
        .spawn()
        .expect("bash not found")
        .wait()
        .expect("")
        .success());
    0
}
fn init_continuous() -> i32 {
    assert!(Command::new("git")
        .arg("clone")
        .arg("--quiet")
        .arg("https://github.com/taishingi/continuous-template.git")
        .arg(".repo")
        .current_dir(".")
        .spawn()
        .expect("git no founded")
        .wait()
        .expect("")
        .success());
    assert!(Command::new("git")
        .arg("fetch")
        .arg("--all")
        .arg("--tags")
        .current_dir(".repo")
        .spawn()
        .expect("git no founded")
        .wait()
        .expect("")
        .success());
    assert!(Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg(env!("USER").to_string().as_str())
        .arg(RELEASE)
        .current_dir(".repo")
        .spawn()
        .expect("git no founded")
        .wait()
        .expect("")
        .success());

    if Path::new(CONTINUOUS).exists() {
        fs::remove_dir_all(CONTINUOUS).expect("Failed to remove the continuous directory");
    }
    assert!(Command::new("mv")
        .arg(".repo")
        .arg(CONTINUOUS)
        .current_dir(".")
        .spawn()
        .expect("failed to find mv")
        .wait()
        .expect("")
        .success());
    gen_script()
}

fn packer(dir: &str) -> i32 {
    assert!(Command::new("packer")
        .arg("validate")
        .arg(".")
        .current_dir(dir)
        .spawn()
        .expect("")
        .wait()
        .expect("")
        .success());
    assert!(Command::new("packer")
        .arg("build")
        .arg(".")
        .current_dir(dir)
        .spawn()
        .expect("")
        .wait()
        .expect("")
        .success());
    send(commit().as_str(), "All tests passes")
}
fn check() -> i32 {
    assert_eq!(push(), 0);
    if Path::new(CONTINUOUS).exists() {
        if yaml("language").as_str().eq("rust") {
            return packer(format!("./{CONTINUOUS}/rust").as_str());
        } else if yaml("language").as_str().eq("d") {
            return packer(format!("{CONTINUOUS}/d").as_str());
        } else if yaml("language").as_str().eq("go") {
            return packer(format!("{CONTINUOUS}/go").as_str());
        }
    }
    1
}

fn push() -> i32 {
    assert!(Command::new("git")
        .arg("push")
        .arg(yaml("remote").as_str())
        .arg("--all")
        .current_dir(".")
        .spawn()
        .expect("git push error")
        .wait()
        .expect("")
        .success());
    assert!(Command::new("git")
        .arg("push")
        .arg(yaml("remote").as_str())
        .arg("--tags")
        .current_dir(".")
        .spawn()
        .expect("git push error")
        .wait()
        .expect("")
        .success());
    0
}
fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 && args.get(1).expect("").eq("init") {
        exit(init());
    } else if args.len() == 2 && args.get(1).expect("").eq("--help") {
        exit(help(&args));
    }
    exit(check());
}
