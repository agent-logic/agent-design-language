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

- The retained 10,000-cycle, 100-by-10-second, and 10-by-600-second platform runs were verified rather than rerun during review.
- Issue #5344 proves selector rollback with a v1 fixture; issue #5343 owns real retained-v1 executable restoration.
- Runtime v1 and Runtime v2 remain installed rollback surfaces, the default remains unchanged, and legacy deletion is not authorized.

## Review Result

Revision: Some("git-blake3:c4e8c18b11934ed4eebc02953772e66a05dc9e1c:b6faaee449d226982a628339826340debb51092214de1b8e9e18eaa089b189d0")

Reviewer: Some("subagent:019fac48-c04a-70f1-8a0e-fca1cb4af863")

Result: pass
