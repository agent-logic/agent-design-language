//! PVF runtime: deterministic local publication-control and installed CLI proof.
use adl::codefriend::{
    evidence::{
        contracts::{Completion, Publication, ReviewRecord, Run},
        hash,
    },
    ingestion::digest,
    publication::{
        admit_local, append_decision, read_decision_head, verify_artifacts, DecisionKind,
        DecisionRecord, ManifestInput,
    },
};
use serde_json::json;
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

struct Fixture {
    _dir: tempfile::TempDir,
    root: PathBuf,
    review_path: PathBuf,
    artifact_root: PathBuf,
    manifest_path: PathBuf,
    destination_root: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let target_tmp = Path::new(env!("CARGO_TARGET_TMPDIR"));
        fs::create_dir_all(target_tmp).unwrap();
        let dir = tempfile::tempdir_in(target_tmp).unwrap();
        let root = dir.path().to_path_buf();
        let review_path = root.join("review-record.json");
        fs::write(
            &review_path,
            include_bytes!("fixtures/codefriend/evidence/review-v1.json"),
        )
        .unwrap();
        let artifact_root = root.join("artifacts");
        fs::create_dir(&artifact_root).unwrap();
        fs::create_dir(artifact_root.join("details")).unwrap();
        fs::write(
            artifact_root.join("report.md"),
            "# Review report\n\nOne bounded finding.\n",
        )
        .unwrap();
        fs::write(
            artifact_root.join("details/findings.json"),
            "{\"finding_count\":1}\n",
        )
        .unwrap();
        let artifacts = vec![
            json!({
                "path":"details/findings.json",
                "digest":digest(&fs::read(artifact_root.join("details/findings.json")).unwrap())
            }),
            json!({
                "path":"report.md",
                "digest":digest(&fs::read(artifact_root.join("report.md")).unwrap())
            }),
        ];
        let manifest_path = root.join("manifest-input.json");
        fs::write(
            &manifest_path,
            serde_json::to_vec_pretty(&json!({
                "schema":"codefriend.publication_manifest_input.v1",
                "artifact_manifest":artifacts,
                "renderer_versions":{"markdown":"v1"},
                "target":"review-output",
                "claims":["Bounded local review"],
                "nonclaims":["No remote publication"]
            }))
            .unwrap(),
        )
        .unwrap();
        let destination_root = root.join("destination");
        fs::create_dir(&destination_root).unwrap();
        Self {
            _dir: dir,
            root,
            review_path,
            artifact_root,
            manifest_path,
            destination_root,
        }
    }

    fn review(&self) -> ReviewRecord {
        serde_json::from_slice(&fs::read(&self.review_path).unwrap()).unwrap()
    }

    fn publication(&self) -> Publication {
        let review = self.review();
        let manifest = ManifestInput::read(&self.manifest_path).unwrap();
        verify_artifacts(&self.artifact_root, &manifest.artifact_manifest).unwrap();
        manifest
            .publication(&review, &self.destination_root)
            .unwrap()
    }
}

fn cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_adl"))
        .arg("codefriend")
        .arg("publication")
        .args(args)
        .output()
        .unwrap()
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "stdout={}\nstderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice::<serde_json::Value>(&output.stdout).unwrap();
}

