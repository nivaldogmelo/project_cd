use std::{env, fs, path::Path};

use std::process::Command;

pub(super) fn install_wrapper() {
    let home = env::var("HOME").expect("HOME not set");
    let cargo_bin = env::var("CARGO_HOME")
	.map(|c| format!("{}/bin", c))
	.unwrap_or_else(|_| format!("{}/.cargo/bin", home));

    let wrapper_path = Path::new(&cargo_bin).join("pcd");
    if wrapper_path.exists() {
	println!("Wrapper already exists at {}", wrapper_path.display());
	return;
    }

    let script = r#"
#!/usr/bin/env bash

help() {
    project_cd -h
}

add() {
    if [[ -z "$OPTARG" ]]; then
	project_cd -a
    else
	project_cd -a "$OPTARG"
    fi
}

search() {
    cd "$(project_cd)"
}

remove() {
    if [[ -z "$OPTARG" ]]; then
	project_cd -r
    else
	project_cd -r "$OPTARG"
    fi
}

version() {
    project_cd -V
}

pcd() {
    if [ $# -eq 0 ]; then
	search
    fi

    while getopts "hva:rs" opt; do
	case $opt in
	    h) help;;
	    v) version;;
	    a) add;;
	    r) remove;;
	    s) search;;
	    *) help;;
	esac
    done
}
"#;

    fs::write(&wrapper_path, script).expect("Unable to write wrapper script");

    Command::new("chmod")
	.arg("+x")
	.arg(&wrapper_path)
	.output()
	.expect("Failed to make wrapper executable");

    println!("Wrapper script created at {}", wrapper_path.display());
    println!(
	"Please add `source {}` to your PATH in order to use the `pcd` command in your terminal",
	cargo_bin
    );
}
