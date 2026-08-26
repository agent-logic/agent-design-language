# Structured Review Prompt

Template: 1.0.0

Issue: 323

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/issues/323
.csdlc/prepared/issues/323
csdlc-v2/src/migration.rs
csdlc-v2/src/bin/csdlc-issue.rs
csdlc-v2/src/lib.rs
csdlc-v2/src/schema.rs
csdlc-v2/tests/topology_migration.rs
csdlc-v2/operator/skills/csdlc-v2-init/SKILL.md

## Prompts

- Does the new operation avoid becoming a generic state editor?
- Does it preserve finish identity invariants while making #5913 -> #322 recoverable?
- Does it fail closed on stale, terminal, conflicting, or unsafe topology cases?
- Does it preserve source provenance and validation truth?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The operation requires repo-local typed target issue evidence rather than performing a live GitHub lookup itself.
- The #5913 PR #320 must be republished/relinked to canonical current issue #322 after this operation lands; this review does not merge #320.

## Review Result

Revision: Some("git-blake3:f0038c54e50b70dc1e16589e423fd6e5858427db:6fe0369bf428fe0cf3d945d11c7673e33c8057cd6f5321f3b919d2a27702e69b")

Reviewer: Some("fresh-session:93d2f508-e849-4528-a3ff-45aca09faa31")

Result: pass
