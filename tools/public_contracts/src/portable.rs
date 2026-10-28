//! Compile the actual public workflow separately and compare its observed boundary.
use super::*;
use anyhow::ensure;
pub fn resume(baseline: &Path, probe: &Path) -> Result<Value> {
    let mut environment = env();
    environment.insert("ADL_OBSERVABILITY".into(), "0".into());
    for args in [
        vec![],
        vec!["--help"],
        vec!["run-id", "--adl"],
        vec!["run-id", "--steer"],
        vec!["run-id", "unexpected"],
        vec!["run-id", "--adl", "literal file", "--bogus"],
    ] {
        let mut argv = vec!["resume"];
        argv.extend(args);
        let old = invoke(baseline, &root(), &argv, &environment)?;
        let new = invoke(probe, &root(), &argv, &environment)?;
        ensure!(
            old.status.code() == new.status.code()
                && old.stdout == new.stdout
                && old.stderr == new.stderr,
            "resume diagnostic difference: {argv:?}"
        );
    }
    Ok(
        json!({"status":"passed","cases":6,"claim":"exact diagnostic/help stdout stderr and exit; no resume execution"}),
    )
}
pub fn workflow(baseline: &Path) -> Result<Value> {
    let temp = tempfile::Builder::new()
        .prefix("adl-portable-workflow-")
        .tempdir()?;
    let work = fs::canonicalize(temp.path())?;
    let repo = fs::canonicalize(root())?;
    fs::create_dir(work.join("src"))?;
    fs::write(work.join("Cargo.toml"),format!("[package]\nname=\"adl-portable-workflow-probe\"\nversion=\"0.1.0\"\nedition=\"2021\"\n[dependencies]\nanyhow=\"1\"\nserde_json=\"1\"\nadl-legacy-contracts={{path={}}}\n[workspace]\n",serde_json::to_string(&repo.join("adl-legacy-contracts"))?))?;
    let mut source = String::new();
    for (module, file) in [
        ("runtime_owner", "runtime_owner.rs"),
        ("public_workflow", "public_workflow.rs"),
        ("usage", "usage.rs"),
    ] {
        source += &format!(
            "#[path={}] mod {module};\n",
            serde_json::to_string(&repo.join("adl/src/cli").join(file))?
        );
    }
    source+="fn usage()-> &'static str {usage::usage()}\nfn resume_usage()-> &'static str {usage::resume_usage()}\nfn main(){let a:Vec<String>=std::env::args().skip(1).collect();let r=public_workflow::run(&a);if let Err(e)=r{eprintln!(\"Error: {e}\");std::process::exit(1);}}\n";
    fs::write(work.join("src/main.rs"), source)?;
    let mut environment = env();
    environment.insert("ADL_OBSERVABILITY".into(), "0".into());
    environment.insert(
        "CARGO_TARGET_DIR".into(),
        work.join("target").to_string_lossy().into_owned(),
    );
    let manifest = work.join("Cargo.toml");
    let build = invoke(
        Path::new("cargo"),
        &work,
        &[
            "build",
            "--offline",
            "--manifest-path",
            manifest.to_str().context("manifest path")?,
        ],
        &environment,
    )?;
    ensure!(
        build.status.success(),
        "portable build failed: {}",
        text(&build).1
    );
    let host = host()?;
    let metadata = invoke(
        Path::new("cargo"),
        &work,
        &[
            "metadata",
            "--locked",
            "--offline",
            "--filter-platform",
            &host,
            "--format-version",
            "1",
            "--manifest-path",
            manifest.to_str().unwrap(),
        ],
        &environment,
    )?;
    ensure!(metadata.status.success(), "portable metadata failed");
    let metadata: Value = serde_json::from_slice(&metadata.stdout)?;
    let forbidden = [
        "adl",
        "adl-runtime",
        "adl-runtime-kernel",
        "adl-provider-core",
        "codefriend",
        "agent-product-support",
    ];
    ensure!(
        !metadata["packages"]
            .as_array()
            .context("packages")?
            .iter()
            .any(|p| forbidden.contains(&p["name"].as_str().unwrap_or(""))),
        "private crate in portable dependency graph"
    );
    let probe = work.join("target/debug/adl-portable-workflow-probe");
    let fixture = repo.join("adl/examples/v0-6-hitl-no-pause.adl.yaml");
    for options in [
        vec!["--print-plan"],
        vec!["--print-prompts"],
        vec!["--print-plan", "--print-prompts"],
    ] {
        let mut argv = vec![fixture.to_str().context("fixture path")?];
        argv.extend(options);
        let old = invoke(baseline, &repo, &argv, &environment)?;
        let new = invoke(&probe, &repo, &argv, &environment)?;
        ensure!(
            old.status.success()
                && new.status.success()
                && old.stdout == new.stdout
                && old.stderr == new.stderr,
            "portable presentation changed: {argv:?}"
        );
    }
    let missing = invoke(
        &probe,
        &repo,
        &[fixture.to_str().unwrap(), "--run"],
        &environment,
    )?;
    ensure!(
        missing.status.code() == Some(1) && text(&missing).1.contains("ADL_RUNTIME_INSTALL_PREFIX"),
        "missing owner refusal changed"
    );
    Ok(
        json!({"status":"passed","presentation_cases":3,"missing_owner_case":1,"private_crates_in_host_dependency_graph":0,"limits":"Portable planning and routing boundary only; full public CLI manifest partition remains required"}),
    )
}
