//! PVF: deterministic local CPU/disk projection derivation, integrity observation,
//! transactional rebuild and recovery; fixed synthetic semantic inputs and templates;
//! required SIM-07 pre-resume qualification input; no network or paid resources.

use csdlc_v3::application::derive_semantic_card_projection;
use csdlc_v3::commands::local::PromptRegistry;
use csdlc_v3::lifecycle::semantic::AmendmentClass;
use csdlc_v3::storage::semantic::{
    AcceptedIntentPlan, Admission, CardProjectionObservation, CommitOutcome, Digest, Error,
    IssueInputs, IssueKey, LocalChange, Observation, PlanStep, Publication, SemanticRoot, Snapshot,
    Validator, SEMANTIC_CARD_KINDS,
};
use csdlc_v3::storage::DurableTransactionStore;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(0);

struct Fixture {
    directory: PathBuf,
    root: SemanticRoot,
    key: IssueKey,
    registry: PromptRegistry,
}

impl Fixture {
    fn new() -> Self {
        let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join(format!(
                "semantic-card-projections-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
        let checkout = directory.join("repo");
        fs::create_dir_all(checkout.join(".git/objects")).unwrap();
        fs::write(checkout.join(".git/HEAD"), "ref: refs/heads/main\n").unwrap();
        fs::write(
            checkout.join(".git/config"),
            "[core]\nrepositoryformatversion = 0\n",
        )
        .unwrap();
        let templates = directory.join("docs/templates/prompts/1.0.5");
        fs::create_dir_all(templates.join("schemas")).unwrap();
        let mut template_paths = BTreeMap::new();
        for kind in SEMANTIC_CARD_KINDS {
            let path = templates.join(format!("{kind}.md"));
            fs::write(
                &path,
                format!("# {kind}\nTitle: <title>\nStatus: <status>\nEvidence: <evidence_ref>\n"),
            )
            .unwrap();
            fs::write(
                templates.join(format!("schemas/{kind}.structure.json")),
                serde_json::to_vec(&serde_json::json!({
                    "schema":"adl.csdlc.prompt_card_structure.v1",
                    "template_set":"1.0.5",
                    "card_kind":kind,
                    "template_path":path,
                    "scaffold_lines":[format!("# {kind}")],
                    "headings":[{"level":1,"text":kind}],
                    "locked_lines":[]
                }))
                .unwrap(),
            )
            .unwrap();
            template_paths.insert(kind.into(), path.to_string_lossy().into_owned());
        }
        Self {
            root: SemanticRoot::from_git_common(checkout.join(".git"), "example/repo").unwrap(),
            key: IssueKey::new("example/repo", 871).unwrap(),
            registry: PromptRegistry {
                version: "1.0.5".into(),
                card_kinds: SEMANTIC_CARD_KINDS.into_iter().map(str::to_owned).collect(),
                template_paths,
            },
            directory,
        }
    }

    fn prepare(&self) -> Snapshot {
        let cards = SEMANTIC_CARD_KINDS
            .into_iter()
            .map(|kind| {
                (
                    kind.into(),
                    serde_json::json!({
                        "evidence_ref": format!(".csdlc/evidence/871/{kind}.json"),
                        "status": "pending",
                        "title": "derived projections"
                    }),
                )
            })
            .collect();
        let inputs = IssueInputs::new(
            "derive all cards".into(),
            AcceptedIntentPlan {
                schema: "csdlc.v3.intent_plan.v1".into(),
                slug: "derived-projections".into(),
                cards,
                validators: vec![Validator {
                    id: "projection".into(),
                    program: "cargo".into(),
                    args: vec!["test".into()],
                    success_marker: "test result: ok.".into(),
                    timeout_seconds: 30,
                }],
                publication: Publication {
                    base: "main".into(),
                    title: "projection".into(),
                    body: "Closes #871".into(),
                    draft: true,
                },
            },
            vec![PlanStep {
                id: "derive".into(),
                acceptance: "six deterministic cards".into(),
            }],
            None,
            Digest::authority(b"fixture authority"),
        )
        .unwrap();
        match DurableTransactionStore::prepare_issue(&self.root, self.key.clone(), inputs).unwrap()
        {
            CommitOutcome::Committed(snapshot) => *snapshot,
            CommitOutcome::Unchanged(_) => panic!("new fixture must commit"),
        }
    }

    fn current(&self) -> Snapshot {
        match DurableTransactionStore::observe_issue(&self.root, &self.key).unwrap() {
            Observation::Current(snapshot) | Observation::ProjectionRepairRequired(snapshot) => {
                *snapshot
            }
            other => panic!("unexpected observation: {other:?}"),
        }
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.directory);
    }
}

fn inventory(path: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    let mut result = BTreeMap::new();
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                result.extend(inventory(&path));
            } else {
                result.insert(path.clone(), fs::read(path).unwrap());
            }
        }
    }
    result
}

