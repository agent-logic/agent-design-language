//! Durable exact-HEAD proof validation, including failure evidence.
use super::*;
use anyhow::ensure;
use std::collections::BTreeSet;
fn array<'a>(v: &'a Value, k: &str) -> Result<&'a Vec<Value>> {
    v[k].as_array().with_context(|| format!("missing {k}"))
}
pub fn validate(report: &Value, proof: &Path, repo: &Path, head: &str) -> Result<Value> {
    ensure!(
        report["schema"] == "adl.public.install-proof.v1" && report["source_revision"] == head,
        "install proof is not bound to current HEAD/schema"
    );
    let consumer = &report["consumer"];
    ensure!(
        consumer["passed"] == true
            && consumer["executed_cases"].as_u64() == Some(8)
            && consumer["expected_cases"].as_u64() == Some(8),
        "consumer scenario denominator did not pass"
    );
    ensure!(
        report["negative_executed"].as_u64() == Some(5)
            && report["negative_expected"].as_u64() == Some(5),
        "negative scenario denominator did not pass"
    );
    let packages = array(report, "packages")?;
    ensure!(
        packages.len() == 5
            && packages
                .iter()
                .filter_map(|p| p["package"].as_str())
                .collect::<BTreeSet<_>>()
                == install::PACKAGES.iter().map(|(n, _)| *n).collect(),
        "the five public package artifacts are required"
    );
    ensure!(
        report["public_packages_consumed"].as_u64() == Some(5),
        "consumer did not consume five public packages"
    );
    let mut seen = BTreeSet::new();
    let mut hashes = serde_json::Map::new();
    for p in packages {
        let name = p["artifact"].as_str().context("artifact name")?;
        ensure!(
            archive::safe_name(name)
                && Path::new(name).components().count() == 1
                && seen.insert(name),
            "invalid or duplicate artifact name"
        );
        let artifact = proof.join(name);
        ensure!(
            artifact.is_file()
                && !artifact.is_symlink()
                && archive::digest(&artifact)? == p["sha256"],
            "package artifact hash mismatch"
        );
        hashes.insert(
            p["package"].as_str().unwrap().to_owned(),
            p["sha256"].clone(),
        );
    }
    let lock = &report["dependency_lock"];
    ensure!(
        lock["path"] == "consumer.Cargo.lock",
        "consumer dependency lock is missing"
    );
    let lock_path = proof.join("consumer.Cargo.lock");
    ensure!(
        lock_path.is_file()
            && !lock_path.is_symlink()
            && archive::digest(&lock_path)? == lock["sha256"],
        "consumer dependency lock hash mismatch"
    );
    let cases = array(consumer, "cases")?;
    ensure!(
        cases.len() == 8
            && cases
                .iter()
                .filter_map(Value::as_str)
                .collect::<BTreeSet<_>>()
                == BTreeSet::from([
                    "language-compile-determinism",
                    "legacy-resolution",
                    "provider-dto-roundtrip",
                    "declaration-endpoint-denials",
                    "uts-three-versioned-schemas",
                    "unsupported-uts-version-denied",
                    "uts-inert-conformance",
                    "uts-canonical-conformance"
                ]),
        "required consumer cases are missing or duplicated"
    );
    let uts = array(consumer, "uts_cases")?;
    ensure!(
        consumer["uts_fixture_count"].as_u64() == Some(21)
            && uts.len() == 21
            && uts
                .iter()
                .filter_map(|c| c["id"].as_str().filter(|s| !s.is_empty()))
                .collect::<BTreeSet<_>>()
                .len()
                == 21
            && uts
                .iter()
                .all(|c| c["passed"] == true && c["execution_granted"] == false),
        "21 distinct inert UTS fixtures must pass"
    );
    let pin: Value = serde_json::from_slice(&fs::read(repo.join("adl-uts/resources-pin.json"))?)?;
    let custody = &report["uts_resources"];
    ensure!(
        custody["pin"] == pin
            && custody["members_verified"].as_u64() == Some(33)
            && custody["archive_reconstructed"] == true,
        "installed UTS resource custody missing"
    );
    let manifest: Value = serde_json::from_slice(&fs::read(
        repo.join("adl-uts/resources/conformance/manifest.json"),
    )?)?;
    let expected: BTreeMap<_, _> = array(&manifest, "fixtures")?
        .iter()
        .map(|f| (f["path"].as_str().unwrap_or(""), f))
        .collect();
    let canonical = &consumer["uts_canonical"];
    let entries = array(canonical, "cases")?;
    ensure!(
        canonical["schema"] == "adl.uts.canonical-conformance.v1"
            && canonical["resource_pin"] == pin
            && canonical["fixture_count"].as_u64() == Some(23)
            && canonical["passed"] == true
            && entries.len() == 23
            && entries
                .iter()
                .filter_map(|c| c["path"].as_str())
                .collect::<BTreeSet<_>>()
                == expected.keys().copied().collect(),
        "canonical UTS corpus or resource identity mismatch"
    );
    for c in entries {
        let f = expected[c["path"].as_str().unwrap()];
        let valid = f["expected"] != "invalid";
        let declaration = f["schema"] != "tool_invocation.v1.1";
        ensure!(
            c["schema"] == f["schema"]
                && c["expected"] == f["expected"]
                && c["schema_valid"] == json!(valid)
                && c["passed"] == true
                && c["execution_granted"] == false
                && c["binding_valid"]
                    == if declaration {
                        json!(valid)
                    } else {
                        Value::Null
                    }
                && c["normalized_valid"]
                    == if declaration && valid {
                        json!(true)
                    } else {
                        Value::Null
                    },
            "canonical UTS case validation mismatch"
        );
    }
    let negatives = array(report, "negative_cases")?;
    ensure!(
        negatives.len() == 5
            && negatives
                .iter()
                .filter_map(|c| c["case"].as_str())
                .collect::<BTreeSet<_>>()
                == BTreeSet::from([
                    "private-credential-denied",
                    "producer-source-denied",
                    "network-denied",
                    "sibling-producer-dependency-denied",
                    "incompatible-package-version"
                ])
            && negatives
                .iter()
                .all(|c| c["status"] == "rejected"
                    && c["exit_code"].as_i64().is_some_and(|n| n != 0)),
        "all five classified negative cases must reject"
    );
    Ok(
        json!({"public_packages_consumed":5,"uts_fixtures_executed":21,"uts_canonical_fixtures_executed":23,"artifact_hashes":hashes,"dependency_lock_sha256":lock["sha256"]}),
    )
}
pub fn run_with<F>(
    repo: &Path,
    override_root: Option<&Path>,
    issue: u64,
    execute: F,
) -> Result<Value>
where
    F: FnOnce(&Path) -> Result<Output>,
{
    ensure!(issue > 0, "issue must be positive");
    let root = override_root
        .map(Path::to_path_buf)
        .unwrap_or_else(|| repo.join("target/public-adl-proof"))
        .join(issue.to_string());
    fs::create_dir_all(&root)?;
    let dir = tempfile::Builder::new()
        .prefix("run-")
        .tempdir_in(&root)?
        .keep();
    let dir = fs::canonicalize(dir)?;
    let proof = dir.join("qualification");
    let report_path = proof.join("report.json");
    let mut result = json!({"schema":"adl.public.install-validator.v1","status":"failed","issue":issue,"exit_code":2,"evidence_directory":dir,"report_path":report_path,"report_sha256":null,"source_revision":null,"consumer_cases_executed":0,"negative_cases_executed":0});
    let attempt = (|| -> Result<()> {
        let completed = execute(&proof)?;
        fs::write(dir.join("stdout.txt"), &completed.stdout)?;
        fs::write(dir.join("stderr.txt"), &completed.stderr)?;
        result["exit_code"] = completed.status.code().unwrap_or(-1).into();
        ensure!(completed.status.success(), "verifier failed");
        ensure!(
            report_path.is_file() && !report_path.is_symlink(),
            "verifier exited zero without regular report.json"
        );
        let report = compatibility::load(&report_path)?;
        let head = Command::new("git")
            .arg("-C")
            .arg(repo)
            .args(["rev-parse", "HEAD"])
            .output()?;
        ensure!(
            head.status.success(),
            "unable to resolve candidate revision"
        );
        let fields = validate(&report, &proof, repo, text(&head).0.trim())?;
        result
            .as_object_mut()
            .unwrap()
            .extend(fields.as_object().unwrap().clone());
        result["status"] = "passed".into();
        result["exit_code"] = 0.into();
        result["report_sha256"] = archive::digest(&report_path)?.into();
        result["source_revision"] = report["source_revision"].clone();
        result["consumer_cases_executed"] = 8.into();
        result["negative_cases_executed"] = 5.into();
        Ok(())
    })();
    if let Err(e) = attempt {
        result["validation_error"] = e.to_string().into();
        if result["exit_code"] == 0 {
            result["exit_code"] = 2.into();
        }
    }
    fs::write(
        dir.join("validator-result.json"),
        format!("{}\n", serde_json::to_string_pretty(&result)?),
    )?;
    Ok(result)
}
pub fn run(issue: u64) -> Result<Value> {
    let override_root = std::env::var_os("ADL_PUBLIC_ADL_PROOF_ROOT").map(PathBuf::from);
    run_with(&root(), override_root.as_deref(), issue, |proof| {
        Ok(Command::new(std::env::current_exe()?)
            .arg("install-proof")
            .arg("--output")
            .arg(proof)
            .current_dir(root())
            .output()?)
    })
}
