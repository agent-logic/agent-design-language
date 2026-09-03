# GCP warm Polis snapshot runbook

Issue #663 provides the GCP equivalent of the two-node warm Runtime/Guardian plus GPU/Ollama topology. The inexpensive idle authority is two versioned Compute Engine snapshots. Zonal Persistent Disks and VMs exist only during preparation, verification, or an active launch.

The executable source of truth is [`infra/gcp/workloads/warm-polis`](../../../../infra/gcp/workloads/warm-polis/README.md). Use distinct remote Terraform state keys for `preparation`, `snapshot-catalog`, and `launch`. Never combine those states: ordinary launch cleanup must be incapable of deleting retained snapshots.

Preparation and live launch require an explicit company GCP project and operator spend authorization. Local `terraform test`, `terraform validate`, shell syntax, startup-policy, and snapshot-retirement checks do not create cloud resources.

Disposable hydration and verification VMs fail closed after the configured temporary-VM observation limit and are cleaned up without deleting completed snapshots. Active Runtime and GPU/Ollama nodes have no termination deadline. Launch receipts retain separate per-node `RUNNING`, guest-ready, guest boot-relative, Terraform-apply, and full-Polis timing fields.

Normal idle posture after a generation is prepared:

- Runtime snapshot retained;
- Ollama/model snapshot retained;
- no preparation VM;
- no verifier VM;
- no staging, verification, or launch Persistent Disk;
- no Runtime or GPU VM.

Snapshot retirement is a separate, exact-generation operation requiring both live-execution and snapshot-retirement authorization gates. Pass the expected generation and both exact snapshot IDs to `retire-snapshot-generation.sh`; any mismatch fails closed.
