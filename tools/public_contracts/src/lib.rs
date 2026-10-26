pub mod adapter_evidence;
pub mod archive;
pub mod codefriend;
pub mod compatibility;
pub mod distribution;
pub mod document_packets;
pub mod install;
pub mod install_validator;
pub mod merge_packet;
pub mod packet_truth;
pub mod planning_template;
pub mod portable;
pub mod prompt_schemas;
pub mod public_docs;
pub mod retained_packets;
pub mod runtime_owner;
pub mod session;
pub mod tool_registry;
pub mod transpiler;
pub mod uts_install;
pub mod warm_cache;
// Native public consumer contracts. Synthetic owner fixtures never imply qualification.
use anyhow::{bail, Context, Result};
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};
pub mod dispatch;
pub fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
pub fn env() -> BTreeMap<String, String> {
    std::env::vars()
        .filter(|(k, _)| !k.starts_with("ADL_"))
        .collect()
}
pub fn text(o: &Output) -> (String, String) {
    (
        String::from_utf8_lossy(&o.stdout).into_owned(),
        String::from_utf8_lossy(&o.stderr).into_owned(),
    )
}
pub fn invoke(
    binary: &Path,
    cwd: &Path,
    args: &[&str],
    env: &BTreeMap<String, String>,
) -> Result<Output> {
    Command::new(binary)
        .args(args)
        .current_dir(cwd)
        .env_clear()
        .envs(env)
        .output()
        .context("invoke public CLI")
}
pub fn executable(path: &Path, text: &str) -> Result<()> {
    fs::write(path, text)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o755))?;
    }
    Ok(())
}
pub fn host() -> Result<String> {
    let o = Command::new("rustc").arg("-vV").output()?;
    if !o.status.success() {
        bail!("rustc host observation failed");
    }
    String::from_utf8(o.stdout)?
        .lines()
        .find_map(|s| s.strip_prefix("host: ").map(str::to_owned))
        .context("host target missing")
}
pub struct RuntimeFixture {
    pub _temp: tempfile::TempDir,
    pub root: PathBuf,
    pub prefix: PathBuf,
    pub package: PathBuf,
    pub executable: PathBuf,
    pub identity: String,
    pub manifest: Value,
}
impl RuntimeFixture {
    pub fn new() -> Result<Self> {
        let tmp = tempfile::tempdir()?;
        let root = fs::canonicalize(tmp.path())?;
        let prefix = root.join("installed");
        let identity = "a".repeat(64);
        let package = prefix.join("versions").join(&identity);
        fs::create_dir_all(package.join("bin"))?;
        let executable_path = package.join("bin/adl-runtime");
        executable(
            &executable_path,
            "#!/bin/sh\nprintf '%s\\n' \"$@\"\nprintf 'fixture-stderr\\n' >&2\nexit 17\n",
        )?;
        fs::write(package.join("archive.sha256"), format!("{identity}\n"))?;
        fs::write(
            package.join("archive.tar"),
            b"synthetic unqualified test receipt",
        )?;
        let manifest = json!({"schema":"runtime.local-package.v1","target":host()?,"files":{"bin/adl-runtime":{"mode":493,"sha256":"b".repeat(64)}}});
        #[cfg(unix)]
        std::os::unix::fs::symlink(format!("versions/{identity}"), prefix.join("current"))?;
        let f = Self {
            _temp: tmp,
            root,
            prefix,
            package,
            executable: executable_path,
            identity,
            manifest,
        };
        f.write()?;
        Ok(f)
    }
    pub fn write(&self) -> Result<()> {
        fs::write(
            self.package.join("manifest.json"),
            self.manifest.to_string(),
        )?;
        Ok(())
    }
    pub fn selected(&self) -> BTreeMap<String, String> {
        let mut e = env();
        e.insert(
            "ADL_RUNTIME_INSTALL_PREFIX".into(),
            self.prefix.to_string_lossy().into_owned(),
        );
        e.insert("ADL_RUNTIME_ARCHIVE_SHA256".into(), self.identity.clone());
        e
    }
}
