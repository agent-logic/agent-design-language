// PVF: local deterministic historical packet consistency; small filesystem
// fixtures; no network, live merge authority, or release qualification.
use adl_public_contracts::{merge_packet as m, root};
use serde_json::{json, Value};
use std::{fs, process::Command};
fn fixture() -> tempfile::TempDir {
    let temp = tempfile::tempdir().unwrap();
    let source = root().join("docs/milestones/v0.91.3/review/merge_readiness");
    for name in m::REQUIRED_FILES {
        fs::copy(source.join(name), temp.path().join(name)).unwrap();
    }
    temp
}
#[test]
fn historical_snapshot_and_markdown_remain_bound() {
    let temp = fixture();
    let dir = temp.path();
    m::check(dir, &root()).unwrap();
    for name in m::REQUIRED_FILES {
        let path = dir.join(name);
        let original = fs::read(&path).unwrap();
        fs::remove_file(&path).unwrap();
        assert!(m::check(dir, &root()).is_err());
        fs::write(path, original).unwrap();
    }
    let path = dir.join("ct_demo_001_merge_gate.md");
    let original = fs::read_to_string(&path).unwrap();
    for marker in m::REQUIRED_GATE_SNIPPETS {
        fs::write(&path, original.replace(marker, "REMOVED")).unwrap();
        assert!(m::check(dir, &root())
            .unwrap_err()
            .to_string()
            .contains("gate record missing required snippets"));
    }
    fs::write(
        &path,
        original.replace("PR state: `MERGED`", "PR state: `OPEN`"),
    )
    .unwrap();
    assert!(m::check(dir, &root())
        .unwrap_err()
        .to_string()
        .contains("drifted"));
    fs::write(path, original).unwrap();
    let path = dir.join("ct_demo_001_merge_gate_snapshot.json");
    let original: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    let required: Value = serde_json::from_str(m::SNAPSHOT).unwrap();
    for key in required.as_object().unwrap().keys() {
        let mut changed = original.clone();
        changed[key] = Value::Null;
        fs::write(&path, changed.to_string()).unwrap();
        assert!(
            m::check(dir, &root())
                .unwrap_err()
                .to_string()
                .contains("snapshot field"),
            "{key}"
        );
    }
    let mut changed = original.clone();
    changed["pr_state"] = json!("OPEN");
    fs::write(&path, changed.to_string()).unwrap();
    assert!(m::check(dir, &root())
        .unwrap_err()
        .to_string()
        .contains("snapshot field 'pr_state'"));
    // Do not retain Python's bool/int equality loophole for a truth field.
    changed = original.clone();
    changed["human_merge_review_required"] = json!(1);
    fs::write(&path, changed.to_string()).unwrap();
    assert!(m::check(dir, &root()).is_err());
    fs::write(&path, "{").unwrap();
    assert!(m::check(dir, &root()).is_err());
}
#[test]
fn relative_paths_existence_and_cli_are_preserved() {
    let temp = fixture();
    let dir = temp.path();
    let path = dir.join("ct_demo_001_merge_gate_snapshot.json");
    let original: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    for key in [
        "evidence_bundle_rel_path",
        "review_synthesis_rel_path",
        "sor_rel_path",
    ] {
        for invalid in [
            json!(null),
            json!("/absolute"),
            json!("../outside"),
            json!("docs/../outside"),
        ] {
            let mut changed = original.clone();
            changed[key] = invalid;
            fs::write(&path, changed.to_string()).unwrap();
            assert!(m::check(dir, &root()).is_err());
        }
    }
    fs::write(&path, original.to_string()).unwrap();
    let absent = tempfile::tempdir().unwrap();
    assert!(m::check(dir, absent.path())
        .unwrap_err()
        .to_string()
        .contains("missing required tracked file"));
    let output = Command::new(env!("CARGO_BIN_EXE_adl-public-contracts"))
        .arg("merge-readiness")
        .arg(dir)
        .env("PATH", "")
        .output()
        .unwrap();
    assert!(output.status.success());
    let output = Command::new(env!("CARGO_BIN_EXE_adl-public-contracts"))
        .arg("merge-readiness")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
}
