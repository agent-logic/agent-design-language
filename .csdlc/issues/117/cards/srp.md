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
    "id": "6f9a-p2-spp-step-status-stale",
    "severity": "p2",
    "summary": "SPP execution status is stale: the issue is implemented and SOR records completed execution/validation, but every SPP step remains pending.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "6f9a-p2-stp-dependencies-omit-terminal-inputs",
    "severity": "p2",
    "summary": "STP dependency truth omits #271/#114/#115/#116 even though AC-1, VPP, validators, design, closeout table, and SOR require those terminal inputs.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "6f9a-p3-sip-required-outcome-duplicate-refs",
    "severity": "p3",
    "summary": "SIP required outcome duplicates #271/#114/#115/#116 in the terminal dependency list.",
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

- Reviewer could rerun git diff --check, but read-only sandbox blocked Python validators and typed doctor/validate startup with exit 71; static inspection and committed proof logs were used for the remaining proof review.
- Publication, hosted CI, merge, and terminal finish were not reviewed because this exact revision remained unpublished.

## Review Result

Revision: Some("git-blake3:63a8b2c08577b51a97c915f28dcc40c7e75c3879:fe5c70e5e8215f32700542cc51df0fb5000f98106e8f7080e5201a9376d32ce7")

Reviewer: Some("fresh-session:6f9a515b-3ccd-4067-b955-b010968082eb")

Result: changes_required
