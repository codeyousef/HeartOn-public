use std::process::Command;

fn main() {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap_or_else(|_| {
            std::process::Output {
                status: std::process::ExitStatus::default(),
                stdout: b"unknown".to_vec(),
                stderr: Vec::new(),
            }
        });

    let git_hash = String::from_utf8(output.stdout)
        .unwrap_or_else(|_| "unknown".to_string())
        .trim()
        .to_string();

    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    println!("cargo:rerun-if-changed=../.git/HEAD");
}
