# Structured Intent Prompt

Template: 1.0.0

Issue: 5347

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Prepare an exact, reviewed, disjoint manifest for deleting only incumbent ADL files whose accepted authority has moved outside the ADL v2 language/compiler/engine/CLI core.

## Required Outcome

After every terminal acceptance and cutover gate, #5346 deletion eligibility, and the live dependency graph are reconciled, a canonical manifest can prove each externally owned deletion candidate has an accepted replacement owner and evidence while every retained file has explicit authority and rationale.

## Scope

- issue-local C-SDLC lifecycle, preparation, review, validation, and evidence records
- docs/milestones/v0.91.8/evidence/wp13-external-bands future manifest and proof packet
- future exact file paths admitted only by reviewed manifest and typed claim amendment
- dependency, ancestry, receipt, owner, reachability, disjointness, COTS, budget, PVF, rollback, and no-deferral gates

## Authority

- #5347 owns only externally owned incumbent deletion candidates individually admitted by an exact reviewed manifest
- #5346 exclusively owns final incumbent language/compiler/engine/CLI deletion and its manifest paths
- ADL v2, Runtime v3, C-SDLC v2, selector, soak, cutover, acceptance, and release implementations are read-only dependencies
- Preparation protects only issue-local and evidence paths; no product path is claimed or deletion-authorized
- Runtime v2 is not edited by this issue and cannot be deleted from metadata-only parity claims
- No AWS, raw gh, credential access, production mutation, publication, PR, merge, or release authority is in scope

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 binaries and semantic card operations only; rendered Markdown remains generated projection state
- Keep the primary checkout clean on main and all tracked #5347 work in the dedicated FastWork worktree
- Preparation only: do not delete code, implement product changes, open a PR, publish, push, merge, close out, use AWS/raw gh for mutation, edit Runtime v2, or touch root main
- Do not execute deletion until #5346 is terminal closed_out with exact reviewed final-core manifest, all closed dependency inputs remain receipt-backed and ancestral, and the #5346/#5347 dependency cycle is authoritatively reconciled
- Treat nonterminal local typed projections such as #5354 or #5675 as manifest-row blockers even when GitHub issue state is closed
- Freeze canonical manifests and prove zero path overlap before typed claim expansion or deletion
- Use /Volumes/FastWork for generated validation and build output; do not use /private/tmp
- Run bounded preparation validation and finish with a clean local branch; no PR, publication, push, merge, closeout, or product-path claim belongs to this preparation pass
