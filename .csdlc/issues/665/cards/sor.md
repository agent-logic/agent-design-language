# Structured Output Record

Template: 1.0.0

Issue: 665

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented explicit typed C-SDLC v2 adoption for verified pre-existing emergency issue worktrees. `csdlc-bind` now supports `adopt_existing: true` with exact ready-record generation/digest, full expected HEAD SHA, actor, registered worktree topology, clean target state, base ancestry, and collision checks; successful adoption records machine-readable audit evidence and advances only ready to bound. Ordinary bind-create remains available, and implicit adoption of an existing worktree now fails closed without exact adoption authority.

## Artifacts

- csdlc-v2/src/lifecycle.rs
- csdlc-v2/tests/gate5.rs
- docs/tooling/EMERGENCY_BRANCH_ADOPTION.md
- .csdlc/prepared/issues/665/validate-emergency-adoption-scope.sh
- .csdlc/prepared/issues/665/validate-emergency-adoption-docs.sh
- .csdlc/issues/665/index.json

## Execution

- Extended `BindRequest` and `BindResult` with explicit adoption authority and result fields while preserving ordinary bind-create behavior.
- Added fail-closed adoption validation for ready/unbound issue state, stale generation/digest, full expected HEAD, non-csdlc-bind actor, registered branch/worktree uniqueness, worktree branch, HEAD match, base ancestry, and clean target worktree.
- Recorded adoption evidence in the bound issue audit and result surface without claiming implementation, review, publication, merge readiness, or closeout.
- Rejected implicit adoption through ordinary bind when the requested worktree already exists and is not the current bound issue-local worktree.
- Added focused positive and negative gate5 regression coverage for exact-head adoption, stale digest rejection, dirty target rejection, explicit authority requirement, and ordinary bind-create preservation.
- Added operator documentation and issue-owned validators for the emergency adoption sequence and stop conditions.

## Validation

[]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
