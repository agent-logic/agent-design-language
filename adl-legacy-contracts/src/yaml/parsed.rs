//! Parse-local scalar spelling, discarded by ordinary Value deserialization.
use super::*;
use std::collections::HashMap;

pub(super) enum Node {
    Scalar(Value, Option<u64>, Option<String>),
    Sequence(Vec<Node>),
    Mapping(Vec<(Node, Node)>),
    Tagged(String, Box<Node>),
}
impl Node {
    fn scalar(value: Value) -> Self {
        Self::Scalar(value, None, None)
    }
    pub(super) fn lexemes(&mut self, lexemes: &HashMap<u64, String>) {
        match self {
            Self::Scalar(_, offset, raw) => *raw = offset.and_then(|o| lexemes.get(&o).cloned()),
            Self::Sequence(nodes) => nodes.iter_mut().for_each(|n| n.lexemes(lexemes)),
            Self::Mapping(nodes) => nodes.iter_mut().for_each(|(k, v)| {
                k.lexemes(lexemes);
                v.lexemes(lexemes);
            }),
            Self::Tagged(_, node) => node.lexemes(lexemes),
        }
    }
    fn is_null(&self) -> bool {
        matches!(self, Self::Scalar(Value::Null, _, _))
    }
}
impl<'de> Deserialize<'de> for Node {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Untagged(Node);
        impl<'de> Deserialize<'de> for Untagged {
            fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                struct V;
                impl<'de> Visitor<'de> for V {
                    type Value = Node;
                    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        f.write_str("a YAML value")
                    }
                    fn visit_unit<E: de::Error>(self) -> Result<Node, E> {
                        Ok(Node::scalar(Value::Null))
                    }
                    fn visit_none<E: de::Error>(self) -> Result<Node, E> {
                        Ok(Node::scalar(Value::Null))
                    }
                    fn visit_bool<E: de::Error>(self, v: bool) -> Result<Node, E> {
                        Ok(Node::scalar(Value::Bool(v)))
                    }
                    fn visit_i64<E: de::Error>(self, v: i64) -> Result<Node, E> {
                        Ok(Node::scalar(Value::Number(if v >= 0 {
                            Number::Unsigned(v as u64)
                        } else {
                            Number::Signed(v)
                        })))
                    }
                    fn visit_u64<E: de::Error>(self, v: u64) -> Result<Node, E> {
                        Ok(Node::scalar(Value::Number(Number::Unsigned(v))))
                    }
                    fn visit_f64<E: de::Error>(self, v: f64) -> Result<Node, E> {
                        Ok(Node::scalar(Value::Number(Number::Float(v))))
                    }
                    fn visit_str<E: de::Error>(self, v: &str) -> Result<Node, E> {
                        Ok(Node::scalar(Value::String(v.into())))
                    }
                    fn visit_string<E: de::Error>(self, v: String) -> Result<Node, E> {
                        Ok(Node::scalar(Value::String(v)))
                    }
                    fn visit_enum<A: de::EnumAccess<'de>>(self, a: A) -> Result<Node, A::Error> {
                        use serde::de::VariantAccess;
                        let (tag, variant) = a.variant::<String>()?;
                        Ok(Node::Tagged(
                            format!("!{tag}"),
                            Box::new(variant.newtype_variant()?),
                        ))
                    }
                    fn visit_seq<A: SeqAccess<'de>>(self, mut a: A) -> Result<Node, A::Error> {
                        let mut v = Vec::new();
                        while let Some(x) = a.next_element()? {
                            v.push(x);
                        }
                        Ok(Node::Sequence(v))
                    }
                    fn visit_map<A: MapAccess<'de>>(self, mut a: A) -> Result<Node, A::Error> {
                        let mut m = Vec::new();
                        while let Some((k, v)) = a.next_entry()? {
                            m.push((k, v));
                        }
                        Ok(Node::Mapping(m))
                    }
                }
                d.deserialize_any(V).map(Untagged)
            }
        }
        let spanned = serde_saphyr::Spanned::<serde_saphyr::Tagged<Untagged>>::deserialize(d)?;
        let serde_saphyr::Tagged(Untagged(mut value), tag) = spanned.value;
        if let Node::Scalar(_, offset, _) = &mut value {
            *offset = spanned.defined.span().byte_offset();
        }
        Ok(match tag {
            Some(t) if !t.starts_with("tag:yaml.org,2002:") => Node::Tagged(t, Box::new(value)),
            _ => value,
        })
    }
}
impl<'de> de::IntoDeserializer<'de, de::value::Error> for Node {
    type Deserializer = Self;
    fn into_deserializer(self) -> Self {
        self
    }
}

// Typed decoding runs through the retained YAML node types rather than Saphyr's
// permissive string-to-number coercions. This also preserves tagged payload types.
impl<'de> Deserializer<'de> for Node {
    type Error = de::value::Error;
    fn deserialize_any<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
        match self {
            Self::Scalar(value, _, _) => value.deserialize_any(v),
            Self::Sequence(seq) => v.visit_seq(de::value::SeqDeserializer::new(seq.into_iter())),
            Self::Mapping(map) => v.visit_map(de::value::MapDeserializer::new(map.into_iter())),
            Self::Tagged(tag, value) => v.visit_enum(NodeEnum(tag, Some(*value))),
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
            Self::Scalar(Value::String(s), _, _) => v.visit_enum(NodeEnum(s, None)),
            Self::Tagged(tag, value) => v.visit_enum(NodeEnum(tag, Some(*value))),
            other => other.deserialize_any(v),
        }
    }
    fn deserialize_ignored_any<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
        v.visit_unit()
    }
    fn deserialize_str<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
        match self {
            Self::Scalar(_, _, Some(raw)) => v.visit_string(raw),
            Self::Tagged(_, node) => node.deserialize_str(v),
            other => other.deserialize_any(v),
        }
    }
    fn deserialize_string<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
        self.deserialize_str(v)
    }
    fn deserialize_char<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Error> {
        self.deserialize_str(v)
    }
    serde::forward_to_deserialize_any! { bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 bytes byte_buf unit unit_struct seq tuple tuple_struct map struct identifier }
}
struct NodeEnum(String, Option<Node>);
impl<'de> de::EnumAccess<'de> for NodeEnum {
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
impl<'de> de::VariantAccess<'de> for NodeEnum {
    type Error = de::value::Error;
    fn unit_variant(self) -> Result<(), Self::Error> {
        match self.1 {
            None => Ok(()),
            Some(v) => Deserialize::deserialize(v),
        }
    }
    fn newtype_variant_seed<T: de::DeserializeSeed<'de>>(
        self,
        seed: T,
    ) -> Result<T::Value, Self::Error> {
        seed.deserialize(self.1.unwrap_or(Node::scalar(Value::Null)))
    }
    fn tuple_variant<V: Visitor<'de>>(self, _: usize, v: V) -> Result<V::Value, Self::Error> {
        self.1
            .unwrap_or(Node::scalar(Value::Null))
            .deserialize_any(v)
    }
    fn struct_variant<V: Visitor<'de>>(
        self,
        _: &'static [&'static str],
        v: V,
    ) -> Result<V::Value, Self::Error> {
        self.1
            .unwrap_or(Node::scalar(Value::Null))
            .deserialize_any(v)
    }
}
