# Structured Review Prompt

Template: 1.0.0

Issue: 117

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

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
    "id": "509e-p1-closeout-validator-exact-field-coverage",
    "severity": "p1",
    "summary": "The #117 closeout validator checks terminal caches are canonical/merged/closed but does not compare every exact PR, merge SHA, head SHA, generation, canonical digest, terminal digest, or integrated-candidate field recorded in the closeout packet.",
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

- Reviewer could rerun git diff --check, but the read-only sandbox blocked Python validators and typed doctor/validate startup with exit 71; committed proof logs were inspected instead.
- Publication, hosted CI, merge, and terminal finish were not reviewed because this exact revision remained unpublished.

## Review Result

Revision: Some("git-blake3:6b46d54de8576a7260ea83e02d92018b05bd2b50:49ab2089dd7040902331a58d5a6a28339dab513a23a65aaa1daf35ac860ebfc9")

Reviewer: Some("fresh-session:509e4418-914a-43af-9985-e2c5e94776d9")

Result: changes_required
