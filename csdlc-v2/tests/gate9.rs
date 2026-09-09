use std::collections::BTreeSet;

use csdlc_v2::{
    decide_cutover, decide_from_evidence, generate_sample_packets, select_generation,
    BudgetEvidence, BudgetKind, CutoverDecision, Generation, GenerationSelector, ParityEvidence,
    ScenarioEvidence, ScenarioOutcome, SoakEvidenceInput, SoakScenario,
};
use strum::IntoEnumIterator;

fn passing_scenarios() -> Vec<ScenarioEvidence> {
    SoakScenario::iter()
        .map(|scenario| ScenarioEvidence {
            scenario,
            outcome: ScenarioOutcome::Passed,
            evidence_refs: vec![format!("evidence/{scenario}.json")],
            findings: Vec::new(),
        })
        .collect()
}

#[test]
fn native_sample_authority_resolves_distinct_native_and_import_families() {
    let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repository root");
    csdlc_v2::registry::validate_native_registry(repo)
        .expect("generation-aware native and legacy registry authority");

    let malformed_repo = tempfile::tempdir().expect("malformed repo");
    let registry = malformed_repo
        .path()
        .join("docs/templates/prompts/current.json");
    std::fs::create_dir_all(registry.parent().unwrap()).unwrap();
    std::fs::write(registry, br#"{"generations":{}}"#).unwrap();
    let output = tempfile::tempdir().expect("sample output");
    let output_path = output.path().join("samples");
    let error = generate_sample_packets(malformed_repo.path(), &output_path)
        .expect_err("malformed authority must fail sample generation");
    assert!(matches!(error.code, csdlc_v2::ErrorCode::InvalidManifest));
    assert!(
        !output_path.exists(),
        "registry failure must precede authoring"
    );
}

// PVF: deterministic local filesystem contract proof; small resource profile.
// Required for #754 acceptance and the C-SDLC v2 standalone CI lane.
fn registry_fixture(version: &str) -> (tempfile::TempDir, serde_json::Value) {
    let root = tempfile::tempdir().expect("registry fixture");
    let mut registry: serde_json::Value =
        serde_json::from_slice(include_bytes!("../../docs/templates/prompts/current.json"))
            .unwrap();
    registry["csdlc_prompt_template_set"] = version.into();
    registry["semver"] = version.into();
    registry["generations"]["legacy_import"]["template_set"] = version.into();
    registry["generations"]["legacy_import"]["path"] =
        format!("docs/templates/prompts/{version}").into();
    let manifest = root.path().join("csdlc-v2/operator/native-card-shape.json");
    std::fs::create_dir_all(manifest.parent().unwrap()).unwrap();
    std::fs::write(
        manifest,
        include_bytes!("../operator/native-card-shape.json"),
    )
    .unwrap();
    (root, registry)
}

fn write_registry(root: &std::path::Path, registry: &serde_json::Value) {
    let path = root.join("docs/templates/prompts/current.json");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, serde_json::to_vec(registry).unwrap()).unwrap();
}

#[test]
fn native_registry_accepts_explicit_legacy_registry_versions() {
    for version in ["1.0.3", "1.0.4"] {
        let (root, registry) = registry_fixture(version);
        write_registry(root.path(), &registry);
        csdlc_v2::registry::validate_native_registry(root.path())
            .unwrap_or_else(|error| panic!("{version}: {error:?}"));
    }
}

