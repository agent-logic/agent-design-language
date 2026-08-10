# Structured Review Prompt

Template: 1.0.0

Issue: 90

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/90/audit.jsonl
.csdlc/issues/90/cards/sip.md
.csdlc/issues/90/cards/sip.values.json
.csdlc/issues/90/cards/sor.md
.csdlc/issues/90/cards/sor.values.json
.csdlc/issues/90/cards/spp.md
.csdlc/issues/90/cards/spp.values.json
.csdlc/issues/90/cards/srp.md
.csdlc/issues/90/cards/srp.values.json
.csdlc/issues/90/cards/stp.md
.csdlc/issues/90/cards/stp.values.json
.csdlc/issues/90/cards/vpp.md
.csdlc/issues/90/cards/vpp.values.json
.csdlc/issues/90/index.json
.csdlc/locks/90.lock
.csdlc/prepared/issues/90/design.md
.csdlc/prepared/issues/90/diagram.mmd
csdlc-v2/operator/skills/csdlc-v2-init/SKILL.md
csdlc-v2/src/bin/csdlc-issue.rs
csdlc-v2/src/git.rs
csdlc-v2/src/lib.rs
csdlc-v2/src/migration.rs
csdlc-v2/src/schema.rs
csdlc-v2/src/store.rs
csdlc-v2/tests/code_repository_migration.rs

## Prompts

- Can any request assign a repository that differs from an effective origin fetch or push identity?
- Can an unbound, dirty, stale, terminal, published, or already-conflicting record migrate?
- Does the atomic update preserve every card, phase, review, publication, readiness, and terminal field?
- Can a reviewed migrated record publish only by satisfying the unchanged exact-head and split-authority checks?
- Is retry behavior deterministic and audit-safe?

## Findings

[
  {
    "id": "90-review-p1-nonexistent-schema-command",
    "severity": "p1",
    "summary": "The first CI remediation named a nonexistent schema command; guidance now uses executable csdlc-edit schema and retains command proof.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:ad6d858f5d9f41d83aaf04a32c72a48729981287:9ebb06db2eb27dd2cab6c36d243e66d94858cdfbe976033a6ecb3fb4b0220488",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- This is a temporary one-sprint compatibility bridge; typed republication must replace the superseded publication intent with this reviewed head.

## Review Result

Revision: Some("git-blake3:ad6d858f5d9f41d83aaf04a32c72a48729981287:9ebb06db2eb27dd2cab6c36d243e66d94858cdfbe976033a6ecb3fb4b0220488")

Reviewer: Some("Carver:019fe94e-cede-7900-90db-4e9b92164cef")

Result: pass
