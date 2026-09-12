//! Fixture-only isolated installation; never replaces the operator binary.
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};
static NEXT: AtomicU64 = AtomicU64::new(0);
pub struct Installation {
    pub root: PathBuf,
    pub binary: PathBuf,
}
impl Installation {
    pub fn new() -> Self {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target/sim02-installed-contract")
            .join(format!(
                "{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
        fs::create_dir_all(root.join("bin")).unwrap();
        let binary = root.join("bin/csdlc");
        fs::copy(env!("CARGO_BIN_EXE_csdlc"), &binary).unwrap();
        assert_eq!(
            fs::read(&binary).unwrap(),
            fs::read(env!("CARGO_BIN_EXE_csdlc")).unwrap()
        );
        Self { root, binary }
    }
    pub fn run(&self, cwd: &Path, args: &[&str]) -> Output {
        Command::new(&self.binary)
            .current_dir(cwd)
            .args(args)
            .output()
            .unwrap()
    }
}
impl Drop for Installation {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}
pub fn git(root: &Path, args: &[&str]) {
    let out = Command::new("git")
        .current_dir(root)
        .args(args)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

pub fn inventory(root: &Path) -> std::collections::BTreeMap<PathBuf, String> {
    fn visit(root: &Path, path: &Path, entries: &mut std::collections::BTreeMap<PathBuf, String>) {
        let metadata = fs::symlink_metadata(path).unwrap();
        let value = if metadata.file_type().is_symlink() {
            format!("link:{:?}", fs::read_link(path).unwrap())
        } else if metadata.is_dir() {
            "directory".to_owned()
        } else {
            format!("file:{}", blake3::hash(&fs::read(path).unwrap()).to_hex())
        };
        entries.insert(path.strip_prefix(root).unwrap().to_path_buf(), value);
        if metadata.is_dir() {
            for entry in fs::read_dir(path).unwrap() {
                visit(root, &entry.unwrap().path(), entries);
            }
        }
    }
    let mut entries = std::collections::BTreeMap::new();
    visit(root, root, &mut entries);
    entries
}
