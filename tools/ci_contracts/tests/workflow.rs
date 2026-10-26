//! PVF local_contract: public CI wiring, package boundaries and hosted proof contracts.
use serde_json::{json, Value};
use std::{fs, path::PathBuf};
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn read(p: &str) -> String {
    fs::read_to_string(root().join(p)).unwrap()
}
fn block(text: &str, name: &str) -> String {
    let start = format!("  {name}:\n");
    let body = text.split_once(&start).unwrap().1;
    let re = regex::Regex::new(r"(?m)^  [A-Za-z0-9_-]+:\n").unwrap();
    body[..re.find(body).map(|m| m.start()).unwrap_or(body.len())].into()
}
#[test]
fn public_ci_wiring() {
    let public_roots = json!([
        "adl-schema/**",
        "adl-uts/**",
        "adl-v2/Cargo.toml",
        "adl-v2/Cargo.lock",
        "adl-v2/crates/adl-language/**",
        "adl-v2/crates/adl-compiler/**",
        "adl-legacy-contracts/**",
        "tools/public_adl/**",
        "adl/Cargo.toml",
        "adl/Cargo.lock",
        "adl/src/main.rs",
        "adl/src/lib.rs",
        "adl/src/cli/public_dispatch.rs",
        "adl/src/cli/public_workflow.rs",
        "adl/src/cli/workflow_args.rs",
        "adl/src/cli/workflow_display.rs",
        "adl/src/cli/resume_args.rs",
        "adl/src/cli/runtime_owner.rs",
        "adl/src/cli/codefriend_owner.rs",
        "adl/src/cli/process_cmd.rs",
        "adl/src/cli/process_cmd/**",
        "adl/src/cli/session_cmd.rs",
        "adl/src/cli/observability.rs",
        "adl/src/cli/usage.rs",
        "adl/src/control_plane.rs",
        "adl/src/session_ledger.rs",
        "adl/src/bin/adl_process.rs",
        "adl/tests/adl_tests.rs",
        "adl/tests/resolve_tests.rs",
        "adl/tests/schema_tests.rs",
        "adl/tests/public_process.rs",
        "adl/tests/public_observation.rs",
        "adl/tests/issue_327_removed_tooling.rs",
        "adl/tests/cli_smoke/process_status.rs",
        "adl/tests/helpers.rs",
        "adl/tools/install_owner_binaries.sh",
        "adl/tools/owner_binary_resolution.sh",
        "adl/tools/test_owner_binary_install.sh"
    ]);
    let public_inputs = json!([
        "adl/tests/fixtures/cli_smoke.adl.yaml",
        "adl/examples/adl-0.1.yaml",
        "adl/examples/v0-2-multi-step-basic.adl.yaml",
        "adl/examples/v0-2-multi-step-file-input.adl.yaml",
        "adl/examples/v0-2-coordinator-agents-sdk.adl.yaml"
    ]);
    let public_packages = json!([
        "adl-schema",
        "adl-uts",
        "adl-language",
        "adl-compiler",
        "adl-legacy-contracts"
    ]);

    let workflow = read(".github/workflows/ci.yaml");
    let guards = block(&workflow, "adl_tooling_contracts");
    assert!(guards.contains(
        "cargo test --locked --manifest-path tools/ci_contracts/Cargo.toml --test actions"
    ));
    for name in ["test_pr_closing_linkage.sh", "check_pr_closing_linkage.sh"] {
        assert!(!guards.contains(&format!("adl/tools/{name}")));
    }
    let selector: Value =
        serde_json::from_str(&read("adl/config/validation_lane_selector.public.v1.json")).unwrap();
    let lane = &selector["special_surfaces"]["rust_pr_fast"];
    assert_eq!(lane["planner"], "manifest_command");
    assert_eq!(lane["id"], "public_source");
    let mut expected = public_roots.as_array().unwrap().clone();
    expected.extend([
        json!("adl/tools/batched_checks.sh"),
        json!("adl/tools/preflight_review.sh"),
    ]);
    expected.extend(public_inputs.as_array().unwrap().clone());
    assert_eq!(lane["path_selectors"], json!(expected));
    assert_eq!(lane["path_hints"], lane["path_selectors"]);
    assert_eq!(lane["run_command"], "bash adl/tools/batched_checks.sh");
    let runner = read("tools/public_adl/run_ci.sh");
    let packages = read("tools/public_adl/run_package_checks.sh");
    for package in public_packages.as_array().unwrap() {
        assert!(packages.contains(&format!("  {}\n", package.as_str().unwrap())));
    }
    for needle in ["cargo test --locked", "cargo fmt", "cargo clippy --locked"] {
        assert!(packages.contains(needle));
    }
    for needle in [
        "bash tools/public_adl/run_package_checks.sh",
        "cargo test --offline --locked",
        "--test public_adl_install_validator",
        "/usr/bin/sandbox-exec",
        "bash tools/public_adl/test_ci_routing.sh",
        "bash tools/public_adl/run_cli_ci.sh",
    ] {
        assert!(runner.contains(needle), "{needle}");
    }
    let policy = block(&workflow, "adl_path_policy");
    for needle in["transpiler_distribution_required: ${{ steps.path-policy.outputs.transpiler_distribution_required }}","TRANSPILE_REQUIRED: ${{ steps.path-policy.outputs.transpiler_distribution_required }}"]{assert!(policy.contains(needle));}
    assert!(!policy.contains("transpiler-selection"));
    let job = block(&workflow, "public_adl_validation");
    for needle in [
        "runs-on: macos-latest",
        "needs.adl_path_policy.outputs.public_adl_validation_required == 'true'",
        "CARGO_TARGET_DIR: ${{ runner.temp }}/public-adl-target",
        "ADL_PUBLIC_ADL_PROOF_ROOT: ${{ runner.temp }}/public-adl-proof",
        "bash tools/public_adl/run_ci.sh",
        "sys.version_info >= (3, 12)",
        "actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a",
        "path: ${{ runner.temp }}/public-adl-proof/1192/",
        "if-no-files-found: error",
    ] {
        assert!(job.contains(needle), "{needle}");
    }
    assert!(!workflow.contains("public_adl_validation:${{ needs.public_adl_validation.result }}"));
    assert!(workflow.contains("PUBLIC_RESULT: ${{ needs.public_adl_validation.result }}"));
    let coverage = block(&workflow, "public_adl_coverage");
    for needle in [
        "runs-on: ubuntu-latest",
        "needs.adl_path_policy.outputs.public_adl_validation_required == 'true'",
        "--mode pull_request --base \"$BASE_SHA\"",
        "--mode full",
        "--version 0.8.2",
        "if-no-files-found: error",
    ] {
        assert!(coverage.contains(needle), "{needle}");
    }
    for (name, lane, kind) in [
        ("adl-ci", "public_adl_validation", "ci"),
        ("adl-coverage", "public_adl_coverage", "coverage"),
    ] {
        let b = block(&workflow, name);
        for needle in["if: always()".into(),format!("      - {lane}\n"),"PATH_POLICY_RESULT: ${{ needs.adl_path_policy.result }}".into(),"PUBLIC_REQUIRED: ${{ needs.adl_path_policy.outputs.public_adl_validation_required }}".into(),format!("PUBLIC_RESULT: ${{{{ needs.{lane}.result }}}}"),format!("run: cargo run --locked --quiet --manifest-path tools/ci_contracts/Cargo.toml -- ci --aggregate {kind}")]{assert!(b.contains(&needle),"{needle}");}
    }
}
#[test]
fn nightly_wiring() {
    let workflow = read(".github/workflows/nightly-coverage-ratchet.yaml");
    assert!(workflow.contains("workflow_dispatch:") && !workflow.contains("  schedule:"));
    assert!(!workflow.contains("run_authoritative_coverage_lane"));
    assert!(workflow.contains("-- coverage --mode full --output \"$COVERAGE_OUTPUT\""));
    assert_eq!(workflow.matches("if: always()").count(), 2);
    let quality = workflow
        .split("      - name: Run public quality gates")
        .nth(1)
        .unwrap()
        .split("      - name:")
        .next()
        .unwrap();
    assert!(!quality.contains("for manifest in adl/Cargo.toml"));
    assert!(quality.contains("--manifest-path adl/Cargo.toml --package adl --lib --bins"));
    for target in [
        "adl_tests",
        "resolve_tests",
        "schema_tests",
        "public_process",
        "issue_327_removed_tooling",
        "public_observation",
    ] {
        assert!(quality.contains(&format!("--test {target}")));
    }
    assert!(workflow.contains("--version 0.8.2"));
    assert!(workflow.contains("if-no-files-found: error"));
    assert!(workflow.contains("-- coverage-report"));
}