#[test]
fn derives_six_deterministic_cards_without_inventing_semantic_facts() {
    let fixture = Fixture::new();
    let snapshot = fixture.prepare();
    let first = derive_semantic_card_projection(&snapshot, &fixture.registry).unwrap();
    let second = derive_semantic_card_projection(&snapshot, &fixture.registry).unwrap();
    assert_eq!(first, second);
    assert_eq!(first.cards().len(), 6);
    assert!(first
        .semantic_digest()
        .as_str()
        .starts_with("semantic-projection-v1:"));
    assert!(first
        .projection_digest()
        .as_str()
        .starts_with("card-projection-v1:"));
    assert_ne!(first.semantic_digest(), first.projection_digest());
    for kind in SEMANTIC_CARD_KINDS {
        let artifact = &first.cards()[kind];
        assert_eq!(
            artifact.template_ref(),
            format!("docs/templates/prompts/1.0.5/{kind}.md")
        );
        let projected: Value = serde_json::from_slice(artifact.values()).unwrap();
        assert_eq!(projected, snapshot.inputs().cards()[kind]);
        let rendered = std::str::from_utf8(artifact.rendered()).unwrap();
        assert!(rendered.contains("Status: pending"));
        assert!(rendered.contains(&format!(".csdlc/evidence/871/{kind}.json")));
        assert!(!rendered.contains("passed"));
        assert!(!rendered.contains("approved"));
        assert!(!rendered.contains("published"));
        assert!(!rendered.contains("complete"));
    }
}

#[test]
fn observation_is_read_only_and_classifies_missing_altered_and_interrupted() {
    let fixture = Fixture::new();
    let snapshot = fixture.prepare();
    let bundle = derive_semantic_card_projection(&snapshot, &fixture.registry).unwrap();
    let before = inventory(&fixture.directory);
    let CardProjectionObservation::Missing { paths } =
        DurableTransactionStore::observe_card_projection(&fixture.root, &snapshot, &bundle)
            .unwrap()
    else {
        panic!("absent projection must be missing");
    };
    assert_eq!(paths.len(), 13);
    assert_eq!(inventory(&fixture.directory), before);

    let proof =
        DurableTransactionStore::write_card_projection(&fixture.root, &snapshot, bundle.clone())
            .unwrap();
    assert_eq!(fixture.current().version(), snapshot.version());
    assert_eq!(
        DurableTransactionStore::observe_card_projection(&fixture.root, &snapshot, &bundle)
            .unwrap(),
        CardProjectionObservation::Healthy
    );

    let card_root = fixture.root.card_projection_directory(&snapshot).unwrap();
    fs::write(card_root.join("stp.md"), "altered\n").unwrap();
    assert_eq!(
        DurableTransactionStore::observe_card_projection(&fixture.root, &snapshot, &bundle)
            .unwrap(),
        CardProjectionObservation::Altered {
            paths: vec!["stp.md".into()]
        }
    );
    DurableTransactionStore::write_card_projection(&fixture.root, &snapshot, bundle.clone())
        .unwrap();
    let suffix = bundle
        .projection_digest()
        .as_str()
        .split(':')
        .nth(1)
        .unwrap();
    let pending = card_root.join(format!(".projection-{suffix}.pending"));
    fs::write(&pending, bundle.manifest_bytes().unwrap()).unwrap();
    assert_eq!(
        DurableTransactionStore::observe_card_projection(&fixture.root, &snapshot, &bundle)
            .unwrap(),
        CardProjectionObservation::Interrupted {
            staged_paths: vec![pending.file_name().unwrap().to_string_lossy().into_owned()]
        }
    );
    let replay =
        DurableTransactionStore::write_card_projection(&fixture.root, &snapshot, bundle.clone())
            .unwrap();
    assert_eq!(
        DurableTransactionStore::observe_card_projection(&fixture.root, &snapshot, &bundle)
            .unwrap(),
        CardProjectionObservation::Healthy
    );

    let committed = DurableTransactionStore::commit_issue_local(
        &fixture.root,
        Admission::new(
            fixture.key.clone(),
            snapshot.version().clone(),
            snapshot.inputs().authority().clone(),
        ),
        LocalChange::AcknowledgeProjection(replay),
    )
    .unwrap();
    let CommitOutcome::Committed(acknowledged) = committed else {
        panic!("acknowledgement must commit");
    };
    assert_eq!(acknowledged.inputs_version(), snapshot.inputs_version());
    let acknowledged_bundle =
        derive_semantic_card_projection(&acknowledged, &fixture.registry).unwrap();
    assert_eq!(
        acknowledged_bundle.manifest_bytes().unwrap(),
        bundle.manifest_bytes().unwrap()
    );
    assert_eq!(
        DurableTransactionStore::observe_card_projection(
            &fixture.root,
            &acknowledged,
            &acknowledged_bundle
        )
        .unwrap(),
        CardProjectionObservation::Healthy
    );
    drop(proof);
}

