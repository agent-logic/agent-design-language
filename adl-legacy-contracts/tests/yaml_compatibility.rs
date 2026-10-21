//! #1339: deterministic local YAML contract regressions; bounded CPU/memory,
//! no network/providers; required public source gate. Baseline: serde_yaml 0.9.34.
use adl_legacy_contracts::yaml as serde_yaml;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

#[test]
fn scalar_types_and_unicode_are_preserved() {
    let value: Value = serde_yaml::from_str(
        "quoted: '42'\ninteger: 42\nyes_word: yes\nboolean: true\nnull_value: null\ntext: |\n  café\n  日本語\n",
    ).unwrap();
    assert_eq!(value["quoted"].as_str(), Some("42"));
    assert_eq!(value["integer"].as_i64(), Some(42));
    assert_eq!(value["yes_word"].as_str(), Some("yes"));
    assert_eq!(value["boolean"].as_bool(), Some(true));
    assert!(value["null_value"].is_null());
    assert_eq!(value["text"].as_str(), Some("café\n日本語\n"));
    let roundtrip: Value = serde_yaml::from_str(&serde_yaml::to_string(&value).unwrap()).unwrap();
    assert_eq!(roundtrip, value);
}

#[test]
fn aliases_expand_but_merge_keys_remain_explicit_values() {
    let value: Value =
        serde_yaml::from_str("base: &base {x: 1}\ncopy: *base\nmerged: {<<: *base, y: 2}\n")
            .unwrap();
    assert_eq!(value["base"], value["copy"]);
    assert_eq!(value["merged"]["<<"]["x"].as_i64(), Some(1));
    assert!(value["merged"]
        .as_mapping()
        .unwrap()
        .get(Value::String("x".into()))
        .is_none());
}

#[test]
fn duplicate_keys_multiple_documents_and_malformed_yaml_are_rejected() {
    for input in [
        "key: 1\nkey: 2\n",
        "---\nx: 1\n---\nx: 2\n",
        "x: [1, 2",
        "x: *missing",
    ] {
        assert!(
            serde_yaml::from_str::<Value>(input).is_err(),
            "accepted {input:?}"
        );
    }
}

#[test]
fn numeric_limits_and_nonfinite_values_are_preserved() {
    let value: Value = serde_yaml::from_str("min: -9223372036854775808\nmax: 18446744073709551615\npositive: .inf\nnegative: -.inf\nnan: .nan\n").unwrap();
    assert_eq!(value["min"].as_i64(), Some(i64::MIN));
    assert_eq!(value["max"].as_u64(), Some(u64::MAX));
    assert_eq!(value["positive"].as_f64(), Some(f64::INFINITY));
    assert_eq!(value["negative"].as_f64(), Some(f64::NEG_INFINITY));
    assert!(value["nan"].as_f64().unwrap().is_nan());
    assert!(serde_yaml::from_str::<u64>("18446744073709551616").is_err());
}