#[test]
fn installed_prepare_approve_inspect_and_atomic_local_admission() {
    let fixture = Fixture::new();
    let publication = fixture.root.join("publication.json");
    let decisions = fixture.root.join("decisions");
    let destination = fixture.destination_root.clone();
    fs::create_dir(&decisions).unwrap();

    let output = cli(&[
        "prepare",
        "--review-record",
        fixture.review_path.to_str().unwrap(),
        "--manifest",
        fixture.manifest_path.to_str().unwrap(),
        "--artifact-root",
        fixture.artifact_root.to_str().unwrap(),
        "--destination-root",
        destination.to_str().unwrap(),
        "--out",
        publication.to_str().unwrap(),
    ]);
    assert_success(&output);
    assert!(!cli(&[
        "prepare",
        "--review-record",
        fixture.review_path.to_str().unwrap(),
        "--manifest",
        fixture.manifest_path.to_str().unwrap(),
        "--artifact-root",
        fixture.artifact_root.to_str().unwrap(),
        "--destination-root",
        destination.to_str().unwrap(),
        "--out",
        publication.to_str().unwrap(),
    ])
    .status
    .success());

    let output = cli(&[
        "approve",
        "--review-record",
        fixture.review_path.to_str().unwrap(),
        "--publication",
        publication.to_str().unwrap(),
        "--actor",
        "operator-fixture",
        "--reason",
        "Exact local artifacts approved",
        "--approval-store",
        decisions.to_str().unwrap(),
    ]);
    assert_success(&output);
    let output = cli(&[
        "inspect",
        "--review-record",
        fixture.review_path.to_str().unwrap(),
        "--publication",
        publication.to_str().unwrap(),
        "--approval-store",
        decisions.to_str().unwrap(),
    ]);
    assert_success(&output);
    let inspected: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(inspected["decision"], "approved");
    assert_eq!(inspected["channel"], "explicit_cli");

    let wrong_destination = fixture.root.join("wrong-destination");
    fs::create_dir(&wrong_destination).unwrap();
    assert!(!cli(&[
        "admit-local",
        "--review-record",
        fixture.review_path.to_str().unwrap(),
        "--publication",
        publication.to_str().unwrap(),
        "--approval-store",
        decisions.to_str().unwrap(),
        "--artifact-root",
        fixture.artifact_root.to_str().unwrap(),
        "--destination-root",
        wrong_destination.to_str().unwrap(),
    ])
    .status
    .success());

    let output = cli(&[
        "admit-local",
        "--review-record",
        fixture.review_path.to_str().unwrap(),
        "--publication",
        publication.to_str().unwrap(),
        "--approval-store",
        decisions.to_str().unwrap(),
        "--artifact-root",
        fixture.artifact_root.to_str().unwrap(),
        "--destination-root",
        destination.to_str().unwrap(),
    ]);
    assert_success(&output);
    let published = destination.join("review-output");
    assert_eq!(
        fs::read(published.join("report.md")).unwrap(),
        fs::read(fixture.artifact_root.join("report.md")).unwrap()
    );
    assert!(published.join("publication-control.json").is_file());
    assert!(published.join("publication-admission.json").is_file());
    assert!(!cli(&[
        "admit-local",
        "--review-record",
        fixture.review_path.to_str().unwrap(),
        "--publication",
        publication.to_str().unwrap(),
        "--approval-store",
        decisions.to_str().unwrap(),
        "--artifact-root",
        fixture.artifact_root.to_str().unwrap(),
        "--destination-root",
        destination.to_str().unwrap(),
    ])
    .status
    .success());
}

