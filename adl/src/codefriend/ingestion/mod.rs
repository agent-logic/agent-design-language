//! Portable, content-addressed acquisition contract shared by adapter transports.
//! Reading a packet validates acquisition evidence; it does not admit durable storage
//! or claim a completed analysis/review.
use anyhow::{bail, ensure, Result};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::Path};
pub mod ci;
pub mod local;

pub const SCHEMA: &str = "codefriend.repository_packet.v1";
pub const MAX_PACKET_BYTES: u64 = 8 * 1024 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Scope {
    pub analysis: Vec<String>,
    pub context: Vec<String>,
    pub max_files: usize,
    pub max_bytes: u64,
    pub max_file_bytes: u64,
}
impl Scope {
    pub fn validate(&self) -> Result<()> {
        ensure!(
            self.max_files > 0 && self.max_files <= 1000,
            "invalid_file_limit"
        );
        ensure!(
            self.max_bytes > 0 && self.max_bytes <= 1024 * 1024,
            "invalid_byte_limit"
        );
        ensure!(
            self.max_file_bytes > 0 && self.max_file_bytes <= self.max_bytes,
            "invalid_file_byte_limit"
        );
        ensure!(!self.analysis.is_empty(), "empty_analysis_scope");
        let mut seen = std::collections::BTreeSet::new();
        for paths in [&self.analysis, &self.context] {
            ensure!(
                paths.windows(2).all(|p| p[0] < p[1]),
                "scope_not_sorted_unique"
            );
            for path in paths {
                validate_path(path)?;
                ensure!(seen.insert(path), "duplicate_scope_path");
            }
        }
        ensure!(seen.len() <= self.max_files, "file_limit_exceeded");
        Ok(())
    }
    pub fn paths(&self) -> Vec<&str> {
        let mut paths: Vec<_> = self
            .analysis
            .iter()
            .chain(&self.context)
            .map(String::as_str)
            .collect();
        paths.sort_unstable();
        paths
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Object {
    pub path: String,
    /// Git object identity, absent only when a requested path is missing.
    pub source_object: Option<String>,
    pub source_bytes: u64,
    /// Digest of admitted content only; omitted bytes are never retained.
    pub content_digest: Option<String>,
    pub content: Option<String>,
    pub disposition: String,
    pub analysis_support: String,
}
/// Git repository storage object format. It participates in packet identity;
/// mixed object-ID lengths and unsupported formats are rejected on readback.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum GitObjectFormat {
    Sha1,
    Sha256,
}
impl GitObjectFormat {
    fn accepts(&self, id: &str) -> bool {
        object_id(id)
            && id.len()
                == match self {
                    Self::Sha1 => 40,
                    Self::Sha256 => 64,
                }
    }
    fn blob_digest(&self, content: &[u8]) -> String {
        use sha2::Digest;
        let header = format!("blob {}\0", content.len());
        match self {
            Self::Sha1 => {
                let mut hash = sha1::Sha1::new();
                hash.update(header.as_bytes());
                hash.update(content);
                hex::encode(hash.finalize())
            }
            Self::Sha256 => {
                let mut hash = sha2::Sha256::new();
                hash.update(header.as_bytes());
                hash.update(content);
                hex::encode(hash.finalize())
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Packet {
    pub schema: String,
    pub repository: String,
    pub revision: String,
    pub object_format: GitObjectFormat,
    pub scope: Scope,
    pub scope_digest: String,
    pub objects: Vec<Object>,
    pub excluded_surfaces: String,
    pub checkout_policy: String,
    pub review_state: String,
    pub completeness: String,
    pub packet_id: String,
}

pub fn digest(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}
fn object_id(value: &str) -> bool {
    matches!(value.len(), 40 | 64)
        && value
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}
pub fn validate_path(path: &str) -> Result<()> {
    ensure!(
        !path.is_empty()
            && path.len() <= 512
            && !path.starts_with('/')
            && !path.contains(['\\', ':']),
        "invalid_scope_path"
    );
    ensure!(
        path.bytes().all(|b| b.is_ascii_graphic() || b == b' '),
        "invalid_scope_path"
    );
    ensure!(
        path.split('/')
            .all(|p| !p.is_empty() && p != "." && p != ".." && !p.eq_ignore_ascii_case(".git")),
        "invalid_scope_path"
    );
    ensure!(!unsafe_content("", path), "unsafe_scope_path");
    Ok(())
}
pub fn validate_repository(value: &str) -> Result<()> {
    let rest = value
        .strip_prefix("https://")
        .ok_or_else(|| anyhow::anyhow!("invalid_repository_identity"))?;
    let (host, path) = rest
        .split_once('/')
        .ok_or_else(|| anyhow::anyhow!("invalid_repository_identity"))?;
    ensure!(
        !host.is_empty()
            && host
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'.' || b == b'-'),
        "invalid_repository_identity"
    );
    ensure!(
        !path.is_empty()
            && path.split('/').all(|p| !p.is_empty()
                && p != "."
                && p != ".."
                && p.bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))),
        "invalid_repository_identity"
    );
    ensure!(!unsafe_content("", value), "unsafe_repository_identity");
    ensure!(
        !value.ends_with(".git"),
        "repository_identity_must_omit_git_suffix"
    );
    Ok(())
}
fn credential_key(key: &str) -> bool {
    let key: String = key
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
        .map(|c| c.to_ascii_lowercase())
        .collect();
    [
        "password",
        "passwd",
        "api_key",
        "apikey",
        "access_token",
        "auth_token",
        "client_secret",
        "private_key",
        "secret",
        "token",
    ]
    .iter()
    .any(|suffix| key.ends_with(suffix))
}
// Inspect decoded keys before a map can collapse duplicate members. Values are
// consumed without materializing a tree; serde_json retains its recursion limit.
struct JsonCredentialScan(bool);
impl<'de> Deserialize<'de> for JsonCredentialScan {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error> {
        struct Scan;
        impl<'de> serde::de::Visitor<'de> for Scan {
            type Value = JsonCredentialScan;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a JSON value")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                mut map: A,
            ) -> std::result::Result<Self::Value, A::Error> {
                let mut found = false;
                while let Some(key) = map.next_key::<String>()? {
                    found |= credential_key(&key);
                    found |= map.next_value::<JsonCredentialScan>()?.0;
                }
                Ok(JsonCredentialScan(found))
            }
            fn visit_seq<A: serde::de::SeqAccess<'de>>(
                self,
                mut seq: A,
            ) -> std::result::Result<Self::Value, A::Error> {
                let mut found = false;
                while let Some(value) = seq.next_element::<JsonCredentialScan>()? {
                    found |= value.0;
                }
                Ok(JsonCredentialScan(found))
            }
            fn visit_unit<E: serde::de::Error>(self) -> std::result::Result<Self::Value, E> {
                Ok(JsonCredentialScan(false))
            }
            fn visit_bool<E: serde::de::Error>(
                self,
                _: bool,
            ) -> std::result::Result<Self::Value, E> {
                Ok(JsonCredentialScan(false))
            }
            fn visit_i64<E: serde::de::Error>(self, _: i64) -> std::result::Result<Self::Value, E> {
                Ok(JsonCredentialScan(false))
            }
            fn visit_u64<E: serde::de::Error>(self, _: u64) -> std::result::Result<Self::Value, E> {
                Ok(JsonCredentialScan(false))
            }
            fn visit_f64<E: serde::de::Error>(self, _: f64) -> std::result::Result<Self::Value, E> {
                Ok(JsonCredentialScan(false))
            }
            fn visit_str<E: serde::de::Error>(
                self,
                _: &str,
            ) -> std::result::Result<Self::Value, E> {
                Ok(JsonCredentialScan(false))
            }
        }
        deserializer.deserialize_any(Scan)
    }
}

