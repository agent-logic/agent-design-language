# Issue 670 design: live GCP warm-Polis qualification

## Decision

Use the merged #663 three-state Terraform design unchanged:

1. `preparation/` owns temporary hydration VMs and staging disks.
2. `snapshot-catalog/` owns exactly one sealed Runtime/Guardian snapshot and one sealed Ollama/model snapshot for the generation.
3. `warm-polis/` owns disposable restored disks and the two live nodes.

The live target is only the Agent Logic company project `cs-poc-cha8mmii0xk0iaw5vpf8mxf`. The operator-authorized incremental budget ceiling is USD 20.00. The execution controller must record resource creation and deletion times, estimate incremental cost conservatively, and destroy issue-owned VMs and disks after proof. The two intended snapshots remain as the inexpensive idle-state artifacts.

## Execution sequence

1. Verify credential identity, billing, required APIs, aggregate GPU quota, regional NVIDIA L4 quota, the selected zone's L4 offering, existing resources, and estimated worst-case cost before mutation. GCP has no non-mutating guarantee of transient accelerator capacity, so the launch create operation is the fail-closed capacity proof; a capacity failure is recorded and cleaned up without a retry loop.
2. Reuse existing #509 networking, OS Login, service account, and artifact surfaces where they match the #663 manifest contract.
3. Hydrate and seal Runtime/Guardian and Ollama/model staging disks, then create the two versioned snapshots.
4. Remove all preparation and verification compute/disks.
5. Restore disposable launch disks, launch Runtime/Guardian and an L4-backed Ollama node, and record required timing events.
6. Prove private Runtime-to-Ollama connectivity, both resident models, and one real agent/tool-path smoke.
7. Destroy launch VMs and restored disks, retain only the two snapshots, inventory residual resources, and calculate actual incremental cost.

## Invariants

- Every paid mutation names the exact company project.
- Incremental cost must remain at or below USD 20.00; stop and clean up before a conservative projection can exceed it.
- Normal launch performs no Git operation, compilation, package installation, or model download.
- Ollama is private-only and OS Login remains the sole SSH authority.
- Failures are diagnostic evidence to repair, not permission to leave required proof red.
- Cleanup never deletes the retained generation snapshots and never touches unrelated pre-existing resources.

## Evidence

Durable, redacted evidence belongs under `.csdlc/evidence/670/`. It includes preflight, plan, launch, readiness, functional, timing, cleanup, residual-resource, and cost receipts. Credential contents and tokens are never recorded.