#[test]
fn stale_semantic_snapshot_cannot_rebuild_or_acknowledge() {
    let fixture = Fixture::new();
    let snapshot = fixture.prepare();
    let bundle = derive_semantic_card_projection(&snapshot, &fixture.registry).unwrap();
    let proof =
        DurableTransactionStore::write_card_projection(&fixture.root, &snapshot, bundle.clone())
            .unwrap();
    let mut cards = snapshot.inputs().cards().clone();
    cards.get_mut("stp").unwrap()["status"] = Value::String("amended".into());
    let amended = match DurableTransactionStore::commit_issue_local(
        &fixture.root,
        Admission::new(
            fixture.key.clone(),
            snapshot.version().clone(),
            snapshot.inputs().authority().clone(),
        ),
        LocalChange::AmendCards(cards),
    )
    .unwrap()
    {
        CommitOutcome::Committed(snapshot) => *snapshot,
        CommitOutcome::Unchanged(_) => panic!("amendment must commit"),
    };
    let amended_bundle = derive_semantic_card_projection(&amended, &fixture.registry).unwrap();
    assert_eq!(amended.causal_invalidations().len(), 6);
    assert!(amended
        .causal_invalidations()
        .iter()
        .all(|item| item.cause == AmendmentClass::ScopeAcceptance));
    DurableTransactionStore::write_card_projection(&fixture.root, &amended, amended_bundle.clone())
        .unwrap();
    let before_stale_write = inventory(&fixture.directory);
    assert!(matches!(
        DurableTransactionStore::write_card_projection(&fixture.root, &snapshot, bundle),
        Err(Error::StaleVersion)
    ));
    assert_eq!(inventory(&fixture.directory), before_stale_write);
    assert_eq!(
        DurableTransactionStore::observe_card_projection(&fixture.root, &amended, &amended_bundle)
            .unwrap(),
        CardProjectionObservation::Healthy
    );
    assert!(DurableTransactionStore::commit_issue_local(
        &fixture.root,
        Admission::new(
            fixture.key.clone(),
            amended.version().clone(),
            amended.inputs().authority().clone(),
        ),
        LocalChange::AcknowledgeProjection(proof),
    )
    .is_err());
    assert_eq!(
        snapshot
            .inputs()
            .cards()
            .values()
            .flat_map(|value| value["evidence_ref"].as_str())
            .collect::<BTreeSet<_>>(),
        amended
            .inputs()
            .cards()
            .values()
            .flat_map(|value| value["evidence_ref"].as_str())
            .collect::<BTreeSet<_>>()
    );
}

