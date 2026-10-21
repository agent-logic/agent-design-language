//! YAML behavior used by the public ADL document contracts.
//!
//! Keep YAML keys and tags until schema validation. Parser-owned filesystem
//! includes and property interpolation are disabled at the dependency level.
use anyhow::Result;
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{
    de::{self, DeserializeOwned, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{
    borrow::Borrow,
    fmt,
    hash::{Hash, Hasher},
    ops::Index,
};

fn options() -> serde_saphyr::Options {
    let mut options = serde_saphyr::Options::default();
    options.strict_booleans = true;
    options.merge_keys = serde_saphyr::MergeKeyPolicy::AsOrdinary;
    options.non_finite_float_policy = serde_saphyr::NonFiniteFloatPolicy::PassThrough;
    let mut budget = serde_saphyr::Budget::default();
    budget.max_depth = 128;
    options.budget = Some(budget);
    options
}

// Use parser spans, never textual substitution, to retain the old scalar type
// boundary. In particular Saphyr accepts numeric text in quoted typed fields and
// infers leading-zero/overflowing decimals differently from serde_yaml.
fn legacy_scalar_types(text: &str) -> std::borrow::Cow<'_, str> {
    use serde_saphyr::granit_parser::{Event, Parser, ScalarStyle};
    let mut inserts = Vec::new();
    for event in Parser::new_from_str(text) {
        let Ok((event, span)) = event else {
            return text.into();
        };
        if let Event::Scalar(value, style, _, tag) = event {
            if tag.as_ref().is_some_and(|t| t.core_suffix().is_some()) {
                continue;
            }
            let digits = value.trim_start_matches(['+', '-']);
            let leading_zero = digits.len() > 1
                && digits.starts_with('0')
                && digits.bytes().all(|b| b.is_ascii_digit());
            let overflow = value.parse::<f64>().is_ok_and(|n| !n.is_finite())
                && !matches!(
                    value.as_ref(),
                    ".inf"
                        | ".Inf"
                        | ".INF"
                        | "-.inf"
                        | "-.Inf"
                        | "-.INF"
                        | "+.inf"
                        | "+.Inf"
                        | "+.INF"
                        | ".nan"
                        | ".NaN"
                        | ".NAN"
                );
            if style == ScalarStyle::Plain && (leading_zero || overflow) {
                if let Some(range) = span.byte_range() {
                    inserts.push((range, tag.is_some(), value.into_owned()));
                }
            }
        }
    }
    if inserts.is_empty() {
        return text.into();
    }
    let mut out = String::with_capacity(text.len() + inserts.len() * 6);
    let mut cursor = 0;
    for (range, custom_tag, value) in inserts {
        out.push_str(&text[cursor..range.start]);
        if custom_tag {
            out.push_str(
                &serde_json::to_string(&value).expect("string serialization is infallible"),
            );
            cursor = range.end;
        } else {
            out.push_str("!!str ");
            cursor = range.start;
        }
    }
    out.push_str(&text[cursor..]);
    out.into()
}

mod parsed;
mod serialized;

pub fn from_str<T: DeserializeOwned>(text: &str) -> Result<T> {
    let normalized = legacy_scalar_types(text);
    let mut value: parsed::Node = serde_saphyr::from_str_with_options(&normalized, options())?;
    let mut lexemes = std::collections::HashMap::new();
    for event in serde_saphyr::granit_parser::Parser::new_from_str(&normalized) {
        let (event, span) = event?;
        if let serde_saphyr::granit_parser::Event::Scalar(text, _, _, _) = event {
            if let Some(range) = span.byte_range() {
                lexemes.insert(range.start as u64, text.into_owned());
            }
        }
    }
    value.lexemes(&lexemes);
    Ok(T::deserialize(value)?)
}
pub fn from_slice<T: DeserializeOwned>(bytes: &[u8]) -> Result<T> {
    from_str(std::str::from_utf8(bytes)?)
}
pub fn to_string<T: Serialize>(value: &T) -> Result<String> {
    let mut options = serde_saphyr::SerializerOptions::default();
    options.tagged_enums = true;
    let value = value.serialize(serialized::Capture)?;
    serialized::preserve_bang_tag(serde_saphyr::to_string_with_options(&value, options)?)
}
pub fn from_value<T: DeserializeOwned>(value: Value) -> Result<T> {
    Ok(T::deserialize(value)?)
}

#[derive(Clone, Debug)]
pub enum Number {
    Signed(i64),
    Unsigned(u64),
    Float(f64),
}
impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Signed(a), Self::Signed(b)) => a == b,
            (Self::Unsigned(a), Self::Unsigned(b)) => a == b,
            (Self::Float(a), Self::Float(b)) => a == b || (a.is_nan() && b.is_nan()),
            _ => false,
        }
    }
}
impl Eq for Number {}
impl Hash for Number {
    fn hash<H: Hasher>(&self, h: &mut H) {
        std::mem::discriminant(self).hash(h);
        match self {
            Self::Signed(n) => n.hash(h),
            Self::Unsigned(n) => n.hash(h),
            Self::Float(n) => {
                let bits = if n.is_nan() {
                    f64::NAN.to_bits()
                } else if *n == 0.0 {
                    0
                } else {
                    n.to_bits()
                };
                bits.hash(h);
            }
        }
    }
}
impl From<i32> for Number {
    fn from(n: i32) -> Self {
        if n >= 0 {
            Self::Unsigned(n as u64)
        } else {
            Self::Signed(n.into())
        }
    }
}
impl Serialize for Number {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Signed(n) => s.serialize_i64(*n),
            Self::Unsigned(n) => s.serialize_u64(*n),
            Self::Float(n) => s.serialize_f64(*n),
        }
    }
}

