#[test]
fn runtime_crate_has_no_compiler_or_csdlc_dependency_edges() {
    let manifest = include_str!("../Cargo.toml");
    assert!(
        !manifest.contains("adl-compiler"),
        "adl-runtime must not depend on adl-compiler"
    );
    assert!(
        !manifest.contains("adl-csdlc"),
        "adl-runtime must not depend on adl-csdlc"
    );
    assert!(
        !manifest.contains("path = \"../adl\""),
        "adl-runtime must not depend back on the monolithic adl crate"
    );
}
