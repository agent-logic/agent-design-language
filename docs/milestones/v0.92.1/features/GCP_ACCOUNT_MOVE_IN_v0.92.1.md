# GCP Account Move-In

v0.92.1 establishes a controlled GCP foundation before treating GCP as a normal Runtime execution environment. The lane preserves the current proof-of-concept estate unless explicitly admitted, separates human and workload authority, uses reviewed Terraform state, defaults to private ingress, and retains hard cost and cleanup boundaries.

The move-in foundation is five ordered, independently finishable issues:

1. **GCP-A — hierarchy and cost decision:** bind the exact organization, folders, projects, billing, region, quota, naming, data-residency, credit-expiry, and first-workload cost envelope from read-only observations.
2. **GCP-B — Terraform bootstrap:** establish a recoverable remote-state backend and short-lived deployment identity with pinned providers and reviewed-plan recovery.
3. **GCP-C — organization and billing baseline:** implement the approved folder/project/billing/group/budget/export/label baseline without accidentally changing unrelated POC resources.
4. **GCP-D — private platform foundation:** establish private network/operator access, storage-class ownership, telemetry, watchdog, and a zero-residue disposable non-GPU workload.
5. **GCP-E — GPU readiness smoke test:** under separate paid authorization, qualify one exact On-Demand L4 shape with a USD 20 ceiling and independent zero-resource proof.

**XCL-01** then provides the portable Runtime workload contract and AWS/GCP Terraform implementations. **DRT-D** remains the later six-resident portability qualification and consumes reviewed merged GCP-E and XCL-01 evidence; it does not execute #269 and does not replace AWS qualification authority.

Production hierarchy, production traffic, broad Shared VPC/centralized platform expansion, and speculative quota acquisition remain separately gated. Planning grants no GCP, billing, IAM, API, DNS, Terraform-apply, or paid-run authority.
