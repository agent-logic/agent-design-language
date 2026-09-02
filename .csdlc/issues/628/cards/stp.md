# Structured Task Prompt

Template: 1.0.0

Issue: 628

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue #628 only: local lifecycle command routes issue/bind/edit/validate/doctor/schedule/shepherd/eligibility, construction-state handling, focused tests, and issue-owned validation.

## Deliverables

- Implemented local lifecycle command routes under the single csdlc binary.
- Typed local construction-state store and missing-state diagnostics.
- Real issue canary proving ready-to-execute local lifecycle setup in three minutes or less.
- Focused tests for positive and failure-path local lifecycle behavior.
- Issue-owned validator for route coverage, non-authority, and no-v2-source-change.

## Acceptance

1. A real issue canary reaches ready-to-execute through v3-local state without v2 operational fallback.
2. Six-card lifecycle order is enforced: SIP -> STP -> SPP -> VPP -> SRP -> SOR.
3. Missing local lifecycle state is initialized or repaired intentionally with typed errors and next actions.
4. Issue start target is measured and can be ready for first useful work in three minutes or less once dependencies are satisfied.
5. Focused tests cover positive, stale-digest, missing-card, unsafe-primary-worktree, and unsupported-transition cases.
6. No csdlc-v2 source changes.

## Dependencies

- #625 sprint umbrella exists.
- #627 command manifest and one-binary denominator are published or locally available on the execution base.

## Inputs

- agent-logic/agent-design-language#628
- agent-logic/agent-design-language#625
- agent-logic/agent-design-language#627
- docs/csdlc-v3/v3-command-manifest.json
- csdlc-v3/src/main.rs
- csdlc-v3/tests/local_commands.rs
- root AGENTS.md

## Non Goals

- No GitHub mutation.
- No PR publication.
- No finish or cleanup.
- No #505 cutover or v2 retirement.
