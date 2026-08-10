use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=Cargo.toml");
    let revision = git_revision().unwrap_or_else(|| "unavailable-or-dirty".to_owned());
    println!("cargo:rustc-env=ADL_BUILD_SOURCE_REVISION={revision}");
}

fn git_revision() -> Option<String> {
    let root = std::env::var("CARGO_MANIFEST_DIR").ok()?;
    emit_git_rerun_path(&root, "HEAD");
    emit_git_rerun_path(&root, "index");
    let status = git(&root, &["status", "--porcelain", "--untracked-files=no"])?;
    if !status.trim().is_empty() {
        return None;
    }
    let revision = git(&root, &["rev-parse", "HEAD"])?;
    let revision = revision.trim().to_owned();
    is_revision(&revision).then_some(revision)
}

fn emit_git_rerun_path(root: &str, name: &str) {
    if let Some(path) = git(
        root,
        &["rev-parse", "--path-format=absolute", "--git-path", name],
    ) {
        println!("cargo:rerun-if-changed={}", path.trim());
    }
}

fn git(root: &str, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8(output.stdout).ok()
}

fn is_revision(value: &str) -> bool {
    value.len() == 40 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}
