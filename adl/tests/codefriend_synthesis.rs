//! PVF lane: runtime. Deterministic local synthesis proof for CodeFriend
//! review records. No provider calls, source mutation, publication, or network.

use adl::codefriend::{
    evidence::{
        contracts::{Completion, Confidence, Finding, ReviewRecord, Run, Severity, CONTRACT},
        Admission, Retention,
    },
    ingestion::{local, Scope},
    review::synthesis::{synthesize, ReviewSynthesis, SYNTHESIS_SCHEMA},
};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
};

struct Fixture {
    temp: PathBuf,
    root: PathBuf,
    revision: String,
    admission: Admission,
}

fn git(root: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap().trim().into()
}

fn unique_temp_dir() -> PathBuf {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("target/codefriend-synthesis-tests");
    fs::create_dir_all(&root).unwrap();
    loop {
        let dir = root.join(format!(
            "pid{}-n{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        match fs::create_dir(&dir) {
            Ok(()) => return dir,
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => panic!("create {}: {error}", dir.display()),
        }
    }
}

impl Fixture {
    fn new() -> Self {
        let temp = unique_temp_dir();
        let root = temp.join("repo");
        fs::create_dir(&root).unwrap();
        git(&root, &["init"]);
        git(
            &root,
            &[
                "remote",
                "add",
                "origin",
                "https://example.com/team/synthesis-target.git",
            ],
        );
        fs::create_dir(root.join("src")).unwrap();
        fs::write(
            root.join("src/lib.rs"),
            "pub fn divide(left: u32, right: u32) -> u32 { left / right }\n",
        )
        .unwrap();
        fs::write(root.join("README.md"), "synthesis fixture\n").unwrap();
        git(&root, &["add", "."]);
        git(
            &root,
            &[
                "-c",
                "user.name=fixture",
                "-c",
                "user.email=fixture@example.com",
                "commit",
                "-m",
                "fixture",
            ],
        );
        let revision = git(&root, &["rev-parse", "HEAD"]);
        let packet = local::acquire(
            &root,
            "https://example.com/team/synthesis-target",
            &revision,
            Scope {
                analysis: vec!["src/lib.rs".into()],
                context: vec!["README.md".into()],
                max_files: 10,
                max_bytes: 64 * 1024,
                max_file_bytes: 64 * 1024,
            },
        )
        .unwrap();
        let admission = Admission::new(packet, Retention { seconds: 3600 }, 100).unwrap();
        Self {
            temp,
            root,
            revision,
            admission,
        }
    }

    fn review_record(&self) -> ReviewRecord {
        let lane_versions = [
            (
                "adversarial".to_string(),
                "codefriend.review_lane.v1".to_string(),
            ),
            (
                "constitutional".to_string(),
                "codefriend.review_lane.v1".to_string(),
            ),
            (
                "correctness".to_string(),
                "codefriend.review_lane.v1".to_string(),
            ),
            (
                "security".to_string(),
                "codefriend.review_lane.v1".to_string(),
            ),
        ]
        .into_iter()
        .collect::<BTreeMap<_, _>>();
        let run = Run::new(
            &self.admission,
            lane_versions,
            "fixture:mock:reviewer".to_string(),
            Completion::Complete,
            vec![],
        )
        .unwrap();
        let evidence_id = self.admission.evidence[0].id.clone();
        let mut findings = vec![
            self.finding(
                &run,
                "correctness",
                "correctness.divide_by_zero",
                "src/lib.rs:divide",
                "Division by zero is unchecked",
                Severity::High,
                "Division can panic when right is zero",
                &evidence_id,
            ),
            self.finding(
                &run,
                "security",
                "security.untrusted_denominator",
                "src/lib.rs:divide",
                "Division by zero is unchecked",
                Severity::Medium,
                "Untrusted callers can trigger a denial-of-service panic",
                &evidence_id,
            ),
            self.finding(
                &run,
                "adversarial",
                "adversarial.review_bypass",
                "src/lib.rs:divide",
                "Distinct adversarial concern",
                Severity::Low,
                "A caller can hide this behind generated tests",
                &evidence_id,
            ),
        ];
        findings.sort_by(|a, b| a.id.cmp(&b.id));
        ReviewRecord {
            admission: self.admission.clone(),
            run,
            findings,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn finding(
        &self,
        run: &Run,
        perspective: &str,
        rule: &str,
        anchor: &str,
        title: &str,
        severity: Severity,
        rationale: &str,
        evidence_id: &str,
    ) -> Finding {
        let mut finding = Finding {
            schema: CONTRACT.to_string(),
            id: String::new(),
            repository: run.repository.clone(),
            perspective: perspective.to_string(),
            rule: rule.to_string(),
            semantic_anchor: anchor.to_string(),
            title: title.to_string(),
            severity,
            rationale: rationale.to_string(),
            confidence: Confidence::Known(80),
            evidence: vec![evidence_id.to_string()],
            inference: format!("{perspective} inference"),
            scope_digest: run.scope_digest.clone(),
            limitations: vec![format!("{perspective} limited to fixture")],
        };
        finding.id = finding.identity().unwrap();
        finding.validate(run, &self.admission).unwrap();
        finding
    }
}

#[test]
fn synthesis_deduplicates_and_preserves_disagreement() {
    let fixture = Fixture::new();
    let record = fixture.review_record();
    let synthesis = synthesize(&record).unwrap();
    assert_eq!(synthesis.schema, SYNTHESIS_SCHEMA);
    assert_eq!(synthesis.input_finding_count, 3);
    assert_eq!(synthesis.synthesized_findings.len(), 2);
    let merged = synthesis
        .synthesized_findings
        .iter()
        .find(|finding| finding.title == "Division by zero is unchecked")
        .unwrap();
    assert_eq!(merged.sources.len(), 2);
    assert_eq!(merged.severity, Severity::High);
    assert!(merged
        .disagreement
        .as_deref()
        .unwrap()
        .contains("severity disagreement retained"));
    assert!(merged
        .sources
        .iter()
        .any(|source| source.perspective == "correctness"));
    assert!(merged
        .sources
        .iter()
        .any(|source| source.perspective == "security"));
}

#[test]
fn synthesis_rejects_incomplete_lane_sets() {
    let fixture = Fixture::new();
    let mut record = fixture.review_record();
    let lane_versions = [
        (
            "adversarial".to_string(),
            "codefriend.review_lane.v1".to_string(),
        ),
        (
            "correctness".to_string(),
            "codefriend.review_lane.v1".to_string(),
        ),
        (
            "security".to_string(),
            "codefriend.review_lane.v1".to_string(),
        ),
    ]
    .into_iter()
    .collect::<BTreeMap<_, _>>();
    record.run = Run::new(
        &record.admission,
        lane_versions,
        "fixture:mock:reviewer".to_string(),
        Completion::Complete,
        vec![],
    )
    .unwrap();
    let err = synthesize(&record).unwrap_err().to_string();
    assert!(err.contains("synthesis_requires_complete_lane_set"));
}

#[test]
fn synthesis_flags_same_severity_distinct_claim_variants() {
    let fixture = Fixture::new();
    let mut record = fixture.review_record();
    let run = record.run.clone();
    let evidence_id = fixture.admission.evidence[0].id.clone();
    record.findings = vec![
        fixture.finding(
            &run,
            "correctness",
            "correctness.unchecked_zero",
            "src/lib.rs:divide",
            "Division by zero is unchecked",
            Severity::High,
            "Division panics when right is zero",
            &evidence_id,
        ),
        fixture.finding(
            &run,
            "security",
            "security.remote_denial_of_service",
            "src/lib.rs:divide",
            "Division by zero is unchecked",
            Severity::High,
            "Remote input can trigger denial of service through a panic",
            &evidence_id,
        ),
    ];
    record.findings.sort_by(|a, b| a.id.cmp(&b.id));
    record.validate().unwrap();
    let synthesis = synthesize(&record).unwrap();
    assert_eq!(synthesis.synthesized_findings.len(), 1);
    let finding = &synthesis.synthesized_findings[0];
    assert_eq!(finding.sources.len(), 2);
    assert!(finding
        .disagreement
        .as_deref()
        .unwrap()
        .contains("distinct claim variants retained"));
}

#[test]
fn installed_cli_writes_create_only_synthesis_artifacts() {
    let fixture = Fixture::new();
    let record = fixture.review_record();
    let input = fixture.temp.join("review-record.json");
    fs::write(&input, serde_json::to_vec_pretty(&record).unwrap()).unwrap();
    let out_dir = fixture.temp.join("synthesis-out");
    let output = Command::new(env!("CARGO_BIN_EXE_adl"))
        .args(["codefriend", "review", "synthesize", "--input"])
        .arg(&input)
        .arg("--out")
        .arg(&out_dir)
        .env("ADL_OBSERVABILITY_OTEL", "0")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let summary: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(summary["schema"], SYNTHESIS_SCHEMA);
    let synthesis: ReviewSynthesis =
        serde_json::from_slice(&fs::read(out_dir.join("synthesis.json")).unwrap()).unwrap();
    assert_eq!(synthesis.synthesized_findings.len(), 2);
    let manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(out_dir.join("manifest.json")).unwrap()).unwrap();
    assert_eq!(manifest["review_record_ref"], "review-record.json");
    assert_eq!(
        fs::read(out_dir.join("review-record.json")).unwrap(),
        fs::read(&input).unwrap()
    );
    let second = Command::new(env!("CARGO_BIN_EXE_adl"))
        .args(["codefriend", "review", "synthesize", "--input"])
        .arg(&input)
        .arg("--out")
        .arg(&out_dir)
        .env("ADL_OBSERVABILITY_OTEL", "0")
        .output()
        .unwrap();
    assert!(!second.status.success());
    assert!(String::from_utf8_lossy(&second.stderr)
        .contains("synthesis_output_directory_already_exists"));
    assert_eq!(git(&fixture.root, &["rev-parse", "HEAD"]), fixture.revision);
}
