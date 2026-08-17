# Structured Review Prompt

Template: 1.0.0

Issue: 282

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/evidence/282/production-polis-interface-qualification.md
.csdlc/evidence/282/validate_qualification_packet.py
.csdlc/prepared/issues/282/validate_preparation_bundle.py
.csdlc/prepared/issues/282/design.md
.csdlc/prepared/issues/282/diagram.mmd
.csdlc/issues/282

## Prompts

- Review the exact-revision qualification packet for stale evidence, overclaims, missing artifact links, and unclear residual risks.
- Review the operator runbook for local/read-only reproducibility without credentials or cloud deployment.
- Review product, architecture, and security synthesis for unsupported readiness claims.

## Findings

[
  {
    "id": "R1-P1-validator-terminal-truth",
    "severity": "p1",
    "summary": "Qualification validator searches for strings but does not invoke cached-terminal validation, compare returned fields with the table, verify Git ancestry, or check referenced evidence files.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "R1-P2-bound-head-wording",
    "severity": "p2",
    "summary": "Qualification packet calls 716f0ff the origin/main/bound HEAD even though the reviewed HEAD is 0befd94f and 716f0ff is its parent integrated candidate.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "R1-P2-spp-step-truth",
    "severity": "p2",
    "summary": "SPP steps remain pending while index/SOR record implemented execution and validation truth.",
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

Revision: Some("git-blake3:0befd94f4aceb186840c92e51533b555d2aa992e:cc036eaafa62dc86c55dbea057e210194d5a9b55348667848180528d2b45d37b")

Reviewer: Some("fresh-session:ddf47a06-b817-433c-8d96-f73e14abe576")

Result: changes_required
