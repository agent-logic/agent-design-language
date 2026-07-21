# Structured Task Prompt

Template: 1.0.0

Issue: 5357

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Prepare six cards, reviewed design/diagram, exact WP-18 dependency gate, immutable corpus and dispatch contracts, reviewer identity/independence boundary, findings schema, budgets, PVF, rollback, and redaction rules; do not send the review or edit product/shared documents.

## Deliverables

- all six current-registry issue-specific typed cards
- reviewed external-review preparation design and Mermaid diagram
- exact preparation-only protected paths and executable WP-18 terminal gate
- immutable exact-revision corpus-manifest and dispatch-receipt templates with fail-closed validators
- reviewer identity, independence, authority, and conflict-disclosure contract
- findings-first output schema separating observed evidence, inference, residual risk, and open author decisions
- COTS, LoC, assertion, time, token, PVF, no-deferral, rollback, and publication/redaction boundaries
- bounded preparation review with all actionable findings fixed

## Acceptance

1. AC-1: No external dispatch starts until #5356 is GitHub merged, typed closed_out, claim-free, backed by a retained merged terminal receipt, and its observed merge SHA is ancestral to the exact #5357 execution revision
2. AC-2: The canonical handoff remains at docs/milestones/v0.91.8/review/THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md and is consumed without issue-number renaming or preparation-time rewrite
3. AC-3: The future corpus manifest binds sorted tracked mode, type, hash, and repository-relative path records plus target/base/head identities; any missing, untracked, changed, duplicate, self-referential, or non-ancestral input fails closed
4. AC-4: The future dispatch receipt binds corpus digest, handoff digest, exact prompt digest, reviewer/provider/model identity, independence and conflict disclosures, dispatch and completion times, outcome, and output digest without granting reviewer authority
5. AC-5: Review output is findings-first in P0-P3 order and separately labels observed evidence, inference, residual risk, and open author decisions with exact file/line or issue/PR support
6. AC-6: Secrets, private prompts, credentials, raw provider payloads, host paths, local scratch roots, personal data, and unverifiable external assertions are excluded or redacted; redaction never changes finding meaning silently
7. AC-7: Preparation changes zero product/shared-document files and adds no dependency; issue-local authored preparation remains within 1800 nonblank lines, 500 per module, fewer than 160 assertions, and 120/300/900-second lane budgets unless exactly reviewed
8. AC-8: Required dependency, corpus, dispatch, output-schema, redaction, complete, exact-review, CI, authorized serialized merge, post-merge, and typed-closeout gates pass without deferral or substitution before WP-20 receives findings

## Dependencies

- WP-18 #5356 merged, typed closed_out, claim-free, retained-receipt-backed, and ancestral to the exact #5357 execution revision

## Inputs

- AGENTS.md
- GitHub issues #5357 and #5356
- docs/templates/prompts/current.json
- csdlc-v2/operator/generation-selector.json
- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- docs/milestones/v0.91.8/WBS_v0.91.8.md
- docs/milestones/v0.91.8/SPRINT_v0.91.8.md
- docs/milestones/v0.91.8/QUALITY_GATE_v0.91.8.md
- docs/milestones/v0.91.8/CANONICAL_DOC_INVENTORY_v0.91.8.md
- docs/milestones/v0.91.8/review/THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md
- future retained #5356 terminal receipt and internal-review register

## Non Goals

- WP-18 internal review, WP-20 remediation, release approval, deployment, release ceremony, or v0.92 activation
- product, runtime, provider, CI, infrastructure, canonical planning, or canonical handoff modification during preparation
- external review dispatch, model call, paid provider use, credential access, or reviewer selection during preparation
- automatic issue creation, finding acceptance, remediation authority, release approval, or lifecycle mutation by the reviewer
- Runtime v2 use or edits, AWS, raw gh, hidden network authority, hard-coded addresses, private transcript retention, PR, publication, merge, or closeout during preparation
