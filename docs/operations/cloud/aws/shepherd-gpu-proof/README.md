# AWS GPU Shepherd proof runner

Issue #345 owns an optional AWS GPU portability proof for the governed Shepherd
adapter. It is not a Runtime dependency, production inference fallback, public
endpoint, or #256 birthday-demo execution path.

The issue-owned command is:

```bash
bash adl/tools/run_issue345_aws_gpu_shepherd_proof.sh preflight
```

The preflight path is read-only. By default, raw local run state is kept under the Git common directory (`.git/csdlc-v2/issue345/aws-gpu-state`) or an explicit `ADL_ISSUE345_STATE_ROOT`; public issue evidence should retain only redacted summaries. It verifies the approved `agent-logic-admin`
profile, `us-west-2`, the operator-provided account hash, a named zero-ingress
security group, a permanent instance profile plus exact role/policy contract,
an issue-scoped enabled deadline reaper, versioned immutable model artifacts,
DLAMI/subnet resolution, GPU quota, bounded On-Demand price, and absence of
stale issue-owned compute. It emits redacted machine-readable JSON:
raw AWS account ids, resource ids, tokens, prompts, responses, private paths,
and environment dumps are not public evidence.

Paid execution is intentionally separate:

```bash
ADL_ISSUE345_PAID_RUN_AUTHORIZATION=authorized \
  bash adl/tools/run_issue345_aws_gpu_shepherd_proof.sh run \
  --commit EXACT_REVIEWED_SHA \
  --run-id adl-issue345-YYYYMMDD-HHMMSS \
  --execute
```

The paid path must not run until a fresh exact-head launch-readiness review
passes and the operator authorizes one exact commit, run id, deadline, and cost
ceiling. The runner launches at most one On-Demand GPU instance and uses the
pre-provisioned account/IAM/security-group/artifact/reaper inputs; it does not
create or broaden IAM, security groups, public ingress, quotas, or permanent
cloud resources. The `--commit` value must match the currently checked-out,
freshly reviewed HEAD in the issue worktree before launch.

Cleanup is owner-bound:

```bash
bash adl/tools/run_issue345_aws_gpu_shepherd_proof.sh cleanup \
  --run-id adl-issue345-YYYYMMDD-HHMMSS \
  --owner-token OWNER_TOKEN \
  --lock-version-id LOCK_VERSION
```

Required non-secret inputs for read-only preflight are `ADL_ISSUE345_EXPECTED_ACCOUNT_SHA256`, `ADL_ISSUE345_ARTIFACT_BUCKET`, `ADL_ISSUE345_ARTIFACT_MANIFEST_VERSION_ID`, and `ADL_ISSUE345_ARTIFACT_MANIFEST_SHA256`. The instance profile contract defaults to `ADLRemoteValidationPermanentRole`, `ADLIssue345ArtifactReadOnly`, and `AmazonSSMManagedInstanceCore`, and may be overridden with the corresponding `ADL_ISSUE345_INSTANCE_PROFILE_ROLE`, `ADL_ISSUE345_INSTANCE_REQUIRED_INLINE_POLICIES`, and `ADL_ISSUE345_INSTANCE_REQUIRED_MANAGED_POLICIES` variables when the operator-approved permanent profile uses different names. The command fails closed before paid AWS mutation if any are absent or drifted.

The deterministic local contract test is:

```bash
bash adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh
```

That test uses a fake `aws` executable and performs no AWS mutation or paid
launch.
