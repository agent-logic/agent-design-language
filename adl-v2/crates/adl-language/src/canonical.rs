use crate::{validate, AdlDocument, Diagnostic};
use serde_json::Value;

pub fn canonical_json(document: &AdlDocument) -> Result<Value, Vec<Diagnostic>> {
    validate(document)?;
    Ok(serde_json::to_value(document).expect("document serialization is infallible"))
}

pub fn canonical_bytes(document: &AdlDocument) -> Result<Vec<u8>, Vec<Diagnostic>> {
    let value = canonical_json(document)?;
    Ok(serde_json::to_vec(&value).expect("canonical serialization is infallible"))
}
