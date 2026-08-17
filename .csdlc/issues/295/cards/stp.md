# Structured Task Prompt

Template: 1.0.0

Issue: 295

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue #295 coverage-tooling classification only; no #258 product mutation, no parent completion, and no optional hosted expansion.

## Deliverables

- Fail-closed exact unified-diff classifier for governed import-only and argument-pass-through fallout
- Per-hunk compile proof and per-owner/API behavioral proof enforcement
- Machine receipt containing file, hunk, token, owner, tests, and rationale
- Focused positive and negative regression fixtures
- PVF classification and preserved coverage authority documentation

## Acceptance

1. AC-1: Accept only exact import-only or argument-pass-through diffs to already governed APIs
2. AC-2: Require compile proof for every accepted hunk
3. AC-3: Require mapped behavioral proof for every owning API path including EstablishedRuntimeAuthority
4. AC-4: Emit a machine receipt with file, hunk, token, owner, tests, and rationale
5. AC-5: Reject semantic, predicate, branch, state, error, and unmapped-file changes
6. AC-6: Preserve the fail-closed 80 percent changed-source coverage gate for non-exempt source
7. AC-7: Introduce no blanket path allowlist
8. AC-8: Introduce no nightly or full-coverage exclusion
9. AC-9: Keep PR-fast evidence explicitly non-authoritative
10. AC-10: Keep #258 republication blocked until #295 is terminal and ancestral

## Dependencies

- #258 republication depends on terminal ancestral #295

## Inputs

- adl/tools/check_coverage_impact.sh
- adl/tools/test_check_coverage_impact.sh
- docs/tooling/COVERAGE_AUTHORITY_AND_RELEASE_PROOF.md
- issue #258 transport/core.rs diff as read-only fixture

## Non Goals

- Blanket path allowlists
- Weakening the 80 percent changed-source coverage gate
- Nightly or full coverage exclusions
- Treating PR-fast coverage as authoritative
- Issue #258 product behavior changes
- Issue #203 or #205 parent completion
- Cloud, paid hosted, or optional job expansion
- Merge or closeout
