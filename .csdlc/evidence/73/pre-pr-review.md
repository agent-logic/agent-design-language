# Issue 73 Pre-PR Review

Reviewer: `codex-subagent:019fe48f-f585-7623-8263-23de39a1b930`

Reviewed target: `5b4d4cd6d2f40455f6590007535a4551034a0c37`

Initial decision: `REQUEST_CHANGES`

Final reviewed target: `64b6360bfb3da05af4af4a149775d894e13cadeb`

Final decision: `PASS`, no remaining P0-P3 findings

## Findings

### P1: Human reviewer identity dependency cycle

V3-12 required authenticated GitHub identity observation while V3-13 owned the
concrete GitHub adapter and depended on V3-12.

Disposition: Fixed. V3-04 now owns `ReviewerIdentityResolver`; V3-12 evaluates
typed/fake principal observations and remains fail-closed for the human path;
V3-13 later supplies the concrete GitHub observer.

### P1: Intent authority contradicted sole-state authority

Durable remote intents could authorize/resume external work while the plan
called `state.json` the sole machine authority.

Disposition: Fixed. `state.json` is now the sole lifecycle/card authority;
intents are authoritative only as pending-operation journals, block competing
mutation, contain no lifecycle/card values, and require exact readback before
state reconciliation.

### P2: Validation evidence exceeded retained commands

The original structure and review-lane commands were weaker than the claims in
their SOR evidence references.

Disposition: Fixed before formal review by rerunning exact counting, receipt,
scope-diff, diagram, and hygiene checks and replacing SOR validation truth with
the exact commands and results.

### P2: STP deliverable retained the original fourteen-issue denominator

Disposition: Accepted lifecycle limitation. After design reapproval, a typed
`replace_planning_collection` request for STP deliverables failed with
`invalid_transition: stp mutation is not allowed during implemented`. No hand
edit or lifecycle bypass was used. SIP declared scope, STP acceptance criteria,
SRP review scope, the canonical GitHub issue, and the implementation plan all
carry the final eighteen-issue denominator. The retained STP deliverable is
initial planning history and is not publication or implementation authority.

Final re-review confirmed the architecture fixes, exact validation commands,
local and upstream link/source proof, and retained Mermaid render. No remaining
P0-P3 findings were reported.
