fn main() {
    assert_eq!(adl_uts::PACKAGE_VERSION, "0.1.0");
    let wire: serde_json::Value = serde_json::from_str(include_str!("../../vendor/adl-uts-0.1.0/schemas/examples/readonly.v1.json")).unwrap();
    let declaration = adl_uts::load_tool_declaration(wire).unwrap();
    assert_eq!(declaration.name, "fixture.safe_read");
    assert_eq!(declaration.side_effect_class, adl_uts::UtsSideEffectClassV1::Read);
    for schema in [adl_uts::V1_SCHEMA_JSON, adl_uts::V1_1_SCHEMA_JSON, adl_uts::V1_1_INVOCATION_SCHEMA_JSON] {
        let value: serde_json::Value = serde_json::from_str(schema).unwrap();
        assert!(value["$id"].is_string());
    }
    assert!(adl_uts::load_tool_declaration(serde_json::json!({"schema_version":"uts.v2"})).is_err());
    println!("adl-uts {} isolated artifact consumed", adl_uts::PACKAGE_VERSION);
}
