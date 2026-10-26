//! PVF local_contract: deterministic committed historical-retirement/custody
//! proof; bounded Git reads, no interpreters/network. Required #1347 delivery gate.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
    process::Command,
};
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn git(args: &[&str]) -> Vec<u8> {
    let o = Command::new("git")
        .args(args)
        .current_dir(root())
        .output()
        .unwrap();
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    o.stdout
}
fn committed(path: &str) -> Value {
    serde_json::from_slice(&git(&["show", &format!("HEAD:{path}")])).unwrap()
}
fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn tree(rev: &str) -> BTreeMap<String, Value> {
    git(&["ls-tree", "-rz", rev])
        .split(|b| *b == 0)
        .filter(|r| !r.is_empty())
        .map(|r| {
            let text = std::str::from_utf8(r).unwrap();
            let (meta, path) = text.split_once('\t').unwrap();
            let fields = meta.split_whitespace().collect::<Vec<_>>();
            (path.into(), json!([fields[0], fields[2]]))
        })
        .collect()
}
#[test]
fn all_historical_dispositions_have_committed_custody_and_no_active_copy() {
    let m = committed("docs/language-cleanup/historical-retirements.json");
    let inventory = committed("docs/language-cleanup/inventory.json");
    let transition = committed("docs/language-cleanup/historical-transition.json");
    assert_eq!(m["issue"], 1347);
    assert_eq!(m["source_commit"], transition["source_commit"]);
    let base = tree(m["source_commit"].as_str().unwrap());
    let current = tree("HEAD");
    let rows = m["entries"].as_array().unwrap();
    assert_eq!(rows.len(), 607);
    let owned = inventory["scripts"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["delivery_issue"] == 1347)
        .map(|r| (r["path"].as_str().unwrap(), r))
        .collect::<BTreeMap<_, _>>();
    assert_eq!(owned.len(), 535);
    let mut seen = BTreeSet::new();
    let mut roles = BTreeMap::<String, usize>::new();
    let mut ruby = 0;
    let mut python = 0;
    for r in rows {
        let p = r["path"].as_str().unwrap();
        assert!(seen.insert(p));
        assert_eq!(base[p], r["before"], "{p}");
        assert!(!current.contains_key(p), "retired source resurrected: {p}");
        assert_eq!(r["custody"]["commit"], m["source_commit"]);
        assert_eq!(r["custody"]["blob"], r["before"][1]);
        assert_eq!(r["custody"]["mode"], r["before"][0]);
        let bytes = git(&["cat-file", "blob", r["before"][1].as_str().unwrap()]);
        assert_eq!(sha(&bytes), r["sha256"]);
        assert_eq!(bytes.len() as u64, r["bytes"].as_u64().unwrap());
        *roles.entry(r["role"].as_str().unwrap().into()).or_default() += 1;
        if let Some(old) = owned.get(p) {
            assert_eq!(r["before"], json!([old["git_mode"], old["git_blob"]]));
            assert_eq!(r["sha256"], old["sha256"]);
            assert_eq!(r["original_inventory_member"], true);
        } else {
            assert_eq!(r["original_inventory_member"], false);
        }
        ruby += usize::from(p.ends_with(".rb"));
        python += usize::from(p.ends_with(".py"));
    }
    assert_eq!((ruby, python), (403, 132));
    assert_eq!(roles["historical_language_source"], 535);
    assert_eq!(roles["historical_interpreter_wrapper_or_payload"], 56);
    assert_eq!(roles["retired_wrapper_build_manifest"], 16);
    assert_eq!(
        seen,
        transition["removed"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect()
    );
    assert!(owned.keys().all(|p| seen.contains(p)));
    for p in current.keys() {
        assert!(!p.ends_with(".rb"), "Ruby remains: {p}");
        if p.starts_with(".csdlc/") || p.starts_with("docs/milestones/") {
            assert!(!p.ends_with(".py"), "historical Python remains: {p}");
        }
    }
    // Preservation is a claim about the retirement delivery, not a permanent
    // freeze of active milestone planning. Check the immutable retirement
    // snapshot while keeping absence/custody checks above on current HEAD.
    // Pin the actual squash-merged delivery, which remains reachable on main.
    let snapshot = "d1f3626762f7c222f38dba048319b47c802ad7ac";
    git(&["merge-base", "--is-ancestor", snapshot, "HEAD"]);
    let retired = tree(snapshot);
    for (p, id) in &base {
        if (p.starts_with(".csdlc/") || p.starts_with("docs/milestones/"))
            && !seen.contains(p.as_str())
        {
            assert_eq!(
                retired.get(p),
                Some(id),
                "protected historical record changed: {p}"
            );
        }
    }
    for p in m["preserved_documentary_candidates"].as_array().unwrap() {
        let p = p.as_str().unwrap();
        assert_eq!(retired.get(p), base.get(p));
    }
}
