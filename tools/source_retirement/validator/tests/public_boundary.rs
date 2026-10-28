//! PVF local_contract: current committed tree + deterministic negative fixtures;
//! small CPU/disk, no historical commits or network. Required public CI gate.
use serde_json::{json, Value};
use source_retirement_validator::{public_boundary, sha, Repo, Tree};
use std::collections::BTreeMap;

fn fixture() -> (Tree, Value, BTreeMap<String, Vec<u8>>) {
    let raw = b"{\"schema\":\"custody.fixture\"}".to_vec();
    (
        BTreeMap::from([("adl-uts/Cargo.toml".into(), json!(["100644", "fixture"]))]),
        json!({"schema":"adl.public_history_boundary.v1", "required_paths":["adl-uts/Cargo.toml"],
       "retired_paths":["old-owner.rs"], "forbidden_prefixes":["runtime-v3/"],
       "manifest_sha256":{"custody.json":sha(&raw)}}),
        BTreeMap::from([("custody.json".into(), raw)]),
    )
}
#[test]
fn current_committed_public_boundary() {
    public_boundary::run(&Repo::new(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.."),
    ))
    .unwrap();
}
#[test]
fn refuses_resurrection_protected_loss_and_provenance_tamper() {
    let (tree, policy, manifests) = fixture();
    public_boundary::validate(&tree, &policy, &manifests).unwrap();
    for path in ["old-owner.rs", "runtime-v3/new.rs"] {
        let mut changed = tree.clone();
        changed.insert(path.into(), json!(["100644", "new"]));
        assert!(public_boundary::validate(&changed, &policy, &manifests).is_err());
    }
    assert!(public_boundary::validate(&Tree::new(), &policy, &manifests).is_err());
    let mut altered = manifests.clone();
    altered.insert("custody.json".into(), b"{}".to_vec());
    assert!(public_boundary::validate(&tree, &policy, &altered).is_err());
    assert!(public_boundary::validate(&tree, &policy, &BTreeMap::new()).is_err());
}
#[test]
fn refuses_malformed_duplicate_and_unbounded_policy() {
    let (tree, policy, manifests) = fixture();
    for key in ["retired_paths", "required_paths", "forbidden_prefixes"] {
        let mut bad = policy.clone();
        bad[key] = json!([]);
        assert!(public_boundary::validate(&tree, &bad, &manifests).is_err());
        let mut bad = policy.clone();
        bad[key]
            .as_array_mut()
            .unwrap()
            .push(policy[key][0].clone());
        assert!(public_boundary::validate(&tree, &bad, &manifests).is_err());
    }
    for value in [json!("bad"), json!(null)] {
        let mut bad = policy.clone();
        bad["manifest_sha256"]["custody.json"] = value;
        assert!(public_boundary::validate(&tree, &bad, &manifests).is_err());
    }
}
