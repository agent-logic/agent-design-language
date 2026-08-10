# Structured Intent Prompt

Template: 1.0.0

Issue: 185

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Qualify distributed identity, authority, TLS, capability, stale-fence, provider-failure, malformed-message, and pre-auth disclosure boundaries.

## Required Outcome

key separation non-voting shepherd identity stale lease and fence cross-polis replay pre-auth disclosure and producer-derived outcomes is produced at an exact revision and independently reproducible.

## Scope

- Node, agent, Shepherd, operator, and Observatory key separation; trust domains; public TLS and private mTLS; permits and capabilities; stale lease/fence; cross-polis replay; provider stalls; malformed traffic; REST/WSS pre-auth behavior.

## Authority

- Issue DRT-05 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main
- Bind only after typed doctor reports ready
- Do not cross dependency or stop gates
- Keep evidence producer-derived and exact-revision bound
