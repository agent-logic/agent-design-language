//! PVF planning_contract: exact historical constraints and original 27 refusal fixtures.
#[test]
fn historical_split_positive_and_twenty_seven_negatives() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let (p, m) = adl_ci_contracts::planning::inputs(&root).unwrap();
    adl_ci_contracts::planning::validate(&root, &p, &m, true).unwrap();
    assert_eq!(
        adl_ci_contracts::planning::negative_probes(&root, &p, &m).unwrap(),
        27
    );
}