#[test]
fn withheld_invalidated_and_changed_identity_are_denied_until_new_approval() {
    let fixture = Fixture::new();
    let review = fixture.review();
    let publication = fixture.publication();
    let decisions = fixture.root.join("decisions");
    fs::create_dir(&decisions).unwrap();
    append_decision(
        &decisions,
        &review,
        &publication,
        DecisionKind::Withheld,
        "operator-fixture",
        "Needs another review",
        10,
    )
    .unwrap();
    let destination = fixture.destination_root.clone();
    assert!(admit_local(
        &review,
        &publication,
        &decisions,
        &fixture.artifact_root,
        &destination,
        11
    )
    .is_err());

    let approved = append_decision(
        &decisions,
        &review,
        &publication,
        DecisionKind::Approved,
        "operator-fixture",
        "Exact artifacts accepted",
        12,
    )
    .unwrap();
    append_decision(
        &decisions,
        &review,
        &approved.publication,
        DecisionKind::Invalidated,
        "operator-fixture",
        "Destination changed",
        13,
    )
    .unwrap();
    assert!(admit_local(
        &review,
        &publication,
        &decisions,
        &fixture.artifact_root,
        &destination,
        14
    )
    .is_err());

    let withheld_after_approval = fixture.root.join("withheld-after-approval");
    fs::create_dir(&withheld_after_approval).unwrap();
    append_decision(
        &withheld_after_approval,
        &review,
        &publication,
        DecisionKind::Approved,
        "operator-fixture",
        "Exact artifacts accepted",
        12,
    )
    .unwrap();
    append_decision(
        &withheld_after_approval,
        &review,
        &publication,
        DecisionKind::Withheld,
        "operator-fixture",
        "Approval withdrawn",
        13,
    )
    .unwrap();
    assert!(admit_local(
        &review,
        &publication,
        &withheld_after_approval,
        &fixture.artifact_root,
        &destination,
        14
    )
    .is_err());

    for change in [
        "renderer",
        "target",
        "destination",
        "claims",
        "scope",
        "findings",
    ] {
        let mut changed = approved.clone();
        match change {
            "renderer" => {
                changed
                    .publication
                    .renderer_versions
                    .insert("markdown".into(), "v2".into());
            }
            "target" => changed.publication.target = "other-output".into(),
            "destination" => changed.publication.destination_digest = "0".repeat(64),
            "claims" => changed.publication.claims.push("Additional claim".into()),
            "scope" => changed.publication.scope_digest = "0".repeat(64),
            _ => changed.publication.finding_set_digest = "0".repeat(64),
        }
        assert!(
            changed.validate(&review).is_err(),
            "change {change} was accepted"
        );
    }

    fs::write(
        fixture.artifact_root.join("report.md"),
        "# Mutated after approval\n",
    )
    .unwrap();
    let stale_approval = fixture.root.join("stale-approval");
    fs::create_dir(&stale_approval).unwrap();
    append_decision(
        &stale_approval,
        &review,
        &publication,
        DecisionKind::Approved,
        "operator-fixture",
        "Original artifact accepted",
        14,
    )
    .unwrap();
    assert!(admit_local(
        &review,
        &publication,
        &stale_approval,
        &fixture.artifact_root,
        &destination,
        15
    )
    .is_err());
    assert!(!destination.join("review-output").exists());

    let input: ManifestInput =
        serde_json::from_slice(&fs::read(&fixture.manifest_path).unwrap()).unwrap();
    let mut updated = input;
    updated.artifact_manifest[1].digest =
        digest(&fs::read(fixture.artifact_root.join("report.md")).unwrap());
    let new_publication = updated
        .publication(&review, &fixture.destination_root)
        .unwrap();
    let renewed_decisions = fixture.root.join("renewed-decisions");
    fs::create_dir(&renewed_decisions).unwrap();
    append_decision(
        &renewed_decisions,
        &review,
        &new_publication,
        DecisionKind::Approved,
        "operator-fixture",
        "Mutated artifact explicitly re-reviewed",
        16,
    )
    .unwrap();
    admit_local(
        &review,
        &new_publication,
        &renewed_decisions,
        &fixture.artifact_root,
        &destination,
        17,
    )
    .unwrap();
}

#[test]
fn revoked_approval_cannot_be_replayed_from_an_alternate_or_truncated_store() {
    let fixture = Fixture::new();
    let review = fixture.review();
    let publication = fixture.publication();
    let binding = publication.binding_digest().unwrap();
    let destination = fixture.destination_root.clone();

    for revocation in [DecisionKind::Invalidated, DecisionKind::Withheld] {
        let name = match revocation {
            DecisionKind::Invalidated => "invalidated",
            DecisionKind::Withheld => "withheld",
            DecisionKind::Approved => unreachable!(),
        };
        let store = fixture.root.join(format!("{name}-store"));
        fs::create_dir(&store).unwrap();
        let approved = append_decision(
            &store,
            &review,
            &publication,
            DecisionKind::Approved,
            "operator-fixture",
            "Exact artifacts accepted",
            10,
        )
        .unwrap();
        let local_head = store.join("heads").join(format!("{binding}.json"));
        let approved_head = fs::read(&local_head).unwrap();
        let revoked = append_decision(
            &store,
            &review,
            &publication,
            revocation,
            "operator-fixture",
            "Approval revoked",
            11,
        )
        .unwrap();
        assert!(admit_local(
            &review,
            &publication,
            &store,
            &fixture.artifact_root,
            &destination,
            12,
        )
        .is_err());

        let replay = fixture.root.join(format!("{name}-replay"));
        fs::create_dir(&replay).unwrap();
        fs::copy(
            store
                .join("decisions")
                .join(&binding)
                .join(format!("{}.json", approved.digest)),
            replay.join(format!("{}.json", approved.digest)),
        )
        .unwrap();
        let error = read_decision_head(&replay, &review, &publication)
            .unwrap_err()
            .to_string();
        assert_eq!(error, "unowned_publication_store");

        fs::remove_file(
            store
                .join("decisions")
                .join(&binding)
                .join(format!("{}.json", revoked.digest)),
        )
        .unwrap();
        let error = admit_local(
            &review,
            &publication,
            &store,
            &fixture.artifact_root,
            &destination,
            12,
        )
        .unwrap_err()
        .to_string();
        assert_eq!(error, "publication_head_replayed");

        fs::write(&local_head, &approved_head).unwrap();
        let error = admit_local(
            &review,
            &publication,
            &store,
            &fixture.artifact_root,
            &destination,
            12,
        )
        .unwrap_err()
        .to_string();
        assert_eq!(error, "publication_external_head_mismatch");
    }
}

