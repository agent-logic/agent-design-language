fn main() {
    assert_eq!(adl_uts::PACKAGE_VERSION, "0.1.0");
    let wire: serde_json::Value = serde_json::from_str(include_str!("../../vendor/adl-uts-0.1.0/fixtures/examples/readonly.v1.json")).unwrap();
    let declaration = adl_uts::load_tool_declaration(wire).unwrap();
    assert_eq!(declaration.name, "fixture.safe_read");
    assert_eq!(declaration.side_effect_class, adl_uts::UtsSideEffectClassV1::Read);
    for (schema, relative) in [
        (adl_uts::V1_SCHEMA_JSON, "v1.0/universal_tool_schema.v1.schema.json"),
        (adl_uts::V1_1_SCHEMA_JSON, "v1.1/universal_tool_schema.v1_1.schema.json"),
        (adl_uts::V1_1_INVOCATION_SCHEMA_JSON, "v1.1/tool_invocation.v1_1.schema.json"),
    ] {
        let resource = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../vendor/adl-uts-0.1.0/resources/schemas/uts").join(relative);
        assert_eq!(schema.as_bytes(), std::fs::read(resource).unwrap());
        let value: serde_json::Value = serde_json::from_str(schema).unwrap();
        assert_eq!(value["$id"], format!("https://raw.githubusercontent.com/agent-logic/agent-design-language/main/adl-spec/schemas/uts/{relative}"));
    }
    assert_eq!(adl_uts::uts_v1_schema_json(), serde_json::from_str::<serde_json::Value>(adl_uts::V1_SCHEMA_JSON).unwrap());
    assert_eq!(adl_uts::uts_v1_1_schema_json(), serde_json::from_str::<serde_json::Value>(adl_uts::V1_1_SCHEMA_JSON).unwrap());
    assert!(adl_uts::load_tool_declaration(serde_json::json!({"schema_version":"uts.v2"})).is_err());
    println!("adl-uts {} isolated artifact consumed", adl_uts::PACKAGE_VERSION);
}
