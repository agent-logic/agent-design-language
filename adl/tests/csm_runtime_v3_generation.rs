use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

use adl_runtime_kernel::{
    activate_config_generation, active_generation_ref, build_config_generation_receipt,
    config_generation_identity_from_env, generation_store, provision_config_generation,
    validate_active_config_generation, REDACTED_SECRET_REFERENCE,
};

fn write_generation_config(
    root: &std::path::Path,
    name: &str,
    secret_ref: &str,
) -> std::path::PathBuf {
    let path = root.join(name);
    fs::write(
        &path,
        format!(
            r#"
schema = "adl.runtime_v3.init.v1"

[credentials]
control_public_key_path = "{secret_ref}"
operation_public_key_path = "{secret_ref}.operation"

[api.tls]
certificate_chain_path = "{secret_ref}.cert"
private_key_path = "{secret_ref}.key"
trust_roots_path = "{secret_ref}.roots"
"#
        ),
    )
    .expect("write runtime config");
    path
}

#[test]
fn config_generation_receipt_is_immutable_and_redacts_secret_references() {
    let root = tempfile::tempdir().expect("temp root");
    let init = write_generation_config(
        root.path(),
        "runtime-init.toml",
        "/secret/runtime/control.pub",
    );

    let (receipt, identity) =
        build_config_generation_receipt(&init, "runtime-generation-one").expect("receipt");
    assert_eq!(receipt.schema, "adl.runtime_v3.config_generation.v1");
    assert_eq!(receipt.generation, receipt.content_sha256);
    assert_eq!(
        receipt.compatible_binary_generation,
        "runtime-generation-one"
    );
    assert_eq!(receipt.config_schema, "adl.runtime_v3.init.v1");
    assert_eq!(
        receipt.secret_references.get("api.tls.private_key_path"),
        Some(&REDACTED_SECRET_REFERENCE.to_owned())
    );
    let receipt_json = serde_json::to_string(&receipt).expect("receipt json");
    assert!(receipt_json.contains("api.tls.private_key_path"));
    assert!(!receipt_json.contains("/secret/runtime/control.pub"));

    let provisioned =
        provision_config_generation(&init, "runtime-generation-one").expect("provision");
    assert_eq!(provisioned, identity);
    let receipt_path = generation_store(&init)
        .expect("store")
        .join(format!("{}.json", identity.generation));
    let first_bytes = fs::read(&receipt_path).expect("receipt bytes");
    fs::write(&receipt_path, b"conflicting retained bytes").expect("tamper retained receipt");
    let error = provision_config_generation(&init, "runtime-generation-one").unwrap_err();
    assert!(error.contains("immutable Runtime configuration receipt conflicts"));
    assert_eq!(
        fs::read(&receipt_path).expect("retained receipt"),
        b"conflicting retained bytes"
    );
    assert_ne!(first_bytes, b"conflicting retained bytes");
}

#[test]
fn pre_activation_receipt_is_not_authoritative() {
    let root = tempfile::tempdir().expect("temp root");
    let init = write_generation_config(
        root.path(),
        "runtime-init.toml",
        "/secret/runtime/control.pub",
    );
    let identity = provision_config_generation(&init, "runtime-generation-one").expect("provision");

    assert!(!active_generation_ref(&init).expect("active ref").exists());
    let error = validate_active_config_generation(&init, "runtime-generation-one").unwrap_err();
    assert!(error.contains("read Runtime configuration active reference"));
    assert!(!error.contains(&identity.receipt_digest));
}

#[test]
fn post_pointer_mismatch_fails_closed_before_authority() {
    let root = tempfile::tempdir().expect("temp root");
    let init = write_generation_config(
        root.path(),
        "runtime-init.toml",
        "/secret/runtime/control.pub",
    );
    let identity = provision_config_generation(&init, "runtime-generation-one").expect("provision");
    activate_config_generation(&init, &identity).expect("activate");
    assert_eq!(
        validate_active_config_generation(&init, "runtime-generation-one").expect("validate"),
        identity
    );

    fs::write(
        active_generation_ref(&init).expect("active ref"),
        format!("{} {}\n", identity.generation, "0".repeat(64)),
    )
    .expect("tamper active ref");
    let error = validate_active_config_generation(&init, "runtime-generation-one").unwrap_err();
    assert!(error.contains("active reference does not match init content"));
}

#[test]
fn candidate_ready_receipt_does_not_replace_active_without_activation() {
    let root = tempfile::tempdir().expect("temp root");
    let active = write_generation_config(
        root.path(),
        "runtime-init.toml",
        "/secret/runtime/current.pub",
    );
    let candidate = write_generation_config(
        root.path(),
        "runtime-init.next.toml",
        "/secret/runtime/next.pub",
    );
    let active_identity =
        provision_config_generation(&active, "runtime-generation-one").expect("provision active");
    activate_config_generation(&active, &active_identity).expect("activate active");
    let candidate_identity = provision_config_generation(&candidate, "runtime-generation-one")
        .expect("provision candidate");

    assert_ne!(active_identity.generation, candidate_identity.generation);
    assert_eq!(
        validate_active_config_generation(&active, "runtime-generation-one").expect("active"),
        active_identity
    );
    assert!(!active_generation_ref(&candidate)
        .expect("candidate ref")
        .exists());
}

