//! PVF owner_binary: deterministic source-bound assessment contracts and actual
//! four-lane runner with synthetic executor; no semantic-truth or live-quality claim.
use adl::codefriend::{
    evidence::{
        assessments::{
            self, AssessmentKind, AssessmentSet, DefectDetails, ProviderAssessment,
            ProviderAssessmentOutput, ProviderCitation,
        },
        contracts::{Completion, Severity},
        store::Store,
        Admission, Retention,
    },
    ingestion::{local, Scope},
    review::runner::{self, ExecutionOptions, LaneExecution},
};
use adl::provider_communication::ProviderInvocationFinalStatusV1;
use std::{fs, path::Path, process::Command};
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
struct Fixture {
    dir: tempfile::TempDir,
    admission: Admission,
    _store: Store,
}
impl Fixture {
    fn new(privacy: bool) -> Self {
        Self::version(privacy, 0)
    }
    fn version(privacy: bool, revision: u8) -> Self {
        Self::source(privacy, revision, "// café\npub fn guarded() {}\n")
    }
    fn source(privacy: bool, revision: u8, content: &str) -> Self {
        let dir = tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap();
        let source = dir.path().join("source");
        fs::create_dir(&source).unwrap();
        git(&source, &["init", "-b", "main"]);
        git(
            &source,
            &[
                "remote",
                "add",
                "origin",
                "https://example.com/review/source",
            ],
        );
        fs::write(source.join("a.rs"), content).unwrap();
        fs::write(
            source.join("b.rs"),
            format!("pub fn other() {{}}\n// version {revision}\n"),
        )
        .unwrap();
        let mut context = Vec::new();
        if privacy {
            fs::write(source.join(".env"), "TOKEN=private\n").unwrap();
            context.push(".env".into());
        }
        git(&source, &["add", "."]);
        git(
            &source,
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
        let revision = git(&source, &["rev-parse", "HEAD"]);
        let packet = local::acquire(
            &source,
            "https://example.com/review/source",
            &revision,
            Scope {
                analysis: vec!["a.rs".into(), "b.rs".into()],
                context,
                max_files: 3,
                max_bytes: 65536,
                max_file_bytes: 32768,
            },
        )
        .unwrap();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let store = Store::open(&dir.path().join("store"), move || now).unwrap();
        let admission = store.admit(packet, Retention { seconds: 3600 }).unwrap();
        Self {
            dir,
            admission,
            _store: store,
        }
    }
    fn item(&self, kind: AssessmentKind) -> ProviderAssessment {
        ProviderAssessment {
            kind,
            summary: "Assessment of guard".into(),
            explanation: "Synthetic component assessment, not semantic verification".into(),
            citations: vec![ProviderCitation {
                evidence_id: self
                    .admission
                    .evidence
                    .iter()
                    .find(|e| e.path == "a.rs")
                    .unwrap()
                    .id
                    .clone(),
                start_byte: 0,
                end_byte: 8,
                quote: "// café".into(),
            }],
            limitations: vec!["Fixture has no independent semantic adjudication".into()],
            defect: (kind == AssessmentKind::DefectCandidate).then(|| DefectDetails {
                severity: Severity::Medium,
                observed_behavior: "Observed fixture behavior".into(),
                expected_behavior: "Expected fixture behavior".into(),
                concrete_trigger: "Declared fixture input".into(),
                impact: "Declared fixture impact".into(),
                proposed_remedy_or_verification: "Verify the declared input".into(),
            }),
        }
    }
    fn execute(
        &self,
        name: &str,
        output: impl Fn(&str) -> String,
    ) -> anyhow::Result<runner::FourPerspectiveReviewRun> {
        runner::run_assessments_with_executor(
            ExecutionOptions {
                out: self.dir.path().join(name),
                run_id: name.into(),
                cancel_file: None,
            },
            self.admission.clone(),
            "fixture:no-provider".into(),
            |lane, prompt, _| {
                assert!(prompt.contains("codefriend.review_lane.v4"));
                assert!(!prompt.contains("[byte 0]"));
                assert!(!prompt.contains("TOKEN=private"));
                Ok(LaneExecution {
                    final_status: ProviderInvocationFinalStatusV1::Ok,
                    output_text: Some(output(lane.id())),
                })
            },
        )
    }
}
fn json(items: Vec<ProviderAssessment>) -> String {
    serde_json::to_string(&ProviderAssessmentOutput { assessments: items }).unwrap()
}
#[test]
fn mixed_assessments_bind_receipts_and_project_only_defects() {
    let f = Fixture::new(false);
    let raw = json(vec![
        f.item(AssessmentKind::DefectCandidate),
        f.item(AssessmentKind::PositiveObservation),
        f.item(AssessmentKind::UnresolvedQuestion),
    ]);
    let run = f.execute("mixed", |_| raw.clone()).unwrap();
    run.successful_execution().unwrap();
    assert_eq!(run.schema, runner::REVIEW_RUN_SCHEMA_V3);
    assert_eq!(run.review_record.findings.len(), 4);
    let counts = run.review_record.assessment_counts().unwrap();
    assert_eq!(
        (
            counts.defect_candidates,
            counts.positive_observations,
            counts.unresolved_questions
        ),
        (4, 4, 4)
    );
    assert_eq!(run.review_record.actionable_findings().unwrap().len(), 4);
    assert!(run
        .lane_results
        .iter()
        .all(|lane| lane.assessment_ids.as_ref().unwrap().len() == 3));
    let synthesis = adl::codefriend::review::synthesis::synthesize(&run.review_record).unwrap();
    assert_eq!(synthesis.synthesized_findings.len(), 4);
    assert_eq!(synthesis.observations.as_ref().unwrap().len(), 8);
    assert_eq!(synthesis.assessment_counts, Some(counts));
    let mut mixed = synthesis.clone();
    mixed.schema = adl::codefriend::review::synthesis::SYNTHESIS_SCHEMA.into();
    assert!(adl::codefriend::review::synthesis::validate_generation(&mixed).is_err());
    let mut mixed = synthesis.clone();
    let defect = run
        .review_record
        .run
        .assessment_set
        .as_ref()
        .unwrap()
        .assessments
        .iter()
        .find(|a| a.kind == AssessmentKind::DefectCandidate)
        .unwrap()
        .clone();
    mixed.observations.as_mut().unwrap().push(defect);
    assert!(adl::codefriend::review::synthesis::validate_generation(&mixed).is_err());
    // Same admission/lane labels do not authorize comparing different contract generations.
    let mut legacy = run.review_record.clone();
    legacy.run = adl::codefriend::evidence::contracts::Run::new(
        &legacy.admission,
        legacy.run.lane_versions.clone(),
        legacy.run.provider_route.clone(),
        Completion::Complete,
        vec![],
    )
    .unwrap();
    legacy.validate().unwrap();
    let comparison = adl::codefriend::evidence::contracts::Comparison {
        schema: adl::codefriend::evidence::contracts::CONTRACT.into(),
        baseline_run: legacy.run.id.clone(),
        current_run: run.review_record.run.id.clone(),
        baseline_version: legacy.run.schema.clone(),
        current_version: run.review_record.run.schema.clone(),
        finding_id: Some(run.review_record.findings[0].id.clone()),
        outcome: adl::codefriend::evidence::contracts::Delta::Unchanged,
        reason: "Fixture comparison across generations".into(),
    };
    assert!(comparison
        .validate(&legacy, &run.review_record)
        .unwrap_err()
        .to_string()
        .contains("comparison_requires_compatible_completed_coverage"));
    let mut tampered = run.clone();
    tampered.lane_results[0].assessment_ids = Some(vec![]);
    assert!(tampered
        .successful_execution()
        .unwrap_err()
        .to_string()
        .contains("assessment_mismatch"));
    let mut changed = run.review_record.clone();
    changed.run.assessment_set.as_mut().unwrap().assessments[0].summary =
        "Resealed outer payload".into();
    assert!(changed.validate().is_err());
}
#[test]
fn valid_nonactionable_only_is_successful_even_with_explicit_privacy_coverage() {
    for privacy in [false, true] {
        let f = Fixture::new(privacy);
        let raw = json(vec![
            f.item(AssessmentKind::PositiveObservation),
            f.item(AssessmentKind::UnresolvedQuestion),
        ]);
        let run = f.execute("observations", |_| raw.clone()).unwrap();
        run.successful_execution().unwrap();
        assert!(run.review_record.findings.is_empty());
        let synthesis = adl::codefriend::review::synthesis::synthesize(&run.review_record).unwrap();
        assert!(synthesis.synthesized_findings.is_empty());
        assert_eq!(synthesis.observations.as_ref().unwrap().len(), 8);
        assert_eq!(synthesis.coverage.is_some(), privacy);
        let tests = adl::codefriend::actions::test_plan::plan(&synthesis).unwrap();
        assert!(tests.test_cases.is_empty() && tests.omitted_findings.is_empty());
        let remedies =
            adl::codefriend::actions::remediation::plan(&synthesis, &run.review_record).unwrap();
        assert!(remedies.actions.is_empty() && remedies.omitted_findings.is_empty());
        assert_eq!(
            run.review_record
                .assessment_counts()
                .unwrap()
                .unresolved_questions,
            4
        );
        assert_eq!(run.review_record.run.coverage.is_some(), privacy);
        assert_eq!(
            run.completion,
            if privacy {
                Completion::Incomplete
            } else {
                Completion::Complete
            }
        );
    }
}
#[test]
fn wrong_file_support_retains_incomplete_run_not_empty_success() {
    let f = Fixture::new(false);
    let good = json(vec![f.item(AssessmentKind::PositiveObservation)]);
    let mut wrong = f.item(AssessmentKind::DefectCandidate);
    wrong.citations[0].evidence_id = f
        .admission
        .evidence
        .iter()
        .find(|e| e.path == "b.rs")
        .unwrap()
        .id
        .clone();
    let bad = json(vec![wrong]);
    f.execute("bad", |lane| {
        if lane == "adversarial" {
            bad.clone()
        } else {
            good.clone()
        }
    })
    .unwrap();
    let run: runner::FourPerspectiveReviewRun =
        serde_json::from_slice(&fs::read(f.dir.path().join("bad/run.json")).unwrap()).unwrap();
    assert_eq!(run.completion, Completion::Incomplete);
    assert!(run.failures.is_empty());
    assert_eq!(
        run.review_record
            .run
            .assessment_coverage
            .as_ref()
            .unwrap()
            .gaps
            .len(),
        1
    );
    assert_eq!(
        run.review_record
            .assessment_counts()
            .unwrap()
            .positive_observations,
        3
    );
    run.successful_execution().unwrap();
}
#[test]
fn citation_utf8_and_actionability_and_aggregate_bounds_fail_closed() {
    let f = Fixture::new(false);
    let good = f.item(AssessmentKind::PositiveObservation);
    assessments::parse_lane("correctness", &json(vec![good.clone()]), &f.admission).unwrap();
    let mut split = good.clone();
    split.citations[0].end_byte = 7;
    split.citations[0].quote = "// invented".into();
    assert!(assessments::parse_lane("correctness", &json(vec![split]), &f.admission).is_err());
    let mut illegal = good.clone();
    illegal.defect = f.item(AssessmentKind::DefectCandidate).defect;
    assert!(assessments::parse_lane("correctness", &json(vec![illegal]), &f.admission).is_err());
    let mut duplicate = good.clone();
    duplicate.citations.push(duplicate.citations[0].clone());
    assert!(assessments::parse_lane("correctness", &json(vec![duplicate]), &f.admission).is_err());
    assert!(assessments::parse_lane("correctness", &json(vec![good; 101]), &f.admission).is_err());
    assert!(assessments::parse_lane(
        "correctness",
        &" ".repeat(assessments::MAX_LANE_BYTES + 1),
        &f.admission
    )
    .is_err());
}
#[test]
fn legacy_complete_and_privacy_run_bytes_omit_assessment_fields() {
    for privacy in [false, true] {
        let f = Fixture::new(privacy);
        let run = runner::run_with_executor(
            ExecutionOptions {
                out: f.dir.path().join("old"),
                run_id: "old".into(),
                cancel_file: None,
            },
            f.admission.clone(),
            "fixture:no-provider".into(),
            |_, _, _| {
                Ok(LaneExecution {
                    final_status: ProviderInvocationFinalStatusV1::Ok,
                    output_text: Some("{\"findings\":[]}".into()),
                })
            },
        )
        .unwrap();
        assert_eq!(
            run.schema,
            if privacy {
                runner::REVIEW_RUN_SCHEMA_V2
            } else {
                runner::REVIEW_RUN_SCHEMA
            }
        );
        let bytes = serde_json::to_vec(&run).unwrap();
        assert!(!String::from_utf8(bytes.clone())
            .unwrap()
            .contains("assessment_"));
        let decoded: runner::FourPerspectiveReviewRun = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(serde_json::to_vec(&decoded).unwrap(), bytes);
        let mut altered = decoded;
        altered.review_record.run.assessment_set =
            Some(AssessmentSet::new(&f.admission, vec![]).unwrap());
        assert!(altered.successful_execution().is_err());
    }
}

#[test]
fn logical_defect_identity_survives_assessment_changes_and_original_revision_changes() {
    use adl::codefriend::evidence::contracts::{Comparison, Delta, CONTRACT};
    let first = Fixture::version(false, 0);
    let second = Fixture::version(false, 1);
    assert_ne!(
        first.admission.packet.revision,
        second.admission.packet.revision
    );
    let a = first
        .execute("first", |_| {
            json(vec![first.item(AssessmentKind::DefectCandidate)])
        })
        .unwrap();
    let mut revised = second.item(AssessmentKind::DefectCandidate);
    revised.explanation = "A revised assessment of the same concrete behavior".into();
    revised.defect.as_mut().unwrap().severity = Severity::High;
    let b = second
        .execute("second", |_| json(vec![revised.clone()]))
        .unwrap();
    assert_ne!(
        a.review_record.run.assessment_set.as_ref().unwrap().digest,
        b.review_record.run.assessment_set.as_ref().unwrap().digest
    );
    for old in &a.review_record.findings {
        let new = b
            .review_record
            .findings
            .iter()
            .find(|new| new.id == old.id)
            .unwrap();
        assert!(!old.same_assessment(new));
        let comparison = Comparison {
            schema: CONTRACT.into(),
            baseline_run: a.review_record.run.id.clone(),
            current_run: b.review_record.run.id.clone(),
            baseline_version: a.review_record.run.schema.clone(),
            current_version: b.review_record.run.schema.clone(),
            finding_id: Some(old.id.clone()),
            outcome: Delta::Changed,
            reason: "Same logical defect with changed assessment".into(),
        };
        comparison
            .validate(&a.review_record, &b.review_record)
            .unwrap();
    }
    let c = second
        .execute("unchanged", |_| {
            json(vec![second.item(AssessmentKind::DefectCandidate)])
        })
        .unwrap();
    for old in &a.review_record.findings {
        let new = c
            .review_record
            .findings
            .iter()
            .find(|new| new.id == old.id)
            .unwrap();
        assert!(old.same_assessment(new));
    }
}

#[test]
fn conflicting_receipts_for_same_logical_defect_are_incomplete_not_silently_deduplicated() {
    let f = Fixture::new(false);
    let one = f.item(AssessmentKind::DefectCandidate);
    let mut other = one.clone();
    other.defect.as_mut().unwrap().severity = Severity::High;
    let raw = json(vec![one, other]);
    assert!(assessments::parse_lane("correctness", &raw, &f.admission)
        .unwrap_err()
        .to_string()
        .contains("assessment_logical_identity_collision"));
    assert!(f.execute("collision", |_| raw.clone()).is_err());
    let retained: runner::FourPerspectiveReviewRun =
        serde_json::from_slice(&fs::read(f.dir.path().join("collision/run.json")).unwrap())
            .unwrap();
    assert_eq!(retained.completion, Completion::Incomplete);
    assert_eq!(retained.failures.len(), 4);
}

#[test]
fn remedy_only_change_preserves_logical_identity_but_changes_assessment() {
    use adl::codefriend::evidence::contracts::{Comparison, Delta, CONTRACT};
    let f = Fixture::new(false);
    let one = f.item(AssessmentKind::DefectCandidate);
    let a = f
        .execute("remedy-before", |_| json(vec![one.clone()]))
        .unwrap();
    let mut two = one;
    two.defect.as_mut().unwrap().proposed_remedy_or_verification =
        "Different concrete verification".into();
    let b = f
        .execute("remedy-after", |_| json(vec![two.clone()]))
        .unwrap();
    for old in &a.review_record.findings {
        let new = b
            .review_record
            .findings
            .iter()
            .find(|item| item.id == old.id)
            .unwrap();
        assert!(!old.same_assessment(new));
        Comparison {
            schema: CONTRACT.into(),
            baseline_run: a.review_record.run.id.clone(),
            current_run: b.review_record.run.id.clone(),
            baseline_version: a.review_record.run.schema.clone(),
            current_version: b.review_record.run.schema.clone(),
            finding_id: Some(old.id.clone()),
            outcome: Delta::Changed,
            reason: "Remedy changes assessment without changing logical defect".into(),
        }
        .validate(&a.review_record, &b.review_record)
        .unwrap();
    }
}

#[test]
fn privacy_assessments_traverse_original_store_planner_and_publication() {
    use adl::codefriend::{actions::test_plan, integration, review::synthesis};
    let f = Fixture::new(true);
    let raw = json(vec![f.item(AssessmentKind::DefectCandidate)]);
    let run = f.execute("privacy-planning", |_| raw.clone()).unwrap();
    run.successful_execution().unwrap();
    let review_path = f.dir.path().join("privacy-review.json");
    fs::write(
        &review_path,
        serde_json::to_vec(&run.review_record).unwrap(),
    )
    .unwrap();
    let synthesis_dir = f.dir.path().join("privacy-synthesis");
    synthesis::synthesize_from_file(synthesis::SynthesisOptions {
        input: review_path.clone(),
        out: synthesis_dir.clone(),
    })
    .unwrap();
    let plan_dir = f.dir.path().join("privacy-plan");
    let plan = test_plan::plan_from_store(
        test_plan::TestPlanOptions {
            input: synthesis_dir.join("synthesis.json"),
            out: plan_dir.clone(),
        },
        &f._store,
    )
    .unwrap();
    assert_eq!(plan.coverage, run.review_record.run.coverage);
    assert!(plan.coverage.is_some());
    assert_eq!(
        plan,
        test_plan::read_plan_from_store(&plan_dir.join("test-plan.json"), &f._store).unwrap()
    );
    assert!(!plan.test_cases.is_empty());
    assert!(test_plan::read_plan_from_file(&plan_dir.join("test-plan.json")).is_err());
    let destination = f.dir.path().join("destination");
    fs::create_dir(&destination).unwrap();
    integration::prepare_publication_bundle_for_format_v2(
        &review_path,
        &f.dir.path().join("privacy-publication"),
        &destination,
        integration::PublicationFormat::Markdown,
    )
    .unwrap();
    assert_eq!(
        f._store.get(&f.admission.packet.packet_id).unwrap(),
        f.admission
    );
}

// PVF #1140: deterministic local owner contracts, original-source authority;
// small CPU/filesystem, no network/provider, required regression proof.
#[test]
fn unique_quotes_derive_utf8_crlf_spans_and_ignore_legacy_offsets() {
    let f = Fixture::source(false, 0, "// préface\r\n// café\r\npub fn guarded() {}\r\n");
    let item = f.item(AssessmentKind::PositiveObservation);
    let mut raw: serde_json::Value = serde_json::from_str(&json(vec![item])).unwrap();
    let quote = &mut raw["assessments"][0]["citations"][0];
    quote["start_byte"] = serde_json::json!(99999);
    quote["end_byte"] = serde_json::json!(1);
    let values = assessments::parse_lane("correctness", &raw.to_string(), &f.admission).unwrap();
    let c = &values[0].citations[0];
    assert_eq!(c.start_byte, "// préface\r\n".len() as u64);
    assert_eq!(c.quote(&f.admission).unwrap(), "// café");
    let mut tampered = c.clone();
    tampered.start_byte += 1;
    assert!(tampered.quote(&f.admission).is_err());
    tampered = c.clone();
    tampered.quote_digest = "0".repeat(64);
    assert!(tampered.quote(&f.admission).is_err());
    raw["assessments"][0]["citations"][0]
        .as_object_mut()
        .unwrap()
        .remove("start_byte");
    raw["assessments"][0]["citations"][0]
        .as_object_mut()
        .unwrap()
        .remove("end_byte");
    assert_eq!(
        assessments::parse_lane("correctness", &raw.to_string(), &f.admission).unwrap(),
        values
    );
}

#[test]
fn overlapping_quotes_are_ambiguous_even_with_claimed_offsets() {
    let f = Fixture::source(false, 0, "aaa");
    let mut item = f.item(AssessmentKind::PositiveObservation);
    item.citations[0].quote = "aa".into();
    let parsed =
        assessments::parse_lane_with_gaps("correctness", &json(vec![item]), &f.admission).unwrap();
    assert!(parsed.assessments.is_empty());
    assert_eq!(parsed.gaps[0].reason, "assessment_quote_ambiguous");
}

#[test]
fn mixed_claims_keep_supported_siblings_and_drop_entire_unsupported_assessment() {
    let f = Fixture::new(false);
    let good = f.item(AssessmentKind::PositiveObservation);
    let mut bad = f.item(AssessmentKind::DefectCandidate);
    let mut missing = bad.citations[0].clone();
    missing.quote = "invented source".into();
    bad.citations.push(missing);
    let raw = json(vec![good, bad]);
    let parsed = assessments::parse_lane_with_gaps("correctness", &raw, &f.admission).unwrap();
    assert_eq!(parsed.assessments.len(), 1);
    assert_eq!(parsed.gaps.len(), 1);
    assert_eq!(parsed.gaps[0].assessment_index, 1);
    assert!(AssessmentSet::new(&f.admission, parsed.assessments)
        .unwrap()
        .findings(&f.admission)
        .unwrap()
        .is_empty());
    let run = f.execute("mixed-gaps", |_| raw.clone()).unwrap();
    assert!(run.review_record.run.assessment_coverage.is_some());
    let run: runner::FourPerspectiveReviewRun =
        serde_json::from_slice(&fs::read(f.dir.path().join("mixed-gaps/run.json")).unwrap())
            .unwrap();
    assert_eq!(run.completion, Completion::Incomplete);
    assert_eq!(
        run.review_record
            .assessment_counts()
            .unwrap()
            .positive_observations,
        4
    );
    assert!(run.review_record.findings.is_empty());
    assert!(run
        .lane_results
        .iter()
        .all(|lane| lane.assessment_gaps.len() == 1 && lane.failure.is_none()));
    run.successful_execution().unwrap();
    let mut tampered = run.clone();
    tampered.lane_results[0].assessment_gaps.clear();
    assert!(tampered.successful_execution().is_err());
}

#[test]
fn fenced_json_requires_one_whole_response_payload() {
    let f = Fixture::new(false);
    let raw = json(vec![f.item(AssessmentKind::PositiveObservation)]);
    for fenced in [
        format!("```json\n{raw}\n```"),
        format!(" \r\n```\r\n{raw}\r\n``` \n"),
    ] {
        assert_eq!(
            assessments::parse_lane("correctness", &fenced, &f.admission)
                .unwrap()
                .len(),
            1
        );
    }
    for bad in [
        format!("Here is JSON: {raw}"),
        format!("```json\n{raw}\n```\ntrailing"),
        format!("```json\n{raw}\n```\n```json\n{raw}\n```"),
        format!("```javascript\n{raw}\n```"),
        format!("```json\n{raw} {raw}\n```"),
    ] {
        assert!(assessments::parse_lane("correctness", &bad, &f.admission).is_err());
    }
}

#[test]
fn all_unsupported_is_distinct_from_genuinely_empty_assessments() {
    let f = Fixture::new(false);
    let mut bad = f.item(AssessmentKind::DefectCandidate);
    bad.citations[0].quote = "absent".into();
    let parsed =
        assessments::parse_lane_with_gaps("correctness", &json(vec![bad]), &f.admission).unwrap();
    assert!(parsed.assessments.is_empty());
    assert_eq!(parsed.gaps.len(), 1);
    let empty =
        assessments::parse_lane_with_gaps("correctness", "{\"assessments\":[]}", &f.admission)
            .unwrap();
    assert!(empty.assessments.is_empty() && empty.gaps.is_empty());
}

#[test]
fn historical_assessment_lane_v2_remains_readable_with_verified_spans() {
    let f = Fixture::new(false);
    let raw = json(vec![f.item(AssessmentKind::PositiveObservation)]);
    let run = f.execute("historical", |_| raw.clone()).unwrap();
    let mut record = run.review_record;
    for version in record.run.lane_versions.values_mut() {
        *version = "codefriend.review_lane.v2".into();
    }
    record.run = adl::codefriend::evidence::contracts::Run::new(
        &record.admission,
        record.run.lane_versions.clone(),
        record.run.provider_route.clone(),
        record.run.completion.clone(),
        record.run.failures.clone(),
    )
    .unwrap()
    .with_assessments(
        &record.admission,
        record.run.assessment_set.clone().unwrap(),
    )
    .unwrap();
    record.successful_execution().unwrap();
    adl::codefriend::review::synthesis::synthesize(&record).unwrap();
}

#[test]
fn mixed_assessment_gaps_traverse_original_store_planner_and_publication() {
    use adl::codefriend::{actions::test_plan, integration, review::synthesis};
    let f = Fixture::new(false);
    let mut unsupported = f.item(AssessmentKind::DefectCandidate);
    unsupported.summary = "Potential unverified problem".into();
    unsupported.citations[0].quote = "invented code".into();
    let raw = json(vec![f.item(AssessmentKind::DefectCandidate), unsupported]);
    let run = f.execute("privacy-planning", |_| raw.clone()).unwrap();
    run.successful_execution().unwrap();
    let review_path = f.dir.path().join("privacy-review.json");
    fs::write(
        &review_path,
        serde_json::to_vec(&run.review_record).unwrap(),
    )
    .unwrap();
    let synthesis_dir = f.dir.path().join("privacy-synthesis");
    synthesis::synthesize_from_file(synthesis::SynthesisOptions {
        input: review_path.clone(),
        out: synthesis_dir.clone(),
    })
    .unwrap();
    let plan_dir = f.dir.path().join("privacy-plan");
    let plan = test_plan::plan_from_store(
        test_plan::TestPlanOptions {
            input: synthesis_dir.join("synthesis.json"),
            out: plan_dir.clone(),
        },
        &f._store,
    )
    .unwrap();
    assert_eq!(plan.coverage, run.review_record.run.coverage);
    assert!(plan.coverage.is_none());
    assert_eq!(
        plan.assessment_coverage,
        run.review_record.run.assessment_coverage
    );
    assert!(plan.assessment_coverage.is_some());
    assert_eq!(run.completion, Completion::Incomplete);
    assert_eq!(
        plan,
        test_plan::read_plan_from_store(&plan_dir.join("test-plan.json"), &f._store).unwrap()
    );
    assert!(!plan.test_cases.is_empty());
    assert!(test_plan::read_plan_from_file(&plan_dir.join("test-plan.json")).is_err());
    let destination = f.dir.path().join("destination");
    fs::create_dir(&destination).unwrap();
    let bundle = integration::prepare_publication_bundle_for_format_v2(
        &review_path,
        &f.dir.path().join("privacy-publication"),
        &destination,
        integration::PublicationFormat::Markdown,
    )
    .unwrap();
    let publication = f.dir.path().join("privacy-publication");
    use adl::codefriend::publication::{
        append_decision, render_markdown, DecisionKind, MarkdownRenderOptions,
    };
    let approval_store = f.dir.path().join("approvals");
    append_decision(
        &approval_store,
        &run.review_record,
        &bundle,
        DecisionKind::Approved,
        "fixture",
        "Approve incomplete report with explicit unverified gaps",
        1_700_000_000,
    )
    .unwrap();
    render_markdown(MarkdownRenderOptions {
        review_record: review_path,
        publication: publication.join("publication.json"),
        approval_store,
        artifact_root: publication.join("artifacts"),
        synthesis: "synthesis/synthesis.json".into(),
        remediation_plan: "remediation/remediation-plan.json".into(),
        test_plan: "tests/test-plan.json".into(),
        destination_root: destination.clone(),
        out: destination.join(&bundle.target),
    })
    .unwrap();

    fn report_bytes(path: &Path) -> String {
        let mut text = String::new();
        for entry in fs::read_dir(path).unwrap() {
            let p = entry.unwrap().path();
            if p.is_dir() {
                text.push_str(&report_bytes(&p));
            } else if p.extension().is_some_and(|e| e == "md") {
                text.push_str(&fs::read_to_string(p).unwrap());
            }
        }
        text
    }
    let rendered = report_bytes(&destination);
    assert!(rendered.contains("Unverified assessment gaps"));
    assert!(rendered.contains("Potential unverified problem"));
    assert!(rendered.contains("incomplete"));
    assert_eq!(
        f._store.get(&f.admission.packet.packet_id).unwrap(),
        f.admission
    );
}

// PVF #1144 owner_binary: fixed source, real admission/prompt/parser; no model call.
#[test]
fn verbatim_prompt_preserves_original_bytes_and_json_quotes() {
    use adl::codefriend::review::lanes::ReviewLane;
    let source = "// café\r\n\tpub fn guarded() {\r\n    let text = \"a\\\\b\";\r\n}\r\n// END INERT SOURCE\r\n// no final newline";
    assert!(source.as_bytes().contains(&13));
    assert!(source.as_bytes().contains(&9));
    let f = Fixture::source(false, 0, source);
    let (manifest, prompt) =
        runner::assessment_lane_input_manifest("verbatim", ReviewLane::Correctness, &f.admission)
            .unwrap();
    assert_eq!(manifest.lane_contract, "codefriend.review_lane.v4");
    for evidence in &f.admission.evidence {
        let object = f
            .admission
            .packet
            .objects
            .iter()
            .find(|o| o.path == evidence.path)
            .unwrap();
        let content = object.content.as_deref().unwrap();
        let header = format!(
            "\nBEGIN INERT SOURCE evidence_id={} path={} digest={} content_bytes={}\n",
            evidence.id,
            evidence.path,
            evidence.content_digest,
            content.len()
        );
        let start = prompt.find(&header).unwrap() + header.len();
        assert_eq!(
            &prompt.as_bytes()[start..start + content.len()],
            content.as_bytes()
        );
        assert!(prompt[start + content.len()..].starts_with(&format!(
            "\nEND INERT SOURCE digest={}\n",
            evidence.content_digest
        )));
    }
    assert!(!prompt.contains("[byte 0]"));
    let mut item = f.item(AssessmentKind::PositiveObservation);
    item.citations[0].quote = "    let text = \"a\\\\b\";\r\n".into();
    let valid =
        assessments::parse_lane("correctness", &json(vec![item.clone()]), &f.admission).unwrap();
    assert_eq!(valid.len(), 1);
    item.citations[0].quote = item.citations[0].quote.trim().replace(' ', "");
    let invalid =
        assessments::parse_lane_with_gaps("correctness", &json(vec![item]), &f.admission).unwrap();
    assert!(invalid.assessments.is_empty());
    assert_eq!(invalid.gaps[0].reason, "assessment_quote_mismatch");
}

#[test]
fn verbatim_prompt_examples_include_complete_shapes_without_padding_requirement() {
    use adl::codefriend::review::lanes::ReviewLane;
    let f = Fixture::new(false);
    let (_, prompt) =
        runner::assessment_lane_input_manifest("shapes", ReviewLane::Correctness, &f.admission)
            .unwrap();
    let line = prompt
        .lines()
        .find(|line| line.starts_with("These are JSON SHAPES ONLY"))
        .unwrap();
    let example = line.split_once("placeholders: ").unwrap().1;
    let shapes: ProviderAssessmentOutput = serde_json::from_str(example).unwrap();
    assert_eq!(shapes.assessments.len(), 3);
    assert_eq!(shapes.assessments[0].kind, AssessmentKind::DefectCandidate);
    assert!(shapes.assessments[0].defect.is_some());
    assert_eq!(
        shapes.assessments[1].kind,
        AssessmentKind::PositiveObservation
    );
    assert_eq!(
        shapes.assessments[2].kind,
        AssessmentKind::UnresolvedQuestion
    );
    assert!(shapes.assessments[1..]
        .iter()
        .all(|item| item.defect.is_none() && item.limitations.is_empty()));
    assert!(prompt.contains("do not pad the response"));
    assert!(prompt.contains("Empty or partial output does not establish full source coverage"));
}