/// Ordered YAML entries, including non-string keys, for include merging.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Mapping(indexmap::IndexMap<Value, Value>);
impl Mapping {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn get(&self, key: impl Borrow<Value>) -> Option<&Value> {
        self.0.get(key.borrow())
    }
    pub fn get_mut(&mut self, key: impl Borrow<Value>) -> Option<&mut Value> {
        self.0.get_mut(key.borrow())
    }
    pub fn contains_key(&self, key: impl Borrow<Value>) -> bool {
        self.0.contains_key(key.borrow())
    }
    pub fn insert(&mut self, key: Value, value: Value) -> Option<Value> {
        self.0.insert(key, value)
    }
    pub fn remove(&mut self, key: &Value) -> Option<Value> {
        self.0.shift_remove(key)
    }
    pub fn keys(&self) -> impl Iterator<Item = &Value> {
        self.0.keys()
    }
}
impl Hash for Mapping {
    fn hash<H: Hasher>(&self, h: &mut H) {
        // Mapping equality ignores insertion order, including for mapping keys.
        let mut entries: Vec<_> = self
            .0
            .iter()
            .map(|entry| {
                let mut hash = std::collections::hash_map::DefaultHasher::new();
                entry.hash(&mut hash);
                hash.finish()
            })
            .collect();
        entries.sort_unstable();
        entries.hash(h);
    }
}
impl<'a> IntoIterator for &'a Mapping {
    type Item = (&'a Value, &'a Value);
    type IntoIter = indexmap::map::Iter<'a, Value, Value>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Sequence(Vec<Value>),
    Mapping(Mapping),
    Tagged(String, Box<Value>),
}
impl Value {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(v) => Some(*v),
            _ => None,
        }
    }
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Number(Number::Signed(n)) => Some(*n),
            Self::Number(Number::Unsigned(n)) => i64::try_from(*n).ok(),
            _ => None,
        }
    }
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Self::Number(Number::Unsigned(n)) => Some(*n),
            Self::Number(Number::Signed(n)) => u64::try_from(*n).ok(),
            _ => None,
        }
    }
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Number(Number::Float(n)) => Some(*n),
            Self::Number(Number::Signed(n)) => Some(*n as f64),
            Self::Number(Number::Unsigned(n)) => Some(*n as f64),
            _ => None,
        }
    }
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }
    pub fn is_sequence(&self) -> bool {
        matches!(self, Self::Sequence(_))
    }
    pub fn as_sequence(&self) -> Option<&Vec<Value>> {
        match self {
            Self::Sequence(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_sequence_mut(&mut self) -> Option<&mut Vec<Value>> {
        match self {
            Self::Sequence(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_mapping(&self) -> Option<&Mapping> {
        match self {
            Self::Mapping(m) => Some(m),
            _ => None,
        }
    }
    pub fn as_mapping_mut(&mut self) -> Option<&mut Mapping> {
        match self {
            Self::Mapping(m) => Some(m),
            _ => None,
        }
    }
    /// Match the former YAML Value -> JSON schema boundary, including tags.
    pub fn to_json(&self) -> Result<serde_json::Value> {
        struct Json<'a>(&'a Value);
        impl Serialize for Json<'_> {
            fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                match self.0 {
                    Value::Tagged(tag, value) => {
                        let mut m = s.serialize_map(Some(1))?;
                        m.serialize_entry(tag, &Json(value))?;
                        m.end()
                    }
                    Value::Mapping(map) => {
                        let mut m = s.serialize_map(Some(map.len()))?;
                        for (k, v) in map {
                            m.serialize_entry(&Json(k), &Json(v))?;
                        }
                        m.end()
                    }
                    Value::Sequence(seq) => {
                        let mut out = s.serialize_seq(Some(seq.len()))?;
                        for v in seq {
                            out.serialize_element(&Json(v))?;
                        }
                        out.end()
                    }
                    other => other.serialize(s),
                }
            }
        }
        Ok(serde_json::to_value(Json(self))?)
    }
}
impl Index<&str> for Value {
    type Output = Value;
    fn index(&self, key: &str) -> &Value {
        self.as_mapping()
            .and_then(|m| m.get(Value::String(key.into())))
            .unwrap_or(&Value::Null)
    }
}
impl Serialize for Value {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Null => s.serialize_unit(),
            Self::Bool(v) => s.serialize_bool(*v),
            Self::Number(n) => n.serialize(s),
            Self::String(v) => s.serialize_str(v),
            Self::Sequence(v) => v.serialize(s),
            Self::Mapping(m) => {
                let mut out = s.serialize_map(Some(m.len()))?;
                for (k, v) in m {
                    out.serialize_entry(k, v)?;
                }
                out.end()
            }
            Self::Tagged(tag, value) => serde_saphyr::Tagged(value, Some(tag.clone())).serialize(s),
        }
    }
}
impl<'de> Deserialize<'de> for Value {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Untagged(Value);
        impl<'de> Deserialize<'de> for Untagged {
            fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                struct V;
                impl<'de> Visitor<'de> for V {
                    type Value = Value;
                    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        f.write_str("a YAML value")
                    }
                    fn visit_unit<E: de::Error>(self) -> Result<Value, E> {
                        Ok(Value::Null)
                    }
                    fn visit_none<E: de::Error>(self) -> Result<Value, E> {
                        Ok(Value::Null)
                    }
                    fn visit_bool<E: de::Error>(self, v: bool) -> Result<Value, E> {
                        Ok(Value::Bool(v))
                    }
                    fn visit_i64<E: de::Error>(self, v: i64) -> Result<Value, E> {
                        Ok(Value::Number(if v >= 0 {
                            Number::Unsigned(v as u64)
                        } else {
                            Number::Signed(v)
                        }))
                    }
                    fn visit_u64<E: de::Error>(self, v: u64) -> Result<Value, E> {
                        Ok(Value::Number(Number::Unsigned(v)))
                    }
                    fn visit_f64<E: de::Error>(self, v: f64) -> Result<Value, E> {
                        Ok(Value::Number(Number::Float(v)))
                    }
                    fn visit_str<E: de::Error>(self, v: &str) -> Result<Value, E> {
                        Ok(Value::String(v.into()))
                    }
                    fn visit_string<E: de::Error>(self, v: String) -> Result<Value, E> {
                        Ok(Value::String(v))
                    }
                    fn visit_enum<A: de::EnumAccess<'de>>(self, a: A) -> Result<Value, A::Error> {
                        use serde::de::VariantAccess;
                        let (tag, variant) = a.variant::<String>()?;
                        Ok(Value::Tagged(
                            format!("!{tag}"),
                            Box::new(variant.newtype_variant()?),
                        ))
                    }
                    fn visit_seq<A: SeqAccess<'de>>(self, mut a: A) -> Result<Value, A::Error> {
                        let mut v = Vec::new();
                        while let Some(x) = a.next_element()? {
                            v.push(x);
                        }
                        Ok(Value::Sequence(v))
                    }
                    fn visit_map<A: MapAccess<'de>>(self, mut a: A) -> Result<Value, A::Error> {
                        let mut m = Mapping::new();
                        while let Some((k, v)) = a.next_entry()? {
                            if m.insert(k, v).is_some() {
                                return Err(de::Error::custom("duplicate YAML key"));
                            }
                        }
                        Ok(Value::Mapping(m))
                    }
                }
                d.deserialize_any(V).map(Untagged)
            }
        }
        let serde_saphyr::Tagged(Untagged(value), tag) =
            serde_saphyr::Tagged::<Untagged>::deserialize(d)?;
        // Standard resolved YAML tags describe the scalar type already captured.
        Ok(match tag {
            Some(t) if !t.starts_with("tag:yaml.org,2002:") => Self::Tagged(t, Box::new(value)),
            _ => value,
        })
    }
}

