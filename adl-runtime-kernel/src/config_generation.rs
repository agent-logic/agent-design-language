//! Immutable Runtime configuration-generation receipts and active-reference validation.

use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const CONFIG_GENERATION_RECEIPT_SCHEMA: &str = "adl.runtime_v3.config_generation.v1";
pub const CONFIG_GENERATION_ENV: &str = "ADL_RUNTIME_V3_CONFIG_GENERATION";
pub const CONFIG_RECEIPT_DIGEST_ENV: &str = "ADL_RUNTIME_V3_CONFIG_RECEIPT_DIGEST";
pub const REDACTED_SECRET_REFERENCE: &str = "[redacted-secret-reference]";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigGenerationReceipt {
    pub schema: String,
    pub generation: String,
    pub content_sha256: String,
    pub config_schema: String,
    pub compatible_binary_generation: String,
    pub secret_references: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigGenerationIdentity {
    pub generation: String,
    pub receipt_digest: String,
}

pub fn generation_store(init: &Path) -> Result<PathBuf, String> {
    let parent = init
        .parent()
        .ok_or_else(|| "Runtime init has no parent directory".to_owned())?;
    Ok(parent.join(".runtime-config-generations"))
}

pub fn active_generation_ref(init: &Path) -> Result<PathBuf, String> {
    let name = init
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| "Runtime init filename is invalid".to_owned())?;
    Ok(init
        .parent()
        .ok_or_else(|| "Runtime init has no parent directory".to_owned())?
        .join(format!(".{name}.active-generation")))
}

pub fn build_config_generation_receipt(
    init: &Path,
    compatible_binary_generation: &str,
) -> Result<(ConfigGenerationReceipt, ConfigGenerationIdentity), String> {
    validate_identifier("compatible binary generation", compatible_binary_generation)?;
    let bytes = fs::read(init).map_err(|error| format!("read Runtime init: {error}"))?;
    let content_sha256 = format!("{:x}", Sha256::digest(&bytes));
    let generation = config_generation_digest(&content_sha256, compatible_binary_generation);
    let text = std::str::from_utf8(&bytes)
        .map_err(|error| format!("Runtime init is not UTF-8: {error}"))?;
    let document: toml::Value = toml::from_str(text)
        .map_err(|error| format!("parse Runtime init for generation receipt: {error}"))?;
    let config_schema = document
        .get("schema")
        .and_then(toml::Value::as_str)
        .ok_or_else(|| "Runtime init schema is missing".to_owned())?
        .to_owned();
    let mut secret_references = BTreeMap::new();
    collect_secret_references("", &document, &mut secret_references)?;
    let receipt = ConfigGenerationReceipt {
        schema: CONFIG_GENERATION_RECEIPT_SCHEMA.to_owned(),
        generation: generation.clone(),
        content_sha256,
        config_schema,
        compatible_binary_generation: compatible_binary_generation.to_owned(),
        secret_references,
    };
    let receipt_digest = receipt_digest(&receipt)?;
    Ok((
        receipt,
        ConfigGenerationIdentity {
            generation,
            receipt_digest,
        },
    ))
}

pub fn provision_config_generation(
    init: &Path,
    compatible_binary_generation: &str,
) -> Result<ConfigGenerationIdentity, String> {
    provision_config_generation_in_store(init, init, compatible_binary_generation)
}

