//! Exact committed-source retirement and custody contracts. No deletion authority.
use anyhow::{bail, ensure, Context, Result};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
    process::Command,
};
pub mod audit;
pub mod cleanup;
pub mod constants;
pub mod custody;
pub mod family;
pub mod inventory;
pub mod overlay;
pub mod public_boundary;
pub mod successors;
pub type Tree = BTreeMap<String, Value>;
pub type Paths = BTreeSet<String>;
pub const PACKET: &str = "docs/milestones/v0.93.1/repository-decomposition/rd07-1223";
pub fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
pub fn rows(v: &Value) -> Result<&Vec<Value>> {
    v.as_array().context("expected array")
}
pub fn string(v: &Value) -> Result<&str> {
    v.as_str().context("expected string")
}
pub fn number(v: &Value) -> Result<usize> {
    Ok(v.as_u64().context("expected nonnegative integer")? as usize)
}
pub fn paths(v: &[Value]) -> Result<Paths> {
    v.iter()
        .map(|r| Ok(string(&r["path"])?.to_owned()))
        .collect()
}
pub fn strings(v: &Value) -> Result<Paths> {
    rows(v)?.iter().map(|x| Ok(string(x)?.to_owned())).collect()
}
pub fn identity(v: &Value) -> Value {
    if v.is_null() {
        Value::Null
    } else {
        json!([v["mode"], v["blob"]])
    }
}
pub fn at<'a>(tree: &'a Tree, path: &str) -> &'a Value {
    tree.get(path).unwrap_or(&Value::Null)
}
pub fn canonical_sha(v: &Value) -> Result<String> {
    Ok(sha(&serde_json::to_vec(v)?))
}
pub fn identities() -> Result<Value> {
    let raw = include_bytes!("../reviewed-identities.json");
    ensure!(
        sha(raw) == constants::IDENTITIES_SHA256,
        "reviewed identity data changed"
    );
    Ok(serde_json::from_slice(raw)?)
}
pub fn protected(path: &str, data: &Value) -> Result<bool> {
    Ok([
        ".csdlc/",
        "csdlc-v3/operator/",
        "csdlc-v2/operator/",
        "docs/templates/",
    ]
    .iter()
    .any(|p| path.starts_with(p))
        || strings(&data["PROTECTED"])?.contains(path))
}
pub struct Repo {
    pub root: PathBuf,
}
impl Repo {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }
    pub fn git(&self, args: &[&str]) -> Result<Vec<u8>> {
        let o = Command::new("git")
            .arg("--no-optional-locks")
            .args(args)
            .current_dir(&self.root)
            .output()?;
        ensure!(
            o.status.success(),
            "Git read failed: {}",
            String::from_utf8_lossy(&o.stderr)
        );
        Ok(o.stdout)
    }
    pub fn text(&self, args: &[&str]) -> Result<String> {
        Ok(String::from_utf8(self.git(args)?)?.trim().to_owned())
    }
    pub fn bytes(&self, path: &str) -> Result<Vec<u8>> {
        self.git(&["show", &format!("HEAD:{path}")])
    }
    pub fn json(&self, path: &str) -> Result<Value> {
        Ok(serde_json::from_slice(&self.bytes(path)?)?)
    }
    pub fn pinned(&self, path: &str, pin: &str) -> Result<Value> {
        let raw = self.bytes(path)?;
        ensure!(
            sha(&raw) == pin,
            "committed manifest identity changed: {path}"
        );
        Ok(serde_json::from_slice(&raw)?)
    }
    pub fn blob(&self, oid: &str) -> Result<Vec<u8>> {
        self.git(&["cat-file", "blob", oid])
    }
    pub fn tree(&self, revision: &str) -> Result<Tree> {
        let raw = self.git(&["ls-tree", "-rz", revision])?;
        let mut out = Tree::new();
        for entry in raw.split(|b| *b == 0).filter(|v| !v.is_empty()) {
            let (meta, path) = entry.split_at(
                entry
                    .iter()
                    .position(|b| *b == b'\t')
                    .context("Git tree record")?,
            );
            let fields = std::str::from_utf8(meta)?
                .split_whitespace()
                .collect::<Vec<_>>();
            ensure!(fields.len() == 3, "Git tree metadata");
            out.insert(
                std::str::from_utf8(&path[1..])?.to_owned(),
                json!([fields[0], fields[2]]),
            );
        }
        Ok(out)
    }
    pub fn packet(&self, name: &str) -> Result<Value> {
        self.json(&format!("{PACKET}/{name}"))
    }
    pub fn pinned_packet(&self, name: &str, pin: &str) -> Result<Value> {
        self.pinned(&format!("{PACKET}/{name}"), pin)
    }
}
pub fn check_references(actual: &Paths, allowed: &Paths) -> Result<()> {
    let extra = actual.difference(allowed).collect::<Vec<_>>();
    ensure!(
        extra.is_empty(),
        "unclassified retained reference: {extra:?}"
    );
    Ok(())
}
pub fn check_rollback(path: &Path, pin: &str) -> Result<()> {
    ensure!(
        path.is_file() && !path.is_symlink(),
        "rollback archive missing"
    );
    ensure!(
        sha(&std::fs::read(path)?) == pin,
        "rollback archive digest mismatch"
    );
    Ok(())
}
pub fn replace(tree: &mut Tree, path: &str, value: &Value) {
    if value.is_null() {
        tree.remove(path);
    } else {
        tree.insert(path.to_owned(), value.clone());
    }
}
pub fn require_action(action: &str, before: &Value, after: &Value) -> Result<()> {
    match action {
        "add" if before.is_null() && !after.is_null() => (),
        "modify" if !before.is_null() && !after.is_null() => (),
        "remove" if !before.is_null() && after.is_null() => (),
        _ => bail!("invalid transition"),
    };
    Ok(())
}
