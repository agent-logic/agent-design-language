# Structured Review Prompt

Template: 1.0.0

Issue: 5344

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/5344
.csdlc/prepared/issues/5344
adl/tools/ci_path_policy.sh
adl/tools/test_ci_path_policy.sh

## Prompts

- Can any path, symlink, environment value, stale receipt, or argument escape the isolated selector root or mutate the default selector?
- Does every selector mutation use the authoritative locked compare-and-swap API and prove exact prior-byte preservation or explicit exact rollback?
- Do successful selection, failed selection, failed soak, interruption, contention, and verification mismatch all have deterministic negative proof?
- Are #5350/#5361 merge, typed closeout, retained receipt, claim release, and ancestry predicates exact and fail-closed?
- Does the manifest cover local, CI, Runtime v3, provider-disposition, demo, negative, and rollback scenarios without production overclaim?
- Are COTS, dependency exclusions, LoC/module/test/time budgets, PVF classification, no-deferral, redaction, exact review, and post-merge proof complete?

## Findings

[
  {
    "id": "F-5344-CI-1",
    "severity": "p1",
    "summary": "Line-oriented component splitting can hide a Windows-illegal suffix after an embedded newline.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-5344-CI-2",
    "severity": "p2",
    "summary": "The regression does not cover spaces or an illegal component after an embedded newline.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-5344-CI-3",
    "severity": "p2",
    "summary": "The SRP prompts remain stale selector questions instead of the bounded CI path-policy recovery scope.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:bf4e123ac967da193393c7ff5efdb1c21c951955:2ef1e23fe28721688dec6ff5133d5d2cae45432d3a53540f9cad67bd999adf8b")

Reviewer: Some("subagent:019fac6c-4d03-74e3-90a2-3c3f07ed609d")

Result: changes_required
