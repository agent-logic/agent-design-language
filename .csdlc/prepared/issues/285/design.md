# #285 ADR 0068 birthday-to-governance handoff evidence reconciliation design

## Boundary

#285 owns only issue-local reconciliation evidence for ADR 0068 under parent #207. It consumes retained and live evidence about birthday-to-governance handoff proof and records whether exact terminal proof exists.

It must not:

- implement governance behavior;
- accept ADR 0068;
- rewrite WP-18 or WP-19 acceptance criteria;
- edit shared ADR documents, the ADR index, the final ADR plan, or the evidence manifest, which belong to #288.

## Evidence strategy

The issue-local packet will separate:

- terminal current-repo evidence that can be machine-checked from `.git/csdlc-v2/derived-terminal`;
- retained local lifecycle state that is not terminal authority;
- live GitHub observations;
- residual gaps and non-claims for #207/#288.

Current pre-bind observations:

- #5839 has derived-terminal authority for the birthday-to-governance handoff PR #289, merge `7f88697ce82215188af941e15cf02a6220c9ad63`.
- #5836 has preserved local lifecycle state at implemented generation 74 but no current derived-terminal cache and no current-repo GitHub issue identity, so #285 must not claim terminal WP-18 birthday proof from it.

## Deliverables

- `.csdlc/evidence/285/evidence-manifest.json`
- `.csdlc/evidence/285/live-observations.json`
- `.csdlc/evidence/285/adr0068-birthday-governance-handoff-reconciliation.md`
- `.csdlc/evidence/285/validate_adr0068_birthday_governance_handoff_evidence.sh`
- `.csdlc/prepared/issues/285/validate_adr0068_birthday_governance_handoff_evidence.sh`

## Validation

The focused validator will check:

- #5839 terminal cache identity, merge SHA, PR number, and canonical digest;
- #5836 retained local lifecycle phase/digest and absence of current derived-terminal authority;
- the evidence packet records the WP-18 residual gap, WP-19 terminal handoff proof, and #207/#288 non-claims.

