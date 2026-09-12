//! Shared observation proof helpers for SIM-01 and the later installed corpus.
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

pub fn inventory(root: &Path) -> BTreeMap<PathBuf, String> {
    fn visit(root: &Path, path: &Path, entries: &mut BTreeMap<PathBuf, String>) {
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
    let mut entries = BTreeMap::new();
    visit(root, root, &mut entries);
    entries
}

pub fn install_candidate(destination: &Path) {
    fs::create_dir_all(destination.parent().unwrap()).unwrap();
    fs::copy(env!("CARGO_BIN_EXE_csdlc"), destination).unwrap();
}
