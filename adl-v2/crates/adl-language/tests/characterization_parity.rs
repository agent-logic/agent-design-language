use adl_language::{canonical_bytes, parse_and_validate_yaml, DiagnosticCode};
use std::{collections::BTreeMap, fs, path::PathBuf};

fn corpus_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../adl-characterization/corpus/v1/fixtures")
}

fn fixture(name: &str) -> String {
    fs::read_to_string(corpus_root().join(name)).expect("reviewed #5337 fixture")
}

#[test]
fn applicable_characterization_cases_map_to_language_outcomes() {
    let cases = BTreeMap::from([
        ("six-primitives.adl.yaml", None),
        ("map-a.adl.yaml", None),
        ("map-b.adl.yaml", None),
        ("sequential-a.adl.yaml", None),
        ("sequential-b.adl.yaml", None),
        ("malformed.adl.yaml", Some(DiagnosticCode::Syntax)),
        (
            "schema-unknown.adl.yaml",
            Some(DiagnosticCode::UnknownField),
        ),
        (
            "unknown-provider.adl.yaml",
            Some(DiagnosticCode::UnknownProvider),
        ),
        ("unknown-agent.adl.yaml", Some(DiagnosticCode::UnknownAgent)),
        ("unknown-task.adl.yaml", Some(DiagnosticCode::UnknownTask)),
        ("unknown-tool.adl.yaml", Some(DiagnosticCode::UnknownTool)),
        (
            "unknown-workflow.adl.yaml",
            Some(DiagnosticCode::UnknownWorkflow),
        ),
        (
            "unsupported-run-field.adl.yaml",
            Some(DiagnosticCode::UnknownField),
        ),
        ("state-missing.adl.yaml", Some(DiagnosticCode::UnknownState)),
        ("cycle.adl.yaml", Some(DiagnosticCode::DependencyCycle)),
    ]);
    for (name, outcome) in cases {
        let result = parse_and_validate_yaml(&fixture(name));
        match outcome {
            None => {
                result.unwrap();
            }
            Some(expected) => {
                let diagnostics = result.unwrap_err();
                assert!(
                    diagnostics
                        .iter()
                        .any(|diagnostic| diagnostic.code == expected),
                    "{name}: expected {expected:?}, got {diagnostics:?}"
                );
            }
        }
    }
}

#[test]
fn characterization_ordering_contract_is_preserved() {
    let map_a = parse_and_validate_yaml(&fixture("map-a.adl.yaml")).unwrap();
    let map_b = parse_and_validate_yaml(&fixture("map-b.adl.yaml")).unwrap();
    assert_eq!(
        canonical_bytes(&map_a).unwrap(),
        canonical_bytes(&map_b).unwrap()
    );

    let sequential_a = parse_and_validate_yaml(&fixture("sequential-a.adl.yaml")).unwrap();
    let sequential_b = parse_and_validate_yaml(&fixture("sequential-b.adl.yaml")).unwrap();
    assert_ne!(
        canonical_bytes(&sequential_a).unwrap(),
        canonical_bytes(&sequential_b).unwrap()
    );
}

#[test]
fn compiler_and_runtime_cases_are_explicitly_outside_wp04() {
    let excluded = [
        ("branch-a.adl.yaml", "pattern expansion belongs to #5338"),
        ("branch-b.adl.yaml", "pattern expansion belongs to #5338"),
        ("fork-join.adl.yaml", "pattern expansion belongs to #5338"),
    ];
    for (name, reason) in excluded {
        let diagnostics = parse_and_validate_yaml(&fixture(name)).unwrap_err();
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::UnknownField),
            "{name}: {reason}"
        );
    }

    // The core can represent and validate this run. Actually executing its
    // mock provider remains deliberately outside WP-04 and belongs to #5340.
    parse_and_validate_yaml(&fixture("mock-run.adl.yaml")).unwrap();
}