#[test]
fn native_registry_rejects_incompatible_authority_before_sample_writes() {
    for version in ["1.0.3", "1.0.4"] {
        let other = if version == "1.0.3" { "1.0.4" } else { "1.0.3" };
        let other_path = format!("docs/templates/prompts/{other}");
        for (pointer, replacement) in [
            ("/semver", other),
            ("/generations/legacy_import/template_set", other),
            ("/generations/legacy_import/path", other_path.as_str()),
            ("/generations/csdlc_v2_native/template_set", "1.0.4"),
            (
                "/generations/csdlc_v2_native/projection_family",
                "legacy_full",
            ),
            (
                "/generations/csdlc_v2_native/shape_manifest_path",
                "../native-card-shape.json",
            ),
        ] {
            let (root, mut registry) = registry_fixture(version);
            *registry.pointer_mut(pointer).unwrap() = replacement.into();
            write_registry(root.path(), &registry);
            let output = root.path().join("samples");
            let error = generate_sample_packets(root.path(), &output).unwrap_err();
            assert_eq!(
                error.code,
                csdlc_v2::ErrorCode::InvalidManifest,
                "{version}: {pointer}"
            );
            assert!(!output.exists(), "{version}: {pointer} wrote output");
        }
    }
    let (root, registry) = registry_fixture("1.0.5");
    write_registry(root.path(), &registry);
    let output = root.path().join("samples");
    assert_eq!(
        generate_sample_packets(root.path(), &output)
            .unwrap_err()
            .code,
        csdlc_v2::ErrorCode::InvalidManifest
    );
    assert!(!output.exists());
}

#[test]
fn native_registry_rejects_missing_and_drifted_shapes_before_sample_writes() {
    for missing in [false, true] {
        let (root, registry) = registry_fixture("1.0.4");
        write_registry(root.path(), &registry);
        let manifest = root.path().join("csdlc-v2/operator/native-card-shape.json");
        if missing {
            std::fs::remove_file(manifest).unwrap();
        } else {
            let mut shape: serde_json::Value =
                serde_json::from_slice(&std::fs::read(&manifest).unwrap()).unwrap();
            shape["cards"]["sip"][0] = "unapproved heading".into();
            std::fs::write(manifest, serde_json::to_vec(&shape).unwrap()).unwrap();
        }
        let output = root.path().join("samples");
        assert_eq!(
            generate_sample_packets(root.path(), &output)
                .unwrap_err()
                .code,
            csdlc_v2::ErrorCode::InvalidManifest
        );
        assert!(!output.exists());
    }
}

fn passing_budgets() -> Vec<BudgetEvidence> {
    BudgetKind::iter()
        .map(|name| {
            let (unit, hard_ceiling, target) = name.contract();
            BudgetEvidence {
                name,
                measured: (hard_ceiling / 2.0).max(0.01),
                target,
                hard_ceiling,
                unit: unit.into(),
                hard_pass: true,
                review_approved: false,
                qualification: None,
                evidence_ref: "evidence/budgets.json".into(),
            }
        })
        .collect()
}

fn passing_parity() -> ParityEvidence {
    ParityEvidence {
        compared_cases: 3,
        critical_differences: 0,
        explained_noncritical_differences: vec!["v2 intentionally has one canonical index".into()],
        evidence_ref: "evidence/parity.json".into(),
    }
}

