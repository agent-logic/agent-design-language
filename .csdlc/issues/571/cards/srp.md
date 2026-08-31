# Structured Review Prompt

Template: 1.0.0

Issue: 571

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

docs/csdlc-v3/owner-proof-lanes.json
docs/csdlc-v3/predecessor-coverage.json
docs/csdlc-v3/construction-decision.json
docs/csdlc-v3/proportional-lifecycle.json
docs/csdlc-v3/CONTRACT.md
.csdlc/prepared/issues/571/validate-v3a-followup.rb
.csdlc/prepared/issues/500/validate-implementation.rb
.csdlc/issues/571

## Prompts

- Does every retained #161-#163 predecessor row have exactly useful owner issue and proof-lane data?
- Does CONTRACT.md bind the V3-A construction decision to measured #162 evidence and #163/Decision 11 approval evidence?
- Can the default lifecycle path still omit retained bind, publication, finish, or cleanup gates?
- Does diff hygiene validation use an exact PR base/head range?
- Does the patch preserve v2 live authority until V3-F/#505 and avoid widening into later v3 slices?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- This corrective follow-up does not cut over authority from C-SDLC v2 to v3; V3-F/#505 remains the explicit authority cutover gate.

## Review Result

Revision: Some("git-blake3:032097bd0c990920f34764b7e38cc6cc56075533:fe442f16d40576fab58b2f3f64e8f056fe621909fd0bb13a9abb4addcfac2973")

Reviewer: Some("review_pr_585_provenance_repair_032097bd")

Result: pass
