//! PVF local_contract: canonical actions plus quoted and alternate syntax refusals.
use adl_ci_contracts::actions::*;
#[test]
fn every_workflow() {
    check(&std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")).unwrap();
}
#[test]
fn quoted_and_alternate_syntax() {
    for text in [
        "uses: \"actions/checkout@v7\"",
        "- uses: 'actions/upload-artifact@v7'",
        "uses: unknown/action@main",
        "uses: \"unknown/action@v1\"",
        "uses: dtolnay/rust-toolchain@1.92.0",
        "uses: docker://image:latest",
    ] {
        assert!(require_pin(&parse_uses(text).unwrap().unwrap())
            .unwrap_err()
            .contains("SHA"));
    }
    for text in [
        "- 'uses': unknown/action@main",
        "- { uses: unknown/action@main }",
    ] {
        assert!(parse_uses(text).unwrap_err().contains("unsupported"));
    }
    require_pin("./.github/workflows/local.yml").unwrap();
    require_pin(&format!("unknown/action@{}", "a".repeat(40))).unwrap();
    assert!(require_pin("./../escape").is_err());
}
