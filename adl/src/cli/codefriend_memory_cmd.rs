use adl::codefriend::{
    evidence::{contracts::ReviewRecord, store::Store},
    memory::{
        baseline::{self, AdmittedBaselines, BaselineAccess, BaselineRef},
        comparison::{self, DeltaReport},
        palace::{self, IndexRequest, RetrieveRequest},
        palace_authority,
    },
};
use anyhow::{ensure, Result};
use std::{collections::BTreeMap, path::Path};
pub(super) const USAGE:&str="adl codefriend memory retain|read|delete|compare|delta-read|palace-index|palace-compare --store <admission-store> --baselines <baseline-directory> ...";
pub(super) fn run(args: &[String]) -> Result<()> {
    ensure!(!args.is_empty(), "missing_memory_command");
    let expected: &[&str] = match args[0].as_str() {
        "palace-index" => &[
            "--store",
            "--baselines",
            "--palace",
            "--trust",
            "--authority",
            "--input",
        ],
        "palace-compare" => &["--store", "--baselines", "--palace", "--input", "--out"],
        "retain" => &["--store", "--baselines", "--input"],
        "read" | "delete" => &["--store", "--baselines", "--reference"],
        "compare" => &["--store", "--baselines", "--baseline", "--current", "--out"],
        "delta-read" => &["--store", "--baselines", "--input"],
        _ => anyhow::bail!("unsupported_memory_command"),
    };
    let mut flags = BTreeMap::new();
    for pair in args[1..].chunks(2) {
        ensure!(
            pair.len() == 2 && expected.contains(&pair[0].as_str()) && !pair[1].starts_with("--"),
            "invalid_memory_arguments"
        );
        ensure!(
            flags.insert(pair[0].as_str(), pair[1].as_str()).is_none(),
            "duplicate_memory_argument"
        );
    }
    ensure!(flags.len() == expected.len(), "missing_memory_argument");
    let store_path = Path::new(flags["--store"]);
    let baseline_path = std::path::absolute(Path::new(flags["--baselines"]))?;
    let absolute_store = std::path::absolute(store_path)?;
    ensure!(
        !baseline_path.starts_with(&absolute_store) && !absolute_store.starts_with(&baseline_path),
        "baseline_and_admission_roots_must_be_separate"
    );
    if let Some(palace) = flags.get("--palace") {
        let palace = std::path::absolute(Path::new(palace))?;
        baseline::safe_path(&palace)?;
        ensure!(
            !palace.starts_with(&baseline_path)
                && !baseline_path.starts_with(&palace)
                && !palace.starts_with(&absolute_store)
                && !absolute_store.starts_with(&palace),
            "palace_managed_roots_overlap"
        );
        if let Some(out) = flags.get("--out") {
            ensure!(
                !std::path::absolute(Path::new(out))?.starts_with(&palace),
                "output_inside_palace_rejected"
            );
        }
    }
    if let Some(out) = flags.get("--out") {
        let out = std::path::absolute(Path::new(out))?;
        ensure!(
            !out.starts_with(&baseline_path) && !out.starts_with(&absolute_store),
            "output_inside_managed_store_rejected"
        );
    }

    baseline::safe_path(store_path)?;
    ensure!(
        store_path.join(".codefriend-store-v1").is_file(),
        "admission_store_missing"
    );
    // Validate caller input before creating a baseline directory.
    let retained: Option<ReviewRecord> = if args[0] == "retain" {
        let record: ReviewRecord = baseline::read_json(Path::new(flags["--input"]))?;
        record.validate()?;
        Some(record)
    } else {
        None
    };
    let store = Store::open(store_path, || {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    })?;
    if let Some(record) = &retained {
        ensure!(
            store.get(&record.run.packet_id)? == record.admission,
            "baseline_admission_mismatch"
        );
    }
    let backend =
        AdmittedBaselines::open(&store, Path::new(flags["--baselines"]), retained.is_some())?;
    let result = match args[0].as_str() {
        "palace-index" => {
            let request: IndexRequest = baseline::read_json(Path::new(flags["--input"]))?;
            let authority = palace_authority::provision(
                Path::new(flags["--trust"]),
                Path::new(flags["--authority"]),
            )?;
            serde_json::to_value(palace::index(
                &backend,
                Path::new(flags["--palace"]),
                &authority,
                &request,
            )?)?
        }
        "palace-compare" => {
            let request: RetrieveRequest = baseline::read_json(Path::new(flags["--input"]))?;
            let report = palace::retrieve(&backend, Path::new(flags["--palace"]), &request)?;
            baseline::write_json(Path::new(flags["--out"]), &report)?;
            serde_json::to_value(report)?
        }

        "retain" => serde_json::to_value(backend.retain(retained.as_ref().unwrap())?)?,
        "read" | "delete" => {
            let reference: BaselineRef = baseline::read_json(Path::new(flags["--reference"]))?;
            if args[0] == "delete" {
                backend.delete(&reference)?;
                serde_json::json!({"deleted":true,"run_id":reference.run_id})
            } else {
                serde_json::to_value(backend.load(&reference)?)?
            }
        }
        "compare" => {
            let old = baseline::read_json(Path::new(flags["--baseline"]))?;
            let new = baseline::read_json(Path::new(flags["--current"]))?;
            let report = comparison::compare(&backend, &old, &new)?;
            baseline::write_json(Path::new(flags["--out"]), &report)?;
            let saved: DeltaReport = baseline::read_json(Path::new(flags["--out"]))?;
            saved.validate(&backend)?;
            serde_json::to_value(saved)?
        }
        "delta-read" => {
            let report: DeltaReport = baseline::read_json(Path::new(flags["--input"]))?;
            report.validate(&backend)?;
            serde_json::to_value(report)?
        }
        _ => unreachable!(),
    };
    eprintln!("adl_event codefriend_memory command={} outcome=ok", args[0]);
    println!("{}", serde_json::to_string(&result)?);
    Ok(())
}
