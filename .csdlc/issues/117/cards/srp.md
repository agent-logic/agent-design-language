# Structured Review Prompt

Template: 1.0.0

Issue: 117

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/117/production-polis-interface-parent-closeout.md
.csdlc/evidence/117/validate_parent_closeout.py
.csdlc/prepared/issues/117/validate_preparation_bundle.py
.csdlc/prepared/issues/117/design.md
.csdlc/prepared/issues/117/diagram.mmd
.csdlc/issues/117

## Prompts

- Verify #117 remains coordination-only and does not absorb child implementation or umbrella closeout authority.
- Verify terminal child evidence is exact, canonical, and consumed read-only.
- Verify residual risks, non-claims, and #110 handoff are truthful.

## Findings

[
  {
    "id": "9bb9-p1-stp-omits-consumed-terminal-inputs",
    "severity": "p1",
    "summary": "STP dependency/input truth omits consumed terminal prerequisites #271/#114/#115/#116 even though the #117 design, diagram, validators, closeout evidence, and AC-1 consume them.",
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

- Publication, hosted CI, merge, and terminal finish were not reviewed because this exact revision remained unpublished.

## Review Result

Revision: Some("git-blake3:af490bc1a7554974b17d054ad0ecfebd92cb56eb:07bf98dba7054134692c21bdedfd0858dc3c40932d6d1c45220478c3dec6066f")

Reviewer: Some("fresh-session:9bb9d4e3-cfc3-42ac-8ff4-e9b1d75f93c0")

Result: changes_required
