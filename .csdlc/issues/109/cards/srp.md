# Structured Review Prompt

Template: 1.0.0

Issue: 109

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/109/focused-srp-contract.log
.csdlc/issues/109/audit.jsonl
.csdlc/issues/109/cards/sip.md
.csdlc/issues/109/cards/sip.values.json
.csdlc/issues/109/cards/sor.md
.csdlc/issues/109/cards/sor.values.json
.csdlc/issues/109/cards/spp.md
.csdlc/issues/109/cards/spp.values.json
.csdlc/issues/109/cards/srp.md
.csdlc/issues/109/cards/srp.values.json
.csdlc/issues/109/cards/stp.md
.csdlc/issues/109/cards/stp.values.json
.csdlc/issues/109/cards/vpp.md
.csdlc/issues/109/cards/vpp.values.json
.csdlc/issues/109/index.json
.csdlc/prepared/issues/109/design.md
.csdlc/prepared/issues/109/diagram.mmd
.csdlc/prepared/issues/109/validate-fresh-session-srp.sh
csdlc-v2/operator/skills/csdlc-v2-review/SKILL.md
docs/tooling/INDEPENDENT_EXACT_HEAD_REVIEW.md

## Prompts

- Review only the named immutable commit SHA in the named worktree; do not inherit or rely on the implementation conversation.
- Operate read-only: do not edit files, lifecycle state, PR state, or GitHub state.
- Report findings first, ordered P0 through P3, with repository-relative file and line evidence; include explicit limitations and state PASS only when no actionable findings remain.
- Check every acceptance criterion and identify any actionable finding that the implementation session must resolve.
- Apply authority-critical precedence: changes to authentication, authorization, security boundaries, lifecycle authority, or proof production require code, security, and evidence review even when the changed files are documentation.
- Verify the standard SRP remains the sole review-result authority and that any substantive fix requires a refreshed SRP and fresh-session review at the new exact head.
- Verify no daemon, scheduler, registry, claim, parallel review record, provider abstraction, lifecycle phase, or redundant broad validation was added.

## Findings

[
  {
    "id": "R109-P1-SRP",
    "severity": "p1",
    "summary": "Standard SRP initially omitted self-contained reviewer instructions.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:beadf360744c0c17fa9326339e63a104b37294b5:5c12681268a3074823537089c058335ae525aeee0645e190835bd31f3685739a",
    "route": null
  },
  {
    "id": "R109-P2-VALIDATOR",
    "severity": "p2",
    "summary": "Initial focused validator did not prove the complete acceptance contract.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:beadf360744c0c17fa9326339e63a104b37294b5:5c12681268a3074823537089c058335ae525aeee0645e190835bd31f3685739a",
    "route": null
  },
  {
    "id": "R109-P2-PRECEDENCE",
    "severity": "p2",
    "summary": "Review-depth rules initially omitted authority-critical precedence.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:beadf360744c0c17fa9326339e63a104b37294b5:5c12681268a3074823537089c058335ae525aeee0645e190835bd31f3685739a",
    "route": null
  },
  {
    "id": "R109-P1-REVIEW-PROOF",
    "severity": "p1",
    "summary": "Validator could pass without completed fresh-session review evidence.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:beadf360744c0c17fa9326339e63a104b37294b5:5c12681268a3074823537089c058335ae525aeee0645e190835bd31f3685739a",
    "route": null
  },
  {
    "id": "R109-P1-REUSABLE-SCOPE",
    "severity": "p1",
    "summary": "Reusable review skill omitted authority-critical scope requirements.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:beadf360744c0c17fa9326339e63a104b37294b5:5c12681268a3074823537089c058335ae525aeee0645e190835bd31f3685739a",
    "route": null
  },
  {
    "id": "R109-P1-ASSIGNMENT-ORDER",
    "severity": "p1",
    "summary": "Review assignment was not required before review activity.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:beadf360744c0c17fa9326339e63a104b37294b5:5c12681268a3074823537089c058335ae525aeee0645e190835bd31f3685739a",
    "route": null
  },
  {
    "id": "R109-P1-SUBSTANTIVE-BINDING",
    "severity": "p1",
    "summary": "Review evidence was not required to match the exact substantive SHA and typed digest.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:beadf360744c0c17fa9326339e63a104b37294b5:5c12681268a3074823537089c058335ae525aeee0645e190835bd31f3685739a",
    "route": null
  },
  {
    "id": "R109-P1-SRP-PARITY",
    "severity": "p1",
    "summary": "Structured SRP fields were not compared with retained review evidence.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:beadf360744c0c17fa9326339e63a104b37294b5:5c12681268a3074823537089c058335ae525aeee0645e190835bd31f3685739a",
    "route": null
  },
  {
    "id": "R109-P1-METADATA-SATISFIABILITY",
    "severity": "p1",
    "summary": "Completed review metadata was rejected by an overbroad raw drift check.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:beadf360744c0c17fa9326339e63a104b37294b5:5c12681268a3074823537089c058335ae525aeee0645e190835bd31f3685739a",
    "route": null
  },
  {
    "id": "R109-P1-METADATA-ALLOWLIST",
    "severity": "p1",
    "summary": "Generic metadata acceptance allowed unrelated lifecycle and evidence drift.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:beadf360744c0c17fa9326339e63a104b37294b5:5c12681268a3074823537089c058335ae525aeee0645e190835bd31f3685739a",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:beadf360744c0c17fa9326339e63a104b37294b5:5c12681268a3074823537089c058335ae525aeee0645e190835bd31f3685739a")

Reviewer: Some("fresh-session:019fea2f-95fc-7212-8891-1de38b56c85e")

Result: pass
