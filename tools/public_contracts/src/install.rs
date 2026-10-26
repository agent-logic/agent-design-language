//! OS-enforced independent public package proof. No unenforced fallback.
use super::*;
use anyhow::ensure;
pub const PACKAGES: [(&str, &str); 5] = [
    ("adl-schema", "adl-schema"),
    ("adl-uts", "adl-uts"),
    ("adl-language", "adl-v2/crates/adl-language"),
    ("adl-compiler", "adl-v2/crates/adl-compiler"),
    ("adl-legacy-contracts", "adl-legacy-contracts"),
];
pub fn resources(package: &Path) -> Result<Value> {
    let pin: Value = serde_json::from_slice(&fs::read(package.join("resources-pin.json"))?)?;
    let mut files = archive::walk(&package.join("resources"))?;
    for (_, mode) in files.values_mut() {
        *mode = 0o644;
    }
    ensure!(
        archive::sha(&archive::pack(&files)?) == pin["archive_sha256"],
        "UTS pinned archive reconstruction mismatch"
    );
    let manifest: Value = serde_json::from_slice(
        &files
            .get("RESOURCE_MANIFEST.json")
            .context("resource manifest missing")?
            .0,
    )?;
    for k in ["source_revision", "resource_version", "source_repository"] {
        ensure!(manifest[k] == pin[k], "UTS resource provenance mismatch");
    }
    for (name, (bytes, _)) in &files {
        if let Some(suffix) = name.strip_prefix("schemas/uts/") {
            ensure!(
                fs::read(package.join("schemas").join(suffix))? == *bytes,
                "UTS embedded schema mirror mismatch"
            );
        }
    }
    Ok(json!({"pin":pin,"members_verified":files.len(),"archive_reconstructed":true}))
}
pub fn sandbox_profile(
    work: &Path,
    home: &Path,
    toolchain: &Path,
    cache: &Path,
    secret: &Path,
    producer: &Path,
) -> String {
    let quote = |p: &Path| serde_json::to_string(&p.to_string_lossy()).expect("path serialization");
    let mut p = "(version 1)\n(allow default)\n(deny network*)\n".to_owned();
    for denied in [
        home,
        Path::new("/Volumes/FastWork"),
        Path::new("/Volumes/home/builds"),
        producer,
        secret,
    ] {
        p += &format!("(deny file-read* (subpath {}))\n", quote(denied));
    }
    p += &format!(
        "(allow file-read* (subpath {}) (subpath {}))\n",
        quote(toolchain),
        quote(cache)
    );
    p += &format!(
        "(allow file-read* (require-all (subpath {}) (require-not (subpath {}))))\n",
        quote(work),
        quote(secret)
    );
    p += &format!("(allow file-read-metadata (subpath {}))\n", quote(home));
    for ancestor in work.ancestors().skip(1) {
        p += &format!("(allow file-read-metadata (literal {}))\n", quote(ancestor));
    }
    p
}
fn checked(command: &mut Command) -> Result<Output> {
    let o = command.output()?;
    ensure!(
        o.status.success(),
        "command failed ({}): {}",
        o.status,
        text(&o).1
    );
    Ok(o)
}
fn path(p: &Path) -> Result<&str> {
    p.to_str().context("UTF-8 path required")
}
pub fn run(output: &Path) -> Result<Value> {
    let repo = fs::canonicalize(root())?;
    let git = |args: &[&str]| -> Result<Vec<u8>> {
        Ok(checked(Command::new("git").arg("-C").arg(&repo).args(args))?.stdout)
    };
    let head = String::from_utf8(git(&["rev-parse", "HEAD"])?)?
        .trim()
        .to_owned();
    let mut status = vec!["status", "--porcelain", "--"];
    status.extend(PACKAGES.iter().map(|(_, p)| *p));
    status.extend(["tools/public_adl", "tools/public_contracts"]);
    ensure!(
        git(&status)?.is_empty(),
        "Commit package and qualification sources before candidate proof."
    );
    ensure!(
        Path::new("/usr/bin/sandbox-exec").is_file(),
        "This qualification requires macOS sandbox-exec; no unenforced fallback."
    );
    ensure!(!output.exists(), "proof output already exists");
    fs::create_dir_all(output)?;
    let output = fs::canonicalize(output)?;
    let tool = |name: &str| -> Result<PathBuf> {
        let o = checked(Command::new("rustup").args(["which", name]))?;
        Ok(fs::canonicalize(text(&o).0.trim())?)
    };
    let cargo = tool("cargo")?;
    let rustc = tool("rustc")?;
    let toolchain = cargo
        .parent()
        .and_then(Path::parent)
        .context("toolchain parent")?;
    let home = fs::canonicalize(std::env::var_os("HOME").context("HOME missing")?)?;
    let cache = home.join(".cargo/registry");
    let scratch = tempfile::Builder::new().prefix("adl-rd03-").tempdir()?;
    let work = fs::canonicalize(scratch.path())?;
    let vendor = work.join("vendor");
    fs::create_dir(&vendor)?;
    let mut versions = BTreeMap::new();
    let mut artifacts = vec![];
    for (name, source) in PACKAGES {
        let names = git(&["ls-tree", "-r", "--name-only", "-z", &head, source])?;
        let mut files = BTreeMap::new();
        for raw in names.split(|b| *b == 0).filter(|p| !p.is_empty()) {
            let member = std::str::from_utf8(raw)?;
            let relative = member
                .strip_prefix(&format!("{source}/"))
                .context("package source prefix")?;
            let mode = String::from_utf8(git(&["ls-tree", &head, "--", member])?)?;
            ensure!(
                mode.starts_with("100644 ") || mode.starts_with("100755 "),
                "package source must be regular"
            );
            let bytes = git(&["show", &format!("{head}:{member}")])?;
            files.insert(format!("{name}/{relative}"), (bytes, 0o644));
        }
        let manifest = files
            .get_mut(&format!("{name}/Cargo.toml"))
            .context("package manifest")?;
        let mut text = String::from_utf8(manifest.0.clone())?;
        let parsed: toml::Value = text.parse()?;
        let version = parsed["package"]["version"]
            .as_str()
            .context("package version")?
            .to_owned();
        if name == "adl-compiler" {
            ensure!(
                text.contains("path = \"../adl-language\""),
                "compiler sibling dependency changed"
            );
        }
        if name == "adl-legacy-contracts" {
            ensure!(
                text.contains("path = \"../adl-schema\""),
                "legacy sibling dependency changed"
            );
        }
        text.push_str("\n[workspace]\n");
        manifest.0 = text.into_bytes();
        let artifact = format!("{name}-{version}.tar");
        fs::write(output.join(&artifact), archive::pack(&files)?)?;
        let unpacked = archive::unpack(&fs::read(output.join(&artifact))?, false)?;
        let package_files = unpacked
            .into_iter()
            .map(|(p, b)| (p.strip_prefix(&format!("{name}/")).unwrap().to_owned(), b))
            .collect();
        archive::extract(&package_files, &vendor.join(name))?;
        artifacts.push(json!({"package":name,"version":version,"source":source,"artifact":artifact,"sha256":archive::digest(&output.join(&artifact))?,"manifest_normalization":"standalone workspace; existing sibling public dependencies retained"}));
        versions.insert(name, version);
    }
    let uts = resources(&vendor.join("adl-uts"))?;
    let consumer = work.join("consumer");
    fs::create_dir_all(consumer.join("src"))?;
    let dependencies = versions
        .iter()
        .map(|(name, version)| {
            format!("{name} = {{ version = \"={version}\", path = \"../vendor/{name}\" }}")
        })
        .collect::<Vec<_>>()
        .join("\n");
    let manifest=format!("[package]\nname=\"rd03-independent-consumer\"\nversion=\"0.0.0\"\nedition=\"2021\"\n[dependencies]\nserde_json=\"1\"\njsonschema={{version=\"0.17\",default-features=false}}\n{dependencies}\n");
    fs::write(consumer.join("Cargo.toml"), &manifest)?;
    fs::copy(
        vendor.join("adl-language/fixtures/six-primitives.adl.yaml"),
        consumer.join("fixture.yaml"),
    )?;
    fs::write(
        consumer.join("src/main.rs"),
        git(&["show", &format!("{head}:tools/public_adl/consumer.rs")])?,
    )?;
    let isolated_home = work.join("home");
    let cargo_home = work.join("cargo-home");
    fs::create_dir(&isolated_home)?;
    fs::create_dir(&cargo_home)?;
    #[cfg(unix)]
    std::os::unix::fs::symlink(&cache, cargo_home.join("registry"))?;
    let secret = work.join("private-credential");
    fs::write(&secret, "synthetic-private-credential-denial-probe\n")?;
    let sandbox = work.join("consumer.sb");
    fs::write(
        &sandbox,
        sandbox_profile(&work, &home, toolchain, &cache, &secret, &repo),
    )?;
    // Copy the native network probe into the only readable scratch tree.
    let probe = work.join("network-probe");
    fs::copy(std::env::current_exe()?, &probe)?;
    let isolated = |args: &[&str], check: bool| -> Result<Output> {
        let mut c = Command::new("/usr/bin/sandbox-exec");
        c.arg("-f")
            .arg(&sandbox)
            .args(args)
            .current_dir(&consumer)
            .env_clear()
            .env("HOME", &isolated_home)
            .env("CARGO_HOME", &cargo_home)
            .env("RUSTC", &rustc)
            .env(
                "PATH",
                format!(
                    "{}:/usr/bin:/bin",
                    path(cargo.parent().context("cargo parent")?)?
                ),
            )
            .env("TMPDIR", &work)
            .env("CARGO_TARGET_DIR", work.join("target"))
            .env("CARGO_NET_OFFLINE", "true");
        if check {
            checked(&mut c)
        } else {
            Ok(c.output()?)
        }
    };
    let denied = |name: &str, args: &[&str], expected: &[&str]| -> Result<Value> {
        let o = isolated(args, false)?;
        let stderr = text(&o).1.to_lowercase();
        ensure!(
            !o.status.success() && expected.iter().any(|s| stderr.contains(s)),
            "{name}: required classified rejection absent: {stderr}"
        );
        Ok(json!({"case":name,"status":"rejected","exit_code":o.status.code().unwrap_or(-1)}))
    };
    let permission = ["operation not permitted", "permission denied"];
    isolated(&["/bin/cat", path(&consumer.join("fixture.yaml"))?], true)?;
    let mut probes = vec![
        denied(
            "private-credential-denied",
            &["/bin/cat", path(&secret)?],
            &permission,
        )?,
        denied(
            "producer-source-denied",
            &["/bin/cat", path(&repo.join("adl-schema/Cargo.toml"))?],
            &permission,
        )?,
        denied(
            "network-denied",
            &[path(&probe)?, "network-probe"],
            &permission,
        )?,
    ];
    isolated(&[path(&cargo)?, "generate-lockfile", "--offline"], true)?;
    fs::copy(
        consumer.join("Cargo.lock"),
        output.join("consumer.Cargo.lock"),
    )?;
    let toolchain_versions = json!({"cargo":text(&isolated(&[path(&cargo)?,"--version"],true)?).0.trim(),"rustc":text(&isolated(&[path(&rustc)?,"--version"],true)?).0.trim()});
    let consumer_report: Value = serde_json::from_slice(
        &isolated(
            &[path(&cargo)?, "run", "--offline", "--locked", "--quiet"],
            true,
        )?
        .stdout,
    )?;
    ensure!(
        consumer_report["passed"] == true
            && consumer_report["executed_cases"].as_u64() == Some(8)
            && consumer_report["expected_cases"].as_u64() == Some(8),
        "consumer case denominator mismatch"
    );
    let uts_cases = consumer_report["uts_cases"]
        .as_array()
        .context("UTS cases")?;
    ensure!(
        consumer_report["uts_fixture_count"].as_u64() == Some(uts_cases.len() as u64)
            && uts_cases.iter().all(|c| c["passed"] == true),
        "UTS cases failed"
    );
    let rustc_info = text(&isolated(&[path(&rustc)?, "-vV"], true)?).0;
    let host = rustc_info
        .lines()
        .find_map(|s| s.strip_prefix("host: "))
        .context("host")?;
    let metadata: Value = serde_json::from_slice(
        &isolated(
            &[
                path(&cargo)?,
                "metadata",
                "--offline",
                "--locked",
                "--filter-platform",
                host,
                "--format-version",
                "1",
            ],
            true,
        )?
        .stdout,
    )?;
    for (name, _) in PACKAGES {
        let p = metadata["packages"]
            .as_array()
            .context("metadata packages")?
            .iter()
            .find(|p| p["name"] == name)
            .context("consumed package absent")?;
        ensure!(
            fs::canonicalize(p["manifest_path"].as_str().context("manifest path")?)?
                == vendor.join(name).join("Cargo.toml"),
            "producer source entered consumer"
        );
    }
    fs::write(
        consumer.join("Cargo.toml"),
        manifest.replace(
            "path = \"../vendor/adl-schema\"",
            &format!("path = \"{}/adl-schema\"", path(&repo)?),
        ),
    )?;
    probes.push(denied(
        "sibling-producer-dependency-denied",
        &[
            path(&cargo)?,
            "metadata",
            "--offline",
            "--format-version",
            "1",
        ],
        &permission,
    )?);
    fs::write(
        consumer.join("Cargo.toml"),
        manifest.replace(
            &format!("adl-schema = {{ version = \"={}\"", versions["adl-schema"]),
            "adl-schema = { version = \"=99.0.0\"",
        ),
    )?;
    probes.push(denied(
        "incompatible-package-version",
        &[
            path(&cargo)?,
            "metadata",
            "--offline",
            "--format-version",
            "1",
        ],
        &["failed to select a version"],
    )?);
    let report = json!({"schema":"adl.public.install-proof.v1","source_revision":head,"packages":artifacts,"consumer":consumer_report,"uts_resources":uts,"dependency_lock":{"path":"consumer.Cargo.lock","sha256":archive::digest(&output.join("consumer.Cargo.lock"))?},"toolchain_versions":toolchain_versions,"sandbox":"macOS sandbox-exec: network, home and producer source denied; toolchain/public registry allowed","negative_cases":probes,"negative_expected":5,"negative_executed":5,"public_packages_consumed":5,"execution_authority_claimed":false,"registry_publication_claimed":false});
    fs::write(
        output.join("report.json"),
        format!("{}\n", serde_json::to_string_pretty(&report)?),
    )?;
    Ok(report)
}