#[test]
fn non_string_keys_survive_parsing_for_adl_owned_validation() {
    let value: Value = serde_yaml::from_str("7: number\n[one, two]: sequence\n").unwrap();
    let map = value.as_mapping().unwrap();
    assert_eq!(map.len(), 2);
    assert_eq!(
        map.get(Value::Number(7.into())).unwrap().as_str(),
        Some("number")
    );
    assert!(map.keys().any(Value::is_sequence));
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Tagged {
    Unit,
    Count(u64),
}

#[test]
fn tagged_enum_and_dynamic_value_roundtrip() {
    let value: Value = serde_yaml::from_str("!Count 7").unwrap();
    assert_eq!(value.to_json().unwrap(), serde_json::json!({"!Count":7}));
    assert_eq!(
        serde_yaml::from_value::<Tagged>(value.clone()).unwrap(),
        Tagged::Count(7)
    );
    let encoded = serde_yaml::to_string(&value).unwrap();
    assert_eq!(
        serde_yaml::from_str::<Tagged>(&encoded).unwrap(),
        Tagged::Count(7)
    );
    assert_eq!(
        serde_yaml::from_str::<Tagged>("Unit").unwrap(),
        Tagged::Unit
    );
}

#[test]
fn bounded_nesting_and_alias_inputs_terminate() {
    let input = format!("{}0{}", "[".repeat(16), "]".repeat(16));
    assert!(serde_yaml::from_str::<Value>(&input).is_ok());
    let excessive = format!("{}0{}", "[".repeat(256), "]".repeat(256));
    assert!(serde_yaml::from_str::<Value>(&excessive).is_err());
    assert!(serde_yaml::from_str::<Value>("&recursive [*recursive]").is_err());
    let input = format!("base: &base [1, 2]\ncopies: [{}]", ["*base"; 16].join(", "));
    assert_eq!(
        serde_yaml::from_str::<Value>(&input).unwrap()["copies"]
            .as_sequence()
            .unwrap()
            .len(),
        16
    );
}

#[test]
fn legacy_scalar_inference_and_quoted_types_are_preserved() {
    for text in ["0123", "1e999"] {
        assert_eq!(
            serde_yaml::from_str::<Value>(text).unwrap().as_str(),
            Some(text)
        );
    }
    assert_eq!(
        serde_yaml::from_str::<Value>("TRUE").unwrap().as_bool(),
        Some(true)
    );
    assert_eq!(
        serde_yaml::from_str::<Value>("False").unwrap().as_bool(),
        Some(false)
    );
    assert_eq!(
        serde_yaml::from_str::<Value>("+0x10").unwrap().as_i64(),
        Some(16)
    );
    assert_eq!(
        serde_yaml::from_str::<Value>("-0x10").unwrap().as_i64(),
        Some(-16)
    );
    assert_eq!(
        serde_yaml::from_str::<Value>("!!str 42").unwrap().as_str(),
        Some("42")
    );
    assert_eq!(
        serde_yaml::from_str::<Value>("!!int 42").unwrap().as_i64(),
        Some(42)
    );
    assert!(serde_yaml::from_str::<u64>("'42'").is_err());
    assert!(serde_yaml::from_str::<bool>("'true'").is_err());
    let value: Value = serde_yaml::from_str(
        "name: café\nscalar: &scalar 0123\ncopy: *scalar\nquoted: '1e999'\nblock: |\n  0123\n",
    )
    .unwrap();
    assert_eq!(value["scalar"].as_str(), Some("0123"));
    assert_eq!(value["copy"], value["scalar"]);
    assert_eq!(value["block"].as_str(), Some("0123\n"));
}

#[test]
fn typed_strings_keep_scalar_spelling_without_numeric_coercion() {
    for (text, expected) in [
        ("42", "42"),
        ("0x10", "0x10"),
        ("TRUE", "TRUE"),
        ("null", "null"),
        ("!!int 42", "42"),
        ("!Count 0123", "0123"),
    ] {
        assert_eq!(
            serde_yaml::from_str::<String>(text).unwrap(),
            expected,
            "{text}"
        );
    }
    let values: Vec<String> = serde_yaml::from_str("[&hex 0x10, *hex, TRUE, null, '42']").unwrap();
    assert_eq!(values, ["0x10", "0x10", "TRUE", "null", "42"]);
    #[derive(Debug, Deserialize, PartialEq)]
    struct Fields {
        name: String,
        values: Vec<String>,
    }
    assert_eq!(
        serde_yaml::from_str::<Fields>("name: TRUE\nvalues: [0x10, null]").unwrap(),
        Fields {
            name: "TRUE".into(),
            values: vec!["0x10".into(), "null".into()]
        }
    );
    #[derive(Debug, Deserialize, PartialEq)]
    enum Count {
        Count(u64),
    }
    assert!(serde_yaml::from_str::<Count>("!Count '42'").is_err());
    assert_eq!(
        serde_yaml::from_str::<Count>("!Count 42").unwrap(),
        Count::Count(42)
    );
    #[derive(Debug, Deserialize, PartialEq)]
    #[serde(untagged)]
    enum Choice {
        Text(String),
        Integer(u64),
    }
    assert_eq!(
        serde_yaml::from_str::<Choice>("42").unwrap(),
        Choice::Integer(42)
    );
    let resolved: Value = serde_yaml::from_str("0x10").unwrap();
    assert!(serde_yaml::from_value::<String>(resolved).is_err());
}

#[test]
fn typed_tags_and_collection_consumption_match_original() {
    assert_eq!(serde_yaml::from_str::<u64>("!X 7").unwrap(), 7);
    assert_eq!(
        serde_yaml::from_value::<u64>(serde_yaml::from_str("!X 7").unwrap()).unwrap(),
        7
    );
    assert_eq!(
        serde_yaml::from_str::<Vec<u64>>("!X [1, 2]").unwrap(),
        [1, 2]
    );
    assert!(serde_yaml::from_str::<(u64, u64)>("[1,2,3]").is_err());
    assert!(
        serde_yaml::from_value::<(u64, u64)>(serde_yaml::from_str("[1,2,3]").unwrap()).is_err()
    );
    assert!(serde_yaml::from_str::<Vec<u64>>("null").is_err());
    assert!(serde_yaml::from_value::<Vec<u64>>(Value::Null)
        .unwrap()
        .is_empty());
    assert!(
        serde_yaml::from_value::<std::collections::BTreeMap<String, u64>>(Value::Null)
            .unwrap()
            .is_empty()
    );
}
