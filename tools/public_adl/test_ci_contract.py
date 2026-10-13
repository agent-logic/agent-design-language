#!/usr/bin/env python3
"""Focused contract for the public-only ADL validation lane."""

import re
import json
import os
import subprocess
import sys
import tempfile
import textwrap
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
PUBLIC_ROOTS = [
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
    "adl/tools/test_owner_binary_install.sh",
]
PUBLIC_INPUTS = ['adl/tests/fixtures/cli_smoke.adl.yaml', 'adl/examples/adl-0.1.yaml', 'adl/examples/v0-2-multi-step-basic.adl.yaml', 'adl/examples/v0-2-multi-step-file-input.adl.yaml', 'adl/examples/v0-2-coordinator-agents-sdk.adl.yaml']

PUBLIC_PACKAGES = [
    "adl-schema",
    "adl-uts",
    "adl-language",
    "adl-compiler",
    "adl-legacy-contracts",
]


def job_block(workflow, name):
    return re.search(r"^  " + re.escape(name) + r":\n.*?(?=^  [A-Za-z0-9_-]+:\n|\Z)", workflow, re.M | re.S).group(0)


def check_nightly_report() -> None:
    """PVF local_contract: deterministic offline report fixtures; required CI wiring gate."""
    workflow = (ROOT / ".github/workflows/nightly-coverage-ratchet.yaml").read_text()
    assert "workflow_dispatch:" in workflow and "  schedule:" not in workflow
    assert "run_authoritative_coverage_lane" not in workflow
    assert 'coverage.py --mode full --output "$COVERAGE_OUTPUT"' in workflow
    assert workflow.count("if: always()") == 2
    quality = workflow.split("      - name: Run public quality gates", 1)[1].split("      - name:", 1)[0]
    assert "for manifest in adl/Cargo.toml" not in quality
    assert "--manifest-path adl/Cargo.toml --package adl --lib --bins" in quality
    for target in ("adl_tests", "resolve_tests", "schema_tests", "public_process", "issue_327_removed_tooling", "public_observation"):
        assert "--test " + target in quality
    assert "--version 0.8.2" in workflow
    assert "if-no-files-found: error" in workflow
    script = textwrap.dedent(workflow.split("<<'PYREPORT'\n", 1)[1].split("          PYREPORT", 1)[0])
    with tempfile.TemporaryDirectory(prefix="nightly-report-") as temporary:
        root = Path(temporary)
        summary = root / "step-summary.md"
        env = dict(os.environ, COVERAGE_OUTPUT=str(root), RUNNER_TEMP=str(root),
                   GITHUB_STEP_SUMMARY=str(summary), COVERAGE_OUTCOME="success")
        data = {"schema": "adl.public_coverage.v1", "mode": "full", "source_head": "fixture",
                "line_count": 100, "covered_lines": 95, "status": "passed", "failures": [],
                "thresholds": {"full_file": 80, "full_aggregate": 90},
                "files": {"adl/src/lib.rs": {"covered": 95, "count": 100}}}
        result_path = root / "result.json"
        for case in ("passed", "failed", "missing"):
            if case == "missing":
                result_path.unlink()
            else:
                data["status"] = case
                data["failures"] = [] if case == "passed" else ["public aggregate below 90% line floor"]
                result_path.write_text(json.dumps(data))
            result = subprocess.run([sys.executable, "-B", "-c", script], env=env, capture_output=True, text=True)
            assert result.returncode == (0 if case == "passed" else 1), result.stderr
            report = (root / "nightly-public-coverage-report.md").read_text()
            expected = {"passed": "95.00%", "failed": "below 90% line floor", "missing": "Coverage result was not produced"}[case]
            assert expected in report
            assert report in summary.read_text()


def main() -> None:
    check_nightly_report()
    selector = json.loads(
        (ROOT / "adl/config/validation_lane_selector.public.v1.json").read_text()
    )
    lane = selector["special_surfaces"]["rust_pr_fast"]
    assert lane["planner"] == "manifest_command"
    assert lane["id"] == "public_source"
    assert lane["path_selectors"] == PUBLIC_ROOTS + ["adl/tools/batched_checks.sh", "adl/tools/preflight_review.sh"] + PUBLIC_INPUTS
    assert lane["path_hints"] == lane["path_selectors"]
    assert lane["run_command"] == "bash adl/tools/batched_checks.sh"

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
    policy_job = job_block(workflow, "adl_path_policy")
    assert "transpiler_distribution_required: ${{ steps.path-policy.outputs.transpiler_distribution_required }}" in policy_job
    assert "TRANSPILE_REQUIRED: ${{ steps.path-policy.outputs.transpiler_distribution_required }}" in policy_job
    assert "transpiler-selection" not in policy_job
    job = job_block(workflow, "public_adl_validation")
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
    assert "PUBLIC_RESULT: ${{ needs.public_adl_validation.result }}" in workflow

    coverage_job = job_block(workflow, "public_adl_coverage")
    assert "runs-on: ubuntu-latest" in coverage_job
    assert "needs.adl_path_policy.outputs.public_adl_validation_required == 'true'" in coverage_job
    assert "--mode pull_request --base \"$BASE_SHA\"" in coverage_job
    assert "--mode full" in coverage_job
    assert "--version 0.8.2" in coverage_job
    assert "if-no-files-found: error" in coverage_job
    for name, lane, kind in [("adl-ci", "public_adl_validation", "ci"), ("adl-coverage", "public_adl_coverage", "coverage")]:
        block = job_block(workflow, name)
        assert "if: always()" in block
        assert "      - " + lane + "\n" in block
        assert "PATH_POLICY_RESULT: ${{ needs.adl_path_policy.result }}" in block
        assert "PUBLIC_REQUIRED: ${{ needs.adl_path_policy.outputs.public_adl_validation_required }}" in block
        assert "PUBLIC_RESULT: ${{ needs." + lane + ".result }}" in block
        assert "run: python3 tools/public_adl/ci_policy.py --aggregate " + kind in block


if __name__ == "__main__":
    main()
