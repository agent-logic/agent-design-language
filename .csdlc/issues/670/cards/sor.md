# Structured Output Record

Template: 1.0.0

Issue: 670

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Live-qualified the snapshot-backed two-node GCP Polis in the authorized company project, repaired only launch-path blockers, proved six real agent/tool cycles across two resident models, destroyed all issue-owned compute and restored disks, and retained exactly the two verified snapshots within the USD 20 ceiling.

## Artifacts

- .csdlc/evidence/670/live/preflight.json
- .csdlc/evidence/670/live/snapshot-verification-g670b.json
- .csdlc/evidence/670/live/launch-g670b.json
- .csdlc/evidence/670/live/cleanup-g670b.json
- .csdlc/evidence/670/live/cost-upper-bound.json

## Execution

- Made live preflight require an explicit target region and zone and carry issue identity and budget into Terraform.
- Hardened snapshot preparation for private Google access, isolated Terraform state, portable Ollama bundles, serial receipt propagation, and no artificial preparation deadline.
- Made Runtime startup compatible with the sealed artifact ABI, supervised Guardian independently of Vector, emitted console diagnostics, and ran the six-resident real agent/tool qualification against private Ollama.
- Reduced staging and disposable disk sizes to the measured requirements and made launch and cleanup non-interactive and receipt-backed.

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/670/validate-preparation.sh"
    ],
    "purpose": "Prove preparation contract, shell syntax, Terraform formatting and validity, warm-start and retirement policies, receipt JSON, and diff hygiene.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/prepared/issues/670/validate-preparation.sh"
  },
  {
    "command": [
      "bash",
      "infra/gcp/workloads/warm-polis/prepare-snapshot-generation.sh",
      ".csdlc/evidence/670/live/preparation.tfvars",
      ".csdlc/evidence/670/live/snapshot-catalog.tfvars"
    ],
    "purpose": "Prove both sealed snapshots are READY, restore-verified, generation-bound, and exactly sized.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/670/live/snapshot-verification-g670b.json"
  },
  {
    "command": [
      "bash",
      "infra/gcp/workloads/warm-polis/run-live-snapshot-launch.sh",
      "launch"
    ],
    "purpose": "Prove a real private two-node L4 launch, two simultaneously resident models, Runtime and Guardian health, and six successful agent/tool cycles.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/670/live/launch-g670b.json"
  },
  {
    "command": [
      "bash",
      "infra/gcp/workloads/warm-polis/run-live-snapshot-launch.sh",
      "destroy"
    ],
    "purpose": "Prove issue-owned compute and restored disks are absent, exactly two intended snapshots remain, and the conservative cost upper bound is below USD 20.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/670/live/cleanup-g670b.json"
  },
  {
    "command": [
      "terraform",
      "-chdir=infra/gcp/workloads/modules/two-node-ollama-runtime",
      "test"
    ],
    "purpose": "Prove the reusable two-node Ollama and Runtime module still satisfies its prepared-artifact launch contract.",
    "outcome": "passed",
    "evidence_ref": "infra/gcp/workloads/modules/two-node-ollama-runtime/tests/issue509_launch_contract.tftest.hcl"
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
