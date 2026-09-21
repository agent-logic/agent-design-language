//! PVF deterministic local integration: real Git acquisition and stage owners.
//! No provider/network, installed candidate, website, or complete Beta acceptance claim.
use crate::codefriend::{
    architecture::structure::{self, BoundaryPolicy},
    evidence::Retention,
    governance::local::{self as fitness, Policy, Rule},
    ingestion::Scope,
    integration::journey::{LocalJourneyOptions, StageStatus},
};
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

fn git(root: &Path, args: &[&str]) -> String {
    let result = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    String::from_utf8(result.stdout).unwrap().trim().into()
}
struct Fixture {
    dir: tempfile::TempDir,
    source: PathBuf,
    revision: String,
}
impl Fixture {
    fn new(source_text: &str) -> Self {
        let dir = tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap();
        let source = dir.path().join("source");
        fs::create_dir(&source).unwrap();
        git(&source, &["init", "-b", "main"]);
        git(
            &source,
            &["remote", "add", "origin", "https://example.com/owner/repo"],
        );
        fs::write(source.join("lib.rs"), source_text).unwrap();
        git(&source, &["add", "lib.rs"]);
        git(
            &source,
            &[
                "-c",
                "user.name=fixture",
                "-c",
                "user.email=fixture@example.com",
                "-c",
                "commit.gpgsign=false",
                "commit",
                "-m",
                "synthetic fixture",
            ],
        );
        let revision = git(&source, &["rev-parse", "HEAD"]);
        Self {
            dir,
            source,
            revision,
        }
    }
    fn options(&self) -> LocalJourneyOptions {
        LocalJourneyOptions {
            checkout: self.source.clone(),
            repository: "https://example.com/owner/repo".into(),
            revision: self.revision.clone(),
            scope: Scope {
                analysis: vec!["lib.rs".into()],
                context: vec![],
                max_files: 1,
                max_bytes: 4096,
                max_file_bytes: 4096,
            },
            store: self.dir.path().join("store"),
            output: self.dir.path().join("journey"),
            retention: Retention { seconds: 3600 },
            boundary_policy: BoundaryPolicy {
                schema: structure::VERSION.into(),
                crate_root: "lib.rs".into(),
                manifest_path: None,
                layers: [("lib.rs".into(), "core".into())].into(),
                allowed: BTreeSet::new(),
                coupling_threshold: 2,
            },
            fitness_policy: Policy {
                schema: fitness::VERSION.into(),
                rules: vec![Rule {
                    id: "no_network".into(),
                    kind: "forbidden_declared_use".into(),
                    source_path: "lib.rs".into(),
                    forbidden_prefix: "reqwest".into(),
                }],
            },
        }
    }
}

use super::{prepare_owned_admission, resume, Continuation, OwnedAdmissionJourneyOptions};
use crate::codefriend::{
    evidence::{store::Store, Admission},
    review::runner::{run_with_executor, ExecutionOptions, LaneExecution},
};