impl<'de> de::IntoDeserializer<'de, de::value::Error> for Value {
    type Deserializer = Self;
    fn into_deserializer(self) -> Self {
        self
    }
}

// Typed decoding runs through the retained YAML node types rather than Saphyr's
// permissive string-to-number coercions. This also preserves tagged payload types.
impl<'de> Deserializer<'de> for Value {
    type Error = de::value::Error;
    fn deserialize_any<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
        match self {
            Self::Null => v.visit_unit(),
            Self::Bool(b) => v.visit_bool(b),
            Self::Number(Number::Signed(n)) => v.visit_i64(n),
            Self::Number(Number::Unsigned(n)) => v.visit_u64(n),
            Self::Number(Number::Float(n)) => v.visit_f64(n),
            Self::String(s) => v.visit_string(s),
            Self::Sequence(seq) => {
                de::value::SeqDeserializer::new(seq.into_iter()).deserialize_any(v)
            }
            Self::Mapping(map) => {
                de::value::MapDeserializer::new(map.0.into_iter()).deserialize_any(v)
            }
            Self::Tagged(tag, value) => v.visit_enum(YamlEnum(tag, Some(*value))),
        }
    }
    fn deserialize_option<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
        if self.is_null() {
            v.visit_none()
        } else {
            v.visit_some(self)
        }
    }
    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _: &'static str,
        v: V,
    ) -> Result<V::Value, Self::Error> {
        v.visit_newtype_struct(self)
    }
    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        v: V,
    ) -> Result<V::Value, Self::Error> {
        match self {
            Self::String(s) => v.visit_enum(YamlEnum(s, None)),
            Self::Tagged(tag, value) => v.visit_enum(YamlEnum(tag, Some(*value))),
            other => other.deserialize_any(v),
        }
    }
    fn deserialize_ignored_any<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
        v.visit_unit()
    }
    fn deserialize_bool<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
        match self {
            Self::Tagged(_, value) => value.deserialize_bool(v),
            other => other.deserialize_any(v),
        }
    }
    fn deserialize_i8<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
        match self {
            Self::Tagged(_, value) => value.deserialize_i8(v),
            other => other.deserialize_any(v),
        }
    }
    fn deserialize_i16<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
        match self {
            Self::Tagged(_, value) => value.deserialize_i16(v),
            other => other.deserialize_any(v),
        }
    }
    fn deserialize_i32<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
        match self {
            Self::Tagged(_, value) => value.deserialize_i32(v),
            other => other.deserialize_any(v),
        }
    }
    fn deserialize_i64<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
        match self {
            Self::Tagged(_, value) => value.deserialize_i64(v),
            other => other.deserialize_any(v),
        }
    }
    fn deserialize_i128<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
        match self {
            Self::Tagged(_, value) => value.deserialize_i128(v),
            other => other.deserialize_any(v),
        }
    }
    fn deserialize_u8<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
        match self {
            Self::Tagged(_, value) => value.deserialize_u8(v),
            other => other.deserialize_any(v),
        }
    }
    fn deserialize_u16<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
        match self {
            Self::Tagged(_, value) => value.deserialize_u16(v),
            other => other.deserialize_any(v),
        }
    }
    fn deserialize_u32<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
        match self {
            Self::Tagged(_, value) => value.deserialize_u32(v),
            other => other.deserialize_any(v),
        }
    }
    fn deserialize_u64<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
        match self {
            Self::Tagged(_, value) => value.deserialize_u64(v),
            other => other.deserialize_any(v),
        }
    }
    fn deserialize_u128<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
        match self {
            Self::Tagged(_, value) => value.deserialize_u128(v),
            other => other.deserialize_any(v),
        }
    }
    fn deserialize_f32<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
        match self {
            Self::Tagged(_, value) => value.deserialize_f32(v),
            other => other.deserialize_any(v),
        }
    }
    fn deserialize_f64<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
        match self {
            Self::Tagged(_, value) => value.deserialize_f64(v),
            other => other.deserialize_any(v),
        }
    }
    fn deserialize_bytes<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
        match self {
            Self::Tagged(_, value) => value.deserialize_bytes(v),
            other => other.deserialize_any(v),
        }
    }
    fn deserialize_byte_buf<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
        match self {
            Self::Tagged(_, value) => value.deserialize_byte_buf(v),
            other => other.deserialize_any(v),
        }
    }
    fn deserialize_identifier<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
        match self {
            Self::Tagged(_, value) => value.deserialize_identifier(v),
            other => other.deserialize_any(v),
        }
    }
    fn deserialize_char<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
        match self {
            Self::Tagged(_, value) => value.deserialize_char(v),
            other => other.deserialize_any(v),
        }
    }
    fn deserialize_str<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
        match self {
            Self::Tagged(_, value) => value.deserialize_str(v),
            other => other.deserialize_any(v),
        }
    }
    fn deserialize_string<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
        match self {
            Self::Tagged(_, value) => value.deserialize_string(v),
            other => other.deserialize_any(v),
        }
    }
    fn deserialize_seq<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
        match self {
            Self::Tagged(_, value) => value.deserialize_seq(v),
            Self::Null => Self::Sequence(Vec::new()).deserialize_any(v),
            node @ Self::Sequence(..) => node.deserialize_any(v),
            _ => Err(de::Error::custom("expected YAML sequence")),
        }
    }
    fn deserialize_map<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
        match self {
            Self::Tagged(_, value) => value.deserialize_map(v),
            Self::Null => Self::Mapping(Mapping::new()).deserialize_any(v),
            node @ Self::Mapping(..) => node.deserialize_any(v),
            _ => Err(de::Error::custom("expected YAML mapping")),
        }
    }
    fn deserialize_tuple<V: Visitor<'de>>(self, _: usize, v: V) -> Result<V::Value, Self::Error> {
        self.deserialize_seq(v)
    }
    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        _: &'static str,
        _: usize,
        v: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_seq(v)
    }
    fn deserialize_struct<V: Visitor<'de>>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        v: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_map(v)
    }
    serde::forward_to_deserialize_any! { unit unit_struct }
}
struct YamlEnum(String, Option<Value>);
impl<'de> de::EnumAccess<'de> for YamlEnum {
    type Error = de::value::Error;
    type Variant = Self;
    fn variant_seed<V: de::DeserializeSeed<'de>>(
        self,
        seed: V,
    ) -> Result<(V::Value, Self), Self::Error> {
        use serde::de::IntoDeserializer;
        let name = self.0.trim_start_matches('!').to_owned();
        Ok((seed.deserialize(name.into_deserializer())?, self))
    }
}
impl<'de> de::VariantAccess<'de> for YamlEnum {
    type Error = de::value::Error;
    fn unit_variant(self) -> Result<(), Self::Error> {
        match self.1 {
            None | Some(Value::Null) => Ok(()),
            Some(v) => Deserialize::deserialize(v),
        }
    }
    fn newtype_variant_seed<T: de::DeserializeSeed<'de>>(
        self,
        seed: T,
    ) -> Result<T::Value, Self::Error> {
        seed.deserialize(self.1.unwrap_or(Value::Null))
    }
    fn tuple_variant<V: Visitor<'de>>(self, _: usize, v: V) -> Result<V::Value, Self::Error> {
        self.1.unwrap_or(Value::Null).deserialize_seq(v)
    }
    fn struct_variant<V: Visitor<'de>>(
        self,
        _: &'static [&'static str],
        v: V,
    ) -> Result<V::Value, Self::Error> {
        self.1.unwrap_or(Value::Null).deserialize_map(v)
    }
}
