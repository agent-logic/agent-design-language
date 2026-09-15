use adl::codefriend::{
    architecture::{
        drift,
        impact::{self, ChangeSet},
        structure::{self, BoundaryPolicy},
    },
    evidence::store::Store,
    memory::baseline::AdmittedBaselines,
};
use anyhow::{ensure, Result};
use std::{collections::BTreeMap, io::Read, path::Path};
pub(super) const USAGE:&str="adl codefriend architecture report --store <directory> --packet-id <id> --policy <policy.json> --out <new-report.json>\nadl codefriend architecture drift --store <directory> --baselines <directory> --baseline <graph.json> --current <graph.json> --out <new-drift.json>\nadl codefriend architecture drift-read --store <directory> --baselines <directory> --input <drift.json>\nadl codefriend architecture impact --store <directory> --graph <structure.json> --changes <changes.json> --out <new-impact.json>\nadl codefriend architecture impact-read --store <directory> --input <impact.json>\nadl codefriend architecture read --store <directory> --input <report.json>";
pub(super) fn run(args: &[String]) -> Result<()> {
    ensure!(!args.is_empty(), "{USAGE}");
    let expected: &[&str] = match args[0].as_str() {
        "drift" => &["--store", "--baselines", "--baseline", "--current", "--out"],
        "drift-read" => &["--store", "--baselines", "--input"],
        "report" => &["--store", "--packet-id", "--policy", "--out"],
        "impact" => &["--store", "--graph", "--changes", "--out"],
        "read" | "impact-read" => &["--store", "--input"],
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
    if args[0] == "impact" || args[0] == "impact-read" {
        let report = if args[0] == "impact" {
            let path = Path::new(flags["--changes"]);
            ensure!(
                std::fs::symlink_metadata(path)?.file_type().is_file(),
                "change_input_not_regular"
            );
            let mut bytes = Vec::new();
            std::fs::File::open(path)?
                .take(128 * 1024 + 1)
                .read_to_end(&mut bytes)?;
            ensure!(bytes.len() <= 128 * 1024, "change_input_too_large");
            let changes: ChangeSet = serde_json::from_slice(&bytes)
                .map_err(|_| anyhow::anyhow!("invalid_change_input"))?;
            let graph = structure::read_report(&store, Path::new(flags["--graph"]))?;
            let report = impact::change_impact_reporter(&store, graph, changes)?;
            impact::write_report(&report, &store, Path::new(flags["--out"]))?;
            report
        } else {
            impact::read_report(&store, Path::new(flags["--input"]))?
        };
        println!(
            "{}",
            serde_json::json!({"schema":impact::VERSION,"digest":report.digest,"run_id":report.record.run.id,"analysis_complete":report.analysis_complete,"impacts":report.impacts.len(),"unknowns":report.unknowns.len()})
        );
        eprintln!("adl_event kind=codefriend_impact status=success analysis_complete={} impacts={} unknowns={}",report.analysis_complete,report.impacts.len(),report.unknowns.len());
        return Ok(());
    }
    if args[0] == "drift" || args[0] == "drift-read" {
        let baselines_root = Path::new(flags["--baselines"]);
        let report = if args[0] == "drift" {
            let baseline = structure::read_report(&store, Path::new(flags["--baseline"]))?;
            let current = structure::read_report(&store, Path::new(flags["--current"]))?;
            let output = std::path::absolute(flags["--out"])?;
            ensure!(
                !output.starts_with(std::path::absolute(root)?)
                    && !output.starts_with(std::path::absolute(baselines_root)?),
                "drift_output_inside_managed_store"
            );
            let baselines = AdmittedBaselines::open(&store, baselines_root, true)?;
            baselines.retain(&baseline.record)?;
            baselines.retain(&current.record)?;
            let report = drift::architecture_drift_reporter(&store, &baselines, baseline, current)?;
            drift::write_report(&report, &store, &baselines, Path::new(flags["--out"]))?;
            report
        } else {
            let baselines = AdmittedBaselines::open(&store, baselines_root, false)?;
            drift::read_report(&store, &baselines, Path::new(flags["--input"]))?
        };
        println!(
            "{}",
            serde_json::json!({"schema":drift::VERSION,"digest":report.digest,"comparable":report.structural_comparison.comparable,"reasons":report.structural_comparison.reasons,"changes":report.structural_comparison.changes.len()})
        );
        eprintln!(
            "adl_event kind=codefriend_drift status=success comparable={} changes={}",
            report.structural_comparison.comparable,
            report.structural_comparison.changes.len()
        );
        return Ok(());
    }
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
