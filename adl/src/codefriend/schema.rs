//! Schema dispatch must not erase duplicate fields before typed validation.
use serde::{
    de::{Error, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use serde_json::Value;
use std::fmt;

pub(crate) struct UniqueValue(pub Value);
impl<'de> Deserialize<'de> for UniqueValue {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct UniqueVisitor;
        impl<'de> Visitor<'de> for UniqueVisitor {
            type Value = UniqueValue;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("JSON with unique object keys")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut result = serde_json::Map::new();
                while let Some(key) = map.next_key::<String>()? {
                    if result.contains_key(&key) {
                        return Err(A::Error::custom("duplicate_json_field"));
                    }
                    let value = map.next_value::<UniqueValue>()?;
                    result.insert(key, value.0);
                }
                Ok(UniqueValue(Value::Object(result)))
            }
            fn visit_seq<A: SeqAccess<'de>>(
                self,
                mut sequence: A,
            ) -> Result<Self::Value, A::Error> {
                let mut values = Vec::new();
                while let Some(value) = sequence.next_element::<UniqueValue>()? {
                    values.push(value.0);
                }
                Ok(UniqueValue(Value::Array(values)))
            }
            fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
                Ok(UniqueValue(Value::String(value.into())))
            }
            fn visit_string<E: Error>(self, value: String) -> Result<Self::Value, E> {
                Ok(UniqueValue(Value::String(value)))
            }
            fn visit_bool<E: Error>(self, value: bool) -> Result<Self::Value, E> {
                Ok(UniqueValue(Value::Bool(value)))
            }
            fn visit_i64<E: Error>(self, value: i64) -> Result<Self::Value, E> {
                Ok(UniqueValue(Value::Number(value.into())))
            }
            fn visit_u64<E: Error>(self, value: u64) -> Result<Self::Value, E> {
                Ok(UniqueValue(Value::Number(value.into())))
            }
            fn visit_f64<E: Error>(self, value: f64) -> Result<Self::Value, E> {
                serde_json::Number::from_f64(value)
                    .map(|n| UniqueValue(Value::Number(n)))
                    .ok_or_else(|| E::custom("nonfinite_json_number"))
            }
            fn visit_unit<E: Error>(self) -> Result<Self::Value, E> {
                Ok(UniqueValue(Value::Null))
            }
            fn visit_none<E: Error>(self) -> Result<Self::Value, E> {
                self.visit_unit()
            }
        }
        deserializer.deserialize_any(UniqueVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::value::{Error as ValueError, F64Deserializer, StringDeserializer};

    #[test]
    fn valid_nested_json_preserves_scalar_types_and_numeric_boundaries() {
        let input = r#"{"array":[null,true,false,-9223372036854775808,18446744073709551615,1.25,"escaped\ntext",{"empty":[]}]}"#;
        let parsed: UniqueValue = serde_json::from_str(input).unwrap();
        assert_eq!(parsed.0, serde_json::from_str::<Value>(input).unwrap());
    }

    #[test]
    fn repeated_keys_are_rejected_at_every_depth_after_escape_decoding() {
        for input in [
            r#"{"schema":"v1","schema":"v2"}"#,
            r#"{"nested":{"key":1,"key":2}}"#,
            r#"[{"key":1,"\u006bey":2}]"#,
        ] {
            let error = serde_json::from_str::<UniqueValue>(input).err().unwrap();
            assert!(error.to_string().contains("duplicate_json_field"));
        }
        let separate: UniqueValue = serde_json::from_str(r#"[{"key":1},{"key":2}]"#).unwrap();
        assert_eq!(separate.0[1]["key"], 2);
    }

    #[test]
    fn malformed_and_trailing_json_never_yield_a_dispatch_value() {
        for input in [r#"{"schema":"v2""#, "[true,]", "{} {}", "1e9999"] {
            assert!(serde_json::from_str::<UniqueValue>(input).is_err());
        }
    }

    #[test]
    fn owned_string_deserialization_preserves_contents() {
        let value =
            UniqueValue::deserialize(StringDeserializer::<ValueError>::new("owned value".into()))
                .unwrap();
        assert_eq!(value.0, Value::String("owned value".into()));
    }

    #[test]
    fn non_json_float_deserializers_cannot_introduce_nonfinite_numbers() {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let result = UniqueValue::deserialize(F64Deserializer::<ValueError>::new(value));
            assert!(result
                .err()
                .unwrap()
                .to_string()
                .contains("nonfinite_json_number"));
        }
    }
}
