//! PVF #870: required deterministic installed tooling proof; isolated CPU/Git/disk,
//! synthetic authority, no live provider or shared stable binary replacement.
#[allow(dead_code)]
#[path = "support/intent_fixture.rs"]
mod fixture;
use fixture::{git, inventory, Fixture};
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Output,
};
fn success(output: Output) -> Value {
    assert!(output.status.success(), "{output:?}");
    serde_json::from_slice(&output.stdout).unwrap()
}
fn setup(label: &str) -> (Fixture, PathBuf, PathBuf) {
    let mut f = Fixture::new(label);
    let root = f.root.clone();
    // Keep exact candidate bytes/mode tracked so the proof epoch regression
    // isolates administrative invalidation at an unchanged HEAD and input digest.
    fs::create_dir_all(root.join(".adl/bin")).unwrap();
    fs::copy(&f.binary, root.join(".adl/bin/csdlc")).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(
            root.join(".adl/bin/csdlc"),
            fs::Permissions::from_mode(0o755),
        )
        .unwrap();
    }
    git(&root, &["add", "-f", ".adl/bin/csdlc"]);
    git(
        &root,
        &[
            "commit",
            "--quiet",
            "-m",
            "Track isolated install destination",
        ],
    );
    let plan=f.write_json("plan.json",&json!({"schema":"csdlc.v3.intent_plan.v1","slug":"semantic-install",
        "cards":{"sip":{},"stp":{},"spp":{"dependencies_inline":"fixture","repo_inputs_inline":"fixture","target_files_surfaces_inline":"fixture","deliverables_inline":"install","validation_plan_inline":"installed test","acceptance_criteria_inline":"exact bytes","notes_risks_inline":"isolated"},"vpp":{},"srp":{},"sor":{}},"validators":[{"id":"install-proof","program":"cargo","args":["test","--manifest-path","fixture-proof/Cargo.toml","--offline"],"success_marker":"test result: ok.","timeout_seconds":60}],"publication":{"base":"main","title":"fixture","body":"Closes #505","draft":true}}));
    success(f.run(&root, &["prepare", "505", "--plan", plan.to_str().unwrap()]));
    success(f.run(&root, &["bind", "505"]));
    let binding: Value = serde_json::from_slice(
        &fs::read(root.join(".git/csdlc-v3/local/bindings/505.json")).unwrap(),
    )
    .unwrap();
    let linked = PathBuf::from(binding["worktree"].as_str().unwrap());
    let input = linked.join(".csdlc/evidence/505/install-input");
    fs::create_dir_all(&input).unwrap();
    let bytes = fs::read(&f.binary).unwrap();
    fs::write(input.join("candidate"), &bytes).unwrap();
    fs::write(
        input.join("provenance.json"),
        serde_json::to_vec(
            &json!({"schema":"csdlc.v3.install_provenance.v1","source":"isolated fixture build"}),
        )
        .unwrap(),
    )
    .unwrap();
    let selector = fs::read(linked.join("csdlc-v3/operator/authority-selector.json")).unwrap();
    fs::write(input.join("selector.json"), &selector).unwrap();
    let digest = blake3::hash(&bytes).to_hex().to_string();
    let selector_digest = blake3::hash(&selector).to_hex().to_string();
    let head = git(&linked, &["rev-parse", "HEAD"]);
    let approval=serde_json::to_vec(&json!({"schema":"csdlc.v3.cutover_approval.v1","authority_issue":505,"decision":"approved","repository":"agent-logic/agent-design-language","exact_head":head,"selected_binary_digest":digest,"selector_metadata_digest":selector_digest})).unwrap();
    fs::write(input.join("approval.json"), &approval).unwrap();
    let prefix = ".csdlc/evidence/505/install-input";
    let operation=f.write_json("install.json",&json!({"artifact_name":"csdlc","artifact_ref":format!("{prefix}/candidate"),"source_provenance_ref":format!("{prefix}/provenance.json"),"selector_metadata_ref":format!("{prefix}/selector.json"),"source_provenance":"isolated fixture build","selected_binary_digest":digest,"observed_binary_digest":digest,"selector_metadata_digest":selector_digest,"destination":".adl/bin/csdlc","stable_destination":true,"executes_install":true,"exact_head":head,"cutover_approval_ref":format!("{prefix}/approval.json"),"cutover_approval_digest":blake3::hash(&approval).to_hex().to_string()}));
    (f, linked, operation)
}
fn snapshot(root: &Path) -> Value {
    let dir = root.join(".git/csdlc-v3/semantic/issues/505");
    let current: Value =
        serde_json::from_slice(&fs::read(dir.join("current.json")).unwrap()).unwrap();
    let digest = current["digest"]
        .as_str()
        .unwrap()
        .split(':')
        .next_back()
        .unwrap();
    serde_json::from_slice(
        &fs::read(dir.join(format!("commits/{}-{digest}.json", current["generation"]))).unwrap(),
    )
    .unwrap()
}
#[test]
fn installed_semantic_install_is_one_shot_and_replay_is_observational() {
    let (mut f, linked, operation) = setup("semantic-install");
    let root = f.root.clone();
    let before = inventory(&root);
    let phase = snapshot(&root)["payload"]["phase"].clone();
    success(f.run(
        &linked,
        &["install", "505", "--operation", operation.to_str().unwrap()],
    ));
    assert_eq!(before, inventory(&root));
    let result = success(f.run(
        &linked,
        &[
            "install",
            "505",
            "--operation",
            operation.to_str().unwrap(),
            "--execute",
        ],
    ));
    assert_eq!(result["native_effect_truth"], "performed");
    assert_eq!(result["status"], "completed");
    assert_eq!(phase, snapshot(&root)["payload"]["phase"]);
    assert_eq!(
        blake3::hash(&fs::read(linked.join(".adl/bin/csdlc")).unwrap()),
        blake3::hash(&fs::read(&f.binary).unwrap())
    );
    let before = inventory(&root);
    let replay = success(f.run(
        &linked,
        &[
            "install",
            "505",
            "--operation",
            operation.to_str().unwrap(),
            "--execute",
        ],
    ));
    assert_eq!(replay["status"], "expected_noop");
    assert_eq!(before, inventory(&root));
    fs::write(linked.join(".adl/bin/csdlc"), b"foreign changed bytes").unwrap();
    let before = inventory(&root);
    assert!(!f
        .run(
            &linked,
            &[
                "install",
                "505",
                "--operation",
                operation.to_str().unwrap(),
                "--execute"
            ]
        )
        .status
        .success());
    assert_eq!(before, inventory(&root));
}
#[test]
fn installed_semantic_install_recovers_reservation_and_completed_native_effect() {
    for point in [
        "semantic_install_after_reservation",
        "semantic_install_after_rename",
        "semantic_install_after_binary",
        "semantic_install_after_receipt_link",
        "semantic_install_after_effect",
    ] {
        let (mut f, linked, operation) = setup(point);
        let root = f.root.clone();
        success(f.run(&linked, &["proof", "505"]));
        assert_eq!(
            success(f.run(&linked, &["status", "505"]))["evidence"]["proof_current"],
            true
        );
        let crash = f.run_with_env(
            &linked,
            &[
                "install",
                "505",
                "--operation",
                operation.to_str().unwrap(),
                "--execute",
            ],
            &[("CSDLC_V3_TEST_CRASH_POINT", point)],
        );
        assert_eq!(crash.status.code(), Some(91));
        #[cfg(unix)]
        let inode = {
            use std::os::unix::fs::MetadataExt;
            fs::metadata(linked.join(".adl/bin/csdlc"))
                .ok()
                .map(|m| m.ino())
        };
        if matches!(
            point,
            "semantic_install_after_rename" | "semantic_install_after_binary"
        ) {
            assert!(!linked
                .join(".csdlc/evidence/505/v3-install/receipt.json")
                .exists());
            assert_eq!(
                blake3::hash(&fs::read(linked.join(".adl/bin/csdlc")).unwrap()),
                blake3::hash(&fs::read(&f.binary).unwrap())
            );
        }
        #[cfg(unix)]
        if point == "semantic_install_after_rename" {
            use std::os::unix::fs::PermissionsExt;
            assert_ne!(
                fs::metadata(linked.join(".adl/bin/csdlc"))
                    .unwrap()
                    .permissions()
                    .mode()
                    & 0o777,
                0o755
            );
        }
        let linked_receipt_stage = if point == "semantic_install_after_receipt_link" {
            let receipt = linked.join(".csdlc/evidence/505/v3-install/receipt.json");
            let bytes = fs::read(&receipt).unwrap();
            let staged = receipt.with_extension(format!("{}.next", blake3::hash(&bytes).to_hex()));
            assert_eq!(fs::read(&staged).unwrap(), bytes);
            Some(staged)
        } else {
            None
        };
        let linked_before = inventory(&linked);
        let before = inventory(&root);
        let status_output = f.run(&linked, &["status", "505"]);
        let status: Value = serde_json::from_slice(&status_output.stdout).unwrap();
        assert_eq!(status["status"], "recovery_required");
        assert_eq!(status["evidence"]["proof_current"], false);
        assert_eq!(status["allowed_next"], json!(["recover"]));
        assert_eq!(before, inventory(&root));
        let preview = success(f.run(&linked, &["recover", "505"]));
        assert_eq!(before, inventory(&root));
        assert!(!f
            .run(
                &linked,
                &["recover", "505", "--execute", "--preview", "wrong"]
            )
            .status
            .success());
        assert_eq!(before, inventory(&root));
        assert_eq!(linked_before, inventory(&linked));
        let digest = preview["preview_digest"].as_str().unwrap();
        let result = success(f.run(
            &linked,
            &["recover", "505", "--execute", "--preview", digest],
        ));
        assert_eq!(result["native_effect_truth"], "performed");
        if let Some(staged) = linked_receipt_stage {
            assert!(!staged.exists());
        }
        #[cfg(unix)]
        if let Some(inode) = inode.filter(|_| point != "semantic_install_after_reservation") {
            use std::os::unix::fs::MetadataExt;
            assert_eq!(
                inode,
                fs::metadata(linked.join(".adl/bin/csdlc")).unwrap().ino(),
                "recovery recopied installed binary"
            );
        }
        assert!(snapshot(&root)["payload"]["pending"].is_null());
    }
}

