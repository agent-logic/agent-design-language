# Structured Intent Prompt

Template: 1.0.0

Issue: 142

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Operationalize a real three-voter Runtime v3 polis and one polis-level Observatory, first with three configurable smaller-model instances on Wuji and then, strictly serially after cleanup, with one Wuji and two AZ-separated AWS voters using private self-hosted local models and governed AWS continuity.

## Required Outcome

Production Guardian/kernel entrypoints launch three authenticated voters plus a configured non-voting shepherd from shepherd_agent_ref; one coherent redacted Observatory is shown for each serial phase; Phase B proves quorum-committed snapshot-root recovery, AWS leader/fence/owner/shepherd activation, governed continuity after live Wuji partition, stale-Wuji demotion, true one-of-three halt, private self-hosted model operation, and complete cleanup.

## Scope

- adl-runtime/src/bin/adl-runtime-guardian.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime/src/guardian.rs
- adl-runtime/src/runtime_api.rs
- adl-runtime/src/distributed/
- adl-runtime/tests/
- docs/api/runtime-v3/v1/distributed.openapi.json
- issue-owned operator runners, documentation, and evidence for #142

## Authority

- Merged WP-04 authority modules remain the sole distributed authority
- One Observatory exists per distributed polis, not per node
- Phase A cleanup is mandatory authority for beginning Phase B
- AWS authority is limited to verified profile agent-logic-admin in the Agent Logic business account
- No public unauthenticated or plaintext runtime surface

## Assumptions

- none

## Operator Constraints

- Run the Wuji-Wuji demo first and alone
- Do not begin Wuji-AWS until Phase A cleanup is machine-proven
- Show the live running polis Observatory to the operator in both phases
- Use permission-safe process status checks and bounded timeouts
- Do not perform async lifecycle closeout in this issue
