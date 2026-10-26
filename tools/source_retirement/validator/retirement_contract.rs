//! PVF local_contract: committed candidate admission, deterministic Git and local
//! pinned custody archives; requires provisioned historical custody. Local gate.
#[test]
fn committed_candidate_retirement_guard() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let report =
        source_retirement_validator::audit::run(&source_retirement_validator::Repo::new(root))
            .expect("committed retirement guard");
    assert_eq!(report["total_removals"], 3407);
    assert_eq!(report["candidate_rows"], 4134);
}