pub fn provision_config_generation_in_store(
    init: &Path,
    store_owner_init: &Path,
    compatible_binary_generation: &str,
) -> Result<ConfigGenerationIdentity, String> {
    let (receipt, identity) = build_config_generation_receipt(init, compatible_binary_generation)?;
    let store = generation_store(store_owner_init)?;
    fs::create_dir_all(&store)
        .map_err(|error| format!("create Runtime configuration generation store: {error}"))?;
    let receipt_path = store.join(format!("{}.json", identity.generation));
    let bytes = serde_json::to_vec_pretty(&receipt)
        .map_err(|error| format!("encode Runtime configuration receipt: {error}"))?;
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&receipt_path)
    {
        Ok(mut file) => {
            file.write_all(&bytes)
                .and_then(|_| file.sync_all())
                .map_err(|error| format!("write Runtime configuration receipt: {error}"))?;
            sync_directory(&store)?;
        }
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            let retained = fs::read(&receipt_path)
                .map_err(|error| format!("read retained Runtime configuration receipt: {error}"))?;
            if retained != bytes {
                return Err(
                    "immutable Runtime configuration receipt conflicts with retained bytes"
                        .to_owned(),
                );
            }
        }
        Err(error) => return Err(format!("create Runtime configuration receipt: {error}")),
    }
    Ok(identity)
}

pub fn activate_config_generation(
    init: &Path,
    identity: &ConfigGenerationIdentity,
) -> Result<(), String> {
    validate_identifier("configuration generation", &identity.generation)?;
    validate_digest("configuration receipt digest", &identity.receipt_digest)?;
    let active = active_generation_ref(init)?;
    let staged = active.with_extension("active-generation.candidate");
    let payload = format!("{} {}\n", identity.generation, identity.receipt_digest);
    fs::write(&staged, payload)
        .and_then(|_| fs::File::open(&staged)?.sync_all())
        .map_err(|error| format!("stage Runtime configuration active reference: {error}"))?;
    fs::rename(&staged, &active)
        .map_err(|error| format!("activate Runtime configuration generation: {error}"))?;
    sync_directory(
        active
            .parent()
            .ok_or_else(|| "active configuration reference has no parent".to_owned())?,
    )
}

pub fn validate_active_config_generation(
    init: &Path,
    compatible_binary_generation: &str,
) -> Result<ConfigGenerationIdentity, String> {
    validate_identifier("compatible binary generation", compatible_binary_generation)?;
    let content_sha256 = format!(
        "{:x}",
        Sha256::digest(fs::read(init).map_err(|error| format!("read Runtime init: {error}"))?)
    );
    let expected_generation =
        config_generation_digest(&content_sha256, compatible_binary_generation);
    let (generation, digest, receipt) = read_active_config_generation_receipt(init)?;
    if generation != expected_generation {
        return Err(
            "Runtime configuration active reference does not match init content".to_owned(),
        );
    }
    let canonical_digest = receipt_digest(&receipt)?;
    let legacy_digest = legacy_receipt_digest(&receipt)?;
    if receipt.schema != CONFIG_GENERATION_RECEIPT_SCHEMA
        || receipt.generation != generation
        || receipt.content_sha256 != content_sha256
        || receipt.compatible_binary_generation != compatible_binary_generation
        || (canonical_digest != digest && legacy_digest != digest)
    {
        return Err(
            "Runtime configuration receipt identity or compatibility is invalid".to_owned(),
        );
    }
    Ok(ConfigGenerationIdentity {
        generation,
        receipt_digest: digest,
    })
}

pub fn validate_active_config_generation_content(
    init: &Path,
) -> Result<ConfigGenerationReceipt, String> {
    let content_sha256 = format!(
        "{:x}",
        Sha256::digest(fs::read(init).map_err(|error| format!("read Runtime init: {error}"))?)
    );
    let (generation, digest, receipt) = read_active_config_generation_receipt(init)?;
    let canonical_digest = receipt_digest(&receipt)?;
    let legacy_digest = legacy_receipt_digest(&receipt)?;
    if receipt.schema != CONFIG_GENERATION_RECEIPT_SCHEMA
        || receipt.generation != generation
        || receipt.content_sha256 != content_sha256
        || (canonical_digest != digest && legacy_digest != digest)
    {
        return Err("Runtime configuration active receipt does not match init content".to_owned());
    }
    Ok(receipt)
}

