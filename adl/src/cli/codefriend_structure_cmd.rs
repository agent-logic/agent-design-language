use adl::codefriend::{
    architecture::artifact::{
        self, BoundaryPolicyArtifact, ChangeSetArtifact, RationaleSelectionArtifact,
    },
    evidence::store::Store,
    memory::baseline::AdmittedBaselines,
};
use anyhow::{ensure, Result};
use std::{collections::BTreeMap, io::Read, path::Path};
pub(super) const USAGE:&str="adl codefriend architecture four-plus-one --store <directory> --graph <structure.json> --provider-request <request.json> --out <new-directory>\nadl codefriend architecture report --store <directory> --packet-id <id> --policy <policy.json> --out <new-report.json>\nadl codefriend architecture drift --store <directory> --baselines <directory> --baseline <graph.json> --current <graph.json> --out <new-drift.json>\nadl codefriend architecture drift-read --store <directory> --baselines <directory> --input <drift.json>\nadl codefriend architecture rationale --store <directory> --graph <structure.json> --selection <selection.json> --out <new-rationale.json>\nadl codefriend architecture rationale-read --store <directory> --input <rationale.json>\nadl codefriend architecture impact --store <directory> --graph <structure.json> --changes <changes.json> --out <new-impact.json>\nadl codefriend architecture impact-read --store <directory> --input <impact.json>\nadl codefriend architecture read --store <directory> --input <report.json>";
pub(super) fn run(args: &[String]) -> Result<()> {
    ensure!(!args.is_empty(), "{USAGE}");
    let expected: &[&str] = match args[0].as_str() {
        "four-plus-one" => &["--store", "--graph", "--provider-request", "--out"],
        "drift" => &["--store", "--baselines", "--baseline", "--current", "--out"],
        "drift-read" => &["--store", "--baselines", "--input"],
        "report" => &["--store", "--packet-id", "--policy", "--out"],
        "read" | "rationale-read" | "impact-read" => &["--store", "--input"],
        "impact" => &["--store", "--graph", "--changes", "--out"],
        "rationale" => &["--store", "--graph", "--selection", "--out"],
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
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    if args[0] == "four-plus-one" {
        use adl::codefriend::{
            architecture::four_plus_one::generation, publication::write_json_create_only,
        };
        use adl::provider_communication::ProviderInvocationRequestV1;
        let graph = artifact::read_report(&store, Path::new(flags["--graph"]), now)?;
        let input = Path::new(flags["--provider-request"]);
        generation::validate_artifact_path(input)?;
        ensure!(
            std::fs::symlink_metadata(input)?.file_type().is_file(),
            "four_plus_one_request_not_regular"
        );
        let mut bytes = Vec::new();
        std::fs::File::open(input)?
            .take(512 * 1024 + 1)
            .read_to_end(&mut bytes)?;
        ensure!(bytes.len() <= 512 * 1024, "four_plus_one_request_too_large");
        let request: ProviderInvocationRequestV1 = serde_json::from_slice(&bytes)
            .map_err(|_| anyhow::anyhow!("four_plus_one_invalid_provider_request"))?;
        ensure!(
            request
                .input_text
                .as_deref()
                .unwrap_or_default()
                .trim()
                .is_empty(),
            "provider_request_must_not_preload_review_input"
        );
        let output = std::path::absolute(flags["--out"])?;
        generation::validate_artifact_path(&output)?;
        ensure!(
            !output.starts_with(std::fs::canonicalize(root)?),
            "four_plus_one_output_inside_store"
        );
        ensure!(!output.exists(), "four_plus_one_output_exists");
        std::fs::create_dir(&output)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&output, std::fs::Permissions::from_mode(0o700))?;
        }
        let report = generation::run_provider(&store, &graph, request, &output)?;
        adl::codefriend::architecture::four_plus_one::render::write_artifacts(
            &report.package,
            &store.get(&report.package.packet_id)?,
            now,
            &output.join("rendered"),
        )?;
        write_json_create_only(&output.join("four-plus-one.json"), &report)?;
        println!(
            "{}",
            serde_json::json!({"schema":report.schema,"digest":report.package.digest,"complete":report.package.complete,"views":report.package.views.len(),"scenarios":report.package.scenarios.len()})
        );
        eprintln!(
            "adl_event kind=codefriend_four_plus_one status=success complete={}",
            report.package.complete
        );
        return Ok(());
    }
    if args[0] == "rationale" || args[0] == "rationale-read" {
        let report = if args[0] == "rationale" {
            let path = Path::new(flags["--selection"]);
            ensure!(
                std::fs::symlink_metadata(path)?.file_type().is_file(),
                "rationale_selection_not_regular"
            );
            let mut bytes = Vec::new();
            std::fs::File::open(path)?
                .take(128 * 1024 + 1)
                .read_to_end(&mut bytes)?;
            ensure!(bytes.len() <= 128 * 1024, "rationale_selection_too_large");
            let selection: RationaleSelectionArtifact = serde_json::from_slice(&bytes)
                .map_err(|_| anyhow::anyhow!("invalid_rationale_selection"))?;
            let graph = artifact::read_report(&store, Path::new(flags["--graph"]), now)?;
            let report = artifact::rationale_report(&store, graph, selection, now)?;
            artifact::write_rationale(&report, &store, Path::new(flags["--out"]), now)?;
            report
        } else {
            artifact::read_rationale(&store, Path::new(flags["--input"]), now)?
        };
        let summary = report.summary();
        println!("{}", summary);
        eprintln!(
            "adl_event kind=codefriend_rationale status=success analysis_complete={} boundaries={}",
            summary["analysis_complete"], summary["boundaries"]
        );
        return Ok(());
    }
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
            let changes: ChangeSetArtifact = serde_json::from_slice(&bytes)
                .map_err(|_| anyhow::anyhow!("invalid_change_input"))?;
            let graph = artifact::read_report(&store, Path::new(flags["--graph"]), now)?;
            let report = artifact::impact_report(&store, graph, changes, now)?;
            artifact::write_impact(&report, &store, Path::new(flags["--out"]), now)?;
            report
        } else {
            artifact::read_impact(&store, Path::new(flags["--input"]), now)?
        };
        let summary = report.summary();
        println!("{}", summary);
        eprintln!("adl_event kind=codefriend_impact status=success analysis_complete={} impacts={} unknowns={}",summary["analysis_complete"],summary["impacts"],summary["unknowns"]);
        return Ok(());
    }
    if args[0] == "drift" || args[0] == "drift-read" {
        let baselines_root = Path::new(flags["--baselines"]);
        let report = if args[0] == "drift" {
            let baseline = artifact::read_report(&store, Path::new(flags["--baseline"]), now)?;
            let current = artifact::read_report(&store, Path::new(flags["--current"]), now)?;
            let output = std::path::absolute(flags["--out"])?;
            ensure!(
                !output.starts_with(std::path::absolute(root)?)
                    && !output.starts_with(std::path::absolute(baselines_root)?),
                "drift_output_inside_managed_store"
            );
            let baselines = AdmittedBaselines::open(&store, baselines_root, true)?;
            baselines.retain(baseline.record())?;
            baselines.retain(current.record())?;
            let report =
                artifact::drift_report_pair(&store, &store, &baselines, baseline, current, now)?;
            artifact::write_drift_pair(
                &report,
                &store,
                &store,
                &baselines,
                Path::new(flags["--out"]),
                now,
            )?;
            report
        } else {
            let baselines = AdmittedBaselines::open(&store, baselines_root, false)?;
            artifact::read_drift_pair(&store, &store, &baselines, Path::new(flags["--input"]), now)?
        };
        let summary = report.summary();
        println!("{}", summary);
        eprintln!(
            "adl_event kind=codefriend_drift status=success comparable={} changes={}",
            summary["comparable"], summary["changes"]
        );
        return Ok(());
    }
    let report = if args[0] == "report" {
        let mut bytes = Vec::new();
        std::fs::File::open(flags["--policy"])?
            .take(128 * 1024 + 1)
            .read_to_end(&mut bytes)?;
        ensure!(bytes.len() <= 128 * 1024, "policy_too_large");
        let policy: BoundaryPolicyArtifact = serde_json::from_slice(&bytes)
            .map_err(|_| anyhow::anyhow!("invalid_boundary_policy"))?;
        let r = artifact::report(&store, flags["--packet-id"], policy, now)?;
        artifact::write_report(&r, &store, Path::new(flags["--out"]), now)?;
        r
    } else {
        artifact::read_report(&store, Path::new(flags["--input"]), now)?
    };
    let summary = report.summary();
    println!("{}", serde_json::to_string(&summary)?);
    eprintln!("adl_event kind=codefriend_architecture status=success analysis_complete={} nodes={} edges={} unknowns={}",summary["analysis_complete"],summary["nodes"],summary["edges"],summary["unknowns"]);
    Ok(())
}
