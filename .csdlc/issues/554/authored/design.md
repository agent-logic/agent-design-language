# Shared coverage baseline remediation design for #554

## Objective

Restore the required workspace coverage baseline that blocks Sprint 1 PR #549
without changing #514 provider-profile behavior, touching #483, weakening
coverage, or hiding failures.

## Bounded scope

- Reconcile the stale v0.92 Memory Palace README invariant expected by
  `adl::memory_palace_tests::v092_docs_name_memory_palace_production_authority_without_broad_completion_claim`.
- Make the Runtime-v2 unified-runtime-kernel coverage lane reliable under the
  existing required coverage posture.
- Preserve required coverage as a failing gate for real defects.

## Non-goals

- No PROV-A provider registry behavior changes.
- No #483 changes.
- No broad release-truth rewrite.
- No skip, ignore, or removal of the failing tests as a way to make CI green.

## Validation intent

Run focused tests for the stale docs invariant and Runtime-v2 unified-runtime-
kernel tests, then rely on the required PR coverage checks to prove the shared
gate is restored.
