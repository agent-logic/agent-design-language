# Structured Task Prompt

Template: 1.0.0

Issue: 629

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue #629 only: github, github-issue, github-pr, pr-state, publish, and review routes plus tests, manifest updates, canaries, and issue-owned validation.

## Deliverables

- Implemented v3 one-binary routes for GitHub issue, GitHub PR, PR state, review, and publication surfaces.
- Authenticated readback model that rejects caller-forged GitHub/PR evidence.
- Closing-vs-part-of linkage handling with visible Closes #xxx behavior for closing publications.
- Credential redaction and no raw token exposure in route outputs.
- Issue-owned validator proving #629 route ownership and no v2 source changes.

## Acceptance

1. v3 records exact-head review assignment and review result with self-review protection.
2. v3 publication refuses missing or stale review truth.
3. PR creation or update visibly includes Closes #xxx when closing linkage is intended.
4. PR-state readback authenticates live GitHub state rather than trusting caller strings.
5. Tests cover closing-vs-part-of linkage, stale PR event recovery, raw-credential redaction, caller-forged readback rejection, and review-authority denial.
6. A real issue canary reaches an open PR through v3.
7. No raw gh lifecycle writes, no v2 operational fallback, and no csdlc-v2 source changes.

## Dependencies

- #625 sprint umbrella exists.
- #627 command denominator is locally available on the execution base.

## Inputs

- agent-logic/agent-design-language#629
- agent-logic/agent-design-language#625
- agent-logic/agent-design-language#627
- docs/csdlc-v3/v3-command-manifest.json
- csdlc-v3/src/main.rs
- csdlc-v3/src/commands/remote
- csdlc-v3/src/publication
- csdlc-v3/src/review
- root AGENTS.md

## Non Goals

- No merge.
- No finish.
- No cleanup.
- No install/proof/soak/cutover authority.
- No #505 closure or v2 retirement.
