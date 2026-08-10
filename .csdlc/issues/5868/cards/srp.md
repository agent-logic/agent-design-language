# Structured Review Prompt

Template: 1.0.0

Issue: 5868

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime/src/distributed/failure_detection.rs
adl-runtime/tests/distributed_failure_detection.rs
.csdlc/prepared/issues/5868/derive-negative-cases.rb
.csdlc/prepared/issues/5868/validate-proof-receipt.rb
.csdlc/evidence/5868
.csdlc/issues/5868

## Prompts

- Is the implementation confined to exclusive paths?
- Do exact tests prove the named behavior and negatives?
- Are receipts exact-revision and digest bound?
- Does rollback restore one authoritative owner without weakening security?

## Findings

[
  {
    "id": "5868-operator-current-generation-attestation",
    "severity": "p1",
    "summary": "Probe acceptance must atomically attest the current observer identity generation and reject retained historical generations regardless of arrival order.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:d856f547ac8ed1779630f79e28cb7cd7701ea0fd:c7acd55af5ff0c7a5d2fa04890aa1b73fcd9bca3d66819fecd96635d953cab54",
    "route": null
  },
  {
    "id": "5868-operator-machine-derived-negatives",
    "severity": "p1",
    "summary": "Negative-case proof must derive exact case identifiers and results from executed test output rather than self-attested receipt entries.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:d856f547ac8ed1779630f79e28cb7cd7701ea0fd:c7acd55af5ff0c7a5d2fa04890aa1b73fcd9bca3d66819fecd96635d953cab54",
    "route": null
  },
  {
    "id": "5868-review-intermediate-symlink-containment",
    "severity": "p2",
    "summary": "The issue evidence producer and verifier must reject intermediate symlink components before evidence reads or writes.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:d856f547ac8ed1779630f79e28cb7cd7701ea0fd:c7acd55af5ff0c7a5d2fa04890aa1b73fcd9bca3d66819fecd96635d953cab54",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Ordinary local-filesystem race windows remain between component validation and later writes; eliminating them would require directory-descriptor APIs and is not actionable for this bounded local producer.
- Module registration and integrated sibling compilation remain explicitly deferred to issue #5878.

## Review Result

Revision: Some("git-blake3:d856f547ac8ed1779630f79e28cb7cd7701ea0fd:c7acd55af5ff0c7a5d2fa04890aa1b73fcd9bca3d66819fecd96635d953cab54")

Reviewer: Some("/root/review_5863/review_5868_operator_repair")

Result: pass
