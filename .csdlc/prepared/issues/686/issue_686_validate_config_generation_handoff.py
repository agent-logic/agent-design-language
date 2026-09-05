#!/usr/bin/env python3
"""Focused static denominator for issue #686 configuration-generation handoff.

This is not a substitute for the focused Rust behavior tests. It is a
denominator guard: if an implementation silently drops one of #686's named
handoff/recovery surfaces, this script must fail instead of letting the SOR
overclaim coverage from a few loose tokens.
"""

from pathlib import Path
import re


ROOT = Path(__file__).resolve().parents[4]


def read(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def require(path: str, needles: tuple[str, ...], *, label: str) -> None:
    text = (ROOT / path).read_text(encoding="utf-8")
    missing = [needle for needle in needles if needle not in text]
    if missing:
        raise SystemExit(f"{path}: missing required #686 {label}: {missing}")


def require_regex(path: str, patterns: tuple[str, ...], *, label: str) -> None:
    text = read(path)
    missing = [pattern for pattern in patterns if not re.search(pattern, text, re.S)]
    if missing:
        raise SystemExit(f"{path}: missing required #686 {label}: {missing}")


def require_order(path: str, needles: tuple[str, ...], *, label: str) -> None:
    text = read(path)
    cursor = -1
    for needle in needles:
        index = text.find(needle, cursor + 1)
        if index == -1:
            raise SystemExit(
                f"{path}: missing required #686 {label} after offset {cursor}: {needle}"
            )
        cursor = index


def main() -> None:
    require(
        "adl-runtime-kernel/src/config_generation.rs",
        (
            'CONFIG_GENERATION_RECEIPT_SCHEMA: &str = "adl.runtime_v3.config_generation.v1"',
            'CONFIG_GENERATION_ENV: &str = "ADL_RUNTIME_V3_CONFIG_GENERATION"',
            'CONFIG_RECEIPT_DIGEST_ENV: &str = "ADL_RUNTIME_V3_CONFIG_RECEIPT_DIGEST"',
            'REDACTED_SECRET_REFERENCE: &str = "[redacted-secret-reference]"',
            "pub struct ConfigGenerationReceipt",
            "pub struct ConfigGenerationIdentity",
            "pub fn build_config_generation_receipt",
            "collect_secret_references",
            "receipt_digest(&receipt)",
            "pub fn provision_config_generation_in_store",
            ".create_new(true)",
            "immutable Runtime configuration receipt conflicts with retained bytes",
            "pub fn activate_config_generation",
            'format!("{} {}\\n", identity.generation, identity.receipt_digest)',
            "fs::rename(&staged, &active)",
            "pub fn validate_active_config_generation",
            "Runtime configuration active reference does not match init content",
            "Runtime configuration receipt identity or compatibility is invalid",
            "pub fn config_generation_identity_from_env",
            "runtime configuration generation environment is required",
            "runtime configuration generation environment is incomplete",
            "pub fn validate_config_generation_identity_matches_active",
            "runtime configuration generation environment does not match active receipt",
        ),
        label="kernel receipt/active-reference contract",
    )
    require_regex(
        "adl-runtime-kernel/src/config_generation.rs",
        (
            r"if key\.ends_with\(\"_path\"\).*REDACTED_SECRET_REFERENCE\.to_owned\(\)",
            r"receipt\.schema != CONFIG_GENERATION_RECEIPT_SCHEMA.*receipt\.generation != generation.*receipt\.compatible_binary_generation != compatible_binary_generation.*receipt_digest\(&receipt\)\? != digest",
        ),
        label="receipt semantic validation",
    )
    require_order(
        "adl/src/cli/csm_runtime_v3_cmd.rs",
        (
            "fn start(args: &RuntimeV3ServiceArgs) -> Result<()>",
            "reconcile_interrupted_reload(args)?;",
            "let init = validated_init(&args.init)?;",
            "prepare_active_config_generation(&args.init, &binary_generation)?;",
        ),
        label="start preflight ordering",
    )
    require_order(
        "adl/src/cli/csm_runtime_v3_cmd.rs",
        (
            "fn reload(args: &RuntimeV3ServiceArgs) -> Result<()>",
            "reconcile_interrupted_reload(args)?;",
            "let current = validated_init(&args.init)?;",
            "prepare_active_config_generation(&args.init, &binary_generation)?;",
            "provision_config_generation_in_store(",
            "args.candidate.as_ref().expect(\"candidate path\")",
            "&args.init",
        ),
        label="reload recovery and active-store candidate provisioning",
    )
    require(
        "adl/src/cli/csm_runtime_v3_cmd.rs",
        (
            "copy_create_new(&active_ref, &backup_ref)",
            "retain last-known-good Runtime configuration generation reference",
            "activate_config_generation(active, candidate_identity)",
            "restore_last_known_good(active, &backup)",
            "validate_active_config_generation(&args.init, &binary_generation)",
            "Runtime v3 readiness config identity does not match active init",
            "Runtime v3 readiness config generation does not match active receipt",
        ),
        label="CSM reload/readiness authority",
    )
    require(
        "adl-runtime/src/bin/adl-runtime-guardian.rs",
        (
            "validate_active_config_generation(&init, &binary_generation)",
            "CONFIG_GENERATION_ENV.to_owned()",
            "CONFIG_RECEIPT_DIGEST_ENV.to_owned()",
        ),
        label="Guardian launch environment propagation",
    )
    require_order(
        "adl-runtime-kernel/src/bin/adl-runtime-kernel.rs",
        (
            "config_generation_identity_from_env(|name| std::env::var(name).ok())",
            "validate_config_generation_identity_matches_active(",
            "service = service.with_config_generation(",
        ),
        label="kernel startup validation before readiness exposure",
    )
    require(
        "adl-runtime-kernel/src/control.rs",
        (
            "config_generation: String",
            "config_receipt_digest: String",
            "pub fn with_config_generation",
            "config_generation: self.config_generation.clone()",
            "config_receipt_digest: self.config_receipt_digest.clone()",
        ),
        label="readiness/status projection fields",
    )
    require(
        "adl-runtime-kernel/src/control/feeds.rs",
        (
            "pub config_generation: String",
            "pub config_receipt_digest: String",
        ),
        label="feed projection fields",
    )
    require(
        "adl/tests/csm_runtime_v3_generation.rs",
        (
            "fn config_generation_receipt_is_immutable_and_redacts_secret_references",
            "assert!(!receipt_json.contains(\"/secret/runtime/control.pub\"))",
            "immutable Runtime configuration receipt conflicts",
            "fn pre_activation_receipt_is_not_authoritative",
            "fn post_pointer_mismatch_fails_closed_before_authority",
            "active reference does not match init content",
            "fn candidate_ready_receipt_does_not_replace_active_without_activation",
            "fn reload_candidate_receipt_is_available_from_active_generation_store",
            "provision_config_generation_in_store(&candidate, &active",
            "assert!(!candidate_store_receipt.exists())",
            "fn prior_generation_remains_authoritative_after_candidate_failure",
            "fn malformed_and_cross_binary_receipts_are_rejected",
            "fn kernel_startup_requires_config_generation_handoff_before_readiness_identity",
            "fn kernel_startup_rejects_forged_config_generation_handoff_identity",
            "fn generation_installer_rejects_mixed_set_and_preserves_current_reference",
        ),
        label="focused Rust regression denominator",
    )
    require(
        "adl/src/cli/csm_runtime_v3_cmd.rs",
        (
            "fn interrupted_reload_reconciles_before_config_generation_preflight",
            "prepare_active_config_generation(&active, \"test-generation\").unwrap_err()",
            "reconcile_interrupted_reload_with(",
            "prepare_active_config_generation(&active, \"test-generation\")",
            "active_identity",
        ),
        label="interrupted reload regression denominator",
    )
    print("issue #686 configuration-generation handoff denominator: PASS")


if __name__ == "__main__":
    main()
