// PVF: docs_only; deterministic local media/package integrity; small CPU/files;
// issue publication gate only, not runtime/release proof or human listening.
#[test]
fn episode_candidate_contract() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().nth(4).unwrap();
    let result = std::process::Command::new("python3")
        .arg("-B").arg(".csdlc/evidence/1166/validation/check.py")
        .current_dir(root).output().expect("run episode package validation");
    assert!(result.status.success(), "{}{}", String::from_utf8_lossy(&result.stdout), String::from_utf8_lossy(&result.stderr));
}
