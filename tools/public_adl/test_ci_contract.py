#!/usr/bin/env python3
"""Focused contract for the public-only ADL validation lane."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
PUBLIC_ROOTS = [
    "adl-schema/**",
    "adl-uts/**",
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
    "adl/tests/cli_smoke/process_status.rs",
    "adl/tests/helpers.rs",
    "adl/tools/install_owner_binaries.sh",
    "adl/tools/owner_binary_resolution.sh",
    "adl/tools/test_owner_binary_install.sh",
]
PUBLIC_PACKAGES = [
    "adl-schema",
    "adl-uts",
    "adl-language",
    "adl-compiler",
    "adl-legacy-contracts",
]


def main() -> None:
    selector = json.loads(
        (ROOT / "adl/config/validation_lane_selector.v0.91.6.json").read_text()
    )
    lane = next(item for item in selector["lanes"] if item["id"] == "public_adl_distribution")
    assert lane["path_selectors"] == PUBLIC_ROOTS
    assert lane["path_hints"] == PUBLIC_ROOTS
    assert lane["run_command"] == "bash tools/public_adl/run_ci.sh"

    runner = (ROOT / "tools/public_adl/run_ci.sh").read_text()
    for package in PUBLIC_PACKAGES:
        assert f"  {package}\n" in runner
    assert "cargo test --locked" in runner
    assert "cargo fmt" in runner
    assert "cargo clippy --locked" in runner
    assert "cargo test --offline --locked" in runner
    assert "--test public_adl_install_validator" in runner
    assert "/usr/bin/sandbox-exec" in runner
    assert "bash tools/public_adl/test_ci_routing.sh" in runner
    assert "bash tools/public_adl/run_cli_ci.sh" in runner

    workflow = (ROOT / ".github/workflows/ci.yaml").read_text()
    job = workflow.split("  public_adl_validation:\n", 1)[1].split("\n  adl_rust_fmt_clippy:\n", 1)[0]
    assert "runs-on: macos-latest" in job
    assert "needs.adl_path_policy.outputs.public_adl_validation_required == 'true'" in job
    proof_step = job.split("      - name: Five public packages and installed consumer proof\n", 1)[1].split("\n      - name:", 1)[0]
    assert "CARGO_TARGET_DIR: ${{ runner.temp }}/public-adl-target" in proof_step
    assert "ADL_PUBLIC_ADL_PROOF_ROOT: ${{ runner.temp }}/public-adl-proof" in proof_step
    assert "bash tools/public_adl/run_ci.sh" in job
    assert "sys.version_info >= (3, 12)" in job
    assert "actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a" in job
    assert "path: ${{ runner.temp }}/public-adl-proof/1192/" in job
    assert "if-no-files-found: error" in job
    assert "public_adl_validation:${{ needs.public_adl_validation.result }}" not in workflow
    assert "PUBLIC_ADL_VALIDATION_RESULT: ${{ needs.public_adl_validation.result }}" in workflow


if __name__ == "__main__":
    main()
