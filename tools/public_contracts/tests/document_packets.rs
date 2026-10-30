//! PVF local_contract: deterministic supported-demo packet checks and filesystem
//! refusals; temporary disk and bounded Cargo helper invocation, no paid/network
//! activity. Demo labels and publication blocks remain source-defined.
use adl_public_contracts::document_packets as packets;
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
fn root() -> PathBuf {
    adl_public_contracts::root()
}
fn write(path: &Path, text: &str) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}
fn read_json(path: &Path) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}
fn demo(script: &str, output: &Path, traps: &Path, log: &Path) -> std::process::Output {
    Command::new("bash")
        .arg(root().join("adl/tools").join(script))
        .arg(output)
        .env(
            "PATH",
            format!(
                "{}:{}",
                traps.display(),
                std::env::var("PATH").unwrap_or_default()
            ),
        )
        .env("ADL_INTERPRETER_TRAP_LOG", log)
        .output()
        .unwrap()
}
fn setup_traps(root: &Path) -> (PathBuf, PathBuf) {
    let traps = root.join("traps");
    fs::create_dir_all(&traps).unwrap();
    let log = root.join("interpreter-called");
    for program in ["python", "python3", "ruby"] {
        adl_public_contracts::executable(
            &traps.join(program),
            "#!/bin/sh\nprintf 'called\\n' >> \"$ADL_INTERPRETER_TRAP_LOG\"\nexit 98\n",
        )
        .unwrap();
    }
    (traps, log)
}
fn preservation(script: &str, output: &Path, temp: &Path, traps: &Path, log: &Path) {
    write(&output.join("sentinel"), "preserve me");
    assert!(!demo(script, output, traps, log).status.success());
    assert_eq!(
        fs::read_to_string(output.join("sentinel")).unwrap(),
        "preserve me"
    );
    #[cfg(unix)]
    {
        let dangling = temp.join("dangling");
        std::os::unix::fs::symlink(temp.join("missing"), &dangling).unwrap();
        assert!(!demo(script, &dangling, traps, log).status.success());
        assert!(fs::symlink_metadata(dangling)
            .unwrap()
            .file_type()
            .is_symlink());
        assert!(!temp.join("missing").exists());
    }
}
#[test]
fn architecture_demo_preserves_packet_and_negative_contracts() {
    let t = tempfile::tempdir().unwrap();
    let out = t.path().join("architecture");
    let (traps, log) = setup_traps(t.path());
    let script = "demo_v090_architecture_document_generation.sh";
    let result = demo(script, &out, &traps, &log);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    packets::architecture(&root(), Some(&out)).unwrap();
    assert!(!log.exists());
    let manifest = out.join("architecture_generation_manifest.json");
    let original = read_json(&manifest);
    assert!(original["skills_represented"]
        .as_array()
        .unwrap()
        .contains(&json!("diagram-author")));
    assert!(original["missing_skill_dependencies"]
        .as_array()
        .unwrap()
        .contains(&json!("documentation-specialist")));
    for (key, value) in [
        ("schema_version", json!("wrong")),
        ("classification", json!("non_proving")),
        ("diagram_sources", json!([])),
        ("skills_represented", json!([])),
    ] {
        let mut changed = original.clone();
        changed[key] = value;
        write(&manifest, &changed.to_string());
        assert!(packets::architecture(&root(), Some(&out)).is_err(), "{key}");
    }
    write(&manifest, &original.to_string());
    let note = out.join("architecture_review_note.md");
    let original = fs::read_to_string(&note).unwrap();
    for leak in [
        "/Users/alice/private.txt",
        "/private/tmp/adl-leak",
        "OPENAI_API_KEY=synthetic",
        "GOOGLE_API_KEY=synthetic",
    ] {
        write(&note, &format!("{original}\n{leak}"));
        assert!(packets::architecture(&root(), Some(&out)).is_err());
    }
    write(&note, &original);
    fs::remove_file(&note).unwrap();
    assert!(packets::architecture(&root(), Some(&out)).is_err());
    write(&note, &original);
    preservation(script, &out, t.path(), &traps, &log);
    assert!(!log.exists());
}
#[test]
fn showcase_demo_preserves_non_proving_publication_and_staged_lane_truth() {
    let t = tempfile::tempdir().unwrap();
    let out = t.path().join("showcase");
    let (traps, log) = setup_traps(t.path());
    let script = "demo_v090_codebuddy_review_showcase.sh";
    let result = demo(script, &out, &traps, &log);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    packets::showcase(&out).unwrap();
    assert!(!log.exists());
    let manifest = out.join("run_manifest.json");
    let original = read_json(&manifest);
    assert!(original["skill_lanes"]
        .as_array()
        .unwrap()
        .iter()
        .any(|v| v["skill"] == "repo-packet-builder" && v["status"] == "represented"));
    for (key, value) in [
        ("schema_version", json!("wrong")),
        ("classification", json!("proving")),
        ("publication_allowed", json!(true)),
        ("skill_lanes", json!([])),
    ] {
        let mut changed = original.clone();
        changed[key] = value;
        write(&manifest, &changed.to_string());
        assert!(packets::showcase(&out).is_err(), "{key}");
    }
    write(&manifest, &original.to_string());
    let mut changed = original.clone();
    let lanes = changed["skill_lanes"].as_array_mut().unwrap();
    lanes
        .iter_mut()
        .find(|v| v["skill"] == "review-quality-evaluator")
        .unwrap()["status"] = json!("represented");
    write(&manifest, &changed.to_string());
    assert!(packets::showcase(&out).is_err());
    write(&manifest, &original.to_string());
    let report = out.join("final_report.md");
    let original = fs::read_to_string(&report).unwrap();
    assert!(original.contains("Finding CB-SEC-001: [P1]"));
    for leak in [
        "/Users/alice/private.txt",
        "/private/tmp/codebuddy-leak",
        "OPENAI_API_KEY=synthetic",
    ] {
        write(&report, &format!("{original}\n{leak}"));
        assert!(packets::showcase(&out).is_err());
    }
    write(
        &report,
        &original.replace("publication_allowed=false", "publication_allowed=true"),
    );
    assert!(packets::showcase(&out).is_err());
    write(&report, &original);
    let quality = out.join("quality_evaluation.md");
    let original = fs::read_to_string(&quality).unwrap();
    write(
        &quality,
        &original.replace("Staged pending #2070", "complete"),
    );
    assert!(packets::showcase(&out).is_err());
    write(&quality, &original);
    preservation(script, &out, t.path(), &traps, &log);
    assert!(!log.exists());
}
#[test]
fn architecture_document_sections_diagrams_and_evidence_remain_required() {
    let t = tempfile::tempdir().unwrap();
    for rel in packets::REQUIRED_DOCS {
        write(
            &t.path().join(rel),
            &fs::read_to_string(root().join(rel)).unwrap(),
        );
    }
    for diagram in packets::REQUIRED_DIAGRAMS {
        let rel = format!("docs/architecture/diagrams/{diagram}");
        write(
            &t.path().join(&rel),
            &fs::read_to_string(root().join(&rel)).unwrap(),
        );
    }
    packets::architecture(t.path(), None).unwrap();
    let p = t.path().join("docs/architecture/ADL_ARCHITECTURE.md");
    let original = fs::read_to_string(&p).unwrap();
    for marker in ["## Known Gaps", "adl/src/trace.rs"] {
        write(&p, &original.replace(marker, "removed"));
        assert!(packets::architecture(t.path(), None).is_err());
    }
    write(&p, &original);
    let p = t
        .path()
        .join("docs/architecture/diagrams/system_context.mmd");
    let original = fs::read_to_string(&p).unwrap();
    write(&p, &original.replace("%% Assumptions:", "removed"));
    assert!(packets::architecture(t.path(), None).is_err());
    fs::remove_file(p).unwrap();
    assert!(packets::architecture(t.path(), None).is_err());
}
#[test]
fn leak_scan_checks_boundaries_without_reprinting_sensitive_matches() {
    for bad in [
        "/Users/alice/data",
        " /home/bob/file",
        "`/tmp/file`",
        "/var/run/file",
        r"C:\private\file",
        "Bearer synthetic_token",
        "sk-abcdefgh",
        "ANTHROPIC_API_KEY=x",
    ] {
        let error = packets::scan(bad, true).unwrap_err().to_string();
        assert!(!error.contains(bad));
    }
    for good in [
        "docs/home/user/file.md",
        "repository/tmp/fixture",
        "/tmp",
        "source.rs",
        "artifacts/example.json",
    ] {
        packets::scan(good, true).unwrap();
    }
    assert!(packets::ordered("## Second\n## First", &["## First", "## Second"]).is_err());
}
