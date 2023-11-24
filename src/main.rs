use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::{exit, Command, ExitCode};

use eywa::{mkdir, touch_with_content};
use serde::Deserialize;
use serde::Serialize;
#[derive(Deserialize, Serialize)]
pub struct Beta {
    directory: String,
    os: String,
    memory: i32,
    cpus: i32,
    before_install: Vec<String>,
    install: Vec<String>,
    after_install: Vec<String>,
    before_script: Vec<String>,
    script: Vec<String>,
    after_success: Vec<String>,
    after_failure: Vec<String>,
}
#[derive(Deserialize, Serialize)]
pub struct Stable {
    directory: String,
    os: String,
    memory: i32,
    cpus: i32,
    before_install: Vec<String>,
    install: Vec<String>,
    after_install: Vec<String>,
    before_script: Vec<String>,
    script: Vec<String>,
    after_success: Vec<String>,
    after_failure: Vec<String>,
}
#[derive(Deserialize, Serialize)]
pub struct Nightly {
    directory: String,
    os: String,
    memory: i32,
    cpus: i32,
    before_install: Vec<String>,
    install: Vec<String>,
    after_install: Vec<String>,
    before_script: Vec<String>,
    script: Vec<String>,
    after_success: Vec<String>,
    after_failure: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Encore {
    stable: Stable,
    beta: Beta,
    nightly: Nightly,
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let arg: &String = args.get(1).expect("failed to get argument");
        if arg.eq("init") {
            touch_with_content!(".git/hooks/post-commit","#!/bin/bash\n\ngit push origin --all && git push origin --tags && again\n\nexit $?");
            assert!(Command::new("chmod")
                .arg("+x")
                .arg(".git/hooks/post-commit")
                .current_dir(".")
                .spawn()
                .expect("failed to run chmod")
                .wait()
                .expect("")
                .success());

            assert!(Command::new("wget")
                .arg("https://raw.githubusercontent.com/taishingi/zuu/master/integration.yml")
                .current_dir(".")
                .spawn()
                .expect("failed to get file")
                .wait()
                .expect("")
                .success());
            exit(0);
        }
        exit(1);
    } else {
        let f: File = std::fs::File::open("./integration.yml").expect("Could not open file.");
        let encore: Encore = serde_yaml::from_reader(f).expect("Could not read values.");

        if !Path::new("Encore").is_dir() {
            mkdir!("Encore");
            touch_with_content!(
                "Encore/Vagrantfile",
                format!(
                    "Vagrant.configure(\"2\") do |config|
                config.vm.define \"stable\" do |stable|
                    stable.vm.box = \"{}\"
                    stable.vm.hostname = \"stable\"
                    stable.vm.provider \"virtualbox\" do |vb|
                        vb.cpus = {}     
                        vb.memory = {}
                    end
                    stable.vm.provision \"shell\", path: \"stable.sh\"
                end       

        config.vm.define \"beta\" do |beta|
            beta.vm.box = \"{}\"
            beta.vm.hostname = \"beta\"
            beta.vm.provider \"virtualbox\" do |vb|
                vb.cpus = {}     
                vb.memory = {}
            end
            beta.vm.provision \"shell\", path: \"beta.sh\"
        end

        config.vm.define \"nightly\" do |nightly|
            nightly.vm.box = \"{}\"
            nightly.vm.hostname = \"nightly\"
            nightly.vm.provider \"virtualbox\" do |vb|
                vb.cpus = {}     
                vb.memory = {}
            end
            nightly.vm.provision \"shell\", path: \"nightly.sh\"
        end
    end
      ",
                    encore.stable.os,
                    encore.stable.cpus,
                    encore.stable.memory,
                    encore.beta.os,
                    encore.beta.cpus,
                    encore.beta.memory,
                    encore.nightly.os,
                    encore.nightly.cpus,
                    encore.nightly.memory
                )
                .as_str()
            );

            let mut f: File =
                File::create("Encore/stable.sh").expect("Failed to create the stable provisioner");
            f.write_all("#!/bin/bash\n\n".as_bytes()).expect("failed");

            for x in encore.stable.before_install.iter() {
                f.write_all(format!("{}\n", x).as_bytes()).expect("failed");
            }

            for x in encore.stable.install.iter() {
                f.write_all(format!("{}\n", x).as_bytes()).expect("failed");
            }

            for x in encore.stable.after_install.iter() {
                f.write_all(format!("{}\n", x).as_bytes()).expect("failed");
            }

            for x in encore.stable.before_script.iter() {
                f.write_all(format!("{}\n", x).as_bytes()).expect("failed");
            }

            for x in encore.stable.script.iter() {
                f.write_all(format!("{}\n", x).as_bytes()).expect("failed");
            }

            f.write_all("if [ \"$?\" -eq \"0\" ];then\n".as_bytes())
                .expect("failed");

            for x in encore.stable.after_success.iter() {
                f.write_all(format!("\t{}\n", x).as_bytes())
                    .expect("failed");
            }
            f.write_all("\texit 0\n".as_bytes()).expect("failed");
            f.write_all("else\n".as_bytes()).expect("failed");

            for x in encore.stable.after_failure.iter() {
                f.write_all(format!("\t{}\n", x).as_bytes())
                    .expect("failed");
            }
            f.write_all("\texit 1\n".as_bytes()).expect("failed");
            f.write_all("fi\n".as_bytes()).expect("failed");

            let mut f: File =
                File::create("Encore/beta.sh").expect("Failed to create the beta provisioner");
            f.write_all("#!/bin/bash\n\n".as_bytes()).expect("failed");

            for x in encore.beta.before_install.iter() {
                f.write_all(format!("{}\n", x).as_bytes()).expect("failed");
            }

            for x in encore.beta.install.iter() {
                f.write_all(format!("{}\n", x).as_bytes()).expect("failed");
            }

            for x in encore.beta.after_install.iter() {
                f.write_all(format!("{}\n", x).as_bytes()).expect("failed");
            }

            for x in encore.beta.before_script.iter() {
                f.write_all(format!("{}\n", x).as_bytes()).expect("failed");
            }

            for x in encore.beta.script.iter() {
                f.write_all(format!("{}\n", x).as_bytes()).expect("failed");
            }

            f.write_all("if [ \"$?\" -eq \"0\" ];then\n".as_bytes())
                .expect("failed");

            for x in encore.beta.after_success.iter() {
                f.write_all(format!("\t{}\n", x).as_bytes())
                    .expect("failed");
            }
            f.write_all("\texit 0\n".as_bytes()).expect("failed");
            f.write_all("else\n".as_bytes()).expect("failed");

            for x in encore.beta.after_failure.iter() {
                f.write_all(format!("\t{}\n", x).as_bytes())
                    .expect("failed");
            }
            f.write_all("\texit 1\n".as_bytes()).expect("failed");
            f.write_all("fi\n".as_bytes()).expect("failed");

            let mut f: File = File::create("Encore/nightly.sh")
                .expect("Failed to create the nightly provisioner");
            f.write_all("#!/bin/bash\n\n".as_bytes()).expect("failed");

            for x in encore.nightly.before_install.iter() {
                f.write_all(format!("{}\n", x).as_bytes()).expect("failed");
            }

            for x in encore.nightly.install.iter() {
                f.write_all(format!("{}\n", x).as_bytes()).expect("failed");
            }

            for x in encore.nightly.after_install.iter() {
                f.write_all(format!("{}\n", x).as_bytes()).expect("failed");
            }

            for x in encore.nightly.before_script.iter() {
                f.write_all(format!("{}\n", x).as_bytes()).expect("failed");
            }

            for x in encore.nightly.script.iter() {
                f.write_all(format!("{}\n", x).as_bytes()).expect("failed");
            }

            f.write_all("if [ \"$?\" -eq \"0\" ];then\n".as_bytes())
                .expect("failed");

            for x in encore.nightly.after_success.iter() {
                f.write_all(format!("\t{}\n", x).as_bytes())
                    .expect("failed");
            }
            f.write_all("\texit 0\n".as_bytes()).expect("failed");
            f.write_all("else\n".as_bytes()).expect("failed");

            for x in encore.nightly.after_failure.iter() {
                f.write_all(format!("{}\n", x).as_bytes()).expect("failed");
            }
            f.write_all("exit 1\n".as_bytes()).expect("failed");
            f.write_all("fi\n".as_bytes()).expect("failed");

            assert!(Command::new("vagrant")
                .arg("up")
                .current_dir("Encore")
                .spawn()
                .expect("failed to run virtual machines")
                .wait()
                .expect("")
                .success());

            assert!(Command::new("vagrant")
                .arg("destroy")
                .arg("-f")
                .current_dir("Encore")
                .spawn()
                .expect("failed to run virtual machines")
                .wait()
                .expect("")
                .success());
            fs::remove_dir_all("Encore").expect("failed");
            exit(0);
        }
        fs::remove_dir_all("Encore").expect("failed");
    }
    exit(1);
}
