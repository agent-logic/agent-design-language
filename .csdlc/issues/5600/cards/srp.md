# Structured Review Prompt

Template: 1.0.0

Issue: 5600

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2
.csdlc/issues/5600
.csdlc/prepared/issues/5600
.csdlc/evidence/5600

## Prompts

- Does every required planning collection have an explicit card-owned typed replacement operation?
- Can any failed operation change a card, generation, digest, projection, or audit event?
- Does acceptance coverage reject stale, missing, duplicate, and extra identifiers across STP, SPP, and VPP?
- Does the #5337 fixture prove a real preparation-to-implementation conversion without direct card edits?
- Are phase authorization and existing serialized operation compatibility preserved?

## Findings

[
  {
    "id": "F-5600-1",
    "severity": "p1",
    "summary": "Acceptance-set cardinality cannot change atomically across STP, SPP, and VPP.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:cf3eb5103802da91e50c2dd54ba23b3d194ebc7d:65b9e83dfece1e867911c4afa0d38b28dceb558ea47174bad32ade456aa29ef0",
    "route": null
  },
  {
    "id": "F-5600-2",
    "severity": "p2",
    "summary": "Operator-constraint and acceptance-criteria replacements are not Bound-only.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:cf3eb5103802da91e50c2dd54ba23b3d194ebc7d:65b9e83dfece1e867911c4afa0d38b28dceb558ea47174bad32ade456aa29ef0",
    "route": null
  },
  {
    "id": "F-5600-3",
    "severity": "p2",
    "summary": "The #5337 real JSON CLI proof covers only SRP review prompts.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:cf3eb5103802da91e50c2dd54ba23b3d194ebc7d:65b9e83dfece1e867911c4afa0d38b28dceb558ea47174bad32ade456aa29ef0",
    "route": null
  },
  {
    "id": "F-5600-4",
    "severity": "p3",
    "summary": "The issue design assigns dependencies, repository inputs, and non-goals to the wrong card.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:cf3eb5103802da91e50c2dd54ba23b3d194ebc7d:65b9e83dfece1e867911c4afa0d38b28dceb558ea47174bad32ade456aa29ef0",
    "route": null
  },
  {
    "id": "F-5600-5",
    "severity": "p1",
    "summary": "The committed publication request has stale generation and digest values after review recovery.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:cf3eb5103802da91e50c2dd54ba23b3d194ebc7d:65b9e83dfece1e867911c4afa0d38b28dceb558ea47174bad32ade456aa29ef0",
    "route": null
  },
  {
    "id": "F-5600-6",
    "severity": "p2",
    "summary": "The publication body contains an unsupported whole-lifecycle process claim.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:cf3eb5103802da91e50c2dd54ba23b3d194ebc7d:65b9e83dfece1e867911c4afa0d38b28dceb558ea47174bad32ade456aa29ef0",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:cf3eb5103802da91e50c2dd54ba23b3d194ebc7d:65b9e83dfece1e867911c4afa0d38b28dceb558ea47174bad32ade456aa29ef0")

Reviewer: Some("subagent:codex-exec-5600-final-head-2")

Result: pass