fn owned(f: &Fixture) -> (OwnedAdmissionJourneyOptions, Admission) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        // Match the server adapter's explicitly private owner boundary.
        fs::set_permissions(f.dir.path(), fs::Permissions::from_mode(0o700)).unwrap();
    }
    let o = f.options();
    let packet = crate::codefriend::ingestion::local::acquire(
        &o.checkout,
        &o.repository,
        &o.revision,
        o.scope,
    )
    .unwrap();
    let store = Store::open(&o.store, super::now).unwrap();
    let admission = store.admit(packet, o.retention).unwrap();
    drop(store);
    let review_root = f.dir.path().join("review");
    let mut calls = 0;
    let run = run_with_executor(
        ExecutionOptions {
            out: review_root.clone(),
            run_id: "hosted-operation".into(),
            cancel_file: None,
        },
        admission.clone(),
        "fixture:no-provider".into(),
        |_, _, _| {
            calls += 1;
            Ok(LaneExecution {
                final_status: crate::provider_communication::ProviderInvocationFinalStatusV1::Ok,
                output_text: Some(r#"{"findings":[]}"#.into()),
            })
        },
    )
    .unwrap();
    assert_eq!(calls, 4);
    assert_ne!(run.run_id, run.review_record.run.id);
    let options = OwnedAdmissionJourneyOptions {
        store: o.store,
        output: o.output,
        owner_root: f.dir.path().into(),
        review_root,
        packet_id: admission.packet.packet_id.clone(),
        admission_digest: admission.digest.clone(),
        operation_id: "hosted-operation".into(),
        candidate_revision: env!("CODEFRIEND_BUILD_REVISION").into(),
        expires_at: admission.expires_at - 1,
        completed_run: run,
        boundary_policy: o.boundary_policy.into(),
        fitness_policy: o.fitness_policy.into(),
    };
    (options, admission)
}
#[test]
fn existing_admission_and_review_resume_without_checkout_or_redispatch() {
    let f = Fixture::new("pub fn answer() -> u8 { 42 }\n");
    let (options, original) = owned(&f);
    let output = options.output.clone();
    let store_path = options.store.clone();
    fs::remove_dir_all(&f.source).unwrap();
    let j = prepare_owned_admission(options).unwrap();
    assert_eq!(j.manifest().admission_digest, original.digest);
    assert_eq!(j.manifest().stages.len(), 18);
    assert_eq!(j.manifest().stages["review"].status, StageStatus::Complete);
    assert_eq!(j.manifest().status, StageStatus::Pending);
    drop(j);
    let mut j = resume(&output).unwrap();
    assert!(j
        .continue_with(Continuation::Review {
            provider_request: f.dir.path().join("never-read"),
            run_id: "again".into(),
            cancel_file: None
        })
        .is_err());
    drop(j);
    let store = Store::open(&store_path, super::now).unwrap();
    assert_eq!(store.get(&original.packet.packet_id).unwrap(), original);
}
#[test]
fn attachment_rejects_foreign_identity_expiry_and_review_before_output() {
    for case in 0..5 {
        let f = Fixture::new("pub fn answer() -> u8 { 42 }\n");
        let (mut o, _) = owned(&f);
        let output = o.output.clone();
        match case {
            0 => o.admission_digest = "0".repeat(64),
            1 => o.operation_id = "foreign".into(),
            2 => o.expires_at = 0,
            3 => o.candidate_revision = "0".repeat(40),
            _ => {
                fs::write(o.review_root.join("review-record.json"), "{}").unwrap();
            }
        }
        assert!(prepare_owned_admission(o).is_err(), "case {case}");
        assert!(!output.exists());
    }
}
#[test]
fn resume_rejects_external_review_mutation_and_deleted_admission() {
    for delete in [false, true] {
        let f = Fixture::new("pub fn answer() -> u8 { 42 }\n");
        let (o, _) = owned(&f);
        let output = o.output.clone();
        let store = o.store.clone();
        let review = o.review_root.clone();
        drop(prepare_owned_admission(o).unwrap());
        if delete {
            fs::remove_dir_all(store).unwrap();
        } else {
            fs::write(review.join("extra.json"), "{}").unwrap();
        }
        assert!(resume(&output).is_err());
    }
}

#[test]
fn interrupted_owned_attachment_cannot_be_resumed_for_review_dispatch() {
    let f = Fixture::new("pub fn answer() -> u8 { 42 }\n");
    let (o, _) = owned(&f);
    let output = o.output.clone();
    drop(prepare_owned_admission(o).unwrap());
    // Simulate a crash immediately after the offline-stage checkpoint, before
    // the existing completed review was imported. Earlier bytes remain exact.
    for name in ["review.json", "journey-0004.json", "checkpoint-0004.json"] {
        fs::remove_file(output.join(name)).unwrap();
    }
    let error = match resume(&output) {
        Ok(_) => panic!("incomplete owned attachment must not permit review dispatch"),
        Err(error) => error,
    };
    assert!(error
        .to_string()
        .contains("journey_owned_attachment_incomplete"));
}

#[cfg(unix)]
#[test]
fn nonprivate_owned_boundary_is_rejected_without_journey_creation() {
    use std::os::unix::fs::PermissionsExt;
    let f = Fixture::new("pub fn answer() -> u8 { 42 }\n");
    let (o, _) = owned(&f);
    let output = o.output.clone();
    fs::set_permissions(f.dir.path(), fs::Permissions::from_mode(0o755)).unwrap();
    let error = match prepare_owned_admission(o) {
        Ok(_) => panic!("a nonprivate owner boundary must be rejected"),
        Err(error) => error,
    };
    assert!(error
        .to_string()
        .contains("journey_private_directory_required"));
    assert!(!output.exists());
}

#[cfg(target_os = "macos")]
#[test]
fn case_alias_destinations_and_owned_store_boundaries_use_identity() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();
    let source = root.join("Source");
    let store = root.join("Store");
    let review = root.join("Review");
    for path in [&source, &store, &review] {
        fs::create_dir(path).unwrap();
    }
    let alias = root.join("source");
    assert!(same_file::is_same_file(&source, &alias).unwrap());
    let checkout = super::PathBoundary::Checkout(source.clone());
    assert!(checkout.check(&alias.join("new/deep/export")).is_err());
    assert!(checkout.check(&alias).is_err());
    assert!(checkout.check(&root).is_err());
    assert!(checkout.check(&root.join("distinct-export")).is_ok());
    let owned = super::PathBoundary::Owned {
        root: root.clone(),
        store,
        review,
    };
    for path in [root.join("store/new"), root.join("review/new")] {
        assert!(owned.check(&path).is_err());
    }
    assert!(owned.check(&root.join("new-export")).is_ok());
    assert!(!source.join("new").exists());
}
