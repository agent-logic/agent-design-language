# CSM Governed Notice Proof - #4998

This packet records the #4998 proof that CSM emits governed degradation and shutdown notices through local recovery artifacts and live AWS control-plane channels.

## Primary Live Proof

`live_eventbridge/` is the current evidence surface.

It proves:

- CSM remains the runtime owner for governed runtime notices.
- Local safe-fail and checkpoint artifacts remain the source of truth before network delivery.
- CloudWatch Logs receives live governed notice events.
- ACIP/SNS receives a live governed notice publication and retains the provider message id.
- EventBridge receives the governed notice through the AWS SDK and returns an event id.
- EventBridge routes the event to the retained Lambda receiver, which writes a CloudWatch receipt.
- Redacted artifacts avoid raw account ids, raw ARNs, function URLs, credentials, and private payload material.

Key artifacts:

- `live_eventbridge/live_governed_notice_summary.json`
- `live_eventbridge/aws_eventbridge_setup.redacted.json`
- `live_eventbridge/logs/cloudwatch_governed_notice_events.redacted.json`
- `live_eventbridge/logs/lambda_receiver_events.redacted.json`
- `live_eventbridge/state/csm_governed_notice_latest.json`
- `live_eventbridge/state/csm_governed_notices.jsonl`
- `live_eventbridge/state/safe_fail_bundle.json`
- `live_eventbridge/state/continuity_checkpoint.json`

## Validation

Run:

```bash
bash adl/tools/validate_v0917_csm_governed_notice_4998_status.sh
```

The validator checks the EventBridge proof by default, including CloudWatch Logs, ACIP/SNS, EventBridge routed receiver receipts, local safe-fail truth, and redaction guardrails.

## Non-Claims

This issue does not claim durable SQS intake, CloudWatch custom metrics, CloudWatch Agent host metrics, or a long-duration liveness soak. Those belong in follow-on WP-07 runtime observability and durability issues.
