# Structured Intent Prompt

Template: 1.0.0

Issue: 142

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Operationalize a real multi-node Runtime v3 polis and prove one polis-level Observatory first across two Wuji nodes and then, strictly serially, across Wuji and AWS.

## Required Outcome

Production Guardian/kernel entrypoints run an authenticated distributed polis; one coherent redacted Observatory is shown live for each serial topology; failure, recovery, shutdown, and complete cleanup are proven.

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
