# Structured Output Record

Template: 1.0.0

Issue: 670

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

The historical paid GCP run reached full two-node Polis readiness in 930 seconds, served six real Runtime agent and ACC tool cycles across two resident models, and ended with zero disposable resources plus two retained snapshots. The controller checkout was dirty during that run: HEAD was 542f8c1 and the receipt-producing changes were later committed at da2d976, so no immutable execution-source revision is claimed. A later paid g670c attempt did not complete qualification and was cleaned with zero disposable resources and the same two retained snapshots. The conservative envelope covers all paid attempts through that final cleanup at USD 13.90 against the authorized USD 20 ceiling. Later controller hardening is proven by focused static and unit validation and is not represented as a second successful paid run.

## Artifacts

- .csdlc/evidence/670/live/preflight.json
- .csdlc/evidence/670/live/snapshot-verification-g670b.json
- .csdlc/evidence/670/live/launch-g670b.json
- .csdlc/evidence/670/live/cleanup-g670b.json
- .csdlc/evidence/670/live/cleanup-g670c.json
- .csdlc/evidence/670/live/residual-inventory-g670b.json
- .csdlc/evidence/670/live/cost-upper-bound.json
- .csdlc/evidence/670/live/remediation-proof-boundary.json

## Execution

- Made the issue preflight use immutable company project, central region and zone, L4, USD 20, USD 2 hourly envelope, eight-hour paid window, and USD 4 storage reserve authority.
- Coupled preparation, snapshot catalog, and launch inputs to that fixed preflight authority before any paid create or apply while keeping exact-project cleanup independently available.
- Propagated the single absolute preflight deadline to Runtime, Ollama, preparation, and verifier guests; every boot schedules poweroff using only the remaining authorized time.
- Bound every paid Terraform apply to the same absolute deadline while leaving cleanup unbounded so deadline expiry cannot prevent resource destruction.
- Ran each paid Terraform operation in an isolated process group, applied bounded TERM grace followed by KILL, and unit-tested a parent that exits while its TERM-resistant provider child remains.
- Made the deadline guard wait briefly after force-killing the process group and made the TERM-resistant child regression poll bounded process exit instead of relying on one immediate kill-zero check.
- Made failed launch and preparation cleanup produce durable cleanup-pending receipts, verify residual disposable inventory, and fail distinctly instead of suppressing destroy errors.
- Hardened snapshot preparation for private Google access, isolated Terraform state, portable Ollama bundles, serial receipt propagation, and cleanup-safe failure handling.
- Made Runtime startup compatible with the sealed artifact ABI, supervised Guardian independently of Vector, and ran the six-resident real agent and ACC tool qualification against private Ollama.
- Bound readiness observations to the latest boot ID and artifact generation so stale serial output cannot satisfy a new launch.
- Expanded the durable launch receipt with project, topology, snapshots, GPU, network, resident-model, six-agent, boot-identity, and timing truth.
- Made cleanup enumerate issue and generation scoped instances, disks, firewalls, images, and addresses, prove the exact two-snapshot retained set, and independently retained a matching live residual inventory.
- Configured separately managed warm data-disk attachments to be ignored by instance drift reconciliation, preventing Terraform from planning their removal.
- Made the focused validator initialize all Terraform roots itself and then execute format, validate and test lanes, shell policy tests, receipt parsing, and diff hygiene.
- Recorded that the historical successful qualification used a dirty controller checkout at HEAD 542f8c1, with receipt-producing logic later committed at da2d976, so the live evidence is not attributed to an immutable execution-source revision.
- Recorded the unsuccessful paid g670c attempt, its successful zero-residual cleanup, and the extended USD 13.90 conservative cost envelope through final cleanup.
- Added .csdlc/local/ to .gitignore so command-scoped local gcloud configuration stays out of publication artifacts.

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/670/validate-preparation.sh"
    ],
    "purpose": "Execute Terraform initialization and tests, a bounded forced-termination unit path, destroy-only cleanup and state-independent teardown policy checks, explicit historical-live/current-static evidence-boundary assertions, warm-start checks, and diff hygiene.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/prepared/issues/670/validate-preparation.sh"
  },
  {
    "command": [
      "bash",
      "infra/gcp/workloads/warm-polis/tests/validate-deadline-guard.sh",
      "x5"
    ],
    "purpose": "Prove the deadline guard deterministically handles a TERM-resistant provider child after R7 found a flaky orphan check.",
    "outcome": "passed",
    "evidence_ref": "terminal-output: five consecutive issue670_deadline_guard=pass runs"
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
    "purpose": "Prove a clean real private two-node L4 launch, two simultaneously resident models, Runtime and Guardian readiness, and six successful agent and ACC tool cycles.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/670/live/launch-g670b.json"
  },
  {
    "command": [
      "bash",
      "infra/gcp/workloads/warm-polis/run-live-snapshot-launch.sh",
      "destroy"
    ],
    "purpose": "Destroy the launch topology and prove its named instances and restored disks are absent and exactly the intended two snapshots remain.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/670/live/cleanup-g670b.json"
  },
  {
    "command": [
      "bash",
      "infra/gcp/workloads/warm-polis/tests/validate-live-residual-inventory.sh"
    ],
    "purpose": "Independently prove zero disposable resources and exactly two READY retained snapshots.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/670/live/residual-inventory-g670b.json"
  },
  {
    "command": [
      "jq",
      "-e",
      "launch and cleanup receipt invariants"
    ],
    "purpose": "Prove exact six-agent execution, two resident models, private-only networking, launch timing, cleanup, and snapshot-set truth.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/670/live/launch-g670b.json"
  }
]

## Integration

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
