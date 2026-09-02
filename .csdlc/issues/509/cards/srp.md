# Structured Review Prompt

Template: 1.0.0

Issue: 509

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/prepared/issues/509/design.md
.csdlc/prepared/issues/509/diagram.mmd
.csdlc/prepared/issues/509/validate-implementation.rb
.csdlc/prepared/issues/509/validate-readiness.rb
adl-runtime/tests/distributed_contract/main.rs
adl-runtime/tests/distributed_contract/validate_drt_d.sh
adl/src/cli/csmctl_cmd.rs
adl/tools/build_issue509_runtime_bundle_on_relay.sh
adl/tools/run_issue268_six_resident_uts_cycle.py
adl/tools/run_issue509_gcp_drt_d_qualification.sh
docs/milestones/v0.92.1/evidence/runtime/drt-d/README.md
docs/milestones/v0.92.1/evidence/runtime/drt-d/qualification.json
infra/gcp/workloads/drt-d-six-resident/.gitignore
infra/gcp/workloads/drt-d-six-resident/.terraform.lock.hcl
infra/gcp/workloads/drt-d-six-resident/main.tf
infra/gcp/workloads/drt-d-six-resident/outputs.tf
infra/gcp/workloads/drt-d-six-resident/provider.tf
infra/gcp/workloads/drt-d-six-resident/startup-ollama.sh
infra/gcp/workloads/drt-d-six-resident/startup-runtime.sh
infra/gcp/workloads/drt-d-six-resident/terraform.tfvars.example
infra/gcp/workloads/drt-d-six-resident/variables.tf
infra/gcp/workloads/drt-d-six-resident/versions.tf
infra/gcp/workloads/modules/two-node-ollama-runtime/.gitignore
infra/gcp/workloads/modules/two-node-ollama-runtime/.terraform.lock.hcl
infra/gcp/workloads/modules/two-node-ollama-runtime/main.tf
infra/gcp/workloads/modules/two-node-ollama-runtime/outputs.tf
infra/gcp/workloads/modules/two-node-ollama-runtime/tests/issue509_launch_contract.tftest.hcl
infra/gcp/workloads/modules/two-node-ollama-runtime/variables.tf
infra/gcp/workloads/modules/two-node-ollama-runtime/versions.tf

## Prompts

- Are GCP account, project, billing, and credentials exact without exposing secrets?
- Does the packet preserve AWS authority?
- Are cost and cleanup-zero receipts independently proving?
- Does the design stop before paid live launch unless authorized?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Reviewer did not run paid/live GCP proof; live evidence is retained from run adl-509-drt-d-20260902192222.
- Cost evidence remains bounded-budget only: billing export was not read during qualification.

## Review Result

Revision: Some("git-blake3:c52cd4834b5f15e495d7ce64bf962b5daf3a3829:b77de19ded115ac1a6f4f6b52950a07952b9d511b0418660c93fc17506e36f02")

Reviewer: Some("fresh-session:210376aa-8441-4b7a-a378-6888278e7a42")

Result: pass
