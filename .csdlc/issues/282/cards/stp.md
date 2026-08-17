# Structured Task Prompt

Template: 1.0.0

Issue: 282

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Produce and review the #282 qualification packet and lifecycle truth only; do not implement product/runtime/browser changes.

## Deliverables

- .csdlc/evidence/282/production-polis-interface-qualification.md
- .csdlc/evidence/282
- .csdlc/issues/282

## Acceptance

1. AC-1: #279, #280, and #281 terminal caches validate with canonical_match=true
2. AC-2: The qualification packet names one exact integrated candidate revision and indexes terminal envelopes, PRs, merge SHAs, reviewed heads, and proof artifacts
3. AC-3: The operator runbook is local/read-only and requires no credentials or cloud deployment
4. AC-4: Product, architecture, and security review outcomes are retained with no unresolved actionable findings
5. AC-5: Residual risks and non-claims are explicit and the packet does not claim Runtime authority changes, UI implementation, cloud/public deployment, Unity implementation, or new product authority

## Dependencies

- #279 terminal and ancestral
- #280 terminal and ancestral
- #281 terminal and ancestral
- #117 parent coordination
- #110 WP-18C umbrella

## Inputs

- .csdlc/issues/279/index.json
- .csdlc/issues/280/index.json
- .csdlc/issues/281/index.json
- .csdlc/prepared/issues/282/design.md
- .csdlc/prepared/issues/282/diagram.mmd
- .csdlc/prepared/issues/282/validate_preparation_bundle.py

## Non Goals

- Implementing fixes discovered by #279, #280, or #281
- Runtime authority changes
- Browser UI or API behavior changes
- Cloud/public deployment
- Unity feature implementation
- Provider credential proof
- Editing #117 or #110 parent issue bodies
