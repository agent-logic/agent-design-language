# Structured Review Prompt

Template: 1.0.0

Issue: 330

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v2/src/projection_cleanup.rs
csdlc-v2/src/projection_recovery.rs
csdlc-v2/src/store.rs
csdlc-v2/tests/issue_330_bridge_cleanup_defect.rs
.csdlc/issues/330
.csdlc/evidence/330/r1

## Prompts

- Does the recovery validator accept post-cleanup retained attempts only under exact cleanup authority?
- Does the cleanup final-receipt race reject before mutation and preserve byte-exact state?
- Are #299 cleanup authority checks preserved or strengthened?
- Does the #300 bridge-fed target pass without synthetic authority?

## Findings

[
  {
    "id": "r1-p1-final-chain",
    "severity": "p1",
    "summary": "Recovery/store can accept a cleanup ledger as authorized without validating the cleanup final receipt predecessor chain at csdlc-v2/src/projection_recovery.rs:3118.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "r1-p2-stale-evidence-head",
    "severity": "p2",
    "summary": "Retained proof logs and SOR validation references bind bb7ab591de6354e03e7f59fc342e083c73aee892 instead of assigned revision f06da59a0c9d16ae5367379888b437d38bb7f1e4.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": null,
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:f06da59a0c9d16ae5367379888b437d38bb7f1e4:7916b3ec0ae1e10edda0be981db6df2e84831cd99c6fb49b39d1d4e6a8af663c")

Reviewer: Some("fresh-session:330-r1-exact-implementation-review")

Result: changes_required
