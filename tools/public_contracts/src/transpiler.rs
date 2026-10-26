//! Installed demo custody and behavioral regressions, without provider execution.
use super::*;
use anyhow::ensure;
use std::io::Write;
pub const DATA: [&str; 3] = [
    "demos/rust-transpiler/workflow/rust_transpiler_demo.yaml",
    "demos/rust-transpiler/output/workflow_runtime.rs",
    "LICENSE",
];
pub fn build(repo: &Path, binary: &Path, output: &Path) -> Result<Value> {
    let git = |args: &[&str]| -> Result<Vec<u8>> {
        let o = Command::new("git")
            .arg("-C")
            .arg(repo)
            .args(args)
            .output()?;
        ensure!(o.status.success(), "committed demo read failed");
        Ok(o.stdout)
    };
    let revision = String::from_utf8(git(&["rev-parse", "HEAD"])?)?
        .trim()
        .to_owned();
    let mut files = BTreeMap::new();
    for path in DATA {
        files.insert(
            path.to_owned(),
            (git(&["show", &format!("{revision}:{path}")])?, 0o644),
        );
    }
    files.insert("bin/transpiler_demo".into(), (fs::read(binary)?, 0o755));
    let members: Vec<_> = files
        .iter()
        .map(|(p, (b, m))| json!({"path":p,"sha256":archive::sha(b),"bytes":b.len(),"mode":m}))
        .collect();
    let manifest = json!({"schema":"adl.transpiler_demo.installed.v1","source_revision":revision,"classification":"bounded_demo_scaffold","members":members});
    files.insert(
        "manifest.json".into(),
        (
            format!("{}\n", serde_json::to_string_pretty(&manifest)?).into_bytes(),
            0o644,
        ),
    );
    let bytes = archive::pack(&files)?;
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)?
        .write_all(&bytes)?;
    Ok(manifest)
}
pub fn install(input: &Path, digest: &str, destination: &Path) -> Result<Value> {
    let bytes = fs::read(input)?;
    ensure!(archive::sha(&bytes) == digest, "archive digest mismatch");
    let mut files = archive::unpack(&bytes, false)?;
    let mut expected: Vec<_> = DATA
        .into_iter()
        .chain(["bin/transpiler_demo", "manifest.json"])
        .map(str::to_owned)
        .collect();
    expected.sort();
    ensure!(
        files.keys().eq(expected.iter()),
        "unexpected members or types"
    );
    let manifest: Value = serde_json::from_slice(&files["manifest.json"].0)?;
    let members = manifest["members"].as_array().context("manifest members")?;
    ensure!(
        manifest["schema"] == "adl.transpiler_demo.installed.v1" && members.len() == 4,
        "invalid manifest"
    );
    let mut seen = std::collections::BTreeSet::new();
    for m in members {
        let path = m["path"].as_str().context("member path")?;
        ensure!(
            path != "manifest.json" && seen.insert(path),
            "duplicate manifest member"
        );
        let (data, actual_mode) = files.get(path).context("missing declared member")?;
        let mode = if path.starts_with("bin/") {
            0o755
        } else {
            0o644
        };
        ensure!(
            Some(data.len() as u64) == m["bytes"].as_u64()
                && archive::sha(data) == m["sha256"]
                && m["mode"].as_u64() == Some(mode)
                && u64::from(*actual_mode) == mode,
            "member mismatch"
        );
    }
    files.get_mut("manifest.json").unwrap().1 = 0o644;
    archive::extract(&files, destination)?;
    Ok(manifest)
}
pub fn regressions(binary: &Path) -> Result<Value> {
    for case in [
        "mapping", "missing", "mismatch", "existing", "symlink", "tamper",
    ] {
        let temp = tempfile::tempdir()?;
        let root = fs::canonicalize(temp.path())?;
        let artifact = root.join("package.tar");
        build(&super::root(), binary, &artifact)?;
        let installed = root.join("installed");
        let digest = archive::digest(&artifact)?;
        install(&artifact, &digest, &installed)?;
        let output = root.join("output");
        let input = installed.join(DATA[0]);
        match case {
            "missing" => fs::remove_file(&input)?,
            "mismatch" => fs::write(
                &input,
                fs::read_to_string(&input)?.replace("step_prepare_input", "step_wrong"),
            )?,
            "existing" => {
                fs::create_dir(&output)?;
                fs::write(output.join("sentinel"), "preserved")?;
            }
            "symlink" => {
                let outside = root.join("outside.yaml");
                fs::rename(&input, &outside)?;
                #[cfg(unix)]
                std::os::unix::fs::symlink(outside, &input)?;
            }
            "tamper" => {
                fs::OpenOptions::new()
                    .append(true)
                    .open(&artifact)?
                    .write_all(b"changed")?;
                let dest = root.join("bad");
                ensure!(
                    install(&artifact, &digest, &dest).is_err() && !dest.exists(),
                    "tampered archive accepted"
                );
                continue;
            }
            _ => {}
        }
        let result = Command::new(installed.join("bin/transpiler_demo"))
            .args([
                "--input-root",
                installed.to_str().context("installed path")?,
                "--output-root",
                output.to_str().context("output path")?,
            ])
            .current_dir(&root)
            .output()?;
        if case == "mapping" {
            ensure!(
                result.status.success(),
                "demo execution failed: {}",
                text(&result).1
            );
            let report: Value = serde_json::from_slice(&fs::read(
                output.join("demos/rust-transpiler/output/transpiler_verification.v0.8.json"),
            )?)?;
            ensure!(
                report["mapping"]["order_check"] == "PASS"
                    && report["mapping"]["pairs"]
                        .as_array()
                        .is_some_and(|p| p.len() == 3 && p.iter().all(|p| p["status"] == "PASS"))
                    && report["adaptive_execution"]["attempts_executed"] == 0,
                "demo mapping changed"
            );
        } else {
            ensure!(
                !result.status.success(),
                "negative demo case accepted: {case}"
            );
            if case == "mismatch" {
                ensure!(
                    text(&result).1.contains("ordering do not match"),
                    "mapping failure diagnostic changed"
                );
            } else if case == "existing" {
                ensure!(
                    fs::read_to_string(output.join("sentinel"))? == "preserved",
                    "output sentinel changed"
                );
            } else {
                ensure!(!output.exists(), "negative case materialized output");
            }
        }
    }
    Ok(json!({"status":"passed","cases":6,"classification":"bounded_demo_scaffold"}))
}
