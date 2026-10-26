//! Deterministic LLVM coverage ownership, denominators and execution receipts.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::{Component, Path, PathBuf},
    process::{Command, Stdio},
};
type Result<T> = std::result::Result<T, String>;
fn array(v: &Value) -> Result<&Vec<Value>> {
    v.as_array().ok_or_else(|| "expected array".into())
}
fn string(v: &Value) -> Result<&str> {
    v.as_str().ok_or_else(|| "expected string".into())
}
fn strings(v: &Value) -> Result<Vec<String>> {
    array(v)?
        .iter()
        .map(|v| string(v).map(str::to_owned))
        .collect()
}
fn optional_strings(v: &Value) -> Result<Vec<String>> {
    if v.is_null() {
        Ok(vec![])
    } else {
        strings(v)
    }
}
fn normalized(p: &Path) -> PathBuf {
    if let Ok(p) = fs::canonicalize(p) {
        return p;
    }
    let mut out = PathBuf::new();
    for c in p.components() {
        match c {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            _ => out.push(c.as_os_str()),
        }
    }
    out
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub package: String,
    pub count: u64,
    pub covered: u64,
}
pub fn check_rows(
    reports: &BTreeMap<String, Value>,
    inventory: &Value,
    root: &Path,
) -> Result<(BTreeMap<String, Row>, Vec<String>)> {
    let mut owners = BTreeMap::new();
    let mut declarations = BTreeSet::new();
    let mut tests = BTreeSet::new();
    let mut packages = BTreeSet::new();
    for p in array(&inventory["packages"])? {
        let name = string(&p["name"])?;
        packages.insert(name.to_owned());
        declarations.extend(optional_strings(&p["declaration_only"])?);
        tests.extend(optional_strings(&p["test_sources"])?);
        for path in strings(&p["sources"])? {
            if owners.insert(path.clone(), name.to_owned()).is_some() {
                return Err(format!("duplicate source ownership: {path}"));
            }
        }
    }
    if tests.iter().any(|p| owners.contains_key(p)) {
        return Err("test exclusion overlaps production source".into());
    }
    if packages != reports.keys().cloned().collect() {
        return Err("missing or unexpected package reports".into());
    }
    let mut rows = BTreeMap::new();
    let root = normalized(root);
    for (package, report) in reports {
        if report["type"] != "llvm.coverage.json.export" || array(&report["data"])?.is_empty() {
            return Err(format!("malformed LLVM report: {package}"));
        }
        let mut seen = BTreeSet::new();
        for data in array(&report["data"])? {
            for item in array(&data["files"])? {
                let path = Path::new(string(&item["filename"])?);
                let path = normalized(&if path.is_absolute() {
                    path.to_path_buf()
                } else {
                    root.join(path)
                });
                let Ok(relative) = path.strip_prefix(&root) else {
                    continue;
                };
                let relative = relative.to_string_lossy().replace('\\', "/");
                if !seen.insert(relative.clone()) {
                    return Err(format!("duplicate measured file: {relative}"));
                }
                let Some(owner) = owners.get(&relative) else {
                    if tests.contains(&relative) {
                        continue;
                    }
                    return Err(format!(
                        "unexpected repository source in coverage: {relative}"
                    ));
                };
                if owner != package {
                    continue;
                }
                let lines = &item["summary"]["lines"];
                let (Some(count), Some(covered)) =
                    (lines["count"].as_u64(), lines["covered"].as_u64())
                else {
                    return Err(format!("invalid line counts: {relative}"));
                };
                if covered > count {
                    return Err(format!("invalid line counts: {relative}"));
                }
                if count == 0 && !declarations.contains(&relative) {
                    return Err(format!("zero production source denominator: {relative}"));
                }
                rows.insert(
                    relative,
                    Row {
                        package: package.clone(),
                        count,
                        covered,
                    },
                );
            }
        }
    }
    let missing: Vec<_> = owners
        .keys()
        .filter(|p| !rows.contains_key(*p))
        .cloned()
        .collect();
    if missing.iter().any(|p| !declarations.contains(p)) {
        return Err("missing production source measurements".into());
    }
    Ok((rows, missing))
}
pub fn enforce(
    rows: &BTreeMap<String, Row>,
    mode: &str,
    changed: &BTreeSet<String>,
) -> Result<Vec<String>> {
    if !["full", "pull_request"].contains(&mode) {
        return Err("invalid coverage mode".into());
    }
    let count: u128 = rows.values().map(|r| u128::from(r.count)).sum();
    let covered: u128 = rows.values().map(|r| u128::from(r.covered)).sum();
    if count == 0 {
        return Err("empty coverage denominator".into());
    }
    let mut failures = vec![];
    for (path, row) in rows {
        if row.count > 0
            && (mode == "full" || changed.contains(path))
            && u128::from(row.covered) * 100 < u128::from(row.count) * 80
        {
            failures.push(format!("{path}: below 80% line floor"));
        }
    }
    if mode == "full" && covered * 100 < count * 90 {
        failures.push("public aggregate below 90% line floor".into());
    }
    Ok(failures)
}
fn read_json(path: &Path) -> Result<Value> {
    serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?).map_err(|e| e.to_string())
}
fn save(path: &Path, value: &Value) -> Result<()> {
    fs::write(
        path,
        format!(
            "{}\n",
            serde_json::to_string_pretty(value).map_err(|e| e.to_string())?
        ),
    )
    .map_err(|e| e.to_string())
}
fn hash(path: &Path) -> Result<String> {
    Ok(format!(
        "{:x}",
        Sha256::digest(fs::read(path).map_err(|e| e.to_string())?)
    ))
}
fn observe(root: &Path, env: &BTreeMap<String, String>, argv: &[String]) -> Result<String> {
    let output = Command::new(&argv[0])
        .args(&argv[1..])
        .current_dir(root)
        .envs(env)
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err(format!(
            "command failed: {}: {}",
            argv[0],
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    String::from_utf8(output.stdout).map_err(|e| e.to_string())
}
fn args(values: &[&str]) -> Vec<String> {
    values.iter().map(|s| s.to_string()).collect()
}
fn clean(root: &Path) -> Result<()> {
    if !super::ci::git(
        root,
        &[
            "status",
            "--porcelain",
            "--untracked-files=all",
            "--",
            ".",
            ":(exclude).csdlc",
        ],
    )?
    .is_empty()
    {
        return Err("coverage checkout inputs must be committed and clean".into());
    }
    Ok(())
}
pub fn run(root: &Path, output: &Path, mode: &str, base: Option<&str>) -> Result<()> {
    if !["full", "pull_request"].contains(&mode) {
        return Err("invalid coverage mode".into());
    }
    let config = root.join("tools/public_adl/coverage_inventory.json");
    let inventory = read_json(&config)?;
    let output = if output.is_absolute() {
        output.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|e| e.to_string())?
            .join(output)
    };
    if output.exists() {
        return Err("coverage output already exists".into());
    }
    fs::create_dir_all(&output).map_err(|e| e.to_string())?;
    let mut sources = BTreeSet::new();
    for p in array(&inventory["packages"])? {
        sources.extend(strings(&p["sources"])?);
    }
    clean(root)?;
    let hashes = || -> Result<BTreeMap<String, String>> {
        sources
            .iter()
            .map(|p| Ok((p.clone(), hash(&root.join(p))?)))
            .collect()
    };
    let source_hashes = hashes()?;
    let head = String::from_utf8(super::ci::git(root, &["rev-parse", "HEAD"])?)
        .map_err(|e| e.to_string())?
        .trim()
        .to_owned();
    let changed: BTreeSet<String> = if mode == "pull_request" {
        let base = base
            .filter(|s| !s.is_empty())
            .ok_or("PR coverage requires an explicit base revision")?;
        let resolved = String::from_utf8(super::ci::git(
            root,
            &[
                "rev-parse",
                "--verify",
                "--end-of-options",
                &format!("{base}^{{commit}}"),
            ],
        )?)
        .map_err(|e| e.to_string())?;
        String::from_utf8(super::ci::git(
            root,
            &[
                "diff",
                "--name-only",
                "--diff-filter=ACMR",
                &format!("{}...HEAD", resolved.trim()),
                "--",
            ],
        )?)
        .map_err(|e| e.to_string())?
        .lines()
        .map(str::to_owned)
        .collect()
    } else {
        BTreeSet::new()
    };
    let mut reports = BTreeMap::new();
    let mut executions = vec![];
    for p in array(&inventory["packages"])? {
        let name = string(&p["name"])?;
        let manifest = string(&p["manifest"])?;
        let work = output.join(name);
        fs::create_dir(&work).map_err(|e| e.to_string())?;
        let target = work.join("target").to_string_lossy().into_owned();
        let mut env: BTreeMap<String, String> = [
            ("CARGO_BUILD_JOBS", "1".into()),
            ("CARGO_INCREMENTAL", "0".into()),
            ("CARGO_TARGET_DIR", target.clone()),
            ("CARGO_LLVM_COV_TARGET_DIR", target),
        ]
        .into_iter()
        .map(|(k, v)| (k.into(), v))
        .collect();
        let declared = observe(
            root,
            &env,
            &args(&["cargo", "llvm-cov", "show-env", "--manifest-path", manifest]),
        )?;
        let profiles: Vec<_> = declared
            .lines()
            .filter_map(|s| s.strip_prefix("LLVM_PROFILE_FILE="))
            .collect();
        if profiles.len() != 1 {
            return Err("missing or malformed LLVM profile declaration".into());
        }
        let profile = shell_words::split(profiles[0]).map_err(|e| e.to_string())?;
        if profile.len() != 1 {
            return Err("missing or malformed LLVM profile declaration".into());
        }
        env.insert("LLVM_PROFILE_FILE".into(), profile[0].clone());
        let mut first = args(&[
            "cargo",
            "llvm-cov",
            "--locked",
            "--no-report",
            "--manifest-path",
            manifest,
            "--package",
            name,
        ]);
        first.extend(strings(&p["selection"])?);
        let mut commands = vec![first];
        if name == "adl" {
            let binary = work.join("target/debug/adl").to_string_lossy().into_owned();
            for contract in [
                "runtime-dispatch",
                "cli-refusals",
                "session",
                "codefriend-dispatch",
            ] {
                commands.push(vec![
                    "cargo".into(),
                    "run".into(),
                    "--locked".into(),
                    "--quiet".into(),
                    "--manifest-path".into(),
                    "tools/public_contracts/Cargo.toml".into(),
                    "--bin".into(),
                    "adl-public-contracts".into(),
                    "--".into(),
                    contract.into(),
                    "--binary".into(),
                    binary.clone(),
                ]);
            }
        }
        let summary = work.join("summary.json").to_string_lossy().into_owned();
        let lcov = work.join("coverage.lcov").to_string_lossy().into_owned();
        commands.push(args(&[
            "cargo",
            "llvm-cov",
            "report",
            "--manifest-path",
            manifest,
            "--package",
            name,
            "--json",
            "--summary-only",
            "--output-path",
            &summary,
        ]));
        commands.push(args(&[
            "cargo",
            "llvm-cov",
            "report",
            "--manifest-path",
            manifest,
            "--package",
            name,
            "--lcov",
            "--output-path",
            &lcov,
        ]));
        for (index, argv) in commands.iter().enumerate() {
            let log = fs::OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(work.join(format!("{index}.log")))
                .map_err(|e| e.to_string())?;
            let status = Command::new(&argv[0])
                .args(&argv[1..])
                .current_dir(root)
                .envs(&env)
                .stdout(Stdio::from(log.try_clone().map_err(|e| e.to_string())?))
                .stderr(Stdio::from(log))
                .status()
                .map_err(|e| e.to_string())?;
            executions
                .push(json!({"package":name,"argv":argv,"exit_code":status.code().unwrap_or(-1)}));
            save(&output.join("executions.json"), &json!(executions))?;
            if !status.success() {
                return Err(format!("coverage producer failed: {name}"));
            }
        }
        reports.insert(name.to_owned(), read_json(&work.join("summary.json"))?);
    }
    clean(root)?;
    if String::from_utf8(super::ci::git(root, &["rev-parse", "HEAD"])?)
        .map_err(|e| e.to_string())?
        .trim()
        != head
    {
        return Err("source HEAD changed during coverage".into());
    }
    if source_hashes != hashes()? {
        return Err("production source changed during coverage".into());
    }
    let (rows, unmeasured) = check_rows(&reports, &inventory, root)?;
    let failures = enforce(&rows, mode, &changed)?;
    let files: BTreeMap<_, _> = rows
        .iter()
        .map(|(p, r)| {
            (
                p,
                json!({"package":r.package,"count":r.count,"covered":r.covered}),
            )
        })
        .collect();
    let receipt = json!({"schema":"adl.public_coverage.v1","source_head":head,"mode":mode,"base":base,"changed_files":changed,"files":files,"source_sha256":source_hashes,"declaration_only_without_measurement":unmeasured,"line_count":rows.values().map(|r|r.count).sum::<u64>(),"covered_lines":rows.values().map(|r|r.covered).sum::<u64>(),"thresholds":{"changed_file":80,"full_file":80,"full_aggregate":90},"inventory_sha256":hash(&config)?,"toolchain":observe(root,&BTreeMap::new(),&args(&["rustc","-vV"]))?,"failures":failures,"status":if failures.is_empty(){"passed"}else{"failed"}});
    save(&output.join("result.json"), &receipt)?;
    if !failures.is_empty() {
        return Err(failures.join("; "));
    }
    Ok(())
}
pub fn cli(root: &Path, argv: &[String]) -> Result<()> {
    let (mut output, mut mode, mut base) = (None, None, None);
    let mut i = 0;
    while i < argv.len() {
        let v = argv.get(i + 1).ok_or("missing option value")?;
        match argv[i].as_str() {
            "--output" => output = Some(v),
            "--mode" => mode = Some(v),
            "--base" => base = Some(v.as_str()),
            _ => return Err("unknown coverage option".into()),
        }
        i += 2;
    }
    run(
        root,
        Path::new(output.ok_or("--output required")?),
        mode.ok_or("--mode required")?,
        base,
    )
}
pub fn report(output: &Path, runner: &Path, summary: &Path, outcome: &str) -> Result<bool> {
    let mut lines = vec![
        "# Public ADL coverage report".into(),
        "".into(),
        "- Invocation: manual workflow dispatch".into(),
        format!("- Coverage step: {outcome}"),
        "- Required floors: 90% aggregate and 80% per measured file".into(),
        "".into(),
    ];
    let result = output.join("result.json");
    let mut failed = true;
    if !result.is_file() {
        lines.extend([
            "## Blocker".into(),
            "- Coverage result was not produced; inspect retained execution logs.".into(),
        ]);
    } else {
        let parsed = (|| -> Result<(Vec<String>, bool)> {
            let d = read_json(&result)?;
            let count = d["line_count"]
                .as_u64()
                .filter(|n| *n > 0)
                .ok_or("empty denominator")?;
            let covered = d["covered_lines"]
                .as_u64()
                .filter(|n| *n <= count)
                .ok_or("invalid count")?;
            if d["schema"] != "adl.public_coverage.v1"
                || d["mode"] != "full"
                || d["thresholds"]["full_file"] != 80
                || d["thresholds"]["full_aggregate"] != 90
            {
                return Err("invalid coverage receipt".into());
            }
            let failures = strings(&d["failures"])?;
            let status = string(&d["status"])?;
            let mut out = vec![
                format!("- Source: `{}`", string(&d["source_head"])?),
                format!("- Measured lines: {covered}/{count}"),
                format!(
                    "- Total coverage: {:.2}%",
                    100.0 * covered as f64 / count as f64
                ),
                "".into(),
                "## Coverage failures".into(),
            ];
            if failures.is_empty() {
                out.push("- None".into())
            } else {
                out.extend(failures.iter().map(|s| format!("- {s}")));
            }
            out.extend(["".into(), "## Lowest measured files".into()]);
            let mut measured = vec![];
            for (path, row) in d["files"].as_object().ok_or("malformed files")? {
                let n = row["count"].as_u64().ok_or("bad file count")?;
                let c = row["covered"]
                    .as_u64()
                    .filter(|c| *c <= n)
                    .ok_or("bad file covered")?;
                if n > 0 {
                    measured.push((path, c, n));
                }
            }
            measured.sort_by(|a, b| {
                (u128::from(a.1) * u128::from(b.2))
                    .cmp(&(u128::from(b.1) * u128::from(a.2)))
                    .then_with(|| a.0.cmp(b.0))
            });
            for (path, c, n) in measured.iter().take(10) {
                out.push(format!(
                    "- `{path}`: {c}/{n} ({:.2}%)",
                    100.0 * *c as f64 / *n as f64
                ));
            }
            Ok((out, status != "passed" || !failures.is_empty()))
        })();
        match parsed {
            Ok((out, bad)) => {
                lines.extend(out);
                failed = bad;
            }
            Err(_) => lines.extend([
                "## Blocker".into(),
                "- Coverage result is malformed or unreadable; inspect retained execution logs."
                    .into(),
            ]),
        }
    }
    let text = lines.join("\n") + "\n";
    fs::write(runner.join("nightly-public-coverage-report.md"), &text)
        .map_err(|e| e.to_string())?;
    fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(summary)
        .and_then(|mut f| f.write_all(text.as_bytes()))
        .map_err(|e| e.to_string())?;
    Ok(!failed)
}
