# v0.95 Feature: CodeFriend v1 And Portable Adapter v2 Proof

> **Current planning boundary (#1407, 2026-10-31):** Follow the [feature acceptance crosswalk](../../../planning/v095-reconciliation/FEATURE_CROSSWALK_1407.md) and [decision register](../../../planning/v095-reconciliation/DECISIONS_AND_DEFERRALS_1407.md). Runtime v4 is v0.93.3; tooling/foundations v0.93.4; GHB v0.93.5; remaining governance v0.93.6; affect/humour v0.93.7. Private implementations remain in their owner repositories. Earlier milestone names, source paths and proposed operations in retained source inputs are historical, not current execution authority. No feature is accepted or scope changed by this candidate document.

## Metadata

- Feature Name: CodeFriend v1 And Portable Adapter v2 Proof
- Milestone Target: `v0.95`
- Status: planned
- Owner: ADL maintainers
- Doc Role: primary
- Supporting Docs:
  - `docs/planning/codefriend/README.md`
  - `docs/milestones/v0.91.2/features/CODEFRIEND_PRODUCTIZATION.md`
- Feature Types: artifact, architecture, policy
- Proof Modes: demo, tests, review

## Template Rules

This is a forward-planning feature document. It defines the required proof
boundary for MVP consumption, not broad product completion.

## Status

Forward-planning feature contract for the pre-`v0.95` proof that `v0.95`
must consume.

This document does not claim broad CodeFriend product completion. It defines
the smallest external-repository proof needed before MVP convergence.

## Purpose

Make CodeFriend v1 and portable adapter v2 visible as an MVP prerequisite:
ADL must be able to clone or prepare an external repository, install the
adapter, run CodeFriend/C-SDLC tooling through ADL, and export reviewable
artifacts without leaking private ADL state.

## Source Inputs

- `docs/planning/V093_V095_MVP_FEATURE_DOC_PLAN_v0.91.5.md`
- `docs/planning/codefriend/README.md`
- `docs/planning/codefriend/CODEFRIEND_SETUP_PLAN.md`
- `docs/planning/codefriend/CODEFRIEND_PRE_ALPHA_REPO_AND_S3_WELCOME_MINI_SPRINT.md`
- `docs/milestones/v0.91.2/features/CODEFRIEND_PRODUCTIZATION.md`
- `docs/planning/ADL_FEATURE_LIST.md`

## Context

CodeFriend v1 needs a portable way to run ADL review and C-SDLC tooling against
repositories outside the ADL workspace. Adapter v2 is the substrate that makes
that proof repeatable.

## Coverage / Ownership

Primary owner doc: this document.

Covered surfaces:

- portable adapter v2 bootstrap and doctor
- external-repo CodeFriend review proof
- review-run manifest and evidence export
- redaction and private-state boundaries

Related / supporting docs:

- `docs/planning/codefriend/README.md`
- `docs/planning/V093_V095_MVP_FEATURE_DOC_PLAN_v0.91.5.md`

## Overview

The feature proves that CodeFriend can run on an arbitrary target repository
through ADL without depending on private ADL workspace state or ad hoc chat
context.

## Scope

This feature should establish:

- a portable adapter v2 bootstrap path for an arbitrary target repository
- repo-local `AGENTS.md` and `adl_project.json` expectations
- `codefriend` and `csdlc` project profiles
- adapter doctor checks for tooling resolution, local-state boundary,
  validation profile, and redaction policy
- a bounded CodeFriend review workflow against a declared commit and scope
- a review-run manifest covering repo identity, commit, scope, reviewer lanes,
  allowed artifacts, validation profile, and redaction policy
- product-safe evidence export for findings, diagrams, tests or test
  recommendations, residual risks, and follow-up issue candidates

## Design

### Core Concepts

- Portable adapter: the installed project-local bridge between ADL and a target
  repository.
- Project profile: declared mode such as `codefriend` or `csdlc`.
- Review-run manifest: the proof record for repository identity, commit,
  scope, reviewer lanes, validation profile, and redaction policy.
- Evidence exporter: the boundary that makes review artifacts product-safe.

### Architecture

- Inputs: target repository, declared commit, adapter configuration, review
  scope, provider/model policy, and redaction rules.
- Outputs: adapter doctor result, review-run manifest, findings packet,
  diagrams, test recommendations, residual risks, and issue candidates.
- Interfaces: adapter installer, adapter doctor, CodeFriend review command,
  C-SDLC mode, and evidence exporter.
- Invariants: no private `.adl` state copied into target repositories; no
  credentials or provider logs in public artifacts; no automatic customer PR
  mutation by default.

### Data / Artifacts

- `adl_project.json`
- repo-local `AGENTS.md`
- adapter doctor report
- review-run manifest
- exported review artifact packet

## Execution Flow

1. Clone or prepare the target repository.
2. Install adapter v2 and project-local instructions.
3. Run adapter doctor and fail closed on setup conflicts.
4. Run a bounded CodeFriend review against a declared commit and scope.
5. Export redaction-safe evidence and follow-up candidates.

## Determinism and Constraints

- The review run must name target repository identity, commit, scope, and
  validation profile.
- Public artifacts must be free of credentials, absolute host paths, raw
  provider logs, and private ADL state.
- Customer repository mutation must require an explicit later policy.

## Integration Points

| System / Surface | Integration Type | Description |
| --- | --- | --- |
| CodeFriend | trigger | Runs the bounded review workflow. |
| C-SDLC | read/write | Provides issue/review artifact structure where allowed. |
| Providers | observe | Supplies reviewer lanes with model identity and redaction boundaries. |
| Evidence exporter | write | Produces product-safe review artifacts. |

## Validation

- Demo: smallest proof should run one external-repo review through the adapter.
- Deterministic / Replay: review-run manifest must pin repo identity, commit,
  scope, and validation profile.
- Schema / Artifact Validation: adapter config and review-run manifest should
  validate.
- Tests: future tests should cover missing adapter, ambiguous authority,
  private-state leakage, and redaction failures.
- Review / Proof Surface: v0.95 convergence may consume only the proof packet,
  not broad product completion.

## Non-goals

- automatic customer PR mutation by default
- new billing/portfolio expansion beyond the accepted Beta1 scope; existing required onboarding, repository connection and readable report/export journeys remain prerequisites, not post-MVP exclusions
- copying private `.adl` state into target repositories
- customer-data ingestion before redaction and credential boundaries exist
- treating external provider output as authoritative without synthesis and
  review

## v0.95 Consumption

`v0.95` consumes the proof packet and boundaries from this feature. The accepted Beta1 product and its required onboarding/repository/report/export journeys are predecessor inputs. Only additional billing, portfolio and wider product expansion remain separately scoped; no required Beta1 outcome is postponed by this portability proof.

## Acceptance Criteria

- The v0.95 feature index links this document.
- The v0.95 WBS names CodeFriend v1 / adapter v2 proof packaging.
- The proof path demonstrates external-repo setup and review without relying on
  private ADL workspace state.
- Consume all already required Beta1 journeys and their exact owner evidence; preserve the separate boundary for additional billing/portfolio and wider product expansion.

## Risks

- Risk: adapter installation leaks ADL-local assumptions. Mitigation: require
  adapter doctor checks and explicit local-state boundaries.
- Risk: CodeFriend proof drifts into product scope. Mitigation: keep
  additional billing/portfolio work separately scoped while preserving all already required Beta1 journeys.

## Future Work

Later separately scoped product milestones may expand billing, portfolio and UX. Reconcile existing Beta1/template/journal/full4+1 requirements and proof first; do not recreate or silently defer them.

## Notes

This document exists so CodeFriend v1 proof is visible from the MVP package
without pulling broad product work into `v0.95`.
