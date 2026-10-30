//! PVF runtime fixture producer for installed comparison proof; inert local Git only.
use adl::codefriend::{
    evidence::{
        contracts::{Completion, Confidence, Finding, ReviewRecord, Run, Severity, CONTRACT},
        store::Store,
        Retention,
    },
    ingestion::{local, Scope},
};
use anyhow::{ensure, Result};
use std::{collections::BTreeMap, fs, path::Path, process::Command};
fn git(root: &Path, args: &[&str]) -> Result<String> {
    let o = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()?;
    ensure!(o.status.success(), "fixture_git_failed");
    Ok(String::from_utf8(o.stdout)?.trim().into())
}
fn main() -> Result<()> {
    let root = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("fixture_output_root_required"))?;
    let root = Path::new(&root);
    fs::create_dir(root)?;
    let source = root.join("source");
    fs::create_dir(&source)?;
    git(&source, &["init", "-b", "main"])?;
    git(
        &source,
        &["remote", "add", "origin", "https://example.com/owner/repo"],
    )?;
    fs::write(source.join("LICENSE"), "MIT fixture\n")?;
    let store = Store::open(&root.join("store"), || {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    })?;
    let mut revisions = Vec::new();
    for (label, content, names) in [
        (
            "baseline",
            "pub fn before() {}\n",
            vec![
                ("same", "Same"),
                ("changed", "Old prose"),
                ("resolved", "Resolved"),
            ],
        ),
        (
            "current",
            "\n\npub fn after() {}\n",
            vec![
                ("same", "Same"),
                ("changed", "New prose"),
                ("added", "Added"),
            ],
        ),
    ] {
        fs::write(source.join("lib.rs"), content)?;
        git(&source, &["add", "."])?;
        git(
            &source,
            &[
                "-c",
                "user.name=fixture",
                "-c",
                "user.email=fixture@example.com",
                "commit",
                "-m",
                label,
            ],
        )?;
        let revision = git(&source, &["rev-parse", "HEAD"])?;
        revisions.push(revision.clone());
        let packet = local::acquire(
            &source,
            "https://example.com/owner/repo",
            &revision,
            Scope {
                analysis: vec!["lib.rs".into()],
                context: vec!["LICENSE".into()],
                max_files: 2,
                max_bytes: 65536,
                max_file_bytes: 32768,
            },
        )?;
        let admission = store.admit(packet, Retention { seconds: 3600 })?;
        for (suffix, version, completion) in [
            ("", "1", Completion::Complete),
            ("-incompatible", "2", Completion::Complete),
            ("-partial", "1", Completion::Incomplete),
        ] {
            let run = Run::new(
                &admission,
                BTreeMap::from([("fixture".into(), version.into())]),
                "local".into(),
                completion,
                vec![],
            )?;
            let findings = names
                .iter()
                .map(|(name, title)| -> Result<Finding> {
                    let mut f = Finding {
                        schema: CONTRACT.into(),
                        id: String::new(),
                        repository: run.repository.clone(),
                        perspective: "fixture".into(),
                        rule: "boundary".into(),
                        semantic_anchor: (*name).into(),
                        title: (*title).into(),
                        severity: Severity::Medium,
                        rationale: "Review the dependency".into(),
                        confidence: Confidence::Unknown,
                        evidence: vec![admission
                            .evidence
                            .iter()
                            .find(|e| e.path == "lib.rs")
                            .unwrap()
                            .id
                            .clone()],
                        inference: "Known fixture assessment".into(),
                        scope_digest: run.scope_digest.clone(),
                        limitations: vec![],
                    };
                    f.id = f.identity()?;
                    Ok(f)
                })
                .collect::<Result<Vec<_>>>()?;
            let record = ReviewRecord {
                admission: admission.clone(),
                run,
                findings,
            };
            record.validate()?;
            fs::write(
                root.join(format!("{label}{suffix}.json")),
                serde_json::to_vec(&record)?,
            )?;
        }
    }
    ensure!(
        git(&source, &["status", "--porcelain"])?.is_empty(),
        "fixture_source_modified"
    );
    fs::write(
        root.join("fixture-proof.json"),
        serde_json::to_vec_pretty(
            &serde_json::json!({"schema":"codefriend.memory_fixture.v1","revisions":revisions,"producer":"production local acquisition, Store admission, Run and Finding contract constructors","source_unchanged":true}),
        )?,
    )?;
    Ok(())
}
