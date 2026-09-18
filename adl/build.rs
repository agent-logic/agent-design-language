use std::{env, path::PathBuf, process::Command};

// Compile identity into the artifact. Runtime configuration and neighboring files
// cannot relabel a copied executable. Only compilation inputs affect cleanliness.
fn main() {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .unwrap()
        .to_path_buf();
    let git = |args: &[&str]| -> Option<String> {
        let out = Command::new("git")
            .arg("-C")
            .arg(&root)
            .args(args)
            .output()
            .ok()?;
        out.status
            .success()
            .then(|| String::from_utf8_lossy(&out.stdout).trim().to_string())
    };
    let mut paths = Vec::new();
    for component in [
        "adl",
        "adl-uts",
        "adl-provider-core",
        "adl-resilience",
        "adl-runtime",
        "adl-runtime-kernel",
    ] {
        for leaf in ["Cargo.toml", "Cargo.lock", "build.rs", "src"] {
            let relative = format!("{component}/{leaf}");
            if root.join(&relative).exists() {
                println!("cargo:rerun-if-changed={}", root.join(&relative).display());
                paths.push(relative);
            }
        }
    }
    paths.push("adl/tools/adl_provider_adapter.rs".into());
    println!(
        "cargo:rerun-if-changed={}",
        root.join("adl/tools/adl_provider_adapter.rs").display()
    );
    // Cargo reruns on both detached HEAD changes and ordinary branch commits.
    for reference in [
        Some("HEAD".to_string()),
        git(&["symbolic-ref", "-q", "HEAD"]),
        Some("packed-refs".to_string()),
    ]
    .into_iter()
    .flatten()
    {
        if let Some(path) = git(&["rev-parse", "--git-path", &reference]) {
            println!("cargo:rerun-if-changed={}", root.join(path).display());
        }
    }
    let revision = git(&["rev-parse", "--verify", "HEAD"]).filter(|s| {
        s.len() == 40 && s.bytes().all(|b| b.is_ascii_hexdigit()) && s.bytes().any(|b| b != b'0')
    });
    let mut args = vec!["status", "--porcelain", "--untracked-files=all", "--"];
    args.extend(paths.iter().map(String::as_str));
    let clean = git(&args).is_some_and(|s| s.is_empty());
    println!(
        "cargo:rustc-env=CODEFRIEND_BUILD_REVISION={}",
        revision.unwrap_or_default()
    );
    println!("cargo:rustc-env=CODEFRIEND_BUILD_CLEAN={clean}");
}
