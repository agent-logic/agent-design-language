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

Paid execution is separate. Retain an authorization JSON file that binds the
exact reviewed commit and revision, unique run id, model set, instance type,
expiry, deadline, hourly ceiling, and total-cost ceiling, then run:

```bash
bash adl/tools/run_issue345_aws_gpu_shepherd_proof.sh run \
  --commit EXACT_REVIEWED_SHA \
  --run-id adl-issue345-YYYYMMDD-HHMMSS \
  --authorization-file /private/operator/path/issue345-authorization.json \
  --execute
```

The runner attempts at most one On-Demand GPU instance and has no fallback or
retry. The reviewed commit must equal the tracked-clean checkout HEAD. The
guest runs one current governed Shepherd request for every configured model,
keeps each model loaded, and only then accepts `/api/ps` proof when the complete
configured set is simultaneously GPU-resident.

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
drift. The deadline reaper check binds code digest, role, least-privilege
policy, schedule, target, and invoke permission. Missing or drifted resources
stop before paid mutation.

The deterministic local contract check is:

```bash
bash adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh
```

It uses the real Git checkout, no fake AWS responses, and no paid launch. Set
`ADL_ISSUE345_LIVE_PREFLIGHT=1` with the required non-secret inputs to require
the real read-only AWS preflight. Without that opt-in the result explicitly
reports `live_aws_preflight: "not_run"`; it is not AWS proof.

The final receipt records authorization and model-set digests, instance type,
elapsed seconds, observed hourly rate, estimated compute cost, authorized
deadline and total-cost ceiling, every governed model proof, simultaneous
residency, and cleanup. It excludes raw account, instance, security-group,
subnet, lock-version, and owner-token identifiers.
