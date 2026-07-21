use adl_compiler::compile;
use adl_language::parse_and_validate_yaml;

const SIX_PRIMITIVES: &str =
    include_str!("../../../../adl-characterization/corpus/v1/fixtures/six-primitives.adl.yaml");
const MAP_A: &str =
    include_str!("../../../../adl-characterization/corpus/v1/fixtures/map-a.adl.yaml");
const SEQUENTIAL_A: &str =
    include_str!("../../../../adl-characterization/corpus/v1/fixtures/sequential-a.adl.yaml");
const BRANCH_A: &str =
    include_str!("../../../../adl-characterization/corpus/v1/fixtures/branch-a.adl.yaml");

#[test]
fn applicable_landed_fixtures_compile() {
    for fixture in [SIX_PRIMITIVES, MAP_A, SEQUENTIAL_A] {
        let document = parse_and_validate_yaml(fixture).unwrap();
        let plan = compile(&document).unwrap();
        assert!(!plan.nodes.is_empty());
    }
}

#[test]
fn legacy_patterns_are_explicitly_rejected_by_language_boundary() {
    let diagnostics = parse_and_validate_yaml(BRANCH_A).unwrap_err();
    let debug = format!("{diagnostics:?}");
    assert!(debug.contains("pattern"));
}