#[test]
fn prior_generation_remains_authoritative_after_candidate_failure() {
    let root = tempfile::tempdir().expect("temp root");
    let active = write_generation_config(
        root.path(),
        "runtime-init.toml",
        "/secret/runtime/current.pub",
    );
    let candidate = write_generation_config(
        root.path(),
        "runtime-init.next.toml",
        "/secret/runtime/next.pub",
    );
    let active_identity =
        provision_config_generation(&active, "runtime-generation-one").expect("provision active");
    activate_config_generation(&active, &active_identity).expect("activate active");
    let candidate_identity =
        provision_config_generation(&candidate, "runtime-generation-one").expect("candidate");
    let candidate_receipt = generation_store(&candidate)
        .expect("candidate store")
        .join(format!("{}.json", candidate_identity.generation));
    fs::write(candidate_receipt, b"{not-json").expect("corrupt candidate receipt");

    let error =
        validate_active_config_generation(&candidate, "runtime-generation-one").unwrap_err();
    assert!(error.contains("read Runtime configuration active reference"));
    assert_eq!(
        validate_active_config_generation(&active, "runtime-generation-one").expect("active"),
        active_identity
    );
}

#[test]
fn malformed_and_cross_binary_receipts_are_rejected() {
    let root = tempfile::tempdir().expect("temp root");
    let init = write_generation_config(
        root.path(),
        "runtime-init.toml",
        "/secret/runtime/control.pub",
    );
    let identity = provision_config_generation(&init, "runtime-generation-one").expect("provision");
    activate_config_generation(&init, &identity).expect("activate");

    let cross_binary =
        validate_active_config_generation(&init, "runtime-generation-two").unwrap_err();
    assert!(cross_binary.contains("active reference does not match init content"));

    let receipt_path = generation_store(&init)
        .expect("store")
        .join(format!("{}.json", identity.generation));
    fs::write(receipt_path, b"{not-json").expect("corrupt receipt");
    let malformed = validate_active_config_generation(&init, "runtime-generation-one").unwrap_err();
    assert!(malformed.contains("parse active Runtime configuration receipt"));
}

#[test]
fn kernel_startup_requires_config_generation_handoff_before_readiness_identity() {
    let generation = "a".repeat(64);
    let receipt_digest = "b".repeat(64);

    let missing = config_generation_identity_from_env(|_| None).unwrap_err();
    assert!(missing.contains("configuration generation environment is required"));

    let partial = config_generation_identity_from_env(|name| {
        (name == "ADL_RUNTIME_V3_CONFIG_GENERATION").then(|| generation.clone())
    })
    .unwrap_err();
    assert!(partial.contains("configuration generation environment is incomplete"));

    let malformed = config_generation_identity_from_env(|name| match name {
        "ADL_RUNTIME_V3_CONFIG_GENERATION" => Some(generation.clone()),
        "ADL_RUNTIME_V3_CONFIG_RECEIPT_DIGEST" => Some("not-a-digest".to_owned()),
        _ => None,
    })
    .unwrap_err();
    assert!(malformed.contains("receipt digest"));

    let identity = config_generation_identity_from_env(|name| match name {
        "ADL_RUNTIME_V3_CONFIG_GENERATION" => Some(generation.clone()),
        "ADL_RUNTIME_V3_CONFIG_RECEIPT_DIGEST" => Some(receipt_digest.clone()),
        _ => None,
    })
    .expect("complete handoff identity");
    assert_eq!(identity.generation, generation);
    assert_eq!(identity.receipt_digest, receipt_digest);
}

#[test]
fn generation_installer_rejects_mixed_set_and_preserves_current_reference() {
    let root = tempfile::tempdir().expect("temp root");
    let sources = root.path().join("sources");
    let install = root.path().join("install");
    fs::create_dir_all(&sources).expect("sources");
    for binary in ["csm", "adl-runtime-guardian", "adl-runtime-kernel"] {
        let path = sources.join(binary);
        fs::write(&path, "#!/bin/sh\nexit 0\n").expect("binary");
        let mut permissions = fs::metadata(&path).expect("metadata").permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions).expect("executable");
    }
    let script = format!(
        "{}/tools/install_runtime_v3_generation.sh",
        env!("CARGO_MANIFEST_DIR")
    );
    let installed = Command::new(&script)
        .args([
            "install",
            "--root",
            install.to_str().unwrap(),
            "--generation",
            "one",
            "--csm",
            sources.join("csm").to_str().unwrap(),
            "--guardian",
            sources.join("adl-runtime-guardian").to_str().unwrap(),
            "--kernel",
            sources.join("adl-runtime-kernel").to_str().unwrap(),
            "--source-revision",
            "test-revision",
            "--build-profile",
            "debug",
        ])
        .status()
        .expect("run installer");
    assert!(installed.success());
    let current_before = fs::read_link(install.join("current")).expect("current");
    fs::write(
        install.join("generations/one/bin/adl-runtime-kernel"),
        "mixed",
    )
    .expect("tamper");
    let verified = Command::new(&script)
        .args(["verify", "--root", install.to_str().unwrap()])
        .status()
        .expect("run verifier");
    assert!(!verified.success());
    assert_eq!(
        fs::read_link(install.join("current")).expect("current"),
        current_before
    );
}
