use adl::codefriend::ingestion::{ci, Scope};
use anyhow::{ensure, Result};
use std::{collections::BTreeMap, io::Read, path::Path};

fn json_file<T: serde::de::DeserializeOwned>(path: &str, limit: u64) -> Result<T> {
    let mut bytes = Vec::new();
    std::fs::File::open(path)
        .map_err(|_| anyhow::anyhow!("ci_input_open_failed"))?
        .take(limit + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| anyhow::anyhow!("ci_input_read_failed"))?;
    ensure!(bytes.len() as u64 <= limit, "ci_input_byte_limit_exceeded");
    serde_json::from_slice(&bytes).map_err(|_| anyhow::anyhow!("invalid_ci_input_json"))
}

pub(super) fn run(args: &[String]) -> Result<()> {
    let expected = [
        "--checkout",
        "--repository",
        "--revision",
        "--scope",
        "--out",
        "--receipt",
        "--candidate-revision",
        "--metadata",
    ];
    let mut flags = BTreeMap::new();
    for pair in args.chunks(2) {
        ensure!(
            pair.len() == 2 && expected.contains(&pair[0].as_str()) && !pair[1].starts_with("--"),
            "invalid_ci_arguments"
        );
        ensure!(
            flags.insert(pair[0].as_str(), pair[1].as_str()).is_none(),
            "duplicate_ci_argument"
        );
    }
    ensure!(flags.len() == expected.len(), "missing_ci_argument");
    let scope: Scope = json_file(flags["--scope"], 128 * 1024)?;
    let metadata = json_file(flags["--metadata"], 4096)?;
    let checkout = Path::new(flags["--checkout"]);
    let (packet, receipt) = ci::acquire(
        checkout,
        flags["--repository"],
        flags["--revision"],
        scope,
        flags["--candidate-revision"],
        metadata,
    )?;
    ci::write(
        checkout,
        Path::new(flags["--out"]),
        Path::new(flags["--receipt"]),
        &packet,
        &receipt,
    )?;
    println!(
        "{}",
        serde_json::to_string(
            &serde_json::json!({"schema":"codefriend.ci_acquisition_result.v1","packet_id":packet.packet_id,"revision":packet.revision,"completeness":packet.completeness,"delivery":"not_established"})
        )?
    );
    Ok(())
}
