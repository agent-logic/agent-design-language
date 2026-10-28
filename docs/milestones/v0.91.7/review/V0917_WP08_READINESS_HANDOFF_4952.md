# WP-08 Readiness Handoff for `#4635`

Status: `ready_for_execution_not_started`
Issue: `#4952`
Date: 2026-07-06

## Scope

This note prepares WP-08 for execution after WP-06 closeout-truth repair. It
does not start WP-08 implementation and does not claim runtime AWS/signal proof.

WP-08 umbrella:

- `#4635` `[v0.91.7][WP-08] Implement runtime AWS and signal operations in full`

Known WP-08 issue wave:

- `#4684` heartbeat publisher
- `#4685` ACIP to SNS integration
- `#4686` AWS signal integration
- `#4687` local polis SSM operations
- `#4688` S3 ObsMem community-memory archive policy

Related dependencies and adjacent surfaces:

- `#4718` integrated logging and OTel proof, consumed by WP-08 AWS/signal
  operations before runtime logging claims are made.
- `#4782` AWS SSM EC2 and remote-builder resilience, relevant to SSM/network
  operation durability and failure classification.
- `#4913` safe-fail serialization and durable state handoff surfaces.
- `#4915` cloud-control/CAV-facing hook work.
- `docs/tooling/REMOTE_BUILD_HOW_TO.md` for safe AWS account, cache, and
  remote-build practices when WP-08 uses AWS proof infrastructure.

## Starting Conditions

- WP-06 remote-build and validation-platform work is operationalized enough for
  WP-08 to consume the docs and wrappers without reopening WP-06.
- The Agent Logic AWS profile for ADL work remains `agent-logic-admin`.
- Paid AWS work must be explicit and evidence-producing.
- The `pr watch` `closeout_needed` ambiguity for already-closeouted issues is
  tracked as tooling bug `#4950`; it should not block WP-08 planning, but it
  should be considered when reading watcher output for closed issues.

## Recommended Execution Order

1. Start with `#4635` readiness/doctor to confirm the current issue graph and
   card state.
2. Execute or split the smallest live AWS proof path first, likely `#4687` or
   `#4686`, because WP-08 depends on durable SSM/network operations and safe AWS
   signal behavior.
3. Keep heartbeat and archive policy work bounded: `#4684` and `#4688` should
   consume logging/OTel and storage truth rather than inventing parallel
   observability.
4. Treat ACIP-to-SNS `#4685` as a security-sensitive integration surface; do not
   claim public or production signal routing without redaction, account, IAM,
   negative-case, and teardown proof.
5. Use focused live AWS proof only where the issue requires it; otherwise use
   dry-run/account-check and local contract tests.

## Validation Expectations

WP-08 issues should record:

- Agent Logic AWS account check before live AWS mutation.
- Explicit live/dry-run boundary for every AWS command.
- Redaction proof for logs, account identifiers, ARNs, credentials, and host
  paths.
- Negative cases for missing credentials, wrong account/profile, missing SSM
  target, network failure, and stale runtime state.
- Cleanup/teardown truth for any created AWS resources.
- Runtime logging/OTel consumption from `#4718` where signal operations claim
  observability.

## Non-Claims

- This note does not implement WP-08.
- This note does not prove live AWS SSM/SNS/heartbeat behavior.
- This note does not authorize automatic paid AWS runs in ordinary PR or push
  CI.
- This note does not resolve `#4950`; it only routes the watcher ambiguity.

