fn main() {
    let mut cases = Vec::new();
    let source = include_str!("../fixture.yaml");
    let document = adl_language::parse_and_validate_yaml(source).unwrap();
    let first = adl_compiler::compile(&document).unwrap();
    let second = adl_compiler::compile(&document).unwrap();
    assert_eq!(adl_compiler::canonical_plan_bytes(&first).unwrap(),
               adl_compiler::canonical_plan_bytes(&second).unwrap());
    assert_eq!(adl_compiler::EXECUTION_PLAN_VERSION, "adl.execution-plan.v1");
    cases.push("language-compile-determinism");
    let legacy: adl_legacy_contracts::adl::AdlDoc = serde_yaml::from_str(source).unwrap();
    let resolved = adl_legacy_contracts::resolve::resolve_run(&legacy).unwrap();
    assert_eq!(resolved.steps.len(), 1);
    cases.push("legacy-resolution");
    let provider: adl_schema::ProviderSpec = legacy.providers["local"].clone();
    let roundtrip: adl_schema::ProviderSpec = serde_json::from_value(serde_json::to_value(&provider).unwrap()).unwrap();
    assert_eq!(roundtrip, provider);
    cases.push("provider-dto-roundtrip");
    assert!(adl_schema::syntax::remote_endpoint_declaration_valid("https://example.invalid"));
    assert!(!adl_schema::syntax::remote_endpoint_declaration_valid("http://example.invalid"));
    assert!(!adl_schema::syntax::remote_endpoint_declaration_valid("file:///secret"));
    cases.push("declaration-endpoint-denials");
    for schema in [adl_uts::V1_SCHEMA_JSON, adl_uts::V1_1_SCHEMA_JSON, adl_uts::V1_1_INVOCATION_SCHEMA_JSON] {
        let value: serde_json::Value = serde_json::from_str(schema).unwrap();
        assert!(value["$id"].is_string());
    }
    cases.push("uts-three-versioned-schemas");
    assert!(adl_uts::load_tool_declaration(serde_json::json!({"schema_version":"uts.v2"})).is_err());
    cases.push("unsupported-uts-version-denied");
    let report = adl_uts::conformance::run_uts_conformance_suite();
    assert!(report.passed);
    assert!(report.fixture_count > 0);
    cases.push("uts-inert-conformance");
    println!("{}", serde_json::json!({"cases": cases, "expected_cases": 7,
        "executed_cases": cases.len(), "uts_fixture_count": report.fixture_count,
        "uts_cases": report.cases, "passed": true}));
}