#[test]
fn incomplete_runs_missing_provenance_and_manifest_attacks_fail_closed() {
    let fixture = Fixture::new();
    let review = fixture.review();
    let publication = fixture.publication();
    for (actor, reason) in [("", "reason"), ("operator", "")] {
        assert!(DecisionRecord::new(
            &review,
            &publication,
            DecisionKind::Approved,
            actor,
            reason,
            10,
            None,
        )
        .is_err());
    }

    let mut incomplete = review.clone();
    incomplete.run = Run::new(
        &incomplete.admission,
        incomplete.run.lane_versions.clone(),
        incomplete.run.provider_route.clone(),
        Completion::Incomplete,
        vec![],
    )
    .unwrap();
    incomplete.validate().unwrap();
    let incomplete_publication = ManifestInput::read(&fixture.manifest_path)
        .unwrap()
        .publication(&incomplete, &fixture.destination_root)
        .unwrap();
    assert!(DecisionRecord::new(
        &incomplete,
        &incomplete_publication,
        DecisionKind::Approved,
        "operator-fixture",
        "Must not approve",
        10,
        None,
    )
    .is_err());

    fs::write(fixture.artifact_root.join("extra.txt"), "undeclared\n").unwrap();
    assert!(verify_artifacts(&fixture.artifact_root, &publication.artifact_manifest).is_err());
    fs::remove_file(fixture.artifact_root.join("extra.txt")).unwrap();
    fs::write(
        fixture.artifact_root.join("report.md"),
        "password = must-not-be-published\n",
    )
    .unwrap();
    let mut unsafe_manifest: ManifestInput =
        serde_json::from_slice(&fs::read(&fixture.manifest_path).unwrap()).unwrap();
    unsafe_manifest.artifact_manifest[1].digest =
        digest(&fs::read(fixture.artifact_root.join("report.md")).unwrap());
    assert!(verify_artifacts(&fixture.artifact_root, &unsafe_manifest.artifact_manifest).is_err());

    let approved = DecisionRecord::new(
        &review,
        &publication,
        DecisionKind::Approved,
        "operator-fixture",
        "Exact artifacts accepted",
        10,
        None,
    )
    .unwrap();
    let mut forged = approved.clone();
    forged.digest = hash(&"forged").unwrap();
    assert!(forged.validate(&review).is_err());
}

#[cfg(unix)]
#[test]
fn symlink_artifacts_and_destination_collision_leave_no_partial_target() {
    use std::os::unix::fs::symlink;

    let fixture = Fixture::new();
    let review = fixture.review();
    let publication = fixture.publication();
    let destination = fixture.destination_root.clone();
    let decisions = fixture.root.join("decisions");
    fs::create_dir(&decisions).unwrap();
    append_decision(
        &decisions,
        &review,
        &publication,
        DecisionKind::Approved,
        "operator-fixture",
        "Exact artifacts accepted",
        10,
    )
    .unwrap();
    fs::remove_file(fixture.artifact_root.join("report.md")).unwrap();
    symlink(
        fixture.artifact_root.join("details/findings.json"),
        fixture.artifact_root.join("report.md"),
    )
    .unwrap();
    assert!(admit_local(
        &review,
        &publication,
        &decisions,
        &fixture.artifact_root,
        &destination,
        11
    )
    .is_err());
    assert!(!destination.join("review-output").exists());
}
