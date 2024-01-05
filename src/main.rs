use std::{
    fs::{self, File, Permissions},
    os::unix::fs::PermissionsExt,
    path::Path,
    process::{exit, Command, ExitCode},
};

const HOOK: &str = ".git/hooks/post-commit";
const TMP_DIR: &str = "/tmp/continuous-testing";
const TMP_HOOK: &str = "/tmp/continuous-testing/post-commit";
const ICON_DIR: &str = ".icon";

fn help(args: &[String]) -> i32 {
    println!("{} init         : Init the repository", args[0]);
    0
}

fn again(args: &[String]) -> ExitCode {
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

    if !Path::new(ICON_DIR).exists() {
        fs::create_dir(ICON_DIR).expect("failed to create the .icon directory");
        fs::copy("/tmp/continuous-testing/.icon/notif.png", ".icon/notif.png")
            .expect("failed to copy image");
    }

    fs::copy(TMP_HOOK, HOOK).expect("Failed to copy hook");

    let file: File = File::open(HOOK).expect("Failed to open hook");
    let mut perms: Permissions = file.metadata().expect("Failed to get").permissions();
    perms.set_mode(0o744);
    assert_eq!(perms.mode(), 0o744);
    exit(0);
}

#[cfg(target_os = "linux")]
fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 {
        if args[1].eq("init") {
            if Path::new(".git").is_dir() {
                if Path::new(HOOK).is_file() {
                    println!("Already initialized");
                    exit(0);
                } else {
                    return again(&args);
                }
            } else {
                assert!(Command::new("git")
                    .arg("init")
                    .spawn()
                    .expect("Git not found")
                    .wait()
                    .expect("")
                    .success());

                return again(&args);
            }
        }
        exit(help(&args));
    }
    exit(help(&args));
}
