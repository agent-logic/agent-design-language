//! PVF local_contract: deterministic committed Git identity/transition tests;
//! small CPU/disk, no provider or external custody. CI cleanup gate.
use serde_json::{json, Value};
use source_retirement_validator::*;
use std::path::Path;
fn repo() -> Repo {
    Repo::new(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.."))
}
fn candidate(base: &Tree, t: &Value) -> Tree {
    let mut c = base.clone();
    for r in rows(&t["changes"]).unwrap() {
        replace(&mut c, string(&r["path"]).unwrap(), &r["after"]);
    }
    c
}
#[test]
fn exact_cleanup_transition_and_drift() {
    let r = repo();
    let t = r.json("tools/repository_cleanup/transition.json").unwrap();
    let base = r.tree(string(&t["source_commit"]).unwrap()).unwrap();
    let c = candidate(&base, &t);
    assert_eq!(successors::cleanup(&r, &c).unwrap(), base);
    let path = string(&t["changes"][0]["path"]).unwrap();
    let mut changed = c.clone();
    changed.insert(path.into(), json!(["100644", "0".repeat(40)]));
    assert!(successors::cleanup(&r, &changed).is_err());
    let mut t2 = t.clone();
    t2["changes"]
        .as_array_mut()
        .unwrap()
        .push(t["changes"][0].clone());
    assert!(successors::transition(&c, &base, &t2).is_err());
    let mut t2 = t.clone();
    t2["changes"][0]["before"] = json!(["100644", "0".repeat(40)]);
    assert!(successors::transition(&c, &base, &t2).is_err());
}
#[test]
fn readiness_exact_resurrection_caller_and_foreign_deletion() {
    let r = repo();
    let t = r
        .json("tools/public_readiness_archive/transition.json")
        .unwrap();
    let base = r.tree(string(&t["source_commit"]).unwrap()).unwrap();
    let mut c = base.clone();
    for e in rows(&t["entries"]).unwrap() {
        c.remove(string(&e["path"]).unwrap());
    }
    let caller = &t["caller_change"];
    replace(&mut c, string(&caller["path"]).unwrap(), &caller["after"]);
    assert_eq!(successors::readiness(&r, &c).unwrap(), base);
    let path = string(&t["entries"][0]["path"]).unwrap();
    let mut changed = c.clone();
    changed.insert(path.into(), base[path].clone());
    assert!(successors::readiness(&r, &changed).is_err());
    let mut changed = c.clone();
    changed.insert(
        string(&caller["path"]).unwrap().into(),
        json!(["100644", "0".repeat(40)]),
    );
    assert!(successors::readiness(&r, &changed).is_err());
    let mut changed = c.clone();
    changed.remove("README.md");
    let normalized = successors::readiness(&r, &changed).unwrap();
    assert!(!normalized.contains_key("README.md"));
    assert_ne!(normalized, base);
    let mut t2 = t.clone();
    t2["entries"][0]["sha256"] = json!("0".repeat(64));
    assert!(successors::readiness_manifest(&r, &c, &t2).is_err());
    let mut t2 = t.clone();
    t2["entries"][0]["bytes"] = json!(0);
    assert!(successors::readiness_manifest(&r, &c, &t2).is_err());
}
#[test]
fn language_successors_exact_preimages_and_tamper() {
    let r = repo();
    for (name, _) in constants::LANGUAGE_TRANSITIONS {
        let t = r.json(&format!("docs/language-cleanup/{name}")).unwrap();
        let base = r.tree(string(&t["source_commit"]).unwrap()).unwrap();
        let c = candidate(&base, &t);
        assert_eq!(successors::language(&r, &c, &t).unwrap(), base);
        let mut changed = c.clone();
        let path = string(&t["changes"][0]["path"]).unwrap();
        changed.insert(path.into(), json!(["100644", "0".repeat(40)]));
        assert!(successors::language(&r, &changed, &t).is_err());
        let mut t2 = t.clone();
        t2["changes"][0]["before_sha256"] = json!("0".repeat(64));
        assert!(successors::language(&r, &c, &t2).is_err());
        if !rows(&t["removed"]).unwrap().is_empty() {
            let mut t2 = t.clone();
            t2["removed"] = json!([]);
            assert!(successors::language(&r, &c, &t2).is_err());
        }
        let mut changed = c.clone();
        changed.remove("README.md");
        if !rows(&t["changes"])
            .unwrap()
            .iter()
            .any(|x| x["path"] == "README.md")
        {
            assert!(!successors::language(&r, &changed, &t)
                .unwrap()
                .contains_key("README.md"));
        }
    }
}
#[test]
fn podcast_original_and_reviewed_readme_successor() {
    let r = repo();
    let t = r.json("tools/podcast_migration/transition.json").unwrap();
    let base = r.tree(string(&t["source_commit"]).unwrap()).unwrap();
    let c = candidate(&base, &t);
    assert_eq!(successors::podcast(&r, &c).unwrap(), base);
    let mut changed = c.clone();
    changed.insert("demos/README.md".into(), json!(["100644", "0".repeat(40)]));
    assert!(successors::podcast(&r, &changed).is_err());
    let successor = r
        .pinned(
            "docs/language-cleanup/podcast-successor-transition.json",
            constants::PODCAST_SUCCESSOR_PIN,
        )
        .unwrap();
    let mut current = c.clone();
    for row in rows(&successor["changes"]).unwrap() {
        replace(&mut current, string(&row["path"]).unwrap(), &row["after"]);
    }
    let normalized = successors::language(&r, &current, &successor).unwrap();
    assert_eq!(normalized, c);
    assert_eq!(successors::podcast(&r, &normalized).unwrap(), base);
    current.insert("demos/README.md".into(), json!(["100644", "0".repeat(40)]));
    assert!(successors::language(&r, &current, &successor).is_err());
}
