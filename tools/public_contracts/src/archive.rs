//! Strict archive payload utilities; validate paths/types before extraction.
use super::*;
use anyhow::ensure;
use sha2::{Digest, Sha256};
use std::io::{Read, Write};
pub fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
pub fn digest(path: &Path) -> Result<String> {
    Ok(sha(&fs::read(path)?))
}
pub fn safe_name(name: &str) -> bool {
    !name.is_empty()
        && !Path::new(name).is_absolute()
        && Path::new(name)
            .components()
            .all(|c| matches!(c, std::path::Component::Normal(_)))
        && !name.contains('\\')
}
pub fn pack(files: &BTreeMap<String, (Vec<u8>, u32)>) -> Result<Vec<u8>> {
    let mut b = tar::Builder::new(Vec::new());
    for (name, (data, mode)) in files {
        ensure!(safe_name(name), "unsafe archive path");
        let mut h = tar::Header::new_ustar();
        h.set_path(name)?;
        h.set_size(data.len() as u64);
        h.set_mode(*mode);
        h.set_uid(0);
        h.set_gid(0);
        h.set_mtime(0);
        h.set_entry_type(tar::EntryType::Regular);
        h.set_cksum();
        // Frozen archives use POSIX's six octal digits, NUL, space form.
        let checksum = format!("{:06o}\0 ", h.cksum()?);
        h.as_mut_bytes()[148..156].copy_from_slice(checksum.as_bytes());
        b.append(&h, data.as_slice())?;
    }
    let mut raw = b.into_inner()?;
    let padded = raw.len().div_ceil(10240) * 10240;
    raw.resize(padded, 0);
    Ok(raw)
}
pub fn gzip(bytes: &[u8]) -> Result<Vec<u8>> {
    let mut g = flate2::GzBuilder::new()
        .mtime(0)
        .operating_system(255)
        .write(Vec::new(), flate2::Compression::best());
    g.write_all(bytes)?;
    Ok(g.finish()?)
}
pub fn unpack(bytes: &[u8], compressed: bool) -> Result<BTreeMap<String, (Vec<u8>, u32)>> {
    let data = if compressed {
        let mut out = vec![];
        flate2::read::GzDecoder::new(bytes).read_to_end(&mut out)?;
        out
    } else {
        bytes.to_vec()
    };
    let mut archive = tar::Archive::new(data.as_slice());
    let mut files = BTreeMap::new();
    for entry in archive.entries()? {
        let mut e = entry?;
        let name = e.path()?.to_str().context("archive path UTF-8")?.to_owned();
        ensure!(
            safe_name(&name) && e.header().entry_type().is_file() && !files.contains_key(&name),
            "invalid archive inventory"
        );
        let mode = e.header().mode()?;
        let size = e.header().size()?;
        let mut payload = vec![];
        e.read_to_end(&mut payload)?;
        ensure!(payload.len() as u64 == size, "archive size mismatch");
        files.insert(name, (payload, mode));
    }
    Ok(files)
}
pub fn extract(files: &BTreeMap<String, (Vec<u8>, u32)>, output: &Path) -> Result<()> {
    ensure!(!output.exists(), "extraction output already exists");
    fs::create_dir_all(output)?;
    for (name, (data, mode)) in files {
        ensure!(safe_name(name), "unsafe extraction path");
        let dest = output.join(name);
        fs::create_dir_all(dest.parent().context("parent")?)?;
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&dest)?;
        file.write_all(data)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(dest, fs::Permissions::from_mode(*mode))?;
        }
    }
    Ok(())
}
pub fn walk(root: &Path) -> Result<BTreeMap<String, (Vec<u8>, u32)>> {
    fn visit(root: &Path, path: &Path, out: &mut BTreeMap<String, (Vec<u8>, u32)>) -> Result<()> {
        for e in fs::read_dir(path)? {
            let p = e?.path();
            let m = fs::symlink_metadata(&p)?;
            ensure!(!m.file_type().is_symlink(), "aliased installed input");
            if m.is_dir() {
                visit(root, &p, out)?;
            } else {
                ensure!(m.is_file(), "unsupported installed input");
                #[cfg(unix)]
                let mode = {
                    use std::os::unix::fs::PermissionsExt;
                    m.permissions().mode() & 0o7777
                };
                #[cfg(not(unix))]
                let mode = 0o644;
                out.insert(
                    p.strip_prefix(root)?.to_string_lossy().replace('\\', "/"),
                    (fs::read(&p)?, mode),
                );
            }
        }
        Ok(())
    }
    let mut out = BTreeMap::new();
    visit(root, root, &mut out)?;
    Ok(out)
}
pub fn real_root(root: &Path) -> Result<()> {
    ensure!(
        root.is_dir() && !root.is_symlink(),
        "explicit real installed root required"
    );
    for p in root.ancestors() {
        ensure!(!p.is_symlink(), "aliased installed root");
    }
    Ok(())
}
