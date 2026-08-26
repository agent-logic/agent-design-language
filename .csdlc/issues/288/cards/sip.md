# Structured Intent Prompt

Template: 1.0.0

Issue: 288

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Serialize the final v0.92 ADR index, plan, evidence manifest, and internal review handoff packet from terminal #283-#287 issue-local evidence.

## Required Outcome

The shared ADR index, ADR plan, machine-readable evidence manifest, and internal review handoff packet agree exactly with terminal issue-local evidence from #283, #284, #285, #286, and #287, move only evidence-proven Deferred ADRs to Proposed, mark no ADR Accepted, and record residual gaps for #207.

## Scope

- Final shared ADR index serialization for ADR 0065, ADR 0066, ADR 0068, ADR 0069, and ADR 0071
- Final v0.92 ADR plan serialization for the same ADRs
- Machine-readable review evidence manifest extension for #283-#287 terminal inputs
- Internal review handoff packet covering architecture, security, documentation, and evidence review lanes
- Issue-owned validator and retained proof for the final serialization truth

## Authority

- #288 consumes #283-#287 terminal issue-local evidence and owns shared serialization only
- No ADR may be marked Accepted by this issue
- Deferred-to-Proposed movement is allowed only where exact terminal evidence proves it
- #207 remains coordination-only and is not closed by #288
- Provider credentials, Unity proof, cloud runs, and implementation acceptance are out of scope

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle only for lifecycle/GitHub writes
- Do not mutate #207 parent lifecycle truth
- Do not run provider credentials, Unity, cloud, or external proof
- Use a FastWork bound worktree for implementation
- Obtain fresh exact-head review before publication
