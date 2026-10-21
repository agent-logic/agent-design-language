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

#[test]
fn structs_require_mappings() {
    #[derive(Debug, Deserialize)]
    struct Fields {
        #[serde(rename = "x")]
        _x: u8,
    }
    assert!(serde_yaml::from_str::<Fields>("[1]").is_err());
    assert!(serde_yaml::from_value::<Fields>(serde_yaml::from_str("[1]").unwrap()).is_err());
    assert!(serde_yaml::from_str::<Fields>("{x: 1}").is_ok());
}

fn same_typed_result<T>(text: &str, expected: T)
where
    T: serde::de::DeserializeOwned + std::fmt::Debug + PartialEq,
{
    assert_eq!(
        serde_yaml::from_str::<T>(text).unwrap_or_else(|e| panic!("{text:?}: {e}")),
        expected,
        "direct {text}"
    );
    assert_eq!(
        serde_yaml::from_value::<T>(serde_yaml::from_str(text).unwrap()).unwrap(),
        expected,
        "value {text}"
    );
}

#[test]
fn tagged_numeric_widths_retain_range_and_type_checks() {
    macro_rules! integer_cases {
        ($($ty:ty),*) => {$({
            same_typed_result::<$ty>("!N 7", 7);
            for text in ["!N '7'", "!N 7.5", "!N true"] {
                assert!(serde_yaml::from_str::<$ty>(text).is_err(), "{text}");
                assert!(serde_yaml::from_value::<$ty>(serde_yaml::from_str(text).unwrap()).is_err(), "{text}");
            }
        })*};
    }
    integer_cases!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
    same_typed_result::<i64>("!N -7", -7);
    same_typed_result::<f32>("!N 1.5", 1.5);
    same_typed_result::<f64>("!N -1.5", -1.5);
    same_typed_result::<bool>("!B true", true);
    same_typed_result::<char>("!C é", 'é');
    same_typed_result::<String>("!S café", "café".into());
    assert!(serde_yaml::from_str::<u8>("256").is_err());
    assert!(serde_yaml::from_str::<i8>("128").is_err());
}

#[test]
fn enum_shapes_options_and_newtypes_roundtrip() {
    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    enum Shape {
        Unit,
        Pair(u8, String),
        Record { value: bool },
    }
    same_typed_result("Unit", Shape::Unit);
    same_typed_result("!Unit null", Shape::Unit);
    same_typed_result("!Pair [7, text]", Shape::Pair(7, "text".into()));
    same_typed_result("!Record {value: true}", Shape::Record { value: true });
    for shape in [
        Shape::Unit,
        Shape::Pair(7, "text".into()),
        Shape::Record { value: false },
    ] {
        same_typed_result(&serde_yaml::to_string(&shape).unwrap(), shape);
    }
    #[derive(Debug, Deserialize, PartialEq)]
    struct New(u16);
    #[derive(Debug, Deserialize, PartialEq)]
    struct Pair(u16, bool);
    same_typed_result("42", New(42));
    same_typed_result("[42, true]", Pair(42, true));
    same_typed_result::<Option<u64>>("null", None);
    same_typed_result::<Option<u64>>("7", Some(7));
    #[derive(Debug, Deserialize, PartialEq)]
    struct Known {
        value: bool,
    }
    same_typed_result(
        "!Record {value: true, ignored: [1,2]}",
        Known { value: true },
    );
    for bad in [
        "!Pair [1, text, 3]",
        "!Pair [1]",
        "!Record [true]",
        "Unknown",
        "17",
    ] {
        assert!(serde_yaml::from_str::<Shape>(bad).is_err());
        assert!(serde_yaml::from_value::<Shape>(serde_yaml::from_str(bad).unwrap()).is_err());
    }
    assert!(serde_yaml::from_slice::<Value>(&[0xff]).is_err());
    assert_eq!(serde_yaml::from_slice::<String>(b"hello").unwrap(), "hello");
}