#[test]
fn selector_requires_opt_in_and_rejects_explicit_v3_before_canonical_cutover() {
    let selector = GenerationSelector {
        schema: "csdlc.generation_selector.v1".into(),
        default_generation: Generation::V1,
        operational_authority: None,
        authority_issue: None,
        authority_pull_request: None,
        review_authority: None,
        approval_authority: None,
        opted_in_issues: BTreeSet::from([9_001]),
    };
    assert_eq!(
        select_generation(&selector, 1, None).unwrap(),
        Generation::V1
    );
    assert_eq!(
        select_generation(&selector, 9_001, Some(Generation::V2)).unwrap(),
        Generation::V2
    );
    assert!(select_generation(&selector, 9_002, Some(Generation::V2)).is_err());
    assert!(select_generation(&selector, 9_002, Some(Generation::V3)).is_err());

    let cutover_default = GenerationSelector {
        default_generation: Generation::V2,
        ..selector
    };
    assert_eq!(
        select_generation(&cutover_default, 9_002, None).unwrap(),
        Generation::V2
    );
    assert_eq!(
        select_generation(&cutover_default, 9_002, Some(Generation::V1)).unwrap(),
        Generation::V1
    );

    let v3_cutover_default = GenerationSelector {
        schema: "csdlc.generation_selector.v2".into(),
        default_generation: Generation::V3,
        operational_authority: Some("csdlc-v3".into()),
        authority_issue: Some(505),
        authority_pull_request: Some(591),
        review_authority: Some("typed-v2-exact-head".into()),
        approval_authority: Some("merged-pr-591-closed-issue-505".into()),
        ..cutover_default
    };
    assert_eq!(
        select_generation(&v3_cutover_default, 9_002, None).unwrap(),
        Generation::V3
    );
    assert_eq!(
        select_generation(&v3_cutover_default, 9_002, Some(Generation::V2)).unwrap(),
        Generation::V2
    );

    let malformed = [
        GenerationSelector {
            schema: "csdlc.generation_selector.v1".into(),
            ..v3_cutover_default.clone()
        },
        GenerationSelector {
            operational_authority: None,
            ..v3_cutover_default.clone()
        },
        GenerationSelector {
            authority_issue: Some(504),
            ..v3_cutover_default.clone()
        },
        GenerationSelector {
            authority_pull_request: Some(590),
            ..v3_cutover_default.clone()
        },
        GenerationSelector {
            review_authority: Some("stale-review".into()),
            ..v3_cutover_default.clone()
        },
        GenerationSelector {
            approval_authority: Some("unmerged-pr".into()),
            ..v3_cutover_default.clone()
        },
    ];
    for selector in malformed {
        assert!(select_generation(&selector, 9_002, None).is_err());
    }
}

#[test]
fn sample_generation_is_idempotent_and_builds_six_ast_validated_cards_each() {
    let root = tempfile::tempdir().unwrap();
    let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repository root");
    let first = generate_sample_packets(repo, root.path()).unwrap();
    let second = generate_sample_packets(repo, root.path()).unwrap();
    assert_eq!(first, second);
    assert_eq!(first.len(), 3);
    for packet in first {
        let packet_root = root.path().join(&packet.root);
        assert_eq!(packet.generation, Generation::V2);
        assert_eq!(packet.card_paths.len(), 6);
        assert!(packet_root.join(&packet.design_path).is_file());
        assert!(packet_root.join(&packet.diagram_path).is_file());
        for path in packet.card_paths.values() {
            let markdown = std::fs::read_to_string(packet_root.join(path)).unwrap();
            assert!(markdown.starts_with("# "));
            assert!(markdown.contains("## "));
        }
    }
}

#[test]
fn proceed_requires_every_scenario_hard_budget_and_zero_critical_parity_loss() {
    let packet = decide_cutover(
        passing_scenarios(),
        passing_budgets(),
        passing_parity(),
        vec!["live GitHub behavior remains provider-dependent".into()],
    );
    assert_eq!(packet.decision, CutoverDecision::Proceed);
    assert!(packet.blockers.is_empty());
    assert_eq!(packet.default_generation, Generation::V1);
    assert!(!packet.rollback_window_started);
    assert!(!packet.importer_expiry_started);
}

#[test]
fn missing_or_waiting_evidence_incubates_but_hard_failure_stops() {
    let mut scenarios = passing_scenarios();
    scenarios.pop();
    assert_eq!(
        decide_cutover(scenarios, passing_budgets(), passing_parity(), Vec::new()).decision,
        CutoverDecision::Incubate
    );

    let mut scenarios = passing_scenarios();
    scenarios[0].outcome = ScenarioOutcome::Failed;
    assert_eq!(
        decide_cutover(scenarios, passing_budgets(), passing_parity(), Vec::new()).decision,
        CutoverDecision::Stop
    );
    assert_eq!(
        decide_cutover(
            passing_scenarios(),
            Vec::new(),
            passing_parity(),
            Vec::new()
        )
        .decision,
        CutoverDecision::Incubate
    );
}

