# DRT-A Distributed Qualification Contract Design

Issue: #506

## Objective

Produce one deterministic distributed-qualification contract that maps the retained #181 and #182 requirements into a current v0.92.1 Runtime proof surface without provisioning live cloud nodes.

## Scope

This issue owns the contract and local conformance proof for:

- the DRT-01 topology/denominator and receipt contract from retained requirement #181;
- the DRT-02 ACIP identity, authority, duplicate-denial, ordering, and replay-conformance contract from retained requirement #182;
- issue-owned evidence under `docs/milestones/v0.92.1/evidence/runtime/drt-a/**`;
- issue-owned Runtime qualification code/tests under `adl-runtime/src/qualification/**` and `adl-runtime/tests/distributed_contract/**`.

The contract consumes existing production Runtime primitives read-only, especially `adl-runtime/src/acip.rs`, `adl-runtime/src/distributed/**`, and the predecessor planning packet under `docs/milestones/v0.92.1/planned-issue-packets/**`.

## Architecture

DRT-A is a deterministic qualification layer, not a new distributed runtime.

1. `adl-runtime/src/qualification/` exposes a small contract model that names:
   - three voters;
   - three governed agents;
   - one non-voting Shepherd;
   - one quorum-leased Observatory;
   - distinct identity, credential, port, state-root, storage-root, and failure-domain bindings for every participant.
2. The same model records the scenario denominator for election, quorum loss, stale lease/fence denial, restart, snapshot, partition, healing, replay, cleanup, duplicate denial, and ACIP mutation cases. Every scenario row must include setup, action, expected commit/election/fence behavior, bounded timeout, receipt fields, cleanup requirement, and fail-closed outcome.
3. The ACIP conformance portion references the production carrier contract from `adl-runtime/src/acip.rs` and records deterministic positive and negative vector outcomes without replacing that implementation. The vector denominator must cover byte-stable encode/decode/re-encode plus identity, authority, permit, causation, correlation, sequence, term, polis, payload, duplicate, reordered, stale, malformed, unsigned, wrong-domain, and cross-polis mutation outcomes.
4. Issue-owned tests under `adl-runtime/tests/distributed_contract/` recompute the contract from producer data, verify uniqueness and non-synthetic identity provenance, prove replay cannot mutate authority, and independently reproduce the retained digest from committed input artifacts.
5. Retained evidence under `docs/milestones/v0.92.1/evidence/runtime/drt-a/` stores the exact mapped requirement table, scenario denominator, receipt schema, and validation output.

## Acceptance mapping

- AC-1 maps requirements #181 and #182 into one current DRT-A contract artifact, including the full topology/scenario denominator and ACIP vector denominator.
- AC-2 proves identity and authority are deterministic by checking participant uniqueness, role boundaries, non-voting Shepherd status, quorum-leased Observatory status, and ACIP identity/authority/permit/causation/correlation/sequence/term/polis/payload field binding.
- AC-3 proves duplicate denial and replay conformance by requiring exact typed outcomes for duplicate, reordered, stale, malformed, unsigned, wrong-domain, and cross-polis cases, plus independent digest replay from retained inputs.
- AC-4 proves the negative matrix fails closed for stale, duplicate, reordered, malformed, unsigned, wrong-domain, cross-Polis, and authority-mutation attempts.

## Validation plan

- `bash adl-runtime/tests/distributed_contract/validate_drt_a.sh qualification-contract`
- `bash adl-runtime/tests/distributed_contract/validate_drt_a.sh acip-authority`
- `bash adl-runtime/tests/distributed_contract/validate_drt_a.sh replay-conformance`
- `bash adl-runtime/tests/distributed_contract/validate_drt_a.sh negative-matrix`
- `git diff --check origin/main...HEAD`

The `distributed_contract` harness is the primary DRT-A proof and must contain distinct qualification-contract, ACIP-authority, replay-conformance, and negative-matrix assertions. The harness may call Cargo with `--manifest-path adl-runtime/Cargo.toml`, but publication must use the governed script lanes recorded in VPP.

## Authority boundary

- #506 must not run paid AWS/GCP resources.
- #506 must not redesign Observatory UI or absorb DRT-B, DRT-C, or DRT-D.
- #506 may add only the smallest issue-owned qualification model, tests, and retained evidence needed to make the contract independently reviewable.
- Later Sprint 7 children consume the contract read-only and add live/six-resident/cloud qualification evidence in their own issues.

## Risks and controls

- Risk: the contract becomes a hard-coded pass table.
  - Control: tests must recompute identity uniqueness, role separation, scenario denominator, and receipt outcome invariants from structured producer artifacts.
- Risk: replay proof mutates authority by accident.
  - Control: replay cases must preserve the original authority digest and emit deterministic denial receipts for invalid sequence/correlation/domain cases.
- Risk: #506 overclaims distributed production readiness.
  - Control: SOR and publication must state that DRT-A is deterministic contract/conformance proof only; live multi-node qualification remains DRT-B/DRT-C/DRT-D.

## Non-goals

- Paid AWS execution.
- Paid GCP execution.
- GCP portability execution.
- Provider credential proof.
- Public cloud exposure.
- DRT-B six-resident qualification.
- DRT-C final distributed Runtime qualification.
- DRT-D GCP portability qualification.
- Running a six-resident workload.
- Final distributed Runtime qualification.
- Observatory product redesign.
