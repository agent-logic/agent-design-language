// PVF: docs_only; deterministic local feed/enclosure contract; small CPU/files; issue gate only.
#[test]
fn prelaunch_feed_contract() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().nth(4).unwrap();
    let result = std::process::Command::new("python3")
        .arg("-B").arg(".csdlc/evidence/1165/validation/check.py")
        .current_dir(root).output().expect("run feed validation");
    assert!(result.status.success(), "{}{}", String::from_utf8_lossy(&result.stdout), String::from_utf8_lossy(&result.stderr));
}