#[test]
fn hard_budget_or_critical_parity_failure_stops_cutover() {
    let mut budgets = passing_budgets();
    let hard = budgets
        .iter_mut()
        .find(|item| item.name == BudgetKind::RustTests)
        .unwrap();
    hard.measured = 151.0;
    hard.hard_pass = false;
    assert_eq!(
        decide_cutover(passing_scenarios(), budgets, passing_parity(), Vec::new()).decision,
        CutoverDecision::Stop
    );

    let mut parity = passing_parity();
    parity.critical_differences = 1;
    assert_eq!(
        decide_cutover(passing_scenarios(), passing_budgets(), parity, Vec::new()).decision,
        CutoverDecision::Stop
    );

    let mut malformed = passing_budgets();
    malformed[0].measured = f64::NAN;
    malformed[0].unit.clear();
    assert_eq!(
        decide_cutover(passing_scenarios(), malformed, passing_parity(), Vec::new()).decision,
        CutoverDecision::Stop
    );

    let mut duplicate = passing_budgets();
    duplicate.push(duplicate[0].clone());
    assert_eq!(
        decide_cutover(passing_scenarios(), duplicate, passing_parity(), Vec::new()).decision,
        CutoverDecision::Incubate
    );

    let mut altered_contract = passing_budgets();
    altered_contract[0].hard_ceiling = f64::MAX;
    altered_contract[0].unit = "invented".into();
    assert_eq!(
        decide_cutover(
            passing_scenarios(),
            altered_contract,
            passing_parity(),
            Vec::new()
        )
        .decision,
        CutoverDecision::Incubate
    );

    let mut reviewed_loc = passing_budgets();
    let loc = reviewed_loc
        .iter_mut()
        .find(|item| item.name == BudgetKind::ImplementationLoc)
        .unwrap();
    loc.measured = 8_200.0;
    loc.hard_pass = false;
    loc.review_approved = true;
    loc.qualification = Some("Reviewed useful code with named owner and rationale.".into());
    assert_eq!(
        decide_cutover(
            passing_scenarios(),
            reviewed_loc,
            passing_parity(),
            Vec::new()
        )
        .decision,
        CutoverDecision::Proceed
    );
}

#[test]
fn evidence_input_rejects_wrong_schema_or_non_v1_default() {
    let mut input = SoakEvidenceInput {
        schema: "wrong".into(),
        default_generation: Generation::V1,
        scenarios: passing_scenarios(),
        budgets: passing_budgets(),
        parity: passing_parity(),
        residual_risks: Vec::new(),
    };
    assert!(decide_from_evidence(input.clone()).is_err());
    input.schema = "csdlc.soak_evidence.v1".into();
    input.default_generation = Generation::V2;
    assert!(decide_from_evidence(input).is_err());
}

#[test]
fn every_retained_behavior_has_current_parity_proof() {
    let registry: serde_json::Value = serde_json::from_slice(include_bytes!(
        "../../docs/architecture/csdlc-v2/csdlc_v2_retained_behavior.v1.json"
    ))
    .unwrap();
    let evidence: serde_json::Value = serde_json::from_slice(include_bytes!(
        "../../docs/architecture/csdlc-v2/gate10d2/PARITY_EVIDENCE.json"
    ))
    .unwrap();
    let required = registry["dispositions"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|entry| !matches!(entry["disposition"].as_str(), Some("delete" | "defer")))
        .map(|entry| entry["capability"].as_str().unwrap().to_owned())
        .collect::<BTreeSet<_>>();
    let proven = evidence["capabilities"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| {
            assert!(!entry["proof_refs"].as_array().unwrap().is_empty());
            entry["capability"].as_str().unwrap().to_owned()
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(proven, required);
    assert_eq!(evidence["coverage_basis_points"], 10_000);
    assert_eq!(evidence["critical_differences"], 0);
}
