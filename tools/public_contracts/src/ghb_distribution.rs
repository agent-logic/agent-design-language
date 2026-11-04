//! Versioned GHB data transport. A caller-supplied digest is a trust input,
//! never a producer acceptance or Runtime execution receipt.
use super::*;
use crate::archive::{self, sha};
use anyhow::ensure;
use serde::{Deserialize, Serialize};
use std::io::Read;

const MAX_BYTES: u64 = 8 * 1024 * 1024;
const SCHEMA: &str = "adl.ghb.contract-distribution.v1";
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Entry {
    path: String,
    bytes: u64,
    sha256: String,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    schema: String,
    package: String,
    version: String,
    source_repository: String,
    source_revision: String,
    files: Vec<Entry>,
}
fn valid_sha(s: &str, n: usize) -> bool {
    s.len() == n
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn admitted_path(s: &str) -> bool {
    archive::safe_name(s)
        && (s == "LICENSE"
            || ((s.starts_with("adl-spec/schemas/ghb/v1/")
                || s.starts_with("adl-spec/examples/ghb/v1/"))
                && s.ends_with(".json")))
}
fn inspect(files: &BTreeMap<String, (Vec<u8>, u32)>, pin: &str) -> Result<Manifest> {
    ensure!(valid_sha(pin, 64), "expected manifest SHA256 required");
    let raw = &files
        .get("source-manifest.json")
        .context("manifest missing")?
        .0;
    ensure!(sha(raw) == pin, "manifest digest mismatch");
    let m: Manifest = serde_json::from_slice(raw)?;
    ensure!(
        m.schema == SCHEMA
            && m.package == "adl-ghb-contracts"
            && m.version == "1.0.0"
            && m.source_repository == "agent-logic/agent-design-language"
            && valid_sha(&m.source_revision, 40),
        "distribution identity mismatch"
    );
    ensure!(
        !m.files.is_empty() && m.files.len() <= 64,
        "invalid inventory size"
    );
    let mut names = std::collections::BTreeSet::new();
    names.insert("source-manifest.json".to_owned());
    for e in &m.files {
        ensure!(
            admitted_path(&e.path) && names.insert(e.path.clone()) && valid_sha(&e.sha256, 64),
            "unsafe/duplicate input"
        );
        let (bytes, mode) = files.get(&e.path).context("payload missing")?;
        ensure!(
            *mode == 0o644 && bytes.len() as u64 == e.bytes && sha(bytes) == e.sha256,
            "payload changed"
        );
    }
    ensure!(
        names.len() == files.len()
            && names.contains("adl-spec/schemas/ghb/v1/packet.schema.json")
            && names.contains("LICENSE"),
        "unexpected/incomplete inventory"
    );
    ensure!(
        files.values().map(|(b, _)| b.len() as u64).sum::<u64>() <= MAX_BYTES,
        "distribution too large"
    );
    Ok(m)
}
pub fn export(repo: &Path, output: &Path) -> Result<Value> {
    archive::real_root(repo)?;
    let git = |args: &[&str]| -> Result<Vec<u8>> {
        let o = Command::new("git").args(args).current_dir(repo).output()?;
        ensure!(o.status.success(), "source Git observation failed");
        Ok(o.stdout)
    };
    let revision = String::from_utf8(git(&["rev-parse", "HEAD"])?)?
        .trim()
        .to_owned();
    ensure!(valid_sha(&revision, 40), "source revision invalid");
    let names = git(&[
        "ls-files",
        "-z",
        "--",
        "LICENSE",
        "adl-spec/schemas/ghb/v1",
        "adl-spec/examples/ghb/v1",
    ])?;
    let mut files = BTreeMap::new();
    for name in names.split(|b| *b == 0).filter(|n| !n.is_empty()) {
        let name = std::str::from_utf8(name)?;
        if !admitted_path(name) {
            continue;
        }
        let bytes = git(&["show", &format!("HEAD:{name}")])?;
        // The single editable canonical copy must match the committed source.
        let mut cursor = repo.to_path_buf();
        for part in Path::new(name).components() {
            cursor.push(part);
            ensure!(
                !fs::symlink_metadata(&cursor)?.file_type().is_symlink(),
                "source alias refused"
            );
        }
        ensure!(fs::read(&cursor)? == bytes, "source differs from revision");
        files.insert(name.to_owned(), (bytes, 0o644));
    }
    let m = Manifest {
        schema: SCHEMA.into(),
        package: "adl-ghb-contracts".into(),
        version: "1.0.0".into(),
        source_repository: "agent-logic/agent-design-language".into(),
        source_revision: revision,
        files: files
            .iter()
            .map(|(p, (b, _))| Entry {
                path: p.clone(),
                bytes: b.len() as u64,
                sha256: sha(b),
            })
            .collect(),
    };
    let raw = serde_json::to_vec_pretty(&m)?;
    let pin = sha(&raw);
    files.insert("source-manifest.json".into(), (raw, 0o644));
    inspect(&files, &pin)?;
    let bytes = archive::gzip(&archive::pack(&files)?)?;
    let mut out = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)?;
    use std::io::Write;
    out.write_all(&bytes)?;
    Ok(
        json!({"schema":SCHEMA,"source_revision":m.source_revision,"manifest_sha256":pin,
        "archive_sha256":sha(&bytes),"archive_bytes":bytes.len(),"producer_acceptance_implied":false}),
    )
}
pub fn verify(artifact: &Path, pin: &str, destination: Option<&Path>) -> Result<Value> {
    ensure!(
        fs::metadata(artifact)?.len() <= MAX_BYTES,
        "archive too large"
    );
    let bytes = fs::read(artifact)?;
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(bytes.as_slice())
        .take(MAX_BYTES + 1)
        .read_to_end(&mut raw)?;
    ensure!(raw.len() as u64 <= MAX_BYTES, "expanded archive too large");
    let files = archive::unpack(&raw, false)?;
    let m = inspect(&files, pin)?;
    if let Some(dest) = destination {
        archive::extract(&files, dest)?;
        verify_root(dest, pin)?;
    }
    Ok(
        json!({"schema":SCHEMA,"source_revision":m.source_revision,"manifest_sha256":pin,
        "archive_sha256":sha(&bytes),"producer_acceptance_implied":false}),
    )
}
fn installed_files(root: &Path) -> Result<BTreeMap<String, (Vec<u8>, u32)>> {
    fn visit(
        root: &Path,
        dir: &Path,
        out: &mut BTreeMap<String, (Vec<u8>, u32)>,
        total: &mut u64,
        dirs: &mut usize,
    ) -> Result<()> {
        *dirs += 1;
        ensure!(*dirs <= 32, "excess installed directories");
        for e in fs::read_dir(dir)? {
            let p = e?.path();
            let m = fs::symlink_metadata(&p)?;
            ensure!(!m.file_type().is_symlink(), "installed symlink refused");
            if m.is_dir() {
                visit(root, &p, out, total, dirs)?;
            } else {
                ensure!(
                    m.is_file() && m.len() <= 2 * 1024 * 1024 && out.len() < 65,
                    "invalid installed file"
                );
                #[cfg(unix)]
                {
                    use std::os::unix::fs::MetadataExt;
                    ensure!(m.nlink() == 1, "installed hardlink refused");
                }
                let name = p
                    .strip_prefix(root)?
                    .to_str()
                    .context("installed UTF8 path")?
                    .replace('\\', "/");
                ensure!(
                    name == "source-manifest.json" || admitted_path(&name),
                    "unexpected installed input"
                );
                *total += m.len();
                ensure!(*total <= MAX_BYTES, "installed distribution too large");
                let mut b = vec![];
                fs::File::open(&p)?
                    .take(2 * 1024 * 1024 + 1)
                    .read_to_end(&mut b)?;
                ensure!(
                    b.len() as u64 == m.len(),
                    "installed input changed during read"
                );
                #[cfg(unix)]
                let mode = {
                    use std::os::unix::fs::PermissionsExt;
                    m.permissions().mode() & 0o7777
                };
                #[cfg(not(unix))]
                let mode = 0o644;
                out.insert(name, (b, mode));
            }
        }
        Ok(())
    }
    let mut out = BTreeMap::new();
    visit(root, root, &mut out, &mut 0, &mut 0)?;
    Ok(out)
}
pub fn verify_root(root: &Path, pin: &str) -> Result<Value> {
    archive::real_root(root)?;
    let files = installed_files(root)?;
    let m = inspect(&files, pin)?;
    Ok(
        json!({"schema":SCHEMA,"source_revision":m.source_revision,"manifest_sha256":pin,
        "producer_acceptance_implied":false}),
    )
}
