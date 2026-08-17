# Structured Review Prompt

Template: 1.0.0

Issue: 282

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

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
    "id": "R3-P2-qualification-validator-evidence-coverage",
    "severity": "p2",
    "summary": "The qualification validator checks existence of only five evidence files while the packet claims nineteen retained artifacts; unchecked regression, typed-validation, and diff-hygiene references can be missing or stale without failing qualification.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "R3-P2-prep-validator-denominator-argument",
    "severity": "p2",
    "summary": "The preparation validator ignores its supplied .csdlc/issues/282/index.json argument, so the preparation proof can pass with an absent, malformed, or unrelated readiness denominator.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": null,
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:d55ce3457bec87337532c78292d3f59948842ac4:a62e6db68094033be6b2823934466051a4d684f1788686f7167e1215beb096d7")

Reviewer: Some("fresh-session:8d227575-106e-4b9d-925e-89768fdea106")

Result: changes_required
