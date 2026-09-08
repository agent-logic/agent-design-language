# Structured Task Prompt

Template: 1.0.0

Issue: 663

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only GCP warm two-node parity and its focused proof; do not redesign Runtime, re-run #509 from scratch, change AWS, or add production HA.

## Deliverables

- snapshot inputs plus disposable Runtime and Ollama/model disk restoration and attachment in the reusable GCP two-node module
- preparation root owning only hydration VMs and staging disks
- snapshot-catalog root owning versioned snapshots and temporary restored-content verifier resources
- launch root owning the controller, disposable restored disks, Runtime VM, and GPU VM
- infra/gcp/workloads/warm-polis/retire-snapshot-generation.sh guarded snapshot-retirement command requiring exact snapshot IDs and generation
- .csdlc/prepared/issues/663/validate-preparation.sh
- infra/gcp/workloads/warm-polis/tests/validate-snapshot-retirement.sh
- infra/gcp/workloads/warm-polis/tests/validate-warm-start-policy.sh
- infra/gcp/workloads/warm-polis/run-live-snapshot-launch.sh
- startup scripts and machine-readable timing receipts
- focused Terraform tests and operator documentation

## Acceptance

1. AC-1: The GCP two-node module supports disposable Runtime and Ollama/model disks restored from exact versioned snapshots; ordinary workload teardown deletes those disks and cannot delete the source snapshots, while a separate retirement command requires the exact expected snapshot IDs and generation and fails closed on mismatch.
2. AC-2: Normal startup performs no Git operation, Rust build, package installation, Ollama pull, model download, or mutable dependency resolution.
3. AC-3: Both nodes mount snapshot-restored content deterministically and fail closed on missing or mismatched snapshot and artifact generation identity.
4. AC-4: Runtime and Guardian start against a private-only Ollama endpoint and existing OS Login/SSH authority remains intact.
5. AC-5: The configured L4 node can make llama3.1:8b and qwen3:8b simultaneously resident from snapshot-restored local content.
6. AC-6: Start receipts distinguish start request, RUNNING, guest start, GPU ready, Runtime ready, and full Polis ready timing denominators.
7. AC-7: Focused local tests prove snapshot-to-disk restoration, teardown deletion of restored disks, snapshot retention, forbidden startup actions, topology, private networking, and receipt contracts.
8. AC-8: If separately authorized, one live GCP snapshot-restore run records actual launch-to-ready time and cleanup truth; otherwise live timing remains explicitly deferred.
9. AC-9: AWS #607 and completed GCP #495/#509 surfaces remain behaviorally unchanged.

## Dependencies

- #607 terminal AWS warm-start design
- #509 terminal GCP two-node qualification
- #495 terminal retained Runtime-disk portability

## Inputs

- infra/gcp/workloads/modules/two-node-ollama-runtime
- infra/gcp/workloads/drt-d-six-resident
- infra/gcp/workloads/xcl-01
- infra/aws/runtime/gpu-proof/warm-storage
- agent-logic/agent-design-language#663

## Non Goals

- production HA, autoscaling, load balancing, DNS, or 24/7 cutover
- AWS changes
- GPU suspend or VRAM preservation
- re-running the full six-resident qualification
- a new networking, identity, or SSH architecture
