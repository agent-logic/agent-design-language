//! Frozen source custody. Verification grants no producer acceptance.
use super::*;
use anyhow::ensure;
use archive::{gzip, pack, sha, unpack};
use std::io::Write;
pub struct Distribution {
    pub manifest: Value,
    pub lock: Value,
    pub manifest_bytes: Vec<u8>,
    pub godel: bool,
    pub directory: PathBuf,
}
impl Distribution {
    pub fn load(godel: bool) -> Result<Self> {
        let directory = root().join(if godel {
            "tools/godel_contract_distribution"
        } else {
            "tools/remote_validation_distribution"
        });
        let manifest_bytes = fs::read(directory.join("source-manifest.json"))?;
        let manifest: Value = serde_json::from_slice(&manifest_bytes)?;
        let lock = if godel {
            serde_json::from_slice(&fs::read(directory.join("artifact-lock.json"))?)?
        } else {
            manifest.clone()
        };
        let d = Self {
            manifest,
            lock,
            manifest_bytes,
            godel,
            directory,
        };
        d.validate_manifest()?;
        Ok(d)
    }
    fn validate_manifest(&self) -> Result<()> {
        if self.godel {
            ensure!(
                sha(&self.manifest_bytes) == self.lock["manifest_sha256"],
                "manifest digest mismatch"
            );
            for k in ["package", "version", "source_revision"] {
                ensure!(
                    self.manifest[k] == self.lock[k],
                    "distribution identity mismatch"
                );
            }
        }
        Ok(())
    }
    pub fn expected(&self) -> Result<BTreeMap<String, Value>> {
        self.validate_manifest()?;
        let mut rows = BTreeMap::new();
        for row in self.manifest["files"]
            .as_array()
            .context("manifest files")?
        {
            let name = if self.godel {
                row["path"].as_str().context("path")?.to_owned()
            } else {
                format!(
                    "adl-remote-validation-0.92.1/{}",
                    row["source_path"]
                        .as_str()
                        .context("source path")?
                        .strip_prefix("tools/remote_validation/")
                        .unwrap_or(row["source_path"].as_str().unwrap())
                )
            };
            ensure!(
                archive::safe_name(&name) && !rows.contains_key(&name),
                "duplicate or unsafe manifest path"
            );
            rows.insert(name, row.clone());
        }
        if self.godel {
            rows.insert(
                "source-manifest.json".into(),
                json!({"bytes":self.manifest_bytes.len(),"sha256":sha(&self.manifest_bytes)}),
            );
        }
        Ok(rows)
    }
    pub fn inspect(&self, data: &[u8]) -> Result<BTreeMap<String, (Vec<u8>, u32)>> {
        let rows = self.expected()?;
        let files = unpack(data, true)?;
        ensure!(
            rows.keys().eq(files.keys()),
            "archive inventory incomplete or unexpected"
        );
        for (name, (bytes, mode)) in &files {
            let row = &rows[name];
            ensure!(
                Some(bytes.len() as u64) == row["bytes"].as_u64() && sha(bytes) == row["sha256"],
                "payload digest or size mismatch"
            );
            ensure!(!self.godel || *mode == 0o644, "invalid archive mode");
        }
        if !self.godel {
            let package: toml::Value =
                std::str::from_utf8(&files["adl-remote-validation-0.92.1/Cargo.toml"].0)?
                    .parse()?;
            ensure!(
                package["package"]["name"].as_str() == self.manifest["package"].as_str()
                    && package["package"]["version"].as_str() == self.manifest["version"].as_str(),
                "package identity mismatch"
            );
        }
        Ok(files)
    }
    pub fn artifact(&self) -> PathBuf {
        self.directory.join(
            self.lock["artifact"]["name"]
                .as_str()
                .expect("artifact name"),
        )
    }
    pub fn verify(&self, path: &Path, output: Option<&Path>) -> Result<Value> {
        let bytes = fs::read(path)?;
        ensure!(
            Some(bytes.len() as u64) == self.lock["artifact"]["bytes"].as_u64()
                && sha(&bytes) == self.lock["artifact"]["sha256"],
            "artifact digest or size mismatch"
        );
        let files = self.inspect(&bytes)?;
        if let Some(dest) = output {
            archive::extract(&files, dest)?;
            if self.godel {
                self.verify_root(dest)?;
            }
        }
        Ok(self.result())
    }
    pub fn verify_root(&self, root: &Path) -> Result<Value> {
        ensure!(
            self.godel,
            "installed root verification applies to Godel distribution"
        );
        archive::real_root(root)?;
        let rows = self.expected()?;
        let files = archive::walk(root)?;
        ensure!(
            rows.keys().eq(files.keys()),
            "missing or unexpected installed input"
        );
        for (name, (bytes, _)) in files {
            ensure!(
                Some(bytes.len() as u64) == rows[&name]["bytes"].as_u64()
                    && sha(&bytes) == rows[&name]["sha256"],
                "installed input changed"
            );
        }
        Ok(self.result())
    }
    fn result(&self) -> Value {
        if self.godel {
            json!({"verified_distribution":self.lock,"producer_acceptance_implied":false})
        } else {
            json!({"verified_source_artifact":self.lock["artifact"],"producer_acceptance_implied":false})
        }
    }
    pub fn export(&self, repo: &Path, output: &Path) -> Result<Value> {
        let rev = self.manifest["source_revision"]
            .as_str()
            .context("source revision")?;
        let git = |args: &[&str]| -> Result<Vec<u8>> {
            let o = Command::new("git")
                .arg("-C")
                .arg(repo)
                .args(args)
                .output()?;
            ensure!(o.status.success(), "frozen source Git read failed");
            Ok(o.stdout)
        };
        if !self.godel {
            let bytes = git(&[
                "ls-tree",
                "-r",
                "--name-only",
                rev,
                "tools/remote_validation/",
            ])?;
            let mut observed: Vec<_> = std::str::from_utf8(&bytes)?.lines().collect();
            observed.sort();
            let mut expected: Vec<_> = self.manifest["files"]
                .as_array()
                .context("files")?
                .iter()
                .filter_map(|r| r["source_path"].as_str())
                .filter(|p| *p != "LICENSE")
                .collect();
            expected.sort();
            ensure!(observed == expected, "source inventory mismatch");
        }
        let mut files = BTreeMap::new();
        for (name, row) in self.expected()? {
            if self.godel && name == "source-manifest.json" {
                files.insert(name, (self.manifest_bytes.clone(), 0o644));
                continue;
            }
            let source = if self.godel {
                &row["path"]
            } else {
                &row["source_path"]
            };
            let spec = format!("{rev}:{}", source.as_str().context("source path")?);
            let bytes = git(&["show", &spec])?;
            let blob = String::from_utf8(git(&["rev-parse", &spec])?)?;
            ensure!(
                blob.trim() == row["git_blob"]
                    && sha(&bytes) == row["sha256"]
                    && Some(bytes.len() as u64) == row["bytes"].as_u64(),
                "frozen source provenance mismatch"
            );
            files.insert(name, (bytes, 0o644));
        }
        let data = gzip(&pack(&files)?)?;
        ensure!(
            sha(&data) == self.lock["artifact"]["sha256"],
            "reproduction digest mismatch"
        );
        self.inspect(&data)?;
        if self.godel {
            fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(output)?
                .write_all(&data)?;
        } else {
            ensure!(!output.exists(), "export output exists");
            fs::create_dir_all(output)?;
            fs::write(
                output.join(
                    self.lock["artifact"]["name"]
                        .as_str()
                        .context("artifact name")?,
                ),
                data,
            )?;
            fs::write(output.join("source-manifest.json"), &self.manifest_bytes)?;
        }
        Ok(self.result())
    }
}

