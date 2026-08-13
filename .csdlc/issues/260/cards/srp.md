# Structured Review Prompt

Template: 1.0.0

Issue: 260

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope



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

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
