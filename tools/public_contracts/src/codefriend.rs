//! Receipt selection and literal argv transport, synthetic fixture only.
use super::*;
use anyhow::ensure;
pub struct Fixture {
    pub _temp: tempfile::TempDir,
    pub root: PathBuf,
    pub executable: PathBuf,
    pub receipt: Value,
    pub manifest: Value,
}
impl Fixture {
    pub fn new() -> Result<Self> {
        let tmp = tempfile::tempdir()?;
        let root = fs::canonicalize(tmp.path())?;
        fs::create_dir(root.join("bin"))?;
        let executable_path = root.join("bin/codefriend");
        executable(
            &executable_path,
            "#!/bin/sh\nprintf '%s\\n' \"$@\"\nexit 17\n",
        )?;
        let receipt = json!({"schema":"codefriend.product_installation.v1","authentication":"external_owner_pins_not_self_authorization","archive_sha256":"a".repeat(64),"candidate":"b".repeat(40),"run_id":"1","run_attempt":"1","workflow_ref":"agent-logic/codefriend/.github/workflows/product-ci.yaml@refs/heads/main","workflow_sha":"c".repeat(40),"job_name":"installed-product"});
        let manifest = json!({"schema":"codefriend.product_distribution.v1","source_revision":"b".repeat(40),"source_clean":true,"builder_context":{"GITHUB_REPOSITORY":"agent-logic/codefriend","GITHUB_RUN_ID":"1","GITHUB_RUN_ATTEMPT":"1","GITHUB_WORKFLOW_REF":receipt["workflow_ref"],"GITHUB_WORKFLOW_SHA":"c".repeat(40),"GITHUB_JOB":"installed-product"},"resources":[{"path":"bin/codefriend","mode":493,"sha256":"d".repeat(64)}]});
        let f = Self {
            _temp: tmp,
            root,
            executable: executable_path,
            receipt,
            manifest,
        };
        f.write()?;
        Ok(f)
    }
    pub fn write(&self) -> Result<()> {
        fs::write(
            self.root.join("INSTALLATION.json"),
            self.receipt.to_string(),
        )?;
        fs::write(self.root.join("manifest.json"), self.manifest.to_string())?;
        Ok(())
    }
    pub fn call(
        &self,
        binary: &Path,
        args: &[&str],
        archive: &str,
        candidate: &str,
        facade: bool,
    ) -> Result<Output> {
        let mut e = env();
        e.insert("ADL_OBSERVABILITY".into(), "0".into());
        let mut argv = vec!["codefriend"];
        if facade {
            e.insert(
                "ADL_CODEFRIEND_INSTALL_PREFIX".into(),
                self.root.to_string_lossy().into_owned(),
            );
            e.insert("ADL_CODEFRIEND_ARCHIVE_SHA256".into(), archive.into());
            e.insert("ADL_CODEFRIEND_CANDIDATE".into(), candidate.into());
        } else {
            argv.extend([self.root.to_str().context("prefix")?, archive, candidate]);
        }
        argv.extend(args);
        invoke(binary, &self.root, &argv, &e)
    }
}
pub fn run(binary: &Path, facade: bool) -> Result<Value> {
    let archive = "a".repeat(64);
    let candidate = "b".repeat(40);
    let refused = |o: Output| -> Result<()> {
        ensure!(
            o.status.code() == Some(if facade { 1 } else { 2 }) && o.stdout.is_empty(),
            "CodeFriend refusal: {o:?}"
        );
        Ok(())
    };
    let f = Fixture::new()?;
    let o = f.call(
        binary,
        &["ingest", "space ; $(literal)"],
        &archive,
        &candidate,
        facade,
    )?;
    ensure!(o.status.code() == Some(17) && text(&o).0 == "ingest\nspace ; $(literal)\n");
    let mut f = Fixture::new()?;
    refused(f.call(binary, &[], &"e".repeat(64), &candidate, facade)?)?;
    refused(f.call(binary, &[], &archive, &"f".repeat(40), facade)?)?;
    f.manifest["builder_context"]["GITHUB_REPOSITORY"] = json!("foreign/repo");
    f.write()?;
    refused(f.call(binary, &[], &archive, &candidate, facade)?)?;
    let mut f = Fixture::new()?;
    f.receipt["run_id"] = json!("2");
    f.write()?;
    refused(f.call(binary, &[], &archive, &candidate, facade)?)?;
    let mut f = Fixture::new()?;
    let row = f.manifest["resources"][0].clone();
    f.manifest["resources"].as_array_mut().unwrap().push(row);
    f.write()?;
    refused(f.call(binary, &[], &archive, &candidate, facade)?)?;
    f.manifest["resources"] = json!([]);
    f.write()?;
    refused(f.call(binary, &[], &archive, &candidate, facade)?)?;
    let f = Fixture::new()?;
    let target = f.root.join("actual");
    fs::rename(&f.executable, &target)?;
    #[cfg(unix)]
    std::os::unix::fs::symlink(&target, &f.executable)?;
    refused(f.call(binary, &[], &archive, &candidate, facade)?)?;
    fs::remove_file(&f.executable)?;
    fs::rename(target, &f.executable)?;
    fs::remove_file(f.root.join("INSTALLATION.json"))?;
    refused(f.call(binary, &[], &archive, &candidate, facade)?)?;
    Ok(
        json!({"status":"passed","cases":5,"scope":"synthetic owner receipt selection, not qualification"}),
    )
}
