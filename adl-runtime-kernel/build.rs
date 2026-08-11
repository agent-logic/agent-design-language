use std::{env, process::Command};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Cargo provides manifest dir");
    track_git_path(&manifest_dir, "HEAD");
    if let Some(symbolic_ref) = git_stdout(&manifest_dir, &["symbolic-ref", "HEAD"]) {
        track_git_path(&manifest_dir, &symbolic_ref);
    }
    let revision = git_stdout(&manifest_dir, &["rev-parse", "HEAD"])
        .filter(|value| value.len() == 40 && value.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .unwrap_or_else(|| "unavailable".to_owned());
    println!("cargo:rustc-env=ADL_RUNTIME_SOURCE_REVISION={revision}");
}

fn track_git_path(manifest_dir: &str, path: &str) {
    if let Some(path) = git_stdout(manifest_dir, &["rev-parse", "--git-path", path]) {
        println!("cargo:rerun-if-changed={path}");
    }
}

fn git_stdout(manifest_dir: &str, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(manifest_dir)
        .args(args)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8(output.stdout)
        .ok()
        .map(|value| value.trim().to_owned())
}
