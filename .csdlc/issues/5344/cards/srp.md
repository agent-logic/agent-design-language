# Structured Review Prompt

Template: 1.0.0

Issue: 5344

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/5344
.csdlc/evidence/5344
.csdlc/prepared/issues/5344
adl-runtime
adl-runtime-kernel
adl-v2/tools/prove-rollback.sh
adl-v2/tools/run-soak.sh
docs/milestones/v0.91.8/evidence/wp12
infra/horust
infra/runtime-v3/runtime-init.toml
infra/rustysd
infra/systemd

## Prompts

- Can any path, symlink, environment value, stale receipt, or argument escape the isolated selector root or mutate the default selector?
- Does every selector mutation use the authoritative locked compare-and-swap API and prove exact prior-byte preservation or explicit exact rollback?
- Do successful selection, failed selection, failed soak, interruption, contention, and verification mismatch all have deterministic negative proof?
- Are #5350/#5361 merge, typed closeout, retained receipt, claim release, and ancestry predicates exact and fail-closed?
- Does the manifest cover local, CI, Runtime v3, provider-disposition, demo, negative, and rollback scenarios without production overclaim?
- Are COTS, dependency exclusions, LoC/module/test/time budgets, PVF classification, no-deferral, redaction, exact review, and post-merge proof complete?

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
