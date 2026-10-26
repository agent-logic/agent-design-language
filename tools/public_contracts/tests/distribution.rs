//! PVF fast, deterministic, offline data custody; no producer qualification.
use adl_public_contracts::{archive, distribution::Distribution, root};
use std::fs;
#[test]
fn frozen_archives_reproduce_and_export() {
    for godel in [true, false] {
        let d = Distribution::load(godel).unwrap();
        let data = fs::read(d.artifact()).unwrap();
        let files = d.inspect(&data).unwrap();
        let raw = archive::pack(&files).unwrap();
        let mut original = Vec::new();
        use std::io::Read;
        flate2::read::GzDecoder::new(data.as_slice())
            .read_to_end(&mut original)
            .unwrap();
        assert_eq!(raw.len(), original.len());
        let differences: Vec<_> = raw
            .iter()
            .zip(&original)
            .enumerate()
            .filter(|(_, (a, b))| a != b)
            .take(20)
            .map(|(i, (a, b))| (i, *a, *b))
            .collect();
        assert!(
            differences.is_empty(),
            "tar byte differences: {differences:?}"
        );
        assert_eq!(
            archive::sha(&archive::gzip(&raw).unwrap()),
            archive::sha(&data)
        );
        let temp = tempfile::tempdir().unwrap();
        let t = fs::canonicalize(temp.path()).unwrap();
        d.export(&root(), &t.join("export")).unwrap();
        d.verify(&d.artifact(), Some(&t.join("installed"))).unwrap();
        if godel {
            assert_eq!(
                d.manifest["files"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|r| r["path"].as_str().unwrap().starts_with("adl-spec/"))
                    .count(),
                7
            );
            d.verify_root(&t.join("installed")).unwrap();
        }
    }
}
#[test]
fn installed_mutations_refuse() {
    for kind in ["changed", "missing", "symlink", "manifest", "extra"] {
        let d = Distribution::load(true).unwrap();
        let t = tempfile::tempdir().unwrap();
        let root = fs::canonicalize(t.path()).unwrap().join("installed");
        d.verify(&d.artifact(), Some(&root)).unwrap();
        let p = root.join(d.manifest["files"][0]["path"].as_str().unwrap());
        match kind {
            "changed" => fs::write(p, b"{}").unwrap(),
            "missing" => fs::remove_file(p).unwrap(),
            "symlink" => {
                fs::remove_file(&p).unwrap();
                #[cfg(unix)]
                std::os::unix::fs::symlink("/outside", p).unwrap();
            }
            "manifest" => fs::write(root.join("source-manifest.json"), b"{}").unwrap(),
            _ => fs::write(root.join("extra"), b"").unwrap(),
        };
        assert!(d.verify_root(&root).is_err(), "{kind}");
    }
}
#[test]
fn artifact_failure_never_writes_and_existing_destinations_survive() {
    for godel in [true, false] {
        let d = Distribution::load(godel).unwrap();
        let t = tempfile::tempdir().unwrap();
        let mut bytes = fs::read(d.artifact()).unwrap();
        bytes.push(1);
        let bad = t.path().join("bad");
        fs::write(&bad, bytes).unwrap();
        let dest = t.path().join("dest");
        assert!(d.verify(&bad, Some(&dest)).is_err());
        assert!(!dest.exists());
        fs::create_dir(&dest).unwrap();
        fs::write(dest.join("sentinel"), "preserve").unwrap();
        assert!(d.verify(&d.artifact(), Some(&dest)).is_err());
        assert!(d.export(&root(), &dest).is_err());
        assert_eq!(
            fs::read_to_string(dest.join("sentinel")).unwrap(),
            "preserve"
        );
        if godel {
            assert!(d.verify_root(&t.path().join("absent")).is_err());
        }
    }
}
#[test]
fn archive_inventory_and_identity_refuse() {
    for godel in [true, false] {
        let mut d = Distribution::load(godel).unwrap();
        let data = fs::read(d.artifact()).unwrap();
        let original = d.inspect(&data).unwrap();
        for kind in ["missing", "changed", "unexpected"] {
            let mut files = original.clone();
            let key = files.keys().next().unwrap().clone();
            match kind {
                "missing" => {
                    files.remove(&key);
                }
                "changed" => {
                    files.get_mut(&key).unwrap().0.push(0);
                }
                _ => {
                    files.insert("unexpected".into(), (vec![], 0o644));
                }
            }
            assert!(d
                .inspect(&archive::gzip(&archive::pack(&files).unwrap()).unwrap())
                .is_err());
        }
        d.manifest["version"] = "9.9.9".into();
        assert!(d.inspect(&data).is_err());
    }
}
#[test]
fn hostile_tar_members_refuse() {
    for kind in ["traversal", "symlink", "duplicate"] {
        let mut b = tar::Builder::new(Vec::new());
        let mut h = tar::Header::new_ustar();
        h.set_path("safe").unwrap();
        h.set_size(0);
        h.set_mode(0o644);
        if kind == "traversal" {
            h.as_mut_bytes()[..10].copy_from_slice(b"../outside");
        }
        if kind == "symlink" {
            h.set_entry_type(tar::EntryType::Symlink);
            h.set_link_name("/outside").unwrap();
        }
        h.set_cksum();
        b.append(&h, &[][..]).unwrap();
        if kind == "duplicate" {
            b.append(&h, &[][..]).unwrap();
        }
        assert!(
            archive::unpack(&b.into_inner().unwrap(), false).is_err(),
            "{kind}"
        );
    }
}
