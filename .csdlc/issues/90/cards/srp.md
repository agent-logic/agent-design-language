# Structured Review Prompt

Template: 1.0.0

Issue: 90

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

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
    "id": "90-review-p2-overbroad-proof-contract",
    "severity": "p2",
    "summary": "The initial plan overpromised crash injection and an exhaustive migration matrix for a temporary one-sprint compatibility bridge; the design and SPP were narrowed and independently reapproved.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:424915b3702215c6f02927fd629a62cfd24280bc:a0f0bf8740980cce71e08ef5704bb3b6d07f1790bfd4e455680180beaee1e33a",
    "route": null
  },
  {
    "id": "90-review-p2-origin-cas-coverage",
    "severity": "p2",
    "summary": "Focused proof lacked stale-generation, missing/non-GitHub/divergent-fetch, and credential-redaction cases; the exact-head suite now passes all eight tests.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:424915b3702215c6f02927fd629a62cfd24280bc:a0f0bf8740980cce71e08ef5704bb3b6d07f1790bfd4e455680180beaee1e33a",
    "route": null
  },
  {
    "id": "90-review-p2-validation-denominator",
    "severity": "p2",
    "summary": "SOR retained the superseded six-test result; typed execution evidence now records the exact-head eight-test result.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:424915b3702215c6f02927fd629a62cfd24280bc:a0f0bf8740980cce71e08ef5704bb3b6d07f1790bfd4e455680180beaee1e33a",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- This command is intentionally a temporary compatibility bridge for the remaining legacy-repository sprint issues; publication still requires the unchanged exact-head and qualified-linkage checks.

## Review Result

Revision: Some("git-blake3:424915b3702215c6f02927fd629a62cfd24280bc:a0f0bf8740980cce71e08ef5704bb3b6d07f1790bfd4e455680180beaee1e33a")

Reviewer: Some("Carver:019fe94e-cede-7900-90db-4e9b92164cef")

Result: pass
