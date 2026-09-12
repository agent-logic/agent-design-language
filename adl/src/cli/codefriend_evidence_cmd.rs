use adl::codefriend::{
    evidence::{store::Store, Retention},
    ingestion::{local, AdmissionInput, Scope},
};
use anyhow::{ensure, Result};
use std::{collections::BTreeMap, io::Read, path::Path};
pub(super) fn evidence(args: &[String]) -> Result<()> {
    ensure!(!args.is_empty(), "missing_evidence_command");
    let expected: &[&str] = match args[0].as_str() {
        "admit-local" => &[
            "--checkout",
            "--repository",
            "--revision",
            "--scope",
            "--store",
            "--retention-seconds",
        ],
        "admit-packet" => &["--input", "--store", "--retention-seconds"],
        "read" | "delete" => &["--store", "--packet-id"],
        _ => anyhow::bail!("unsupported_evidence_command"),
    };
    let mut flags = BTreeMap::new();
    for pair in args[1..].chunks(2) {
        ensure!(
            pair.len() == 2 && expected.contains(&pair[0].as_str()) && !pair[1].starts_with("--"),
            "invalid_evidence_arguments"
        );
        ensure!(
            flags.insert(pair[0].as_str(), pair[1].as_str()).is_none(),
            "duplicate_evidence_argument"
        );
    }
    ensure!(flags.len() == expected.len(), "missing_evidence_argument");
    // Acquire/validate before opening the store, so rejected packets leave no durable input bytes.
    let packet = match args[0].as_str() {
        "admit-local" => {
            let mut bytes = Vec::new();
            std::fs::File::open(flags["--scope"])?
                .take(128 * 1024 + 1)
                .read_to_end(&mut bytes)?;
            ensure!(bytes.len() <= 128 * 1024, "scope_byte_limit_exceeded");
            let scope: Scope = serde_json::from_slice(&bytes)
                .map_err(|_| anyhow::anyhow!("invalid_scope_json"))?;
            Some(local::acquire(
                Path::new(flags["--checkout"]),
                flags["--repository"],
                flags["--revision"],
                scope,
            )?)
        }
        "admit-packet" => Some(
            AdmissionInput::read(Path::new(flags["--input"]))?
                .packet()
                .clone(),
        ),
        _ => None,
    };
    let store_path = Path::new(flags["--store"]);
    if args[0] == "admit-local" {
        let root = Path::new(flags["--checkout"]).canonicalize()?;
        let absolute = std::path::absolute(store_path)?;
        ensure!(!absolute.starts_with(&root), "store_inside_source_rejected");
    }
    let store = Store::open(store_path, || {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    })?;
    let result = if let Some(packet) = packet {
        let seconds = flags["--retention-seconds"]
            .parse()
            .map_err(|_| anyhow::anyhow!("invalid_retention_seconds"))?;
        let a = store.admit(packet, Retention { seconds })?;
        serde_json::json!({"schema":"codefriend.admission_result.v1","packet_id":a.packet.packet_id,"admission_digest":a.digest,"evidence":a.evidence,"expires_at":a.expires_at,"completeness":a.packet.completeness,"review_state":a.packet.review_state})
    } else if args[0] == "read" {
        serde_json::to_value(store.get(flags["--packet-id"])?)?
    } else {
        store.delete(flags["--packet-id"])?;
        serde_json::json!({"schema":"codefriend.deletion_result.v1","packet_id":flags["--packet-id"],"deleted":true})
    };
    println!("{}", serde_json::to_string(&result)?);
    Ok(())
}
