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

- Production Guardian and kernel entrypoints plus canonical Runtime init configuration
- Explicit shepherd_agent_ref and per-node local provider/model profile validation
- Three-voter polis assembly using the merged distributed authority modules
- One quorum-leased movable polis Observatory API and OpenAPI contract
- Focused deterministic integration tests and exact issue-owned receipt validation
- Strictly serial Wuji-only and Wuji-plus-two-AZ-AWS live runners with complete cleanup
- Operator runbook and redacted live demonstration artifacts

## Authority

- Merged WP-04 ledgers, certificates, membership, fencing, migration, recovery, and projection modules remain the sole distributed authority.
- The configured shepherd launches with the polis but is never a consensus voter and cannot mint membership, lease, fence, activation, snapshot, or Observatory authority.
- Exactly one quorum-leased Observatory exists per distributed polis; node-local views are not additional Observatories.
- A snapshot is authoritative only when its boundary is quorum committed and independently materialized with the same canonical digest by every healthy voter; manual copying is non-proving.
- AWS takeover requires a newer quorum term, the bounded safety window, expiry of the Wuji Observatory lease, a durable Wuji fence, and separate owner then shepherd activation.
- Model services are private bounded inference dependencies only; model identity or capability cannot change consensus or mutation authority.
- AWS authority is limited to the verified agent-logic-admin profile in the approved Agent Logic business account, private network paths, and issue-scoped ephemeral resources.
- No public unauthenticated or plaintext Runtime, model, or Observatory surface is permitted.

## Assumptions

- none

## Operator Constraints

- Run Phase A with exactly three Wuji voters and one polis Observatory first and alone.
- Do not begin Phase B until Phase A cleanup is machine-proven; never run the two live demonstrations in parallel.
- Runtime configuration must select the non-voting shepherd through shepherd_agent_ref and select a bounded provider/model profile independently for every voter and shepherd.
- Wuji may use smaller local models so three instances fit concurrently; authority semantics cannot depend on model capability.
- Phase B uses one Wuji voter and two AWS voters in distinct Availability Zones, all with private self-hosted local models and no hosted-model fallback.
- Show exactly one live running Observatory for each polis phase and visibly demonstrate AWS continuity after Wuji is partitioned.
- Use only verified profile agent-logic-admin in the approved Agent Logic business account, private connectivity, permission-safe process checks, bounded timeouts, and ephemeral resources.
- Do not perform asynchronous lifecycle closeout in this issue.
