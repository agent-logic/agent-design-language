//! Fitness schema dispatch preserves original payloads and owner validation.
use super::{language, local};
use crate::codefriend::evidence::{contracts::ReviewRecord, store::Store};
use anyhow::Result;
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub enum PolicyArtifact {
    V1(local::Policy),
    V2(language::Policy),
}
#[derive(Debug, Clone)]
pub enum FitnessArtifact {
    V1(local::Report),
    V2(language::Report),
}

macro_rules! payload_dispatch {
    ($name:ident, $old:ty, $new:ty) => {
        impl Serialize for $name {
            fn serialize<S: Serializer>(
                &self,
                serializer: S,
            ) -> std::result::Result<S::Ok, S::Error> {
                match self {
                    Self::V1(v) => v.serialize(serializer),
                    Self::V2(v) => v.serialize(serializer),
                }
            }
        }
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
                let value = crate::codefriend::schema::UniqueValue::deserialize(d)?.0;
                match value.get("schema").and_then(serde_json::Value::as_str) {
                    Some(local::VERSION) => serde_json::from_value::<$old>(value)
                        .map(Self::V1)
                        .map_err(D::Error::custom),
                    Some(language::VERSION) => serde_json::from_value::<$new>(value)
                        .map(Self::V2)
                        .map_err(D::Error::custom),
                    _ => Err(D::Error::custom("unsupported_fitness_schema")),
                }
            }
        }
    };
}
payload_dispatch!(PolicyArtifact, local::Policy, language::Policy);
payload_dispatch!(FitnessArtifact, local::Report, language::Report);
impl PolicyArtifact {
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::V1(v) => v.validate(),
            Self::V2(v) => v.validate(),
        }
    }
}
impl FitnessArtifact {
    pub fn schema(&self) -> &'static str {
        match self {
            Self::V1(_) => local::VERSION,
            Self::V2(_) => language::VERSION,
        }
    }
    pub fn byte_limit(&self) -> usize {
        match self {
            Self::V1(_) => 16 * 1024 * 1024,
            Self::V2(_) => 4 * 1024 * 1024,
        }
    }
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::V1(v) => v.status.exit_code(),
            Self::V2(v) => v.status.exit_code(),
        }
    }
    pub fn json_bytes(&self, pretty: bool) -> Result<Vec<u8>> {
        struct Writer {
            bytes: Vec<u8>,
            limit: usize,
        }
        impl std::io::Write for Writer {
            fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
                if self
                    .bytes
                    .len()
                    .checked_add(b.len())
                    .is_none_or(|n| n > self.limit)
                {
                    return Err(std::io::Error::other("fitness_output_limit"));
                }
                self.bytes.extend_from_slice(b);
                Ok(b.len())
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        let mut writer = Writer {
            bytes: Vec::new(),
            limit: self.byte_limit(),
        };
        if pretty {
            serde_json::to_writer_pretty(&mut writer, self)?;
        } else {
            serde_json::to_writer(&mut writer, self)?;
        }
        Ok(writer.bytes)
    }

    pub fn record(&self) -> &ReviewRecord {
        match self {
            Self::V1(v) => &v.record,
            Self::V2(v) => &v.record,
        }
    }
    pub fn digest(&self) -> &str {
        match self {
            Self::V1(v) => &v.digest,
            Self::V2(v) => &v.digest,
        }
    }
    pub fn passes(&self) -> bool {
        match self {
            Self::V1(v) => v.status == local::Status::Pass,
            Self::V2(v) => v.status == language::Status::Pass,
        }
    }
    pub fn validate(&self, store: &Store, now: u64) -> Result<()> {
        match self {
            Self::V1(v) => v.validate(store),
            Self::V2(v) => v.validate(store, now),
        }
    }
}
pub fn evaluate(
    store: &Store,
    packet_id: &str,
    policy: PolicyArtifact,
    now: u64,
) -> Result<FitnessArtifact> {
    match policy {
        PolicyArtifact::V1(v) => {
            local::local_fitness_runner(store, packet_id, v).map(FitnessArtifact::V1)
        }
        PolicyArtifact::V2(v) => {
            language::evaluate(store, packet_id, &v, now).map(FitnessArtifact::V2)
        }
    }
}

impl From<local::Policy> for PolicyArtifact {
    fn from(value: local::Policy) -> Self {
        Self::V1(value)
    }
}
impl From<language::Policy> for PolicyArtifact {
    fn from(value: language::Policy) -> Self {
        Self::V2(value)
    }
}
