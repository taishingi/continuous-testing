use std::{
    fs::{self, File, Permissions},
    os::unix::fs::PermissionsExt,
    path::Path,
    process::{exit, Command, ExitCode},
};

const HOOK: &str = ".git/hooks/post-commit";
const HOOK_DIR: &str = ".git/hooks";
const TMP_DIR: &str = "/tmp/continuous-testing";
const CONTINUOUS: &str = "continuous";
const TMP_HOOK: &str = "/tmp/continuous-testing/post-commit";
const ICON_DIR: &str = ".icon";

fn help(args: &[String]) -> i32 {
    println!("{} init         : Init the repository", args[0]);
    println!("{} upgrade      : Upgrade the hook file", args[0]);
    1
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

    if !Path::new(ICON_DIR).exists() {
        fs::create_dir(ICON_DIR).expect("failed to create the .icon directory");
        assert!(fs::copy("/tmp/continuous-testing/.icon/notif.png", ".icon/notif.png").is_ok());
    }

    assert!(fs::copy(TMP_HOOK, HOOK).is_ok());

    let file: File = File::open(HOOK).expect("Failed to open hook");
    let mut perms: Permissions = file.metadata().expect("Failed to get").permissions();
    perms.set_mode(0o744);
    assert_eq!(perms.mode(), 0o744);

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

    if args.len() == 2 {
        if args.get(1).expect("Failed to get argument").eq("init") {
            if Path::new(".git").is_dir() && Path::new(HOOK).exists() {
                println!("Already initialized");
                exit(0);
            }
            exit(init(&args));
        } else if !Path::new(".git").exists() {
            assert!(Command::new("git")
                .arg("init")
                .spawn()
                .expect("Git not found")
                .wait()
                .expect("")
                .success());
            exit(init(&args));
        } else if args.get(1).expect("failed to get argument").eq("upgrade")
            && Path::new(HOOK).exists()
        {
            fs::remove_file(HOOK).expect("failed to remove file");
            assert!(Command::new("wget")
                .arg("https://raw.githubusercontent.com/taishingi/continuous-testing/master/post-commit")
                .current_dir(HOOK_DIR)
                .spawn()
                .expect("Failed to upgrade the script")
                .wait()
                .expect("")
                .success());
            exit(0);
        } else {
            println!("run -> again init");
            exit(1);
        }
    }
    exit(help(&args));
}
