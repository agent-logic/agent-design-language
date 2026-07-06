# Runtime AWS Signal Integration

Issue `#4686` is the WP-08 integrated AWS signal proof path. It does not replace
the child publishers from `#4684` and `#4685`; it proves that both live AWS
signal paths can run together under one Agent Logic account guard:

- runtime heartbeat to CloudWatch Logs
- ACIP projection to SNS

## Live Proof

Build the repo-owned binaries first:

```bash
cargo build --manifest-path adl/Cargo.toml --bin csm --bin run_wp08_acip_sns_live_proof
```

Then run the integrated proof from a bound issue worktree:

```bash
ADL_AWS_SIGNAL_INTEGRATION_ACCOUNT_SHA256="<operator-approved-agent-logic-account-sha256>" \
  bash adl/tools/run_wp08_aws_signal_integration_live_proof.sh \
    --out docs/milestones/v0.91.7/review/runtime/wp08_aws_signal_integration_4686 \
    --profile agent-logic-admin \
    --region us-west-2 \
    --csm-bin adl/target/debug/csm \
    --acip-proof-bin adl/target/debug/run_wp08_acip_sns_live_proof
```

The wrapper verifies the selected AWS profile against the approved account hash
before child proof scripts can mutate AWS state. It then runs:

- `adl/tools/run_wp08_heartbeat_live_proof.sh`
- `adl/tools/run_wp08_acip_sns_live_proof.sh`

Child proof outputs are written to a temporary workspace and distilled into the
integrated summary. The `#4686` retained artifact is intentionally one redacted
summary rather than a copy of every child proof file.

The retained integrated summary is:

`docs/milestones/v0.91.7/review/runtime/wp08_aws_signal_integration_4686/aws_signal_integration_summary.json`

Validate it with:

```bash
python3 adl/tools/validate_wp08_aws_signal_integration_live_proof.py \
  docs/milestones/v0.91.7/review/runtime/wp08_aws_signal_integration_4686/aws_signal_integration_summary.json
```

## Redaction

The integrated summary records only short account/topic hashes, CloudWatch log
group/stream names, retention days, event count, SNS topic name, and SNS message
id. It must not record raw AWS account ids, full account digests, credentials,
raw SNS topic ARNs, or private ACIP content.

The child ACIP proof retains its own historical `#4685` summary shape when run
directly. The `#4686` integrated proof keeps those child outputs transient and
distills only the redacted fields needed to prove the combined path.

## Negative Cases

The integrated path records negative-case coverage from the child runtime tests
and wrapper contract:

- heartbeat missing approval and unsupported target fail closed in
  `runtime_aws_signal` tests
- ACIP missing profile, missing topic, denied projection, and SNS publish
  failure classes are retained from the ACIP/SNS proof
- account mismatch is covered by
  `adl/tools/test_run_wp08_aws_signal_integration_live_proof.sh`, which proves
  the wrapper stops before invoking child proof scripts
