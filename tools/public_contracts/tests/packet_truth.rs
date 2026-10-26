// PVF: deterministic local packet consistency; bounded temporary filesystem;
// historical evidence only, no live execution or release qualification.
use adl_public_contracts::{packet_truth as p, root};
use serde_json::{json, Value};
use std::{fs, path::Path, process::Command};
fn packet(name: &str) -> tempfile::TempDir {
    let temp = tempfile::tempdir().unwrap();
    let source = root().join("docs/milestones/v0.91.3/review").join(name);
    for file in fs::read_dir(source).unwrap() {
        let file = file.unwrap();
        if file.path().is_file() {
            fs::copy(file.path(), temp.path().join(file.file_name())).unwrap();
        }
    }
    temp
}
fn check_cli(kind: &str, path: &Path, usage: i32) {
    let output = Command::new(env!("CARGO_BIN_EXE_adl-public-contracts"))
        .arg(kind)
        .arg(path)
        .env("PATH", "")
        .current_dir(std::env::temp_dir())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let output = Command::new(env!("CARGO_BIN_EXE_adl-public-contracts"))
        .arg(kind)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(usage));
}
#[test]
fn five_minute_sprint_console_keeps_estimated_partial_truth() {
    let temp = packet("five_minute_sprint_console");
    let dir = temp.path();
    p::console(dir).unwrap();
    check_cli("sprint-console", dir, 2);
    for (file, from, to, diagnostic) in [
        (
            "FIVE_MINUTE_SPRINT_CONSOLE_PACKET_v0.91.3.md",
            "- evidence type: `estimated`",
            "- evidence type: `measured`",
            "timebox truth must stay `estimated`",
        ),
        (
            "FIVE_MINUTE_SPRINT_CONSOLE_PACKET_v0.91.3.md",
            "- run status: `passed`",
            "- run status: `failed`",
            "packet run status",
        ),
        (
            "FIVE_MINUTE_SPRINT_CONSOLE_PACKET_v0.91.3.md",
            "`partial`",
            "complete",
            "explicitly partial",
        ),
        (
            "ct_demo_003_sprint_console_proof_report.md",
            "`partial`",
            "complete",
            "explicitly partial",
        ),
    ] {
        let path = dir.join(file);
        let original = fs::read_to_string(&path).unwrap();
        assert!(original.contains(from));
        fs::write(&path, original.replace(from, to)).unwrap();
        assert!(p::console(dir)
            .unwrap_err()
            .to_string()
            .contains(diagnostic));
        fs::write(&path, original).unwrap();
    }
    for (file, markers) in p::CONSOLE_REQUIRED_SNIPPETS {
        for marker in *markers {
            let path = dir.join(file);
            let original = fs::read_to_string(&path).unwrap();
            fs::write(&path, original.replace(marker, "REMOVED")).unwrap();
            assert!(p::console(dir).is_err());
            fs::write(&path, original).unwrap();
        }
    }
    for file in p::CONSOLE_REQUIRED_FILES {
        let path = dir.join(file);
        let original = fs::read(&path).unwrap();
        fs::remove_file(&path).unwrap();
        assert!(p::console(dir).is_err());
        fs::write(&path, original).unwrap();
    }
    let report = dir.join("ct_demo_003_sprint_console_proof_report.md");
    let original = fs::read_to_string(&report).unwrap();
    fs::write(&report, original.replace("## Result", "## Removed Result")).unwrap();
    assert!(p::console(dir).is_err());
    for (extension, marker) in [
        ("html", "Five-Minute Sprint Console"),
        ("js", "DEMO_PHASES"),
        ("css", "progress-rail"),
    ] {
        assert!(fs::read_to_string(root().join(format!(
            "demos/v0.91.3/five_minute_sprint_console_demo.{extension}"
        )))
        .unwrap()
        .contains(marker));
    }
}
#[test]
fn first_proof_readiness_preserves_upstream_identity_and_nonexecution() {
    let temp = packet("first_proof_readiness");
    let dir = temp.path();
    p::readiness(dir, &root()).unwrap();
    check_cli("first-proof-readiness", dir, 1);
    let path = dir.join("ct_demo_001_first_proof_readiness.md");
    let original = fs::read_to_string(&path).unwrap();
    for marker in p::READINESS_REQUIRED_SECTIONS
        .iter()
        .chain(p::READINESS_REQUIRED_SNIPPETS)
    {
        fs::write(&path, original.replace(marker, "REMOVED")).unwrap();
        assert!(p::readiness(dir, &root()).is_err());
    }
    fs::write(&path, format!("{original}\n.adl/docs/TBD")).unwrap();
    assert!(p::readiness(dir, &root()).is_err());
    fs::write(&path, &original).unwrap();
    let repo = tempfile::tempdir().unwrap();
    for name in p::READINESS_REQUIRED_PROOF_PATHS {
        let path = repo.path().join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::copy(root().join(name), path).unwrap();
    }
    p::readiness(dir, repo.path()).unwrap();
    for name in p::READINESS_REQUIRED_PROOF_PATHS {
        let path = repo.path().join(name);
        let original = fs::read(&path).unwrap();
        fs::remove_file(&path).unwrap();
        assert!(p::readiness(dir, repo.path()).is_err());
        fs::write(path, original).unwrap();
    }
    let manifest = repo.path().join(p::READINESS_REQUIRED_PROOF_PATHS[0]);
    let value: Value = serde_json::from_slice(&fs::read(&manifest).unwrap()).unwrap();
    for (key, _) in p::READINESS_EXPECTED_MANIFEST_PATHS {
        let mut changed = value.clone();
        changed[key] = json!("other");
        fs::write(&manifest, changed.to_string()).unwrap();
        assert!(p::readiness(dir, repo.path()).is_err());
    }
}
#[test]
fn obsmem_handoff_rejects_local_missing_and_mismatched_source_truth() {
    let temp = packet("obsmem_handoff");
    let dir = temp.path();
    p::memory(dir, &root()).unwrap();
    check_cli("obsmem-handoff", dir, 1);
    let path = dir.join("ct_demo_001_obsmem_handoff.json");
    let original: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    for pointer in [
        "/schema_version",
        "/transition_id",
        "/srp_memory_entry/entry_kind",
        "/sor_memory_entry/source_truth",
        "/outside_memory",
        "/srp_memory_entry/citations",
        "/tracked_supporting_artifacts/evidence_bundle_rel_path",
    ] {
        let mut changed = original.clone();
        *changed.pointer_mut(pointer).unwrap() = Value::Null;
        fs::write(&path, changed.to_string()).unwrap();
        assert!(p::memory(dir, &root()).is_err(), "{pointer}");
    }
    for (source, message) in [
        (
            ".adl/local/cards/srp.md",
            "tracked packet artifacts, not local-only .adl paths",
        ),
        ("docs/not-srp.md", "must end with cards/srp.md"),
        ("docs/missing/cards/srp.md", "does not exist in the repo"),
        ("../cards/srp.md", "without traversal"),
        ("C:\\cards\\srp.md", "without traversal"),
        ("", "non-empty"),
    ] {
        let mut changed = original.clone();
        changed["srp_memory_entry"]["source_record_rel_path"] = json!(source);
        changed["srp_memory_entry"]["citations"] = json!([source]);
        fs::write(&path, changed.to_string()).unwrap();
        assert!(p::memory(dir, &root())
            .unwrap_err()
            .to_string()
            .contains(message));
    }
    for citations in [
        json!([]),
        json!(["docs/other.md"]),
        json!([original["srp_memory_entry"]["source_record_rel_path"], 3]),
        json!([
            original["srp_memory_entry"]["source_record_rel_path"],
            ".adl/local"
        ]),
    ] {
        let mut changed = original.clone();
        changed["srp_memory_entry"]["citations"] = citations;
        fs::write(&path, changed.to_string()).unwrap();
        assert!(p::memory(dir, &root()).is_err());
    }
    fs::write(&path, original.to_string()).unwrap();
    for (file, markers) in p::MEMORY_REQUIRED_FILES {
        for marker in *markers {
            let file = dir.join(file);
            let original = fs::read_to_string(&file).unwrap();
            fs::write(&file, original.replace(marker, "REMOVED")).unwrap();
            assert!(p::memory(dir, &root()).is_err());
            fs::write(&file, original).unwrap();
        }
    }
}
