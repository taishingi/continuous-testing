use std::{
    path::Path,
    process::{exit, Command, ExitCode},
};

use eywa::touch_with_content;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    if args.is_empty() {
        println!("missing value");
        exit(1);
    }
    if args.len() == 2 {
        if args[1].eq("init") {
            if Path::new(".git").is_dir() {
                if Path::new(".git/hooks/post-commit").is_file() {
                    println!("Already initialized");
                    exit(1);
                } else {
                    touch_with_content!(
                        ".git/hooks/post-commit",
                        "#!/bin/bash

unset GIT_DIR
git push origin --all
git push origin --tags
if [ -d continuous ];then
    cd continuous/rust
    packer validate .
    packer build .
else
    wget https://github.com/taishingi/continuous-template/archive/refs/tags/0.0.1.zip
    unzip 0.0.1.ziz 
    mv continuous-0.0.1 continuous/
    rm 0.0.1.zip
    cd continuous/rust
    packer validate .
    packer build .
fi

"
                    );
                    assert!(Command::new("chmod")
                        .arg("+x")
                        .arg(".git/hooks/post-commit")
                        .spawn()
                        .expect("failed to run chmod")
                        .wait()
                        .expect("msg")
                        .success());
                    exit(0);
                }
            } else {
                println!("not .git directory founded");
                exit(1);
            }
        }
        exit(0);
    }
    println!("bad param");
    exit(1);
}
