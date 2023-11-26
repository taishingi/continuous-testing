use std::{
    env::current_dir,
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
    if [ \"$?\" == 0 ];then
        rsbadges --label continuous-testing --msg success --msg-color \"#00ff00\" --save-to-svg-at  ./continuous.svg --style flat
        exit 0                    
    else
        rsbadges --label continuous-testing --msg failure --msg-color \"#ff0000\" --save-to-svg-at  ./continuous.svg --style flat
        exit 1                
    fi
else
    wget https://github.com/taishingi/continuous-template/archive/refs/tags/0.0.1.zip
    unzip 0.0.1.zip
    mv -f continuous-template-0.0.1/ continuous/
    rm 0.0.1.zip
    cd continuous/rust
    packer validate .
    packer build .
    if [ \"$?\" == 0 ];then
        rsbadges --label continuous-testing --msg success --msg-color \"#00ff00\" --save-to-svg-at  ./continuous.svg --style flat
        exit 0                
    else
        rsbadges --label continuous-testing --msg failure --msg-color \"#ff0000\" --save-to-svg-at  ./continuous.svg --style flat
        exit 1                
    fi
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
                    println!(
                        "The {} is now tracked by continuous testing",
                        current_dir().expect("faied to get current dir").display()
                    );
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
