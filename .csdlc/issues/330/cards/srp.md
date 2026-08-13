# Structured Review Prompt

Template: 1.0.0

Issue: 330

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/src/projection_cleanup.rs
csdlc-v2/src/projection_recovery.rs
csdlc-v2/src/store.rs
csdlc-v2/tests/issue_330_bridge_cleanup_defect.rs
.csdlc/issues/330
.csdlc/evidence/330/r2

## Prompts

- Does the recovery validator accept post-cleanup retained attempts only under exact cleanup authority?
- Does the cleanup final-receipt race reject before mutation and preserve byte-exact state?
- Are #299 cleanup authority checks preserved or strengthened?
- Does the #300 bridge-fed target pass without synthetic authority?

## Findings

[
  {
    "id": "r2-p2-private-delete-residue",
    "severity": "p2",
    "summary": "Recovery-side cleanup-ledger validation at csdlc-v2/src/projection_recovery.rs:3277 does not reject unexpected contents inside private-delete before authorizing store/recovery skip; mirror production final-validation emptiness predicate and add regression proof.",
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

Revision: Some("git-blake3:1bcb8159fd31d792fc7239b0f7c49b4b67f6b115:2aab98e56d8b85d4f3488c1347e4daac698b978a0c19ae6134530f037d64dd20")

Reviewer: Some("fresh-session:330-r2-exact-implementation-review")

Result: changes_required
