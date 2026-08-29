# Issue #345 design — AWS GPU Shepherd proof runner

## Purpose

Recover the bounded AWS GPU Shepherd runner retained at commit
`7a26886c47962e71c128489f5176a045ae8e9a64`, rename and harden it as an
issue-#345-owned command, and prove that the current governed Shepherd adapter
can execute one real model-backed request on Agent Logic-owned AWS GPU compute.

This is an optional remote portability proof. It does not replace the local
Runtime-to-Shepherd acceptance path, make AWS a Runtime dependency, or grant
standing authority for paid launches.

## Authority boundary

Issue #345 owns only:

- `adl/tools/run_issue345_aws_gpu_shepherd_proof.sh`
- `adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh`
- `.csdlc/prepared/issues/345/**`
- `.csdlc/evidence/345/**`
- focused documentation that describes this optional proof lane

The historical `run_wp5795_aws_gpu_proof.sh` and its retained evidence are
read-only recovery inputs. Runtime, governed admission, Shepherd adapter, local
model, Observatory, IAM, security-group, quota, and S3 artifact authorities
remain with their existing owners. The runner may verify pre-provisioned AWS
resources but may not create or broaden IAM roles, instance profiles, security
groups, or public ingress.

## Execution design

The runner exposes three explicit actions: `preflight`, `run`, and `cleanup`.
Preflight is read-only and verifies the `agent-logic-admin` profile resolves to
the approved business account, the selected region and GPU instance type are
available, the expected permanent instance profile and policies match exactly,
the named security group has no ingress, the immutable S3 manifest and object
versions match their pinned digests, the quota and current On-Demand price fit
the declared bounds, and no stale issue-owned compute remains.

Paid execution requires a full commit SHA, unique run ID, explicit `--execute`,
and a separately recorded operator authorization and cost ceiling. One
On-Demand GPU instance is launched with encrypted delete-on-termination root
storage, no public ingress, SSM transport, exact issue/run/owner/deadline tags,
and no fallback or retry to a different purchase option. The guest restores
the exact versioned model/runtime artifacts, checks every digest, checks the
GPU driver and model residency, builds or reuses only revision-bound Runtime
artifacts, and invokes the current governed Shepherd proof at the requested
repository revision.

Three independent cleanup layers remain mandatory: local trap cleanup, a guest
deadline timer, and the pre-existing tag-scoped deadline reaper. Cleanup is
owner-bound and idempotent. Any uncertainty about account, resource identity,
lock ownership, proof revision, artifact identity, deadline, or cleanup is a
terminal refusal rather than authority to continue.

## Evidence and privacy

The public evidence packet records only bounded redacted facts: source
revision, model/backend/artifact digests, GPU class and residency, proof result,
elapsed time, estimated compute cost, cleanup disposition, and hashes of any
private raw logs. It must not retain credentials, tokens, account IDs, raw AWS
resource IDs, model prompts, model output, private paths, or environment dumps.

Deterministic local tests exercise parsing, preflight refusal, account/profile
mismatch, security-group ingress, IAM drift, stale revision, lock collision,
deadline handling, interruption, cleanup failure, and redaction without AWS
mutation. A paid pass is valid only when the exact reviewed runner revision and
current Shepherd adapter revision produce a real model-backed receipt and all
temporary resources are then absent.

## Dependencies

- Historical recovery commit `7a26886c47962e71c128489f5176a045ae8e9a64`
- Current governed Shepherd adapter and local proof contract derived from the
  former #5795 lane
- Existing Agent Logic business-account profile and operator-provisioned IAM,
  security-group, S3, quota, and deadline-reaper resources
- #256 only as a downstream birthday-demo consumer; #345 does not absorb or
  execute #256

Preparation and deterministic tests require no paid AWS authority. Any live
launch remains separately operator-authorized and budget-bound.

## Rollback

Rollback removes or disables only the issue-#345 runner and its optional proof
documentation. It does not alter Runtime, the local Shepherd adapter, S3 model
artifacts, or permanent business-account controls. Cleanup must still finish
before rollback can be considered complete.
