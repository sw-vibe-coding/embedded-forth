use std::process::Command;

fn capture(command: &str, args: &[&str]) -> String {
    Command::new(command)
        .args(args)
        .output()
        .ok()
        .filter(|out| out.status.success())
        .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_owned())
        .unwrap_or_else(|| "unknown".into())
}

fn main() {
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs");
    for (key, value) in [
        ("HOST", capture("hostname", &[])),
        ("COMMIT", capture("git", &["rev-parse", "--short", "HEAD"])),
        ("TIME", capture("date", &["-u", "+%Y-%m-%dT%H:%M:%SZ"])),
    ] {
        println!("cargo:rustc-env=FORTH_BUILD_{key}={value}");
    }
}