#[test]
fn mapping_keys_use_semantic_equality_and_numeric_hashes() {
    let value: Value = serde_yaml::from_str(
        "? {a: 1, b: 2}\n: result\n? .nan\n: nan\n? -0.0\n: zero\n? -7\n: negative\n",
    )
    .unwrap();
    let mapping = value.as_mapping().unwrap();
    assert_eq!(
        mapping
            .get(serde_yaml::from_str::<Value>("{b: 2, a: 1}").unwrap())
            .unwrap()
            .as_str(),
        Some("result")
    );
    assert_eq!(
        mapping
            .get(serde_yaml::from_str::<Value>(".nan").unwrap())
            .unwrap()
            .as_str(),
        Some("nan")
    );
    assert_eq!(
        mapping
            .get(serde_yaml::from_str::<Value>("0.0").unwrap())
            .unwrap()
            .as_str(),
        Some("zero")
    );
    assert_eq!(
        mapping
            .get(serde_yaml::from_str::<Value>("-7").unwrap())
            .unwrap()
            .as_str(),
        Some("negative")
    );
    assert!(!mapping.is_empty());
}

#[test]
fn serialization_preserves_typed_scalar_and_container_shapes() {
    macro_rules! numeric_roundtrips {
        ($($ty:ty),*) => {$({
            let value: $ty = 7 as $ty;
            same_typed_result(&serde_yaml::to_string(&value).unwrap(), value);
        })*};
    }
    numeric_roundtrips!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64);
    same_typed_result(&serde_yaml::to_string(&-7i64).unwrap(), -7i64);
    same_typed_result(&serde_yaml::to_string(&'é').unwrap(), 'é');
    same_typed_result(&serde_yaml::to_string(&Some(7u16)).unwrap(), Some(7u16));
    same_typed_result(&serde_yaml::to_string(&None::<u16>).unwrap(), None::<u16>);
    same_typed_result(&serde_yaml::to_string(&(7u16, true)).unwrap(), (7u16, true));
    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct New(u16);
    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct Pair(u16, bool);
    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct Unit;
    same_typed_result(&serde_yaml::to_string(&New(7)).unwrap(), New(7));
    same_typed_result(
        &serde_yaml::to_string(&Pair(7, true)).unwrap(),
        Pair(7, true),
    );
    same_typed_result(&serde_yaml::to_string(&Unit).unwrap(), Unit);
    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct Fields {
        label: String,
        enabled: bool,
    }
    let fields = Fields {
        label: "hello".into(),
        enabled: true,
    };
    same_typed_result(&serde_yaml::to_string(&fields).unwrap(), fields);
}

#[test]
fn serialization_retains_wide_integer_and_float_spelling() {
    assert_eq!(
        serde_yaml::to_string(&u128::MAX).unwrap().trim(),
        u128::MAX.to_string()
    );
    assert_eq!(
        serde_yaml::to_string(&i128::MIN).unwrap().trim(),
        i128::MIN.to_string()
    );
    #[derive(Serialize)]
    enum Empty {
        #[serde(rename = "")]
        Value(u8),
    }
    assert_eq!(
        serde_yaml::to_string(&Empty::Value(1)).unwrap(),
        "!<%21> 1\n"
    );
    assert_eq!(serde_yaml::to_string(&0.1f32).unwrap().trim(), "0.1");
    same_typed_result(&serde_yaml::to_string(&"! 1").unwrap(), "! 1".to_owned());
    let nested = vec![Empty::Value(1), Empty::Value(2)];
    let text = serde_yaml::to_string(&nested).unwrap();
    assert!(text.contains("!<%21> 1"));
    assert!(text.contains("!<%21> 2"));
    let parsed: Value = serde_yaml::from_str(&text).unwrap();
    assert_eq!(parsed.as_sequence().unwrap().len(), 2);
}

#[test]
fn duplicate_serialized_keys_are_not_silently_discarded() {
    struct Duplicate;
    impl Serialize for Duplicate {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            use serde::ser::SerializeMap;
            let mut map = serializer.serialize_map(Some(2))?;
            map.serialize_entry("key", &1)?;
            map.serialize_entry("key", &2)?;
            map.end()
        }
    }
    let yaml = serde_yaml::to_string(&Duplicate).unwrap();
    assert!(yaml.contains("key: 1"));
    assert!(yaml.contains("key: 2"));
    assert!(serde_yaml::from_str::<Value>(&yaml).is_err());
}
