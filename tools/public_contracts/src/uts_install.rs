//! Bounded standalone UTS crate packaging and offline extracted-artifact consumer.
use super::*;
use anyhow::ensure;
pub fn run() -> Result<Value> {
    let repo = fs::canonicalize(root())?;
    let git = |args: &[&str]| -> Result<Vec<u8>> {
        let o = Command::new("git")
            .arg("-C")
            .arg(&repo)
            .args(args)
            .output()?;
        ensure!(o.status.success(), "UTS source Git read failed");
        Ok(o.stdout)
    };
    let head = String::from_utf8(git(&["rev-parse", "HEAD"])?)?
        .trim()
        .to_owned();
    ensure!(
        git(&[
            "status",
            "--porcelain",
            "--",
            "adl-uts",
            "adl-spec/schemas/uts",
            "tools/public_contracts"
        ])?
        .is_empty(),
        "Commit package/schema/proof sources before clean-artifact proof"
    );
    let parent = repo.join(".adl/877-install").join(&head);
    fs::create_dir_all(&parent)?;
    let dest = tempfile::Builder::new()
        .prefix("run-")
        .tempdir_in(&parent)?
        .keep();
    let snapshot = dest.join("snapshot");
    let names = git(&[
        "ls-tree",
        "-r",
        "--name-only",
        "-z",
        &head,
        ".gitignore",
        "adl-uts",
        "adl-spec/schemas/uts",
    ])?;
    let mut files = BTreeMap::new();
    for raw in names.split(|b| *b == 0).filter(|p| !p.is_empty()) {
        let name = std::str::from_utf8(raw)?;
        let mode = String::from_utf8(git(&["ls-tree", &head, "--", name])?)?;
        ensure!(
            mode.starts_with("100644 ") || mode.starts_with("100755 "),
            "snapshot member must be regular"
        );
        files.insert(
            name.to_owned(),
            (
                git(&["show", &format!("{head}:{name}")])?,
                if mode.starts_with("100755 ") {
                    0o755
                } else {
                    0o644
                },
            ),
        );
    }
    archive::extract(&files, &snapshot)?;
    for (name, (bytes, _)) in &files {
        if name.starts_with("adl-uts/schemas/") && name.ends_with(".schema.json") {
            ensure!(
                fs::read(snapshot.join(name.replacen(
                    "adl-uts/schemas/",
                    "adl-spec/schemas/uts/",
                    1
                )))? == *bytes,
                "schema mirror mismatch"
            );
        }
    }
    let call = |program: &str,
                args: &[&str],
                cwd: &Path,
                extra: &BTreeMap<&str, String>|
     -> Result<Output> {
        let o = Command::new(program)
            .args(args)
            .current_dir(cwd)
            .envs(extra)
            .output()?;
        ensure!(o.status.success(), "UTS command failed: {}", text(&o).1);
        Ok(o)
    };
    let mut environment = BTreeMap::new();
    environment.insert(
        "CARGO_TARGET_DIR",
        snapshot
            .join("adl-uts/target")
            .to_string_lossy()
            .into_owned(),
    );
    call("git", &["init", "--quiet"], &snapshot, &environment)?;
    call(
        "git",
        &["add", ".gitignore", "adl-uts", "adl-spec"],
        &snapshot,
        &environment,
    )?;
    let epoch = String::from_utf8(git(&["show", "-s", "--format=%ct", &head])?)?;
    environment.insert("GIT_AUTHOR_DATE", format!("{} +0000", epoch.trim()));
    environment.insert("GIT_COMMITTER_DATE", format!("{} +0000", epoch.trim()));
    call(
        "git",
        &[
            "-c",
            "user.name=UTS package proof",
            "-c",
            "user.email=uts-proof@example.invalid",
            "commit",
            "--quiet",
            "-m",
            "Exact source snapshot",
        ],
        &snapshot,
        &environment,
    )?;
    call(
        "cargo",
        &[
            "package",
            "--offline",
            "--locked",
            "--manifest-path",
            "adl-uts/Cargo.toml",
        ],
        &snapshot,
        &environment,
    )?;
    let artifact = snapshot.join("adl-uts/target/package/adl-uts-0.1.0.crate");
    let vendor = dest.join("vendor");
    archive::extract(&archive::unpack(&fs::read(&artifact)?, true)?, &vendor)?;
    let consumer = dest.join("consumer");
    fs::create_dir_all(consumer.join("src"))?;
    fs::write(consumer.join("Cargo.toml"),"[package]\nname=\"isolated-uts-consumer\"\nversion=\"0.0.0\"\nedition=\"2021\"\n[dependencies]\nadl-uts={version=\"=0.1.0\",path=\"../vendor/adl-uts-0.1.0\"}\nserde_json=\"1\"\n")?;
    fs::write(
        consumer.join("src/main.rs"),
        include_str!("uts_consumer.rs"),
    )?;
    environment.insert(
        "CARGO_TARGET_DIR",
        dest.join("target").to_string_lossy().into_owned(),
    );
    let result = call(
        "cargo",
        &[
            "run",
            "--offline",
            "--quiet",
            "--manifest-path",
            "Cargo.toml",
        ],
        &consumer,
        &environment,
    )?;
    let metadata: Value = serde_json::from_slice(
        &call(
            "cargo",
            &[
                "metadata",
                "--offline",
                "--format-version",
                "1",
                "--manifest-path",
                "Cargo.toml",
            ],
            &consumer,
            &environment,
        )?
        .stdout,
    )?;
    let package = metadata["packages"]
        .as_array()
        .context("packages")?
        .iter()
        .find(|p| p["name"] == "adl-uts")
        .context("UTS package absent")?;
    ensure!(
        fs::canonicalize(package["manifest_path"].as_str().context("manifest path")?)?
            == fs::canonicalize(vendor.join("adl-uts-0.1.0/Cargo.toml"))?,
        "consumer reached producer source"
    );
    let report = json!({"schema":"adl.uts.install_proof.v1","source_revision":head,"package":"adl-uts","version":package["version"],"artifact_sha256":archive::digest(&artifact)?,"consumer_uses_extracted_artifact":true,"schema_mirrors_equal":true,"consumer_output":text(&result).0.trim(),"execution_authority_claimed":false});
    fs::write(
        dest.join("report.json"),
        format!("{}\n", serde_json::to_string_pretty(&report)?),
    )?;
    Ok(report)
}
