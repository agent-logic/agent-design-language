# Structured Task Prompt

Template: 1.0.0

Issue: 309

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Inventory the complete current denominator, execute only proof-supported deletion bands, validate exact behavior and rollback, and publish the truthful achieved reduction. Stop rather than absorb migration or aesthetic refactoring.

## Deliverables

- Proof-led independently reversible dead-code reductions under adl/src
- adl/tools/run_pr_fast_test_lane.sh
- adl/tools/test_run_pr_fast_test_lane.sh
- .csdlc/prepared/issues/309/refresh_crate_reference_edges.py
- .csdlc/prepared/issues/309/refresh_dead_code_band.py
- .csdlc/prepared/issues/309/run_gemini_dead_code_audit.py
- .csdlc/prepared/issues/309/validate_reduction_inventory.py
- .csdlc/prepared/issues/309/validate_rollback_proof.py
- .csdlc/prepared/issues/309/validate_hosted_linux_receipt.py
- .csdlc/evidence/309/baseline-manifest.json
- .csdlc/evidence/309/reference-edge-manifest.json
- .csdlc/evidence/309/disposition-manifest.json
- .csdlc/evidence/309/reduction-report.json
- .csdlc/evidence/309/rollback-proof.json
- .csdlc/evidence/309/gemini-dead-code-audit.md
- .csdlc/evidence/309/github-linux-ci.json
- Exact-head macOS and hosted Linux proof

## Acceptance

1. AC1: #308 terminal/canonical/ancestral/clean truth and the exact e926e3bc execution baseline are verified before deletion.
2. AC2: Every baseline adl/src Rust file and every normalized active reference edge has exactly one valid disposition with stable identity, owner, evidence, validation, and rollback source; the scanned path/blob denominator is complete.
3. AC3: Every currently provable dead or superseded path is removed without moving, hiding, gating, or copying it into an unowned compatibility surface.
4. AC4: Runtime v2 paths remain only for exact active consumers or timed exceptions; additional migration is recorded and left to its owner.
5. AC5: Supported commands, exit codes, artifacts, traces, persistence, errors, clean installation, Runtime v3 behavior, and #414 continuity retain parity on macOS and required Linux proof.
6. AC6: Each band is one commit with an exact blob manifest and executable git revert/reapply proof that preserves unrelated later work.
7. AC7: Exact physical file and line reduction against the pinned baseline is reported without a pass/fail quota and every retained path has accountable authority.
8. AC8: One exact-head independent review has no unresolved actionable finding and the implementation PR uses Closes #309 without claiming #310 or milestone completion.

## Dependencies

- #308 terminal generation 17 and merge 9f373f5f04b0f8c9dc6e3e6cbf348fddec98486c
- #414 continuity behavior merged and protected
- Current Runtime v2 and Runtime v3 production consumers
- #310 starts only after terminal reviewed #309

## Inputs

- GitHub issue #309
- Git tree c57bae97083b42125d7308047595ec2e96033240
- adl/src
- adl/Cargo.toml and adl/Cargo.lock
- current module graph, CLI routes, docs, tests, artifacts, and repository-wide references
- #308 and #414 terminal evidence

## Non Goals

- Mandatory reduction quotas
- WP-21A or #310 refactoring
- New product features or Runtime v4 migration
- Deleting active Runtime v2 authority without a landed replacement
- AWS or paid runners
- Claiming Sprint 6 or milestone completion