pub fn config_generation_identity_from_env(
    mut get_env: impl FnMut(&str) -> Option<String>,
) -> Result<ConfigGenerationIdentity, String> {
    match (
        get_env(CONFIG_GENERATION_ENV),
        get_env(CONFIG_RECEIPT_DIGEST_ENV),
    ) {
        (Some(generation), Some(receipt_digest)) => {
            validate_digest("Runtime configuration generation", &generation)?;
            validate_digest("Runtime configuration receipt digest", &receipt_digest)?;
            Ok(ConfigGenerationIdentity {
                generation,
                receipt_digest,
            })
        }
        (None, None) => Err("runtime configuration generation environment is required".to_owned()),
        _ => Err("runtime configuration generation environment is incomplete".to_owned()),
    }
}

pub fn validate_config_generation_identity_matches_active(
    init: &Path,
    compatible_binary_generation: &str,
    supplied: &ConfigGenerationIdentity,
) -> Result<(), String> {
    let expected = validate_active_config_generation(init, compatible_binary_generation)?;
    if &expected != supplied {
        return Err(
            "runtime configuration generation environment does not match active receipt".to_owned(),
        );
    }
    Ok(())
}

pub fn receipt_digest(receipt: &ConfigGenerationReceipt) -> Result<String, String> {
    serde_jcs::to_vec(receipt)
        .map(|bytes| blake3::hash(&bytes).to_hex().to_string())
        .map_err(|error| format!("encode Runtime configuration receipt digest: {error}"))
}

fn legacy_receipt_digest(receipt: &ConfigGenerationReceipt) -> Result<String, String> {
    serde_json::to_vec(receipt)
        .map(|bytes| blake3::hash(&bytes).to_hex().to_string())
        .map_err(|error| format!("encode legacy Runtime configuration receipt digest: {error}"))
}

fn read_active_config_generation_receipt(
    init: &Path,
) -> Result<(String, String, ConfigGenerationReceipt), String> {
    let active = active_generation_ref(init)?;
    let text = fs::read_to_string(&active)
        .map_err(|error| format!("read Runtime configuration active reference: {error}"))?;
    let mut values = text.split_whitespace();
    let generation = values.next().unwrap_or_default();
    let digest = values.next().unwrap_or_default();
    if values.next().is_some() {
        return Err(
            "Runtime configuration active reference does not match init content".to_owned(),
        );
    }
    validate_digest("configuration generation", generation)?;
    validate_digest("configuration receipt digest", digest)?;
    let receipt_path = generation_store(init)?.join(format!("{generation}.json"));
    let receipt: ConfigGenerationReceipt = serde_json::from_slice(
        &fs::read(&receipt_path)
            .map_err(|error| format!("read active Runtime configuration receipt: {error}"))?,
    )
    .map_err(|error| format!("parse active Runtime configuration receipt: {error}"))?;
    Ok((generation.to_owned(), digest.to_owned(), receipt))
}

fn collect_secret_references(
    prefix: &str,
    value: &toml::Value,
    output: &mut BTreeMap<String, String>,
) -> Result<(), String> {
    if let Some(table) = value.as_table() {
        for (key, child) in table {
            let qualified = if prefix.is_empty() {
                key.clone()
            } else {
                format!("{prefix}.{key}")
            };
            if key.ends_with("_path")
                && (qualified.starts_with("credentials.") || qualified.starts_with("api.tls."))
            {
                child
                    .as_str()
                    .ok_or_else(|| format!("secret reference {qualified} must be a string path"))?;
                output.insert(qualified, REDACTED_SECRET_REFERENCE.to_owned());
            } else {
                collect_secret_references(&qualified, child, output)?;
            }
        }
    }
    Ok(())
}

fn validate_identifier(name: &str, value: &str) -> Result<(), String> {
    if value.is_empty()
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
    {
        return Err(format!("{name} is invalid"));
    }
    Ok(())
}

fn validate_digest(name: &str, value: &str) -> Result<(), String> {
    if value.len() != 64 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(format!("{name} is invalid"));
    }
    Ok(())
}

