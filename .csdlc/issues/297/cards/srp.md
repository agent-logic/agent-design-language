# Structured Review Prompt

Template: 1.0.0

Issue: 297

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/src/projection_recovery.rs
csdlc-v2/src/projection_cleanup.rs
csdlc-v2/src/lib.rs
csdlc-v2/tests/gate5.rs
.csdlc/issues/297
.csdlc/evidence/297/bridge-r2
.csdlc/evidence/297/noether-300-routing-bridge-gap.md

## Prompts

- Review every acceptance criterion with code, security, test, and evidence coverage, emphasizing crash consistency, immutable receipt ordering, inode ownership, symlink/hardlink and rename races, exact cleanup authority, topology/CAS enforcement, no evidence loss, and subsequent ordinary-commit behavior.

## Findings

[
  {
    "id": "297-bridge-r2-p1-replay-poisons-recovery-validator",
    "severity": "p1",
    "summary": "The bridge writes cleanup authority JSON directly into the completed recovery attempt root, so a same-operation replay causes the completed-recovery validator to see extra receipt-shaped JSON before the bridge can exercise idempotent artifact handling.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": "Keep bridge authority in a validator-recognized namespace, preserve same-operation replay, reject conflicting cleanup authority, and add focused replay/conflict proof."
  },
  {
    "id": "297-bridge-r2-p2-directory-link-count-relaxation-too-broad",
    "severity": "p2",
    "summary": "Cleanup identity comparison ignores directory link count for every directory comparison, including initial source capture and leaf-directory removal, instead of only allowing drift caused by previously authorized child-directory cleanup.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": "Constrain directory link-count drift to directories with authorized child nodes after child cleanup, keep leaf and file identity strict, and add regression proof."
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Review was inspection-only and did not run dynamic validation.
- Publication, hosted CI, and #300 bridge-fed integration proof remain deferred until remediation and fresh exact-head review.

## Review Result

Revision: Some("git-blake3:2320074475180ed1f52a42f9104236dad6896c8a:a289cbfe49163ddfb7f0e757fd09fb1c751e5167bd060263c6b79779c42256d1")

Reviewer: Some("fresh-session:564aca81-5882-4af7-8bbd-3958a9caa14a")

Result: changes_required
