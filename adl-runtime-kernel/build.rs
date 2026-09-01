use std::{env, process::Command};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Cargo provides manifest dir");
    println!("cargo:rerun-if-env-changed=ADL_RUNTIME_SOURCE_REVISION");
    track_git_path(&manifest_dir, "HEAD");
    if let Some(symbolic_ref) = git_stdout(&manifest_dir, &["symbolic-ref", "HEAD"]) {
        track_git_path(&manifest_dir, &symbolic_ref);
    }
    let revision = match env::var("ADL_RUNTIME_SOURCE_REVISION") {
        Ok(value) if valid_revision(&value) => value,
        Ok(_) => panic!("ADL_RUNTIME_SOURCE_REVISION must be an exact lowercase Git commit"),
        Err(env::VarError::NotPresent) => git_stdout(&manifest_dir, &["rev-parse", "HEAD"])
            .filter(|value| valid_revision(value))
            .unwrap_or_else(|| "unavailable".to_owned()),
        Err(env::VarError::NotUnicode(_)) => {
            panic!("ADL_RUNTIME_SOURCE_REVISION must be valid UTF-8")
        }
    };
    println!("cargo:rustc-env=ADL_RUNTIME_SOURCE_REVISION={revision}");
}

fn valid_revision(value: &str) -> bool {
    value.len() == 40
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
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