fn config_generation_digest(content_sha256: &str, compatible_binary_generation: &str) -> String {
    let mut digest = Sha256::new();
    digest.update(CONFIG_GENERATION_RECEIPT_SCHEMA.as_bytes());
    digest.update(b"\0content_sha256\0");
    digest.update(content_sha256.as_bytes());
    digest.update(b"\0compatible_binary_generation\0");
    digest.update(compatible_binary_generation.as_bytes());
    format!("{:x}", digest.finalize())
}

fn sync_directory(path: &Path) -> Result<(), String> {
    fs::File::open(path)
        .and_then(|directory| directory.sync_all())
        .map_err(|error| format!("sync Runtime configuration directory: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configuration_receipt_digest_uses_canonical_json() {
        let receipt = ConfigGenerationReceipt {
            schema: CONFIG_GENERATION_RECEIPT_SCHEMA.to_owned(),
            generation: "11".repeat(32),
            content_sha256: "22".repeat(32),
            config_schema: "adl.runtime_v3.init.v1".to_owned(),
            compatible_binary_generation: "generation-a".to_owned(),
            secret_references: BTreeMap::from([
                (
                    "credentials.operation_public_key_path".to_owned(),
                    REDACTED_SECRET_REFERENCE.to_owned(),
                ),
                (
                    "api.tls.private_key_path".to_owned(),
                    REDACTED_SECRET_REFERENCE.to_owned(),
                ),
            ]),
        };

        assert_eq!(
            String::from_utf8(serde_jcs::to_vec(&receipt).expect("canonical receipt"))
                .expect("receipt is UTF-8"),
            concat!(
                "{\"compatible_binary_generation\":\"generation-a\",",
                "\"config_schema\":\"adl.runtime_v3.init.v1\",",
                "\"content_sha256\":\"2222222222222222222222222222222222222222222222222222222222222222\",",
                "\"generation\":\"1111111111111111111111111111111111111111111111111111111111111111\",",
                "\"schema\":\"adl.runtime_v3.config_generation.v1\",",
                "\"secret_references\":{",
                "\"api.tls.private_key_path\":\"[redacted-secret-reference]\",",
                "\"credentials.operation_public_key_path\":\"[redacted-secret-reference]\"}}"
            )
        );
        assert_eq!(
            receipt_digest(&receipt).expect("receipt digest"),
            "3c0c73f1a3d82a01bc0f983e70b5441e0941f9a99ed746b52fc227fc747d3217"
        );
    }

    #[test]
    fn unchanged_legacy_receipt_can_advance_to_canonical_generation() {
        let directory = tempfile::tempdir().expect("temporary config root");
        let init = directory.path().join("runtime-init.toml");
        fs::write(
            &init,
            concat!(
                "schema = \"adl.runtime_v3.init.v1\"\n",
                "[credentials]\n",
                "operation_public_key_path = \"credentials/operation.hex\"\n"
            ),
        )
        .expect("write init");
        let (receipt, identity) =
            build_config_generation_receipt(&init, "legacy-generation").expect("build receipt");
        let store = generation_store(&init).expect("generation store");
        fs::create_dir_all(&store).expect("create store");
        fs::write(
            store.join(format!("{}.json", receipt.generation)),
            serde_json::to_vec_pretty(&receipt).expect("legacy receipt bytes"),
        )
        .expect("write receipt");
        fs::write(
            active_generation_ref(&init).expect("active ref"),
            format!(
                "{} {}\n",
                receipt.generation,
                legacy_receipt_digest(&receipt).expect("legacy digest")
            ),
        )
        .expect("write active ref");

        assert_ne!(
            identity.receipt_digest,
            legacy_receipt_digest(&receipt).expect("legacy digest")
        );
        assert_eq!(
            validate_active_config_generation_content(&init).expect("legacy content remains valid"),
            receipt
        );
    }
}
