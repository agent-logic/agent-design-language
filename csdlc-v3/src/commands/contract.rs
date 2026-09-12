//! The installed command descriptor is embedded from the tracked current contract.
//! Historical denominator files remain immutable; aliases are counted separately.
use serde_json::Value;
use std::sync::OnceLock;

pub const RESULT_SCHEMA: &str = "csdlc.v3.command_result.v1";
pub const AUTHORITY_HELP: &str = "C-SDLC v3 is operational after #505 / PR #591; authenticated canonical selector and reconciliation receipt validation are required. Missing or stale proof suspends authority.";

pub fn manifest() -> &'static Value {
    static MANIFEST: OnceLock<Value> = OnceLock::new();
    MANIFEST.get_or_init(|| {
        serde_json::from_str(include_str!(
            "../../../docs/csdlc-v3/v3-command-manifest.json"
        ))
        .expect("compiled command contract must be valid JSON")
    })
}

pub fn descriptors() -> impl Iterator<Item = &'static Value> {
    manifest()["commands"]
        .as_array()
        .unwrap()
        .iter()
        .chain(manifest()["aliases"].as_array().unwrap())
}

pub fn descriptor(name: &str) -> Option<&'static Value> {
    descriptors().find(|row| row["command"] == name)
}

pub fn root_help() -> String {
    let mut help = String::from("usage: csdlc <command>\n\nOperational commands:\n");
    for group in ["operational", "historical_administrative"] {
        if group == "historical_administrative" {
            help.push_str("\nExplicit historical / administrative commands (no alternate operational authority):\n");
        }
        for row in descriptors().filter(|row| row["discovery_group"] == group) {
            help.push_str(&format!("  {}\n", row["usage"].as_str().unwrap()));
        }
    }
    help.push_str("\nDiscovery: --contract prints the complete descriptor; <command> --describe prints its input/effect/result contract.\n");
    help.push_str(&format!("\nauthority: {AUTHORITY_HELP}"));
    help
}

/// Fail closed if a separately observed installation disagrees with its source contract.
/// This is a public verifier for installation proof; it never installs or executes routes.
pub fn verify_discovery(observed: &Value, help: &str) -> Result<(), String> {
    if observed != manifest() {
        return Err("installed_contract_mismatch".into());
    }
    if help.trim_end() != root_help().trim_end() {
        return Err("installed_help_mismatch".into());
    }
    Ok(())
}

/// Verify recorded provenance and actual installed bytes against the independently
/// selected source build. Provenance alone is not sufficient: a stale executable
/// with a copied metadata file fails the digest comparison.
pub fn verify_installation(
    provenance: &Value,
    installed_bytes: &[u8],
    expected_source: &str,
    expected_build: &str,
    expected_binary_digest: &str,
) -> Result<(), String> {
    if expected_source.len() != 40
        || !expected_source.bytes().all(|byte| byte.is_ascii_hexdigit())
        || expected_build.trim().is_empty()
        || expected_binary_digest.len() != 64
    {
        return Err("expected_installation_identity_invalid".into());
    }
    if provenance["schema"] != "csdlc.v3.installation_provenance.v1"
        || provenance["source_revision"] != expected_source
        || provenance["build_identity"] != expected_build
    {
        return Err("installed_provenance_mismatch".into());
    }
    let digest = blake3::hash(installed_bytes).to_hex().to_string();
    if provenance["installed_digest"] != digest || digest != expected_binary_digest {
        return Err("installed_binary_digest_mismatch".into());
    }
    Ok(())
}

/// Required argv inputs are enforced before owner dispatch. Typed deserializers
/// and owner guards remain responsible for values, unknown fields and admission.
pub fn validate_required_inputs(command: &str, args: &[String]) -> Result<(), String> {
    if args == ["--help"] || args == ["-h"] || args == ["--describe"] {
        return Ok(());
    }
    let Some(row) = descriptor(command) else {
        return Ok(());
    };
    let variants = row["input_variants"]
        .as_array()
        .expect("descriptor input variants");
    let variant = variants
        .iter()
        .find(|variant| {
            let prefix = variant["positional_prefix"].as_array().unwrap();
            !prefix.is_empty() && prefix.iter().zip(args).all(|(prefix, arg)| prefix == arg)
        })
        .or_else(|| {
            variants.iter().find(|variant| {
                variant["name"] == "execute" && args.iter().any(|arg| arg == "--execute")
            })
        })
        .unwrap_or(&variants[0]);
    let prefix = variant["positional_prefix"].as_array().unwrap();
    if args.len() == prefix.len() + 1
        && args
            .last()
            .is_some_and(|arg| arg == "--help" || arg == "-h")
    {
        return Ok(());
    }
    for required in variant["required_flags"].as_array().unwrap() {
        let flag = required.as_str().unwrap();
        if !args.iter().any(|arg| arg == flag) {
            return Err(serde_json::json!({
                "schema":"csdlc.v3.command_input_failure.v1", "command":command,
                "status":"failed", "read_only":true, "performed_mutation":false,
                "findings":[{"code":"required_input_missing", "message":format!("missing {flag}; usage: csdlc {}", row["usage"].as_str().unwrap())}]
            }).to_string());
        }
    }
    Ok(())
}
