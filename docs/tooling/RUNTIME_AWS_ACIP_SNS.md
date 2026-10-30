# Runtime AWS ACIP To SNS Proof

This is the reusable WP-08 path for proving that a bounded ACIP signal can be
projected to AWS SNS in the Agent Logic business account.

`csm` is the runtime owner for the proof command:

```bash
cargo build --manifest-path adl/Cargo.toml --bin csm
csm aws-signal acip-sns-proof --out <proof-dir>
```

The wrapper below uses `adl/target/debug/csm` by default and invokes
`csm aws-signal acip-sns-proof`. The historical
`run_wp08_acip_sns_live_proof` binary remains only as a compatibility shim over
the same implementation.

## Live Proof

Use the repo-owned wrapper from a bound issue worktree:

```bash
ADL_AWS_PROFILE=agent-logic-admin \
AWS_PROFILE=agent-logic-admin \
ADL_AWS_ACIP_SNS_ACCOUNT_SHA256=<approved-agent-logic-account-sha256> \
bash adl/tools/run_wp08_acip_sns_live_proof.sh \
  --out docs/milestones/v0.91.7/review/runtime/wp08_acip_sns_4685 \
  --profile agent-logic-admin \
  --region us-west-2
```

The wrapper creates or reuses SNS topic
`adl-v0917-wp08-acip-sns-4685`, runs the repo-owned `csm` proof command, and
writes:

- `acip_sns_summary.json`
- `sns_resource_summary.json`

The summaries record short account/topic hashes, SNS message id, ACIP
projection metadata, and redaction status. They must not record AWS
credentials, raw account ids, full account digests, raw topic ARNs, or raw
private ACIP content. The wrapper verifies the selected AWS profile against the
approved full account SHA-256 before creating or publishing to SNS, but that
full digest is input-only guard material and is not retained in proof summaries.

## Validation

Fast wrapper-contract proof:

```bash
bash adl/tools/test_run_wp08_acip_sns_live_proof.sh
```

Retained live summary validation:

```bash
python3 adl/tools/validate_wp08_acip_sns_live_proof.py \
  docs/milestones/v0.91.7/review/runtime/wp08_acip_sns_4685/acip_sns_summary.json \
  docs/milestones/v0.91.7/review/runtime/wp08_acip_sns_4685/sns_resource_summary.json
```

Focused Rust proof:

```bash
cargo test --manifest-path adl/Cargo.toml runtime_aws_signal -- --nocapture --test-threads=1
cargo test --manifest-path adl/Cargo.toml --bin adl cli::csm_cmd::tests:: -- --nocapture
```

## Operational Notes

- Use `agent-logic-admin`; do not use a personal/default AWS account for ADL
  runtime proof.
- Set `ADL_AWS_ACIP_SNS_ACCOUNT_SHA256` or pass
  `--expected-account-sha256`; the wrapper fails before SNS mutation when the
  profile does not match.
- `ADL_AWS_SIGNAL_APPROVED=true` is required for live publishing.
- Missing profile, missing topic, denied projection, access denial, and SNS
  publish failure all fail closed with machine-readable failure classes.
- Use `--cleanup` only for disposable topic tests. Cleanup is registered as an
  exit trap after topic creation so failed disposable runs still attempt to
  delete the topic. The default topic is intended to remain reusable for
  repeated WP-08/CodeFriend validation.
