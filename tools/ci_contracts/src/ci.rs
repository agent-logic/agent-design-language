//! Public CI routing and selected-result contracts. No interpreter subprocesses.
use std::{collections::BTreeMap, fs::OpenOptions, io::Write, path::Path, process::Command};
pub fn select(event: &str, paths: &[String]) -> BTreeMap<String, String> {
    let full = event != "pull_request" || paths.is_empty();
    let (mut public, mut transpiler) = (full, full);
    let mut reason = if full {
        "manual_or_empty_public_baseline"
    } else {
        "documentation_only"
    };
    for path in paths {
        if ["Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "LICENSE"].contains(&path.as_str())
            || [
                ".github/",
                "tools/source_retirement/",
                "tools/godel_contract_distribution/",
            ]
            .iter()
            .any(|p| path.starts_with(p))
        {
            public = true;
            transpiler = true;
            reason = "repository_policy";
        } else if [
            "demos/transpiler_demo/",
            "demos/rust-transpiler/",
            "tools/transpiler_distribution/",
        ]
        .iter()
        .any(|p| path.starts_with(p))
        {
            transpiler = true;
            reason = "transpiler_distribution";
        } else if [
            "adl-schema/",
            "adl-uts/",
            "adl-v2/",
            "adl-legacy-contracts/",
            "adl/",
            "tools/public_adl/",
        ]
        .iter()
        .any(|p| path.starts_with(p))
        {
            public = true;
            reason = "public_source_or_contract";
        } else if !(path.ends_with(".md") && (path.starts_with("docs/") || !path.contains('/'))) {
            public = true;
            transpiler = true;
            reason = "unknown_or_retired_path_public_baseline";
        }
    }
    [
        ("public_adl_validation_required", public.to_string()),
        ("transpiler_distribution_required", transpiler.to_string()),
        ("ci_contracts_required", "true".into()),
        ("reason", reason.into()),
        ("automatic_pr_entrypoint", "ci".into()),
        ("optional_workflows_status", "deferred".into()),
        (
            "optional_workflows_reason",
            "explicit_dispatch_required".into(),
        ),
        ("soak_workflows_status", "deferred".into()),
        ("duplicate_head_status", "canceled".into()),
        (
            "duplicate_head_reason",
            "source_branch_concurrency_cancel_in_progress".into(),
        ),
    ]
    .into_iter()
    .map(|(k, v)| (k.into(), v))
    .collect()
}
pub fn validate_result(selected: Option<&str>, result: Option<&str>) -> Result<(), String> {
    match (selected, result) {
        (Some("true"), Some("success")) | (Some("false"), Some("skipped")) => Ok(()),
        _ => Err("selected CI lane must succeed; unselected lane must be skipped".into()),
    }
}
pub fn aggregate(kind: &str, env: &BTreeMap<String, String>) -> Result<(), String> {
    let get = |key| env.get(key).map(String::as_str);
    if !["ci", "coverage"].contains(&kind) {
        return Err("unknown aggregate kind".into());
    }
    if get("PATH_POLICY_RESULT") != Some("success") {
        return Err("path policy must succeed".into());
    }
    validate_result(get("PUBLIC_REQUIRED"), get("PUBLIC_RESULT"))?;
    if kind == "ci" {
        if get("CONTRACT_RESULT") != Some("success") {
            return Err("repository contracts must succeed".into());
        }
        validate_result(get("TRANSPILE_REQUIRED"), get("TRANSPILE_RESULT"))?;
    }
    Ok(())
}
pub fn git(root: &Path, args: &[&str]) -> Result<Vec<u8>, String> {
    let out = Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(format!(
            "Git observation failed: {}",
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    Ok(out.stdout)
}
pub fn paths(root: &Path, base: &str, head: &str) -> Result<Vec<String>, String> {
    if base.is_empty() || head.is_empty() {
        return Err("pull request revision pair required".into());
    }
    let commit = |r: &str| -> Result<String, String> {
        String::from_utf8(git(
            root,
            &[
                "rev-parse",
                "--verify",
                "--end-of-options",
                &format!("{r}^{{commit}}"),
            ],
        )?)
        .map(|s| s.trim().to_owned())
        .map_err(|e| e.to_string())
    };
    let (base, head) = (commit(base)?, commit(head)?);
    git(
        root,
        &[
            "diff",
            "--name-only",
            "-z",
            "--no-renames",
            &base,
            &head,
            "--",
        ],
    )?
    .split(|b| *b == 0)
    .filter(|p| !p.is_empty())
    .map(|p| String::from_utf8(p.to_vec()).map_err(|e| e.to_string()))
    .collect()
}
pub fn cli(args: &[String]) -> Result<(), String> {
    let mut event = "workflow_dispatch";
    let mut base = "";
    let mut head = "HEAD";
    let mut output = None;
    let mut kind = None;
    let mut i = 0;
    while i < args.len() {
        let value = args.get(i + 1).ok_or("missing option value")?;
        match args[i].as_str() {
            "--event-name" => event = value,
            "--base" => base = value,
            "--head" => head = value,
            "--ref" => {}
            "--github-output" => output = Some(value),
            "--aggregate" => kind = Some(value),
            _ => return Err(format!("unknown option: {}", args[i])),
        };
        i += 2;
    }
    if let Some(kind) = kind {
        aggregate(kind, &std::env::vars().collect())?;
        println!("PASS public {kind} selected-result contract");
        return Ok(());
    }
    let paths = if event == "pull_request" {
        paths(
            &std::env::current_dir().map_err(|e| e.to_string())?,
            base,
            head,
        )?
    } else {
        vec![]
    };
    let text = select(event, &paths)
        .iter()
        .map(|(k, v)| format!("{k}={v}\n"))
        .collect::<String>();
    print!("{text}");
    if let Some(output) = output {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(output)
            .and_then(|mut f| f.write_all(text.as_bytes()))
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}
