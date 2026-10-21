//! PVF local_contract: deterministic, CPU-only declaration boundary proof.
//! Required for #1335; no provider, network, registry, or execution authority.
use adl_uts::*;
use serde_json::{json, Value};

fn v11() -> Value {
    serde_json::from_str(include_str!(
        "../resources/conformance/valid/registry-preview-v1_1.json"
    ))
    .unwrap()
}

#[test]
fn lifecycle_and_optional_metadata_roundtrip_without_nulls() {
    for lifecycle in ["draft", "experimental", "stable", "deprecated", "sunset"] {
        let mut value = v11();
        value["lifecycle"] = json!(lifecycle);
        let wire = serde_json::to_value(load_tool_declaration(value).unwrap()).unwrap();
        assert_eq!(wire["lifecycle"], lifecycle);
        load_tool_declaration(wire).unwrap();
    }
    for field in [
        "lifecycle",
        "categories",
        "side_effects",
        "observability",
        "planning",
    ] {
        let mut value = v11();
        value[field] = Value::Null;
        assert!(
            load_tool_declaration(value).is_err(),
            "{field} null accepted"
        );
    }
    let mut value = v11();
    value["planning"] = json!({"high_risk": null});
    assert!(load_tool_declaration(value).is_err());
}

#[test]
fn string_lengths_count_unicode_scalars_without_trimming() {
    for (description, accepted) in [
        ("éééééé", false),
        ("éééééééééééé", true),
        ("            ", true),
    ] {
        let mut value = v11();
        value["description"] = json!(description);
        assert_eq!(load_tool_declaration(value).is_ok(), accepted);
    }
}

#[test]
fn v11_relaxed_fields_do_not_relax_original_v1_validation() {
    let mut value = v11();
    value["version"] = json!("release-candidate");
    value["categories"] = json!([]);
    value["resources"][0]["resource_type"] = json!("External Resource");
    value["errors"][0]["code"] = json!("External Error");
    load_tool_declaration(value).unwrap();
    let original: Value = serde_json::from_str(include_str!(
        "../resources/conformance/valid/safe-read-v1.json"
    ))
    .unwrap();
    for field in ["version", "resource_type", "code"] {
        let mut invalid = original.clone();
        match field {
            "version" => invalid[field] = json!("release-candidate"),
            "resource_type" => invalid["resources"][0][field] = json!("External Resource"),
            _ => invalid["errors"][0][field] = json!("External Error"),
        }
        assert!(
            load_tool_declaration(invalid).is_err(),
            "original v1 {field} bypassed"
        );
    }
    let normalized = load_tool_declaration(original).unwrap();
    validate_uts_v1_1(&normalized).unwrap();
    let wire = serde_json::to_value(normalized).unwrap();
    assert!(wire.get("lifecycle").is_none());
    load_tool_declaration(wire).unwrap();
}

#[test]
fn extensions_are_structural_metadata_not_execution_permission() {
    for key in ["x-acc", "x-freedom_gate", "x-vendor"] {
        let mut value = v11();
        value["extensions"] = json!({key: {"required": true}});
        let declaration = load_tool_declaration(value).unwrap();
        assert!(extension_declares_required(&declaration.extensions[key]));
    }
    for key in [
        "x-",
        "x-authority",
        "x-grant",
        "x-execute",
        "x-permission",
        "x-UPPER",
    ] {
        let mut value = v11();
        value["extensions"] = json!({key: true});
        assert!(load_tool_declaration(value).is_err(), "{key}");
    }
}

#[test]
fn canonical_declaration_cases_run_separately_from_historical_admission() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("resources");
    let manifest: Value = serde_json::from_str(
        &std::fs::read_to_string(root.join("conformance/manifest.json")).unwrap(),
    )
    .unwrap();
    let mut count = 0;
    for case in manifest["fixtures"].as_array().unwrap() {
        if case["schema"] == "tool_invocation.v1.1" {
            continue;
        }
        let path = case["path"].as_str().unwrap();
        let value =
            serde_json::from_str(&std::fs::read_to_string(root.join(path)).unwrap()).unwrap();
        assert_eq!(
            load_tool_declaration(value).is_ok(),
            case["expected"] != "invalid",
            "{path}"
        );
        count += 1;
    }
    assert_eq!(count, 13);
    let historical = conformance::run_uts_conformance_suite();
    assert_eq!(historical.fixture_count, 21);
    assert!(historical.passed);
}
