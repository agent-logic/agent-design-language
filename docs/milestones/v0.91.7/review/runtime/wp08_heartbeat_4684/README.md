# WP-08 Heartbeat Live Proof `#4684`

This directory retains the live AWS proof for issue `#4684`.

## What Ran

```sh
AWS_PROFILE=agent-logic-admin ADL_AWS_PROFILE=agent-logic-admin \
  bash adl/tools/run_wp08_heartbeat_live_proof.sh \
    --out docs/milestones/v0.91.7/review/runtime/wp08_heartbeat_4684 \
    --profile agent-logic-admin \
    --region us-west-2 \
    --run-id wp08-4684-live-20260706T082608Z \
    --csm-bin adl/target/debug/csm
```

## Result

- Status: `passed`
- AWS profile: `agent-logic-admin`
- Region: `us-west-2`
- CloudWatch log group: `/adl/v0917/wp08/4684/runtime-heartbeat`
- CloudWatch log stream: `run-wp08-4684-live-20260706T082608Z`
- Retention: `7` days
- Events returned from CloudWatch: `4`
- Heartbeat sequence selected for summary: `4`

The retained summary is `live_heartbeat_summary.json`. It records a hashed AWS
account id rather than the raw account id and records no AWS credentials. The
run state is scoped under `state/wp08-4684-live-20260706T082608Z/`.

## Validation

```sh
python3 adl/tools/validate_wp08_heartbeat_live_proof.py \
  docs/milestones/v0.91.7/review/runtime/wp08_heartbeat_4684/live_heartbeat_summary.json
```

The issue also includes focused Rust tests for disabled mode, mock publication,
approval gating, unsupported target handling, missing profile handling, and
cursor behavior.
