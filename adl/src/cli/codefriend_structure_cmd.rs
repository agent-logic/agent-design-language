use adl::codefriend::{
    architecture::structure::{self, BoundaryPolicy},
    evidence::store::Store,
};
use anyhow::{ensure, Result};
use std::{collections::BTreeMap, io::Read, path::Path};
pub(super) const USAGE:&str="adl codefriend architecture report --store <directory> --packet-id <id> --policy <policy.json> --out <new-report.json>\nadl codefriend architecture read --store <directory> --input <report.json>";
pub(super) fn run(args: &[String]) -> Result<()> {
    ensure!(!args.is_empty(), "{USAGE}");
    let expected: &[&str] = match args[0].as_str() {
        "report" => &["--store", "--packet-id", "--policy", "--out"],
        "read" => &["--store", "--input"],
        _ => anyhow::bail!("unsupported_architecture_command"),
    };
    let mut flags = BTreeMap::new();
    for p in args[1..].chunks(2) {
        ensure!(
            p.len() == 2 && expected.contains(&p[0].as_str()) && !p[1].starts_with("--"),
            "invalid_architecture_arguments"
        );
        ensure!(
            flags.insert(p[0].as_str(), p[1].as_str()).is_none(),
            "duplicate_architecture_argument"
        );
    }
    ensure!(
        flags.len() == expected.len(),
        "missing_architecture_argument"
    );
    // Do not bootstrap a new store while attempting to analyze an existing admission.
    let root = Path::new(flags["--store"]);
    ensure!(
        root.join(".codefriend-store-v1").is_file(),
        "evidence_store_missing"
    );
    let store = Store::open(root, || {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    })?;
    let report = if args[0] == "report" {
        let mut bytes = Vec::new();
        std::fs::File::open(flags["--policy"])?
            .take(128 * 1024 + 1)
            .read_to_end(&mut bytes)?;
        ensure!(bytes.len() <= 128 * 1024, "policy_too_large");
        let policy: BoundaryPolicy = serde_json::from_slice(&bytes)
            .map_err(|_| anyhow::anyhow!("invalid_boundary_policy"))?;
        let r = structure::repository_structure_reporter(&store, flags["--packet-id"], policy)?;
        structure::write_report(&r, &store, Path::new(flags["--out"]))?;
        r
    } else {
        structure::read_report(&store, Path::new(flags["--input"]))?
    };
    println!(
        "{}",
        serde_json::to_string(
            &serde_json::json!({"schema":structure::VERSION,"digest":report.digest,"run_id":report.record.run.id,"analysis_complete":report.analysis_complete,"nodes":report.nodes.len(),"edges":report.edges.len(),"findings":report.record.findings.len(),"unknowns":report.unknowns.len()})
        )?
    );
    eprintln!("adl_event kind=codefriend_architecture status=success analysis_complete={} nodes={} edges={} unknowns={}",report.analysis_complete,report.nodes.len(),report.edges.len(),report.unknowns.len());
    Ok(())
}
