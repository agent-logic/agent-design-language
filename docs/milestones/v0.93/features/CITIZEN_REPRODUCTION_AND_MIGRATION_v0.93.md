> **Superseded combined plan.** The operator-approved split is planned under #922 in [v0.93.1](../../v0.93.1/README.md) (repo split, templates and CodeFriend Beta 1 launch) and [v0.93.2](../../v0.93.2/README.md) (remaining platform work). The original combined scope below is retained for traceability; it does not require Runtime v4 before Beta 1 launch or authorize execution.

# Citizen reproduction and migration

## Status

Reconciled draft under #922. Operator-confirmed v0.93 scope; no implementation, milestone opening or release acceptance claimed.

## Purpose

Deliver bounded installed same-identity citizen migration and distinct-identity descendant creation, with preserved lineage, scoped inheritance and interruption recovery.

## Metadata

Template: feature_doc 1.1.0. Milestone: v0.93. Authoring issue: #922.

## Template Rules

Planning structure validation does not establish implemented behavior.

## Context

Follow accepted v0.92.2 closure, explicit v0.93 opening and RD-11 repository-split acceptance. Early #922 drafting does not satisfy final #921 handoff.

## Coverage / Ownership

Owner: agent-logic-runtime. Results: CM-01, CM-02, CM-03, CM-04. Named people and resource limits are assigned before execution.

## Overview

- **CM-01:** A reviewed contract distinguishes same-identity migration and recovery from new-identity descendants, replication and independent clones; it defines authority, lineage, consent, state inheritance, fencing and recovery outcomes.
- **CM-02:** Two owned Runtime endpoints transfer one authorized citizen checkpoint and scoped continuity evidence; the source is fenced before destination activation and interruption can recover without two active holders or lost lineage.
- **CM-03:** An authorized parent creates one distinctly identified descendant with explicit lineage, selected inherited state and newly scoped authority while preserving the parent and protecting private state.
- **CM-04:** Independent installed scenarios exercise migration and descendant creation across restart, interruption and retry, verify lineage/privacy/fencing and expose the resulting identity and custody status to the operator.

## Design

Migration moves custody of one identity; reproduction creates a new identity with lineage. A backup or replica is not automatically an independently active citizen. CM-01 selects the concrete transfer protocol, checkpoint compatibility and fencing model using current continuity primitives; it must define failure states before implementation.

Use two declared owned Runtime endpoints and one bounded descendant scenario. Fence the source before activating destination custody; interrupted transfers retain a discoverable owner and deterministic recovery path. Replayed or stale operations cannot create duplicate active holders. Descendants receive new identity and explicit grants: parental standing, credentials and private cognition are not copied as authority. State inheritance needs explicit scope/consent and projection rules. Retain parent continuity, selected inheritance evidence and challengeable lineage.

Do not reinterpret Runtime plugin generation migration or repository extraction as citizen migration. Whole-polis reproduction, open-ended evolution, arbitrary provider/model transfer and unbounded clone factories are outside this bounded baseline. These limits narrow implementation design without moving the confirmed citizen reproduction/migration outcome out of v0.93.

## Execution Flow

- CM-01 depends on RV-08, GOV-01.
- CM-02 depends on CM-01, WP-S3.
- CM-03 depends on CM-01, GOV-11, WP-S5.
- CM-04 depends on CM-02, CM-03.

## Determinism and Constraints

Pin source, contracts, installed artifacts and fixtures. Persist explicit retry/recovery state. Preserve identity, authority and evidence; no undocumented fallback or implied publication.

## Integration Points

Consume versioned Runtime and product contracts after RD-11. CM-04 is required by INTEGRATE and final QUALIFY.

## Validation

- CM-01: Copy treated as same active identity; Implicit grant inheritance; Ambiguous ownership during transfer.
- CM-02: Stale or replayed transfer; Unauthorized destination; Interrupted handover; Source continues after destination activation; Wrong key or corrupt checkpoint.
- CM-03: Unauthorized cloning; Duplicate descendant on retry; Private state inherited without scope; Parent standing or credentials copied as authority.
- CM-04: Split-brain identity; Orphaned in-transit custody; Silent lineage loss; Retry creates extra descendant.

Record actual positive/negative/recovery counts and exact installed versions. Design-only CM-01/CT-01 cannot substitute for downstream implementation proof.

## Acceptance Criteria

Every result above and its declared negatives passes at its consumer; no enabled kind or admitted identity transition is silently omitted.

## Non-goals

No autonomous public publication, whole-polis reproduction platform, automatic private-state cloning, unrelated subsystem rewrite or inferred legal personhood.

## Risks

Inconsistent versions, unsupported scope and weak recovery proofs can create false completion. Resolve actionable findings before acceptance.

## Future Work

Only explicitly excluded scope may be deferred; the admitted v0.93 results remain release requirements.

## Notes

EXECUTION_PLAN_v0.93.json is the dependency authority; FEATURE_COVERAGE_v0.93.md records feature mappings.

## Current Candidate Mapping

CM-01, CM-02, CM-03, CM-04
