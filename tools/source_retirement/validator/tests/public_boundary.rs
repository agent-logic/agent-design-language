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
    for path in [
        "old-owner.rs",
        "runtime-v3/new.rs",
        "new.rb",
        "docs/milestones/future/new.py",
    ] {
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

fn archive_fixture() -> (Tree, Value, BTreeMap<String, Vec<u8>>) {
    use std::io::Write;
    let (tree, mut policy, mut manifests) = fixture();
    let path = "docs/milestones/v0.8/old-custody.json";
    let digest = sha(b"{\"schema\":\"historical.custody\"}");
    let catalog = json!({"schema":"adl.historical_file_catalog.v1","files":{
        path:{"sha256":digest,"blob":"a".repeat(40),"mode":"100644","bytes":31}
    }});
    let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    encoder
        .write_all(&serde_json::to_vec(&catalog).unwrap())
        .unwrap();
    let packed = encoder.finish().unwrap();
    policy["schema"] = json!("adl.public_history_boundary.v2");
    policy["archive_catalog"] = json!("archive.json.gz");
    policy["manifest_sha256"]["archive.json.gz"] = json!(sha(&packed));
    policy["archived_manifest_sha256"] = json!({path:digest});
    manifests.insert("archive.json.gz".into(), packed);
    (tree, policy, manifests)
}

#[test]
fn archived_provenance_keeps_current_boundary_and_original_digests() {
    let (tree, policy, manifests) = archive_fixture();
    public_boundary::validate(&tree, &policy, &manifests).unwrap();
    let path = "docs/milestones/v0.8/old-custody.json";
    let mut restored = tree.clone();
    restored.insert(path.into(), json!(["100644", "a".repeat(40)]));
    assert!(public_boundary::validate(&restored, &policy, &manifests).is_err());
    let mut tampered = policy.clone();
    tampered["archived_manifest_sha256"][path] = json!("b".repeat(64));
    assert!(public_boundary::validate(&tree, &tampered, &manifests).is_err());
    let mut missing = policy.clone();
    missing["archived_manifest_sha256"] = json!({});
    assert!(public_boundary::validate(&tree, &missing, &manifests).is_err());
    let mut bad = policy.clone();
    bad["archived_manifest_sha256"] = json!({"../escape": "a".repeat(64)});
    assert!(public_boundary::validate(&tree, &bad, &manifests).is_err());
    let mut unpinned = policy.clone();
    unpinned["archive_catalog"] = json!("missing.json.gz");
    assert!(public_boundary::validate(&tree, &unpinned, &manifests).is_err());
    let mut corrupt = manifests.clone();
    corrupt.insert("archive.json.gz".into(), b"not gzip".to_vec());
    assert!(public_boundary::validate(&tree, &policy, &corrupt).is_err());
    // Archiving provenance must never waive resurrection or required-source guards.
    for path in ["old-owner.rs", "runtime-v3/new.rs"] {
        let mut bad = tree.clone();
        bad.insert(path.into(), json!(["100644", "new"]));
        assert!(public_boundary::validate(&bad, &policy, &manifests).is_err());
    }
    assert!(public_boundary::validate(&Tree::new(), &policy, &manifests).is_err());
}
