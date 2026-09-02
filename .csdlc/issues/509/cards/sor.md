# Structured Output Record

Template: 1.0.0

Issue: 509

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Finalize #509 DRT-D GCP portability after a live two-node GCP qualification run using the stored GCS Runtime/Ollama artifact bundle, six resident agent receipts, and clean teardown.

## Artifacts

- adl/src/cli/csmctl_cmd.rs
- adl/tools/build_issue509_runtime_bundle_on_relay.sh
- adl/tools/run_issue268_six_resident_uts_cycle.py
- adl/tools/run_issue509_gcp_drt_d_qualification.sh
- adl-runtime/tests/distributed_contract/main.rs
- adl-runtime/tests/distributed_contract/validate_drt_d.sh
- docs/milestones/v0.92.1/evidence/runtime/drt-d/README.md
- docs/milestones/v0.92.1/evidence/runtime/drt-d/qualification.json
- infra/gcp/workloads/drt-d-six-resident/main.tf
- infra/gcp/workloads/drt-d-six-resident/outputs.tf
- infra/gcp/workloads/drt-d-six-resident/startup-ollama.sh
- infra/gcp/workloads/drt-d-six-resident/startup-runtime.sh
- infra/gcp/workloads/drt-d-six-resident/variables.tf
- infra/gcp/workloads/drt-d-six-resident/versions.tf
- infra/gcp/workloads/modules/two-node-ollama-runtime/main.tf
- infra/gcp/workloads/modules/two-node-ollama-runtime/outputs.tf
- infra/gcp/workloads/modules/two-node-ollama-runtime/tests/issue509_launch_contract.tftest.hcl
- infra/gcp/workloads/modules/two-node-ollama-runtime/variables.tf
- infra/gcp/workloads/modules/two-node-ollama-runtime/versions.tf

## Execution

- Added the GCP DRT-D two-node Terraform module/root and startup scripts for a private Runtime node talking to a private Ollama GPU node.
- Added the #509 GCP qualification runner and relay bundle builder so Runtime/Ollama binaries and model artifacts are built once, stored in GCS, and consumed by live runs without repeated rebuilds.
- Reused the #268 six-resident UTS cycle on GCP with private Ollama-backed Runtime execution and retained the successful qualification receipt.
- Recorded successful live GCP proof for run adl-509-drt-d-20260902183922 with Terraform cleanup destroying the ephemeral run resources.

## Validation

[
  {
    "command": [
      "cargo",
      "check",
      "--manifest-path",
      "adl/Cargo.toml",
      "--bin",
      "csm"
    ],
    "purpose": "Run cargo check for the CSM binary.",
    "outcome": "passed",
    "evidence_ref": "issue509-csm-check.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Run git diff hygiene.",
    "outcome": "passed",
    "evidence_ref": "issue509-diff-hygiene.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/509/validate-implementation.rb"
    ],
    "purpose": "Run the issue-owned implementation validator.",
    "outcome": "passed",
    "evidence_ref": "issue509-implementation-validator.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/509/validate-readiness.rb"
    ],
    "purpose": "Run the issue-owned readiness validator.",
    "outcome": "passed",
    "evidence_ref": "issue509-readiness-validator.log"
  },
  {
    "command": [
      "bash",
      "-lc",
      "PYTHONPYCACHEPREFIX=.csdlc/evidence/509/pycache python3 -m py_compile adl/tools/run_issue268_six_resident_uts_cycle.py && bash -n adl/tools/build_issue509_runtime_bundle_on_relay.sh && bash -n adl/tools/run_issue509_gcp_drt_d_qualification.sh && bash -n infra/gcp/workloads/drt-d-six-resident/startup-runtime.sh && bash -n infra/gcp/workloads/drt-d-six-resident/startup-ollama.sh"
    ],
    "purpose": "Run syntax checks for #509 helper scripts.",
    "outcome": "passed",
    "evidence_ref": "issue509-script-syntax.log"
  },
  {
    "command": [
      "bash",
      "-lc",
      "terraform -chdir=infra/gcp/workloads/modules/two-node-ollama-runtime fmt -recursive -check && terraform -chdir=infra/gcp/workloads/drt-d-six-resident fmt -recursive -check && terraform -chdir=infra/gcp/workloads/modules/two-node-ollama-runtime test && terraform -chdir=infra/gcp/workloads/drt-d-six-resident validate"
    ],
    "purpose": "Run Terraform static validation for the GCP two-node module/root.",
    "outcome": "passed",
    "evidence_ref": "issue509-terraform-static.log"
  }
]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
