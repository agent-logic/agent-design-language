# Structured Task Prompt

Template: 1.0.0

Issue: 5360

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Prepare six current-registry cards, reviewed design and diagram, exact preparation and future documentation paths, COTS, budgets, PVF, and a fail-closed #5351 gate; do not edit shared documentation or product code.

## Deliverables

- all six current-registry issue-specific typed cards
- reviewed documentation alignment design and Mermaid diagram
- exact preparation-only protected paths and exact future documentation path manifest
- executable #5351 merge, typed closeout, receipt, claim-release, and ancestry gate
- zero-new-dependency COTS posture, documentation claim taxonomy, and owner-boundary model
- LoC, module, assertion, time, token, and PVF budgets
- bounded preparation review with all actionable findings fixed
- future exact-revision alignment packet, blocker register, post-merge proof, and WP-18 release predicate

## Acceptance

1. AC-1: No shared-document or product edit starts until #5351 is merged, typed closed_out, claim-free, backed by a retained merged terminal receipt, and its observed merge SHA is ancestral to the exact #5360 execution revision
2. AC-2: Every future changed material claim names its source path, prior and resulting classification, exact evidence reference, owning product, and disposition
3. AC-3: Proven, planned, blocked, deferred, superseded, and explicit_non_claim remain distinct; only exact retained evidence supports proven and no unsupported statement is promoted
4. AC-4: README, feature, WBS, sprint, checklist, handoff, issue-wave, deployment, release, proof-coverage, and ownership surfaces agree without erasing separate ADL v2, Runtime v3, and C-SDLC v2 authority
5. AC-5: Structured YAML and JSON are processed with structured parsers or owner tools; Markdown changes preserve local structure, links, repository-relative evidence, redaction, and exact revision identity
6. AC-6: Preparation adds no dependency and changes zero product/shared-document files; future documentation delta stays within 2500 changed lines and preparation stays within 1500 nonblank lines, 500 per module, and fewer than 150 focused assertions unless exactly reviewed
7. AC-7: Every required focused, complete, exact-review, CI, authorized serialized merge, post-merge, typed-closeout, and WP-18 release gate passes without hidden deferral
8. AC-8: Product defects, missing proof, path collisions, budget variance, and unsupported claims route to their owning issue and never become documentation-only green truth

## Dependencies

- WP-16 #5351 merged, typed closed_out, claim-free, retained-receipt-backed, and ancestral to the exact #5360 execution revision

## Inputs

- AGENTS.md
- GitHub issues #5360 and #5351
- docs/templates/prompts/current.json
- csdlc-v2/operator/generation-selector.json
- README.md
- docs/planning/ADL_FEATURE_LIST.md
- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- docs/milestones/v0.91.8/WBS_v0.91.8.md
- docs/milestones/v0.91.8/SPRINT_v0.91.8.md
- docs/milestones/v0.91.8/SPRINT_PLAN_v0.91.8.md
- docs/milestones/v0.91.8/QUALITY_GATE_v0.91.8.md
- docs/milestones/v0.91.8/MILESTONE_CHECKLIST_v0.91.8.md
- docs/milestones/v0.91.8/RELEASE_PLAN_v0.91.8.md
- docs/milestones/v0.91.8/RELEASE_NOTES_v0.91.8.md
- docs/milestones/v0.91.8/BASELINE_AND_OWNERSHIP_v0.91.8.md
- docs/milestones/v0.91.8/FEATURE_PROOF_COVERAGE_v0.91.8.md
- docs/milestones/v0.91.8/FEATURE_PRESERVATION_CROSSWALK_v0.91.8.md
- docs/milestones/v0.91.8/NEXT_MILESTONE_HANDOFF_v0.91.8.md
- docs/milestones/v0.91.8/features/V092_HANDOFF_v0.91.8.md
- docs/milestones/v0.91.8/features/PLATFORM_ACCEPTANCE_AND_DEPLOYMENT_v0.91.8.md
- docs/milestones/v0.91.8/review/THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md
- future retained #5351 terminal receipt and merged quality-gate evidence

## Non Goals

- WP-16 quality-gate execution, WP-18 formal review, product repair, deployment, or release ceremony
- ADL v2, Runtime v3, C-SDLC v2, provider, runtime, infrastructure, test, CI, or application implementation
- new documentation generator, release database, parser framework, workflow engine, deployment manager, signer, telemetry system, or evidence store
- Runtime v2 use or edits, AWS, provider credentials, paid services, raw gh, hidden network authority, hard-coded addresses, or private transcript retention
- shared-document implementation, PR, publication, merge, or closeout during preparation
