# AWS GPU Shepherd proof runner

Issue #345 owns an optional AWS GPU portability proof for the governed Shepherd
adapter. It is not a Runtime dependency, production inference fallback, public
endpoint, or #256 birthday-demo execution path.

The read-only command is:

```bash
bash adl/tools/run_issue345_aws_gpu_shepherd_proof.sh preflight
```

Raw local state stays under the Git common directory
(`.git/csdlc-v2/issue345/aws-gpu-state`) or an explicit
`ADL_ISSUE345_STATE_ROOT`. Public evidence retains only redacted summaries.
Preflight verifies the `agent-logic-admin` business account, exact instance
profile and policy set, zero-ingress security group, immutable S3 artifacts,
deadline reaper, DLAMI and subnet resolution, quota, price, total cost, and
absence of stale issue compute. It never launches compute.

The artifact manifest schema is `adl.shepherd.portable_model_bundle.v2`.
`models` and `ADL_ISSUE345_MODEL_IDENTITIES_JSON` must identify the same unique
set of at least two models. Each model has one version-pinned
`ollama_model_store` archive, plus exactly one Ollama runtime and rustup
installer.

Paid execution is separate. Retain a
`adl.issue345.paid_run_authorization.v2` JSON file that binds the exact reviewed
commit and revision, unique run id, model set, business-account hash, immutable
manifest coordinates and digest, exact IAM/security-group/reaper pins, resolved
AMI and subnet hashes, instance type, expiry, deadline, 300-second reaper-lag
allowance, maximum billable seconds, hourly ceiling, total-cost ceiling, and
conservative gp3/public-IPv4/request overheads. The default 200 GiB encrypted
gp3 root volume provides more than
10x the approximately 9.9 GB compressed two-model store while leaving room for
extraction and Runtime builds. Then run:

```bash
bash adl/tools/run_issue345_aws_gpu_shepherd_proof.sh run \
  --commit EXACT_REVIEWED_SHA \
  --run-id adl-issue345-YYYYMMDD-HHMMSS \
  --authorization-file /private/operator/path/issue345-authorization.json \
  --execute
```

The authorized reviewed revision must exactly equal the current passing typed
C-SDLC review, and the reviewed source commit must remain unchanged across all
substantive proof surfaces at the lifecycle head. The runner resolves the
security group, AMI, and subnet once, verifies those exact values through the
authorized preflight, and reuses the same values for `run-instances`.

Before acquiring compute, the runner writes a create-only, versioned S3 marker
keyed by the canonical JSON digest of the authorization. JSON whitespace and
object-key order therefore cannot create a second consumption identity. Cleanup
never deletes that marker, so the same authorization cannot be replayed from
another checkout or host. A lock collision is checked before consuming the
authorization.

The runner attempts at most one On-Demand GPU instance and has no fallback or
retry. The reviewed commit must equal the tracked-clean checkout HEAD. On that
one host, the guest proves the Guardian-supervised Runtime v3 lifecycle, runs
the current governed Shepherd adapter once per configured model, and runs real
long-lived Runtime agents whose Ollama responses are compiled and executed
through UTS, ACC, the Freedom Gate, and the harmless `runtime.observe` adapter.
It keeps every configured model loaded and only accepts `/api/ps` proof when
the complete configured set is simultaneously GPU-resident.

Runtime v3 does not yet expose an Ollama provider ingress. The receipt therefore
records the Guardian/Runtime-v3 and governed-agent/model/tool paths separately
and explicitly sets `runtime_v3_to_ollama_transit_proved` to `false`. Component
co-location is not represented as a transitive integration claim.

Cleanup is owner-bound and re-verifies the AWS account before mutation:

```bash
bash adl/tools/run_issue345_aws_gpu_shepherd_proof.sh cleanup \
  --run-id adl-issue345-YYYYMMDD-HHMMSS \
  --owner-token OWNER_TOKEN \
  --lock-version-id LOCK_VERSION
```

Required non-secret preflight inputs are:

- `ADL_ISSUE345_EXPECTED_ACCOUNT_SHA256`
- `ADL_ISSUE345_ARTIFACT_BUCKET`
- `ADL_ISSUE345_ARTIFACT_MANIFEST_VERSION_ID`
- `ADL_ISSUE345_ARTIFACT_MANIFEST_SHA256`
- `ADL_ISSUE345_MODEL_IDENTITIES_JSON`
- `ADL_ISSUE345_INSTANCE_REQUIRED_INLINE_POLICY_SHA256`
- `ADL_ISSUE345_DEADLINE_REAPER_CODE_SHA256_B64`

The default role policy set is exactly `ADLIssue345ArtifactReadOnly` plus the
managed-policy ARN for `AmazonSSMManagedInstanceCore`; extra policies are
drift. Preflight also verifies the exact EC2 and Lambda assume-role trust
policies. The deadline reaper check binds code digest, role, least-privilege
policy, schedule, target, and invoke permission. Any active issue instance or
issue-tagged EBS volume, missing resource, or drift stops before paid mutation.

The deterministic local contract check is:

```bash
bash adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh
```

It uses the real Git checkout, no fake AWS responses, and no paid launch. Its
isolated executable fixtures cover canonical authorization identity, EC2 and
Lambda trust drift, and cleanup-owner mismatch; malformed paid invocations fail
before AWS access. Set
`ADL_ISSUE345_LIVE_PREFLIGHT=1` with the required non-secret inputs to require
the real read-only AWS preflight. Without that opt-in the result explicitly
reports `live_aws_preflight: "not_run"`; it is not AWS proof.

The final receipt records authorization and model-set digests, instance type,
elapsed seconds, observed hourly rate, estimated cost bounds, every governed
model proof, the redacted Runtime-agent ACC receipts, simultaneous residency,
the explicit Runtime-v3 provider-boundary non-claim, and cleanup. It excludes
raw account, instance, security-group, subnet, lock-version, and owner-token
identifiers.
