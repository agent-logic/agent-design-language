use crate::{validate, AdlDocument, Diagnostic, DiagnosticCode};
use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde::Deserialize;
use serde_json::{Map, Number, Value};
use std::{collections::BTreeSet, fmt};

pub fn parse_yaml(source: &str) -> Result<AdlDocument, Diagnostic> {
    let deserializer = yaml_serde::Deserializer::from_str(source);
    let value = StrictValue
        .deserialize(deserializer)
        .map_err(classify_parse_error)?;
    AdlDocument::deserialize(value).map_err(classify_typed_error)
}

pub fn parse_json(source: &str) -> Result<AdlDocument, Diagnostic> {
    let mut deserializer = serde_json::Deserializer::from_str(source);
    let value = StrictValue
        .deserialize(&mut deserializer)
        .map_err(classify_parse_error)?;
    deserializer.end().map_err(classify_parse_error)?;
    AdlDocument::deserialize(value).map_err(classify_typed_error)
}

pub fn parse_and_validate_yaml(source: &str) -> Result<AdlDocument, Vec<Diagnostic>> {
    let document = parse_yaml(source).map_err(|error| vec![error])?;
    validate(&document)?;
    Ok(document)
}

pub fn parse_and_validate_json(source: &str) -> Result<AdlDocument, Vec<Diagnostic>> {
    let document = parse_json(source).map_err(|error| vec![error])?;
    validate(&document)?;
    Ok(document)
}

fn classify_parse_error(error: impl fmt::Display) -> Diagnostic {
    let message = error.to_string();
    let code = if message.contains("duplicate key") {
        DiagnosticCode::DuplicateKey
    } else {
        DiagnosticCode::Syntax
    };
    Diagnostic::new(code, "$", message)
}

fn classify_typed_error(error: impl fmt::Display) -> Diagnostic {
    let message = error.to_string();
    let code = if message.contains("unknown field") {
        DiagnosticCode::UnknownField
    } else if message.contains("duplicate field") {
        DiagnosticCode::DuplicateKey
    } else {
        DiagnosticCode::Syntax
    };
    Diagnostic::new(code, "$", message)
}

struct StrictValue;

impl<'de> DeserializeSeed<'de> for StrictValue {
    type Value = Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(StrictValueVisitor)
    }
}

struct StrictValueVisitor;

impl<'de> Visitor<'de> for StrictValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a YAML or JSON value with string object keys")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Value, E> {
        Ok(Value::Bool(value))
    }
    fn visit_i64<E>(self, value: i64) -> Result<Value, E> {
        Ok(Value::Number(value.into()))
    }
    fn visit_u64<E>(self, value: u64) -> Result<Value, E> {
        Ok(Value::Number(value.into()))
    }
    fn visit_f64<E: de::Error>(self, value: f64) -> Result<Value, E> {
        Number::from_f64(value)
            .map(Value::Number)
            .ok_or_else(|| E::custom("non-finite number"))
    }
    fn visit_str<E>(self, value: &str) -> Result<Value, E> {
        Ok(Value::String(value.into()))
    }
    fn visit_string<E>(self, value: String) -> Result<Value, E> {
        Ok(Value::String(value))
    }
    fn visit_none<E>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }
    fn visit_unit<E>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }
    fn visit_some<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<Value, D::Error> {
        StrictValue.deserialize(deserializer)
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut sequence: A) -> Result<Value, A::Error> {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element_seed(StrictValue)? {
            values.push(value);
        }
        Ok(Value::Array(values))
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Value, A::Error> {
        let mut seen = BTreeSet::new();
        let mut values = Map::new();
        while let Some(key) = map.next_key::<String>()? {
            if !seen.insert(key.clone()) {
                return Err(de::Error::custom(format!("duplicate key `{key}`")));
            }
            values.insert(key, map.next_value_seed(StrictValue)?);
        }
        Ok(Value::Object(values))
    }
}
