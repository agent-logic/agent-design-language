//! Capture Serde enum shapes before emitting YAML; ordinary maps stay maps.
use super::*;
use serde::ser::{
    self, SerializeStruct, SerializeStructVariant, SerializeTuple, SerializeTupleStruct,
    SerializeTupleVariant,
};
type Error = de::value::Error;
// Emission maps retain entries verbatim, including duplicate keys emitted by
// custom Serialize implementations. Parsing remains responsible for rejecting
// duplicate-key documents, matching the former streaming serializer.
pub(super) enum Emission {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Float32(f32),
    Signed128(i128),
    Unsigned128(u128),
    Sequence(Vec<Emission>),
    Mapping(Vec<(Emission, Emission)>),
    Tagged(String, Box<Emission>),
}
impl Serialize for Emission {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Null => s.serialize_unit(),
            Self::Bool(v) => s.serialize_bool(*v),
            Self::Number(v) => v.serialize(s),
            Self::String(v) => s.serialize_str(v),
            Self::Float32(v) => s.serialize_f32(*v),
            Self::Signed128(v) => s.serialize_i128(*v),
            Self::Unsigned128(v) => s.serialize_u128(*v),
            Self::Sequence(v) => v.serialize(s),
            Self::Mapping(entries) => {
                let mut map = s.serialize_map(Some(entries.len()))?;
                for (k, v) in entries {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
            Self::Tagged(tag, v) => serde_saphyr::Tagged(v, Some(tag.clone())).serialize(s),
        }
    }
}
pub(super) struct Capture;
pub(super) struct Sequence {
    values: Vec<Emission>,
    tag: Option<String>,
    tagged_wrapper: bool,
}
pub(super) struct Map {
    values: Vec<(Emission, Emission)>,
    key: Option<Emission>,
    tag: Option<String>,
}
fn tagged(tag: String, value: Emission) -> Emission {
    Emission::Tagged(tag, Box::new(value))
}
macro_rules! signed { ($($name:ident: $ty:ty),*) => {$(fn $name(self, n: $ty) -> Result<Emission,Error> { self.serialize_i64(n.into()) })*}; }
macro_rules! unsigned { ($($name:ident: $ty:ty),*) => {$(fn $name(self, n: $ty) -> Result<Emission,Error> { self.serialize_u64(n.into()) })*}; }
impl Serializer for Capture {
    type Ok = Emission;
    type Error = Error;
    type SerializeSeq = Sequence;
    type SerializeTuple = Sequence;
    type SerializeTupleStruct = Sequence;
    type SerializeTupleVariant = Sequence;
    type SerializeMap = Map;
    type SerializeStruct = Map;
    type SerializeStructVariant = Map;
    fn serialize_bool(self, v: bool) -> Result<Emission, Error> {
        Ok(Emission::Bool(v))
    }
    signed!(serialize_i8:i8,serialize_i16:i16,serialize_i32:i32);
    unsigned!(serialize_u8:u8,serialize_u16:u16,serialize_u32:u32);
    fn serialize_i64(self, v: i64) -> Result<Emission, Error> {
        Ok(Emission::Number(if v >= 0 {
            Number::Unsigned(v as u64)
        } else {
            Number::Signed(v)
        }))
    }
    fn serialize_u64(self, v: u64) -> Result<Emission, Error> {
        Ok(Emission::Number(Number::Unsigned(v)))
    }
    fn serialize_i128(self, v: i128) -> Result<Emission, Error> {
        match i64::try_from(v) {
            Ok(n) => self.serialize_i64(n),
            Err(_) => Ok(Emission::Signed128(v)),
        }
    }
    fn serialize_u128(self, v: u128) -> Result<Emission, Error> {
        match u64::try_from(v) {
            Ok(n) => self.serialize_u64(n),
            Err(_) => Ok(Emission::Unsigned128(v)),
        }
    }
    fn serialize_f32(self, v: f32) -> Result<Emission, Error> {
        Ok(Emission::Float32(v))
    }
    fn serialize_f64(self, v: f64) -> Result<Emission, Error> {
        Ok(Emission::Number(Number::Float(v)))
    }
    fn serialize_char(self, v: char) -> Result<Emission, Error> {
        self.serialize_str(&v.to_string())
    }
    fn serialize_str(self, v: &str) -> Result<Emission, Error> {
        Ok(Emission::String(v.into()))
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<Emission, Error> {
        v.serialize(self)
    }
    fn serialize_none(self) -> Result<Emission, Error> {
        self.serialize_unit()
    }
    fn serialize_some<T: Serialize + ?Sized>(self, v: &T) -> Result<Emission, Error> {
        v.serialize(self)
    }
    fn serialize_unit(self) -> Result<Emission, Error> {
        Ok(Emission::Null)
    }
    fn serialize_unit_struct(self, _: &'static str) -> Result<Emission, Error> {
        self.serialize_unit()
    }
    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        v: &'static str,
    ) -> Result<Emission, Error> {
        self.serialize_str(v)
    }
    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        _: &'static str,
        v: &T,
    ) -> Result<Emission, Error> {
        v.serialize(self)
    }
    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        v: &T,
    ) -> Result<Emission, Error> {
        Ok(tagged(format!("!{variant}"), v.serialize(self)?))
    }
    fn serialize_seq(self, len: Option<usize>) -> Result<Sequence, Error> {
        Ok(Sequence {
            values: Vec::with_capacity(len.unwrap_or(0)),
            tag: None,
            tagged_wrapper: false,
        })
    }
    fn serialize_tuple(self, len: usize) -> Result<Sequence, Error> {
        self.serialize_seq(Some(len))
    }
    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Sequence, Error> {
        let mut s = self.serialize_seq(Some(len))?;
        if name == "__yaml_tagged" {
            if len != 2 {
                return Err(ser::Error::custom("invalid YAML tag wrapper length"));
            }
            s.tagged_wrapper = true;
        }
        Ok(s)
    }
    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Sequence, Error> {
        let mut s = self.serialize_seq(Some(len))?;
        s.tag = Some(format!("!{variant}"));
        Ok(s)
    }
    fn serialize_map(self, _: Option<usize>) -> Result<Map, Error> {
        Ok(Map {
            values: Vec::new(),
            key: None,
            tag: None,
        })
    }
    fn serialize_struct(self, _: &'static str, len: usize) -> Result<Map, Error> {
        self.serialize_map(Some(len))
    }
    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Map, Error> {
        let mut m = self.serialize_map(Some(len))?;
        m.tag = Some(format!("!{variant}"));
        Ok(m)
    }
}
impl Sequence {
    fn push<T: Serialize + ?Sized>(&mut self, v: &T) -> Result<(), Error> {
        self.values.push(v.serialize(Capture)?);
        Ok(())
    }
    fn finish(self) -> Result<Emission, Error> {
        if self.tagged_wrapper {
            let mut values = self.values.into_iter();
            let Some(Emission::String(tag)) = values.next() else {
                return Err(ser::Error::custom("invalid YAML tag wrapper tag"));
            };
            let value = values
                .next()
                .ok_or_else(|| ser::Error::custom("missing YAML tag wrapper value"))?;
            if values.next().is_some() {
                return Err(ser::Error::custom("extra YAML tag wrapper fields"));
            }
            return Ok(tagged(tag, value));
        }
        let value = Emission::Sequence(self.values);
        Ok(match self.tag {
            Some(t) => tagged(t, value),
            None => value,
        })
    }
}
impl SerializeSeq for Sequence {
    type Ok = Emission;
    type Error = Error;
    fn serialize_element<T: Serialize + ?Sized>(&mut self, v: &T) -> Result<(), Error> {
        self.push(v)
    }
    fn end(self) -> Result<Emission, Error> {
        self.finish()
    }
}
impl SerializeTuple for Sequence {
    type Ok = Emission;
    type Error = Error;
    fn serialize_element<T: Serialize + ?Sized>(&mut self, v: &T) -> Result<(), Error> {
        self.push(v)
    }
    fn end(self) -> Result<Emission, Error> {
        self.finish()
    }
}
impl SerializeTupleStruct for Sequence {
    type Ok = Emission;
    type Error = Error;
    fn serialize_field<T: Serialize + ?Sized>(&mut self, v: &T) -> Result<(), Error> {
        self.push(v)
    }
    fn end(self) -> Result<Emission, Error> {
        self.finish()
    }
}
impl SerializeTupleVariant for Sequence {
    type Ok = Emission;
    type Error = Error;
    fn serialize_field<T: Serialize + ?Sized>(&mut self, v: &T) -> Result<(), Error> {
        self.push(v)
    }
    fn end(self) -> Result<Emission, Error> {
        self.finish()
    }
}
impl Map {
    fn entry<T: Serialize + ?Sized>(&mut self, key: &str, v: &T) -> Result<(), Error> {
        self.values
            .push((Emission::String(key.into()), v.serialize(Capture)?));
        Ok(())
    }
    fn finish(self) -> Result<Emission, Error> {
        if self.key.is_some() {
            return Err(ser::Error::custom("YAML map key without value"));
        }
        let value = Emission::Mapping(self.values);
        Ok(match self.tag {
            Some(t) => tagged(t, value),
            None => value,
        })
    }
}
impl SerializeMap for Map {
    type Ok = Emission;
    type Error = Error;
    fn serialize_key<T: Serialize + ?Sized>(&mut self, k: &T) -> Result<(), Error> {
        if self.key.is_some() {
            return Err(ser::Error::custom("YAML map key without value"));
        }
        self.key = Some(k.serialize(Capture)?);
        Ok(())
    }
    fn serialize_value<T: Serialize + ?Sized>(&mut self, v: &T) -> Result<(), Error> {
        let key = self
            .key
            .take()
            .ok_or_else(|| ser::Error::custom("YAML map value without key"))?;
        self.values.push((key, v.serialize(Capture)?));
        Ok(())
    }
    fn end(self) -> Result<Emission, Error> {
        self.finish()
    }
}
impl SerializeStruct for Map {
    type Ok = Emission;
    type Error = Error;
    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        k: &'static str,
        v: &T,
    ) -> Result<(), Error> {
        self.entry(k, v)
    }
    fn end(self) -> Result<Emission, Error> {
        self.finish()
    }
}
impl SerializeStructVariant for Map {
    type Ok = Emission;
    type Error = Error;
    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        k: &'static str,
        v: &T,
    ) -> Result<(), Error> {
        self.entry(k, v)
    }
    fn end(self) -> Result<Emission, Error> {
        self.finish()
    }
}

// Saphyr spells the resolved tag "!" as a non-specific bare bang. The former
// serializer used a verbatim tag. Rewrite only scanner-identified tag tokens;
// quoted/block scalar contents and ordinary text are never substitution targets.
pub(super) fn preserve_bang_tag(text: String) -> Result<String> {
    use serde_saphyr::granit_parser::{Scanner, StrInput, TokenType};
    let mut ranges = Vec::new();
    for token in Scanner::new(StrInput::new(&text)) {
        let (span, kind) = token?.into_parts();
        if matches!(kind, TokenType::Tag(..)) {
            if let Some(range) = span.byte_range() {
                if &text[range.clone()] == "!" {
                    ranges.push(range);
                }
            }
        }
    }
    if ranges.is_empty() {
        return Ok(text);
    }
    let mut output = String::with_capacity(text.len() + ranges.len() * 6);
    let mut cursor = 0;
    for range in ranges {
        output.push_str(&text[cursor..range.start]);
        output.push_str("!<%21>");
        cursor = range.end;
    }
    output.push_str(&text[cursor..]);
    Ok(output)
}
