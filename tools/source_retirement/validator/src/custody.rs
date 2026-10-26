use crate::*;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};
pub fn mode(v: &Value) -> Result<u32> {
    let s = string(v)?;
    let tail = if s.len() > 3 { &s[s.len() - 3..] } else { s };
    Ok(u32::from_str_radix(tail, 8)?)
}
#[derive(Clone)]
pub struct Member {
    pub regular: bool,
    pub mode: u32,
    pub sha256: String,
}
pub struct Archive {
    pub sha256: String,
    pub members: BTreeMap<String, Member>,
    pub count: usize,
}
#[derive(Default)]
pub struct Archives {
    loaded: HashMap<PathBuf, Archive>,
}
impl Archives {
    pub fn load(&mut self, path: &Path, pin: &str) -> Result<&Archive> {
        if !self.loaded.contains_key(path) {
            let file = File::open(path)
                .with_context(|| format!("custody archive missing: {}", path.display()))?;
            let mut reader = BufReader::new(file);
            let mut hash = Sha256::new();
            let mut buf = [0; 65536];
            loop {
                let n = reader.read(&mut buf)?;
                if n == 0 {
                    break;
                }
                hash.update(&buf[..n]);
            }
            let digest = format!("{:x}", hash.finalize());
            ensure!(digest == pin, "archive changed: {}", path.display());
            let mut magic = [0; 2];
            File::open(path)?.read_exact(&mut magic)?;
            let reader: Box<dyn Read> = if magic == [0x1f, 0x8b] {
                Box::new(flate2::read::GzDecoder::new(File::open(path)?))
            } else {
                Box::new(File::open(path)?)
            };
            let mut tar = tar::Archive::new(reader);
            let mut members = BTreeMap::new();
            let mut count = 0;
            for entry in tar.entries()? {
                let mut e = entry?;
                let name = e
                    .path()?
                    .to_str()
                    .context("non-UTF8 custody member")?
                    .to_owned();
                let regular = e.header().entry_type().is_file();
                let mode = e.header().mode()?;
                let mut hash = Sha256::new();
                loop {
                    let n = e.read(&mut buf)?;
                    if n == 0 {
                        break;
                    }
                    hash.update(&buf[..n]);
                }
                ensure!(
                    !members.contains_key(&name),
                    "duplicate custody archive member: {name}"
                );
                members.insert(
                    name,
                    Member {
                        regular,
                        mode,
                        sha256: format!("{:x}", hash.finalize()),
                    },
                );
                count += 1;
            }
            self.loaded.insert(
                path.to_owned(),
                Archive {
                    sha256: digest,
                    members,
                    count,
                },
            );
        }
        let result = &self.loaded[path];
        ensure!(
            result.sha256 == pin,
            "archive pin mismatch: {}",
            path.display()
        );
        Ok(result)
    }
    pub fn member(
        &mut self,
        path: &Path,
        pin: &str,
        name: &str,
        mode: u32,
        digest: &str,
    ) -> Result<()> {
        let a = self.load(path, pin)?;
        let m = a.members.get(name).context("custody member missing")?;
        ensure!(
            m.regular && m.mode == mode && m.sha256 == digest,
            "custody identity mismatch: {name}"
        );
        Ok(())
    }
    pub fn exact(&mut self, path: &Path, pin: &str, expected: &Paths) -> Result<()> {
        let a = self.load(path, pin)?;
        ensure!(
            a.count == expected.len() && a.members.keys().cloned().collect::<Paths>() == *expected,
            "supplemental member scope changed"
        );
        Ok(())
    }
}
pub fn codefriend_six(
    a: &mut Archives,
    batch: &Value,
    original: &Path,
    data: &Value,
) -> Result<()> {
    for r in rows(&batch["removed"])? {
        a.member(
            original,
            string(&data["ARCHIVE_SHA256"])?,
            string(&r["rollback"]["archive_member"])?,
            mode(&r["rollback"]["mode"])?,
            string(&r["before"]["sha256"])?,
        )?;
    }
    Ok(())
}
pub fn codefriend(a: &mut Archives, batch: &Value, original: &Path, data: &Value) -> Result<()> {
    let supplement = &batch["rollback_custody"]["supplement"];
    let extra = Path::new(string(&supplement["path"])?);
    a.load(extra, string(&supplement["sha256"])?)?;
    let files = rows(&batch["files"])?;
    for c in rows(&batch["rollback_custody"]["rows"])? {
        let p = string(&c["path"])?;
        let r = files
            .iter()
            .find(|r| r["path"] == p && r["action"] == "remove")
            .context("custody source missing")?;
        let (path, pin) = if c["current_bytes_match_archive"] == true {
            (original, string(&data["ARCHIVE_SHA256"])?)
        } else {
            (extra, string(&supplement["sha256"])?)
        };
        a.member(
            path,
            pin,
            &format!("source/{p}"),
            mode(&r["before"]["mode"])?,
            string(&r["before"]["sha256"])?,
        )?;
    }
    Ok(())
}
pub fn csdlc(a: &mut Archives, batch: &Value, original: &Path, data: &Value) -> Result<()> {
    let s = &batch["rollback"]["supplement"];
    let extra = Path::new(string(&s["path"])?);
    let pin = string(&s["sha256"])?;
    let rs = rows(&batch["rollback"]["rows"])?;
    let expected = rs
        .iter()
        .filter(|r| r["original_archive_matches"] != true)
        .map(|r| Ok(format!("source/{}", string(&r["path"])?)))
        .collect::<Result<Paths>>()?;
    a.exact(extra, pin, &expected)?;
    for r in rs {
        let (path, pin) = if r["original_archive_matches"] == true {
            (original, string(&data["ARCHIVE_SHA256"])?)
        } else {
            (extra, pin)
        };
        a.member(
            path,
            pin,
            string(&r["member"])?,
            number(&r["mode"])? as u32,
            string(&r["sha256"])?,
        )?;
    }
    Ok(())
}
pub fn runtime(a: &mut Archives, batch: &Value, original: &Path, data: &Value) -> Result<()> {
    let expected = rows(&batch["prepared_patch"]["deletions"])?
        .iter()
        .filter(|r| r["rollback"]["matches_current"] != true)
        .map(|r| Ok(string(&r["path"])?.to_owned()))
        .collect::<Result<Paths>>()?;
    let mut matches = Vec::new();
    for s in rows(&batch["prepared_patch"]["custody"])? {
        if paths(rows(&s["entries"])?)? == expected {
            matches.push(s);
        }
    }
    ensure!(matches.len() == 1, "Runtime supplement scope changed");
    let s = matches[0];
    let extra = Path::new(string(&s["path"])?);
    a.exact(extra, string(&s["sha256"])?, &expected)?;
    for r in rows(&batch["files"])?
        .iter()
        .filter(|r| r["action"] == "remove")
    {
        let p = string(&r["path"])?;
        let (path, pin, name) = if expected.contains(p) {
            (extra, string(&s["sha256"])?, p.to_owned())
        } else {
            (
                original,
                string(&data["ARCHIVE_SHA256"])?,
                format!("source/{p}"),
            )
        };
        a.member(
            path,
            pin,
            &name,
            mode(&r["before"]["mode"])?,
            string(&r["before"]["sha256"])?,
        )?;
    }
    Ok(())
}
pub fn infrastructure(a: &mut Archives, batch: &Value) -> Result<()> {
    for r in rows(&batch["files"])? {
        let c = &r["rollback"];
        a.member(
            Path::new(string(&c["archive"])?),
            string(&c["archive_sha256"])?,
            string(&c["member"])?,
            mode(&c["mode"])?,
            string(&r["before"]["sha256"])?,
        )?;
    }
    Ok(())
}
pub fn conversion(a: &mut Archives, batch: &Value, spec: &Value) -> Result<()> {
    let s = if spec["kind"] == "conversion" {
        &batch["rollback"]["supplement"]
    } else {
        &batch["custody"]
    };
    let path = Path::new(string(&s["path"])?);
    let pin = string(&s["sha256"])?;
    let rs = rows(&batch["files"])?
        .iter()
        .filter(|r| r["action"] == "remove")
        .collect::<Vec<_>>();
    let expected = rs
        .iter()
        .map(|r| Ok(format!("source/{}", string(&r["path"])?)))
        .collect::<Result<Paths>>()?;
    a.exact(path, pin, &expected)?;
    for r in rs {
        a.member(
            path,
            pin,
            &format!("source/{}", string(&r["path"])?),
            mode(&r["before"]["mode"])?,
            string(&r["before"]["sha256"])?,
        )?;
    }
    Ok(())
}
pub fn recent(a: &mut Archives, batch: &Value, spec: &Value) -> Result<()> {
    let removed = rows(&batch["files"])?
        .iter()
        .filter(|r| r["action"] == "remove")
        .map(|r| Ok((string(&r["path"])?, r)))
        .collect::<Result<BTreeMap<_, _>>>()?;
    let cs = spec["custody"].as_object().context("custody map missing")?;
    ensure!(
        cs.keys().cloned().collect::<Paths>() == removed.keys().map(|s| s.to_string()).collect(),
        "Recent owner custody scope changed"
    );
    let mut checked = Paths::new();
    for archive in rows(&spec["archives"])? {
        let path = Path::new(string(&archive["path"])?);
        let pin = string(&archive["sha256"])?;
        a.load(path, pin)?;
        for (source, c) in cs {
            if c["archive"] != archive["path"] {
                continue;
            }
            let before = &removed[source.as_str()]["before"];
            a.member(
                path,
                pin,
                string(&c["member"])?,
                mode(&before["mode"])?,
                string(&before["sha256"])?,
            )?;
            checked.insert(source.clone());
        }
    }
    ensure!(
        checked == cs.keys().cloned().collect(),
        "custody archive not declared"
    );
    Ok(())
}
