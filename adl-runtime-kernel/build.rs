use std::{env, process::Command};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Cargo provides manifest dir");
    if let Ok(output) = Command::new("git")
        .args(["-C", &manifest_dir, "rev-parse", "--git-path", "HEAD"])
        .output()
    {
        if output.status.success() {
            if let Ok(path) = String::from_utf8(output.stdout) {
                println!("cargo:rerun-if-changed={}", path.trim());
            }
        }
    }
    let revision = Command::new("git")
        .args(["-C", &manifest_dir, "rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|value| value.trim().to_owned())
        .filter(|value| value.len() == 40 && value.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .unwrap_or_else(|| "unavailable".to_owned());
    println!("cargo:rustc-env=ADL_RUNTIME_SOURCE_REVISION={revision}");
}
