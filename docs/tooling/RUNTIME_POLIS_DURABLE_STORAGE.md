# Runtime Polis Durable Storage Proof

This is the WP-08 runtime path for proving Polis state artifact storage against
the Agent Logic S3 archive bucket created by `#4688`.

The runtime-owned command is:

```bash
csm storage prove-s3 \
  --out docs/milestones/v0.91.7/review/runtime/csm_polis_storage_4913 \
  --bucket <wp08-obsmem-archive-bucket> \
  --prefix community-memory/ \
  --profile agent-logic-admin \
  --region us-west-2 \
  --expected-account-sha256 <approved-agent-logic-account-sha256> \
  --run-id wp08-4913-polis-storage \
  --json
```

For normal issue execution, use the wrapper from a bound worktree:

```bash
ADL_AWS_PROFILE=agent-logic-admin \
AWS_PROFILE=agent-logic-admin \
ADL_AWS_POLIS_STORAGE_ACCOUNT_SHA256=<approved-agent-logic-account-sha256> \
bash adl/tools/run_wp08_polis_storage_live_proof.sh \
  --out docs/milestones/v0.91.7/review/runtime/csm_polis_storage_4913 \
  --profile agent-logic-admin \
  --region us-west-2
```

The wrapper derives the default bucket from the approved account hash:

```text
adl-wp08-obsmem-community-archive-<account-hash>-us-west-2
```

## Proof Scope

The proof writes one synthetic Polis snapshot artifact under
`community-memory/polis-state/<run-id>/snapshot.json`, records SHA-256 metadata,
reads S3 object metadata, restores the object to a clean local staging path, and
checks the restored checksum. It also records bounded negative cases for a
missing object, corrupted local restore, and unsigned/public access denial.

The retained summary is:

```text
polis_storage_proof_summary.json
```

The proof also writes:

```text
artifact_durability_taxonomy.json
polis_state_snapshot.json
restore/polis_state_snapshot.restored.json
```

## Validation

Wrapper contract:

```bash
bash adl/tools/test_run_wp08_polis_storage_live_proof.sh
```

Retained live proof validation:

```bash
python3 adl/tools/validate_wp08_polis_storage_live_proof.py \
  docs/milestones/v0.91.7/review/runtime/csm_polis_storage_4913/polis_storage_proof_summary.json
```

Focused Rust proof:

```bash
cargo test --manifest-path adl/Cargo.toml csm_polis_storage -- --nocapture
```

## Claim Boundary

This is a live object-level proof, not a mathematical proof of AWS durability.
The selected single-region S3 backend is recorded as vendor 11-nines
per-object durability with versioning, governance Object Lock, SSE-S3,
public-access block, and lifecycle controls. The summary must explicitly retain
the non-claim that this is not a mathematical 12-nines proof.

Do not use a personal/default AWS profile for this proof. Do not retain raw AWS
account ids, credentials, raw ARNs, or raw AWS error payloads in tracked
evidence.
