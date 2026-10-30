//! PVF deterministic local custody, no provider or lifecycle authority.
use adl_public_contracts::{archive, transpiler};
use std::{fs, process::Command};
#[test]
fn uses_committed_source_despite_dirty_checkout() {
    let t = tempfile::tempdir().unwrap();
    let root = t.path();
    let git = |args: &[&str]| {
        let o = Command::new("git")
            .arg("-C")
            .arg(root)
            .args(args)
            .output()
            .unwrap();
        assert!(o.status.success());
        o.stdout
    };
    git(&["init", "-q"]);
    for p in transpiler::DATA {
        let dest = root.join(p);
        fs::create_dir_all(dest.parent().unwrap()).unwrap();
        fs::write(dest, format!("committed {p}")).unwrap();
    }
    git(&["add", "."]);
    git(&[
        "-c",
        "user.name=Test",
        "-c",
        "user.email=test@example.invalid",
        "commit",
        "-qm",
        "fixture",
    ]);
    let revision = String::from_utf8(git(&["rev-parse", "HEAD"])).unwrap();
    for p in transpiler::DATA {
        fs::write(root.join(p), "dirty replacement").unwrap();
    }
    let bin = root.join("binary");
    fs::write(&bin, "synthetic binary").unwrap();
    let artifact = root.join("package.tar");
    let manifest = transpiler::build(root, &bin, &artifact).unwrap();
    assert_eq!(manifest["source_revision"], revision.trim());
    let dest = root.join("installed");
    transpiler::install(&artifact, &archive::digest(&artifact).unwrap(), &dest).unwrap();
    for p in transpiler::DATA {
        assert_eq!(
            fs::read_to_string(dest.join(p)).unwrap(),
            format!("committed {p}")
        );
    }
}