pub fn consumer(output: &Path) -> Result<Value> {
    ensure!(!output.exists(), "consumer output exists");
    fs::create_dir_all(output)?;
    let output = fs::canonicalize(output)?;
    let d = Distribution::load(false)?;
    d.verify(&d.artifact(), Some(&output.join("dependencies")))?;
    let consumer = output.join("consumer");
    fs::create_dir_all(consumer.join("src"))?;
    fs::copy(
        d.directory.join("consumer.rs"),
        consumer.join("src/main.rs"),
    )?;
    fs::copy(
        d.directory.join("consumer.Cargo.lock"),
        consumer.join("Cargo.lock"),
    )?;
    fs::write(consumer.join("Cargo.toml"),"[package]\nname=\"remote-validation-artifact-consumer\"\nversion=\"0.1.0\"\nedition=\"2021\"\npublish=false\n[dependencies]\nadl-remote-validation={version=\"=0.92.1\",path=\"../dependencies/adl-remote-validation-0.92.1\"}\n")?;
    let call = |args: &[&str]| -> Result<Output> {
        Ok(Command::new("cargo")
            .args(args)
            .current_dir(&consumer)
            .env("CARGO_TARGET_DIR", output.join("target"))
            .output()?)
    };
    let run = call(&["run", "--offline", "--locked"])?;
    fs::write(
        output.join("command-0.log"),
        [run.stdout.as_slice(), run.stderr.as_slice()].concat(),
    )?;
    ensure!(
        run.status.success(),
        "consumer command 0 failed; see retained log"
    );
    let meta = call(&["metadata", "--offline", "--locked", "--format-version", "1"])?;
    ensure!(meta.status.success(), "consumer metadata failed");
    let meta: Value = serde_json::from_slice(&meta.stdout)?;
    let local: Vec<_> = meta["packages"]
        .as_array()
        .context("packages")?
        .iter()
        .filter(|p| p["source"].is_null())
        .collect();
    ensure!(
        local
            .iter()
            .filter_map(|p| p["name"].as_str())
            .collect::<std::collections::BTreeSet<_>>()
            == std::collections::BTreeSet::from([
                "remote-validation-artifact-consumer",
                "adl-remote-validation"
            ]),
        "unexpected local package dependency"
    );
    for p in local {
        ensure!(
            fs::canonicalize(p["manifest_path"].as_str().context("manifest path")?)?
                .starts_with(&output),
            "consumer reached outside the extracted artifact"
        );
    }
    let rustc = Command::new("rustc").arg("--version").output()?;
    ensure!(rustc.status.success(), "rustc version failed");
    let record = json!({"status":"passed","artifact":d.manifest["artifact"],"source_revision":d.manifest["source_revision"],"consumer_lock_sha256":archive::digest(&consumer.join("Cargo.lock"))?,"rustc":text(&rustc).0.trim(),"scope":"offline local artifact consumer; no OS sandbox or cloud/installed qualification claim"});
    fs::write(
        output.join("result.json"),
        format!("{}\n", serde_json::to_string_pretty(&record)?),
    )?;
    Ok(record)
}