#[test]
fn newer_projection_recovers_strict_staging_residue_from_prior_version() {
    let fixture = Fixture::new();
    let first = fixture.prepare();
    let first_bundle = derive_semantic_card_projection(&first, &fixture.registry).unwrap();
    DurableTransactionStore::write_card_projection(&fixture.root, &first, first_bundle.clone())
        .unwrap();
    let card_root = fixture.root.card_projection_directory(&first).unwrap();
    let old_suffix = first_bundle
        .projection_digest()
        .as_str()
        .split(':')
        .nth(1)
        .unwrap();
    let old_pending = card_root.join(format!(".projection-{old_suffix}.pending"));
    let old_next = card_root.join(format!(".stp.md-{old_suffix}.next"));
    fs::write(&old_pending, first_bundle.manifest_bytes().unwrap()).unwrap();
    fs::write(&old_next, first_bundle.cards()["stp"].rendered()).unwrap();

    let mut cards = first.inputs().cards().clone();
    cards.get_mut("stp").unwrap()["status"] = Value::String("amended".into());
    let second = match DurableTransactionStore::commit_issue_local(
        &fixture.root,
        Admission::new(
            fixture.key.clone(),
            first.version().clone(),
            first.inputs().authority().clone(),
        ),
        LocalChange::AmendCards(cards),
    )
    .unwrap()
    {
        CommitOutcome::Committed(snapshot) => *snapshot,
        CommitOutcome::Unchanged(_) => panic!("amendment must commit"),
    };
    let second_bundle = derive_semantic_card_projection(&second, &fixture.registry).unwrap();
    assert!(matches!(
        DurableTransactionStore::observe_card_projection(&fixture.root, &second, &second_bundle)
            .unwrap(),
        CardProjectionObservation::Interrupted { .. }
    ));
    let proof = DurableTransactionStore::write_card_projection(
        &fixture.root,
        &second,
        second_bundle.clone(),
    )
    .unwrap();
    assert!(!old_pending.exists());
    assert!(!old_next.exists());
    assert_eq!(
        DurableTransactionStore::observe_card_projection(&fixture.root, &second, &second_bundle)
            .unwrap(),
        CardProjectionObservation::Healthy
    );
    assert!(matches!(
        DurableTransactionStore::commit_issue_local(
            &fixture.root,
            Admission::new(
                fixture.key.clone(),
                second.version().clone(),
                second.inputs().authority().clone(),
            ),
            LocalChange::AcknowledgeProjection(proof),
        )
        .unwrap(),
        CommitOutcome::Committed(_)
    ));
}

#[test]
fn corrupt_prior_projection_residue_fails_closed_without_mutation() {
    for corrupt_pending in [true, false] {
        let fixture = Fixture::new();
        let first = fixture.prepare();
        let first_bundle = derive_semantic_card_projection(&first, &fixture.registry).unwrap();
        DurableTransactionStore::write_card_projection(&fixture.root, &first, first_bundle.clone())
            .unwrap();
        let card_root = fixture.root.card_projection_directory(&first).unwrap();
        let old_suffix = first_bundle
            .projection_digest()
            .as_str()
            .split(':')
            .nth(1)
            .unwrap();
        let old_pending = card_root.join(format!(".projection-{old_suffix}.pending"));
        if corrupt_pending {
            fs::write(&old_pending, b"{\"corrupt\":true}\n").unwrap();
        } else {
            fs::write(&old_pending, first_bundle.manifest_bytes().unwrap()).unwrap();
            fs::write(
                card_root.join(format!(".stp.md-{old_suffix}.next")),
                b"torn staged bytes",
            )
            .unwrap();
        }

        let mut cards = first.inputs().cards().clone();
        cards.get_mut("stp").unwrap()["status"] = Value::String("amended".into());
        let second = match DurableTransactionStore::commit_issue_local(
            &fixture.root,
            Admission::new(
                fixture.key.clone(),
                first.version().clone(),
                first.inputs().authority().clone(),
            ),
            LocalChange::AmendCards(cards),
        )
        .unwrap()
        {
            CommitOutcome::Committed(snapshot) => *snapshot,
            CommitOutcome::Unchanged(_) => panic!("amendment must commit"),
        };
        DurableTransactionStore::write_issue_projection(&fixture.root, &second).unwrap();
        let second_bundle = derive_semantic_card_projection(&second, &fixture.registry).unwrap();
        let before = inventory(&fixture.directory);
        assert!(matches!(
            DurableTransactionStore::write_card_projection(&fixture.root, &second, second_bundle,),
            Err(Error::EvidenceMismatch)
        ));
        assert_eq!(inventory(&fixture.directory), before);
    }
}