fn toml_credential(value: &toml::Value) -> bool {
    match value {
        toml::Value::Table(table) => table
            .iter()
            .any(|(key, value)| credential_key(key) || toml_credential(value)),
        toml::Value::Array(values) => values.iter().any(toml_credential),
        _ => false,
    }
}

/// Conservative omission policy, deliberately not a universal secret detector.
/// Known credential files, credential assignments/markers and local host paths are
/// never emitted. Unknown credentials require operator scope review.
pub fn unsafe_content(path: &str, content: &str) -> bool {
    let name = path.to_ascii_lowercase();
    let text = content.to_ascii_lowercase();
    let sensitive_name = name.split('/').any(|p| {
        p == ".env"
            || p.starts_with(".env.")
            || p == ".git-credentials"
            || p == ".pypirc"
            || p == ".ssh"
            || p == ".aws"
            || p == ".docker"
            || p == ".netrc"
            || p == ".npmrc"
            || p == "credentials"
            || p == "id_rsa"
            || p == "id_ed25519"
            || p.ends_with(".pem")
            || p.ends_with(".key")
    });
    // Parse JSON keys as data so later/nested keys and escaped spellings are
    // checked. Non-JSON formats still use a linear scan of every assignment,
    // rather than only the first delimiter on each line.
    let candidate = content.trim_start_matches('\u{feff}').trim_start();
    let json_array = candidate.strip_prefix('[').is_some_and(|tail| {
        let tail = tail.trim_start();
        tail.is_empty()
            || tail.starts_with(['[', '{', ']', '"', '-'])
            || tail.starts_with(|c: char| c.is_ascii_digit())
            || ["true", "false", "null"]
                .iter()
                .any(|value| tail.starts_with(value))
    });
    let json_credential = match serde_json::from_str::<JsonCredentialScan>(content) {
        Ok(scan) => scan.0,
        // A parse/depth failure must not downgrade JSON to a raw scan that
        // cannot decode escaped keys. Malformed JSON is omitted, not retained.
        Err(_) => match toml::from_str::<toml::Value>(content) {
            // A complete TOML parse disambiguates array-table and literal-like
            // section headers. Decoded keys remain subject to the same policy.
            Ok(value) if !name.ends_with(".json") => toml_credential(&value),
            _ => name.ends_with(".json") || candidate.starts_with('{') || json_array,
        },
    };
    let credential_assignment = text
        .split_inclusive(['=', ':'])
        .any(|segment| segment.strip_suffix(['=', ':']).is_some_and(credential_key));
    let url_userinfo = text.split("://").skip(1).any(|tail| {
        tail.split(['/', ' ', '\t', '\r', '\n', '"', '\''])
            .next()
            .is_some_and(|authority| authority.contains('@'))
    });
    sensitive_name
        || json_credential
        || credential_assignment
        || url_userinfo
        || [
            "-----begin private key",
            "-----begin rsa private key",
            "-----begin openssh private key",
            "ghp_",
            "github_pat_",
            "sk-proj-",
            "sk-ant-",
            "xoxb-",
            "xoxp-",
            "akia",
            "/users/",
            "/home/",
            "/volumes/",
            "/private/",
            "/tmp/",
            "c:\\",
            "authorization: bearer",
            "password=",
            "password =",
            "\"api_key\"",
            "\"password\"",
            "\"access_token\"",
            "api_key=",
            "api_key =",
            "token=",
            "token =",
            "secret_access_key",
        ]
        .iter()
        .any(|needle| text.contains(needle))
}
impl Packet {
    pub(crate) fn seal(&mut self) -> Result<()> {
        self.scope_digest = digest(&serde_json::to_vec(&self.scope)?);
        self.packet_id.clear();
        self.packet_id = digest(&serde_json::to_vec(self)?);
        self.validate()
    }
    pub fn validate(&self) -> Result<()> {
        ensure!(self.schema == SCHEMA, "unsupported_packet_schema");
        validate_repository(&self.repository)?;
        self.scope.validate()?;
        ensure!(
            self.object_format.accepts(&self.revision),
            "invalid_revision_identity"
        );
        ensure!(
            self.scope_digest == digest(&serde_json::to_vec(&self.scope)?),
            "scope_digest_mismatch"
        );
        ensure!(
            self.excluded_surfaces == "all_paths_outside_declared_scope"
                && self.checkout_policy == "committed_blobs_only_dirty_and_untracked_excluded"
                && self.review_state == "not_reviewed",
            "invalid_packet_claim"
        );
        let paths = self.scope.paths();
        ensure!(
            paths.len() == self.objects.len(),
            "scope_membership_mismatch"
        );
        let mut total = 0u64;
        let mut partial = false;
        for (path, object) in paths.iter().zip(&self.objects) {
            ensure!(*path == object.path, "scope_membership_mismatch");
            total = total
                .checked_add(object.source_bytes)
                .ok_or_else(|| anyhow::anyhow!("byte_limit_exceeded"))?;
            ensure!(
                object.source_bytes <= self.scope.max_file_bytes && total <= self.scope.max_bytes,
                "byte_limit_exceeded"
            );
            ensure!(
                object
                    .source_object
                    .as_ref()
                    .is_none_or(|id| self.object_format.accepts(id)),
                "invalid_source_object"
            );
            let support =
                if self.scope.analysis.contains(&object.path) && object.path.ends_with(".rs") {
                    "rust_source_not_yet_analyzed"
                } else {
                    "context_or_unsupported_analysis"
                };
            ensure!(object.analysis_support == support, "invalid_analysis_claim");
            match object.disposition.as_str() {
                "included" => {
                    let content = object
                        .content
                        .as_ref()
                        .ok_or_else(|| anyhow::anyhow!("missing_object_content"))?;
                    ensure!(
                        object.source_object.is_some()
                            && content.len() as u64 == object.source_bytes
                            && !content.contains('\0')
                            && !unsafe_content(path, content),
                        "unsafe_object_content"
                    );
                    ensure!(
                        object.source_object.as_ref()
                            == Some(&self.object_format.blob_digest(content.as_bytes())),
                        "source_blob_digest_mismatch"
                    );
                    ensure!(
                        object.content_digest.as_ref() == Some(&digest(content.as_bytes())),
                        "object_digest_mismatch"
                    );
                }
                "missing" | "omitted_unsafe" | "omitted_binary" | "omitted_unsupported_object" => {
                    partial = true;
                    ensure!(
                        object.content.is_none() && object.content_digest.is_none(),
                        "omitted_content_retained"
                    );
                    if object.disposition == "missing" {
                        ensure!(
                            object.source_object.is_none() && object.source_bytes == 0,
                            "invalid_missing_object"
                        );
                    } else {
                        ensure!(object.source_object.is_some(), "missing_source_object");
                    }
                }
                _ => bail!("invalid_object_disposition"),
            }
        }
        ensure!(
            self.completeness
                == if partial {
                    "partial"
                } else {
                    "complete_scoped_acquisition"
                },
            "invalid_completeness_claim"
        );
        let mut unsealed = self.clone();
        unsealed.packet_id.clear();
        ensure!(
            self.packet_id == digest(&serde_json::to_vec(&unsealed)?),
            "packet_digest_mismatch"
        );
        Ok(())
    }
}
/// The sole production read boundary for later evidence admission consumers.
/// Validating this input grants no retention, review or publication authority.
pub struct AdmissionInput {
    packet: Packet,
}
impl AdmissionInput {
    pub fn read(path: &Path) -> Result<Self> {
        let file = File::open(path).map_err(|_| anyhow::anyhow!("packet_open_failed"))?;
        let mut bytes = Vec::new();
        file.take(MAX_PACKET_BYTES + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| anyhow::anyhow!("packet_read_failed"))?;
        ensure!(
            bytes.len() as u64 <= MAX_PACKET_BYTES,
            "packet_byte_limit_exceeded"
        );
        let packet: Packet =
            serde_json::from_slice(&bytes).map_err(|_| anyhow::anyhow!("packet_parse_failed"))?;
        packet.validate()?;
        Ok(Self { packet })
    }
    pub fn packet(&self) -> &Packet {
        &self.packet
    }
}