#[test]
fn installed_semantic_install_invalidates_proof_until_new_execution() {
    let (mut f, linked, operation) = setup("semantic-install-proof-invalidation");
    let first = success(f.run(&linked, &["proof", "505"]));
    assert_eq!(
        success(f.run(&linked, &["status", "505"]))["evidence"]["proof_current"],
        true
    );
    success(f.run(
        &linked,
        &[
            "install",
            "505",
            "--operation",
            operation.to_str().unwrap(),
            "--execute",
        ],
    ));
    assert_eq!(
        success(f.run(&linked, &["status", "505"]))["evidence"]["proof_current"],
        false
    );
    let renewed = success(f.run(&linked, &["proof", "505"]));
    assert_eq!(renewed["status"], "completed");
    assert_ne!(
        first["operation_id"], renewed["operation_id"],
        "administrative invalidation reused old proof operation"
    );
    assert_eq!(
        success(f.run(&linked, &["status", "505"]))["evidence"]["proof_current"],
        true
    );
}

#[test]
fn installed_semantic_install_preserves_preexisting_foreign_receipt() {
    let (mut f, linked, operation) = setup("semantic-install-foreign-receipt");
    let receipt = linked.join(".csdlc/evidence/505/v3-install/receipt.json");
    fs::create_dir_all(receipt.parent().unwrap()).unwrap();
    fs::write(&receipt, b"foreign immutable receipt\n").unwrap();
    let destination = fs::read(linked.join(".adl/bin/csdlc")).unwrap();
    let state = snapshot(&f.root);
    let before = inventory(&f.root);
    let linked_before = inventory(&linked);
    for execute in [false, true] {
        let mut args = vec!["install", "505", "--operation", operation.to_str().unwrap()];
        if execute {
            args.push("--execute");
        }
        let output = f.run(&linked, &args);
        assert!(!output.status.success(), "{output:?}");
        assert!(String::from_utf8_lossy(&output.stdout).contains("install_receipt_preexists"));
        assert_eq!(fs::read(&receipt).unwrap(), b"foreign immutable receipt\n");
        assert_eq!(
            fs::read(linked.join(".adl/bin/csdlc")).unwrap(),
            destination
        );
        assert_eq!(snapshot(&f.root), state);
        assert_eq!(inventory(&f.root), before);
        assert_eq!(inventory(&linked), linked_before);
    }
}
