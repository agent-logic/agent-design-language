use std::process::Command;

fn main() {
    println!("cargo:rerun-if-env-changed=ADL_SOURCE_REVISION");
    let revision = std::env::var("ADL_SOURCE_REVISION")
        .ok()
        .filter(|value| is_revision(value))
        .or_else(git_revision)
        .unwrap_or_else(|| "unavailable".to_owned());
    println!("cargo:rustc-env=ADL_BUILD_SOURCE_REVISION={revision}");
}

fn git_revision() -> Option<String> {
    let root = std::env::var("CARGO_MANIFEST_DIR").ok()?;
    let output = Command::new("git")
        .args(["-C", &root, "rev-parse", "HEAD"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let revision = String::from_utf8(output.stdout).ok()?.trim().to_owned();
    is_revision(&revision).then_some(revision)
}

fn is_revision(value: &str) -> bool {
    value.len() == 40 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}
