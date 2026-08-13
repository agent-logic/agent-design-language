# Structured Review Prompt

Template: 1.0.0

Issue: 260

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime/src/distributed/capability_advertisement.rs
adl-runtime/src/distributed/migration.rs
adl-runtime/src/distributed/placement.rs
adl-runtime/src/distributed/projection.rs
adl-runtime/src/distributed/recovery.rs
adl-runtime/src/distributed/resource_weather.rs
adl-runtime/src/distributed/snapshot_catalog.rs
adl-runtime/tests/distributed_capability_advertisement.rs
adl-runtime/tests/distributed_migration.rs
adl-runtime/tests/distributed_placement.rs
adl-runtime/tests/distributed_recovery.rs
adl-runtime/tests/distributed_snapshot_catalog.rs
adl-runtime/tests/distributed_authority_adapter_callers_260.rs
.csdlc/issues/260
.csdlc/prepared/issues/260
.csdlc/evidence/260

## Prompts

- Review every acceptance criterion AC-1 through AC-5 against the exact assigned immutable revision and typed scope; identify missing, contradicted, or unsupported criteria.
- Report findings first, ordered P0 through P3, with repository-relative file and line evidence for every actionable finding.
- Review code, security/authority boundaries, tests, and lifecycle/evidence integrity, including governed production adapters, cfg(test)-only raw seams, fail-closed errors, deterministic retry semantics, and no #258/#259/#203/#205 scope absorption.
- Verify the R1 placement and SPP/VPP repairs and R2 distinct command-bound evidence logs, commands, results, references, and hashes.
- State explicit validation limitations, including commands not independently rerun, broad suites or CI not inspected, and live GitHub/dependency state not verified.
- Operate read-only: do not edit worktree, lifecycle, Git, PR, or GitHub state.
- Return PASS only when no actionable P0-P3 finding remains; otherwise FAIL with exact revisions and findings.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Broad workspace suite and live CI were not part of the independent local review; publication CI remains required.
- Parent #203 integration and #205 remain explicitly out of scope.

## Review Result

Revision: Some("git-blake3:f6055539dfca07db8c930f1962527522fa642d6a:02b5b5a2ba238c475fb407fe58f0c6262fee496b02ebaf98623891f4899c59dc")

Reviewer: Some("fresh-session:84c1e627-9f55-4ea8-8b43-d20f6a731e0c")

Result: pass
