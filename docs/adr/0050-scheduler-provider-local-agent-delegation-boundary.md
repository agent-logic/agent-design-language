# ADR 0050: Scheduler, Provider, And Local-Agent Delegation Boundary

- Status: Accepted
- Date: 2026-07-06
- Accepted in: v0.91.7
- Related issues: #4671, #4672, #4673, #4674, #4675, #4849, #4932, #4989
- Related ADRs: ADR 0039, ADR 0041
- Source evidence:
  - `docs/milestones/v0.91.7/review/V0917_WP05_SCHEDULER_PROVIDER_LOCAL_AGENT_CLOSEOUT_4632.md`
  - `docs/milestones/v0.91.7/review/scheduler/COGNITIVE_SCHEDULER_V1_4671.md`
  - `docs/milestones/v0.91.7/review/provider/PROVIDER_PROFILE_SELECTION_4672.md`
  - `docs/milestones/v0.91.7/review/provider/MODEL_SUITABILITY_SELECTION_4673.md`
  - `docs/milestones/v0.91.7/review/provider/CHEAPEST_VALIDATED_OUTCOME_POLICY_4674.md`
  - `docs/milestones/v0.91.7/review/provider/LOCAL_AGENT_DELEGATION_READINESS_4675.md`

## Context

WP-05 implemented scheduler/provider/local-agent decision surfaces, repaired a
provider-route/model-suitability mismatch, and used #4932 to repair stale
local-agent delegation evidence. The architecture needs a boundary between
deterministic planning and live delegated execution.

## Decision

ADL should allow scheduler/provider/local-agent logic to produce deterministic,
machine-readable recommendations and proof packets. It must fail closed when a
provider route and selected model identity disagree. Local-agent delegation is
advisory/shadow-mode unless an issue grants explicit runtime authority.

## Consequences

- Cheapest validated outcome can be represented without granting live authority.
- Model/provider identity mismatches are architecture defects.
- Local-agent acceleration can evolve without bypassing C-SDLC boundaries.

## Alternatives Considered

### Let scheduler output directly invoke or mutate repositories

Rejected. Scheduler output is planning evidence unless a separate authority
boundary grants execution.

## Validation Notes

Check WP-05 closeout, #4849 remediation, and retained scheduler/provider
artifacts for identity alignment.

## Non-Claims

- This ADR does not certify all local agents for production use.
- This ADR does not grant merge, closeout, or repo-mutation authority.
