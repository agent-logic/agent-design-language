# Proposed GHB vocabulary and contract migration — ADL #1414

Status: proposed ADR/migration mapping; not an accepted architecture decision.
The [v1 proposal](NORMATIVE_SPEC_1414.md) defines source-contract requirements;
Runtime integration and installed qualification remain separately owned.

## Context and proposed decision

Historical conceptual names assign different work to the same phases. Adopt
explicit transition names for machine contracts and retain Gödel/Hadamard/Bayes
as historical explanatory labels. Preserve historical documents and accepted
[ADR 0008](../../../adr/0008-godel-stage-loop-v08.md); do not rewrite them into
claims about a current resident engine.

| Source | Historical meaning | Proposed mapping / limitation |
| --- | --- | --- |
| [v0.86 feature](../../v0.86/features/GODEL_HADAMARD_BAYES_ALGORITHM.md) | Gödel = state analysis; Hadamard = hypotheses; Bayes = evaluation/update | `observe`/`hypothesize`; `propose_candidate`; `measure`/`update_belief`/`disposition`. Historical claims that nondeterminism occurs only in Hadamard do not promise deterministic fresh inference. |
| [v0.93.3 research snapshot](../../v0.93.3/sources/ghb/DESIGN_AND_RESEARCH.md) §4 | Observe, inadequacy/Gödel, candidate/Hadamard, Bayes, experiment, disposition, activate | Canonical v1 splits experiment admission, actual invocation, measurement and authorization/reconciliation. Research wording about atomic activation does not prove cross-store atomicity. |
| Research snapshot §2 references to v0.89 ideas | Gödel as structure expansion; Hadamard as transformation; observe/structure/explore/evaluate/commit | These descriptions are secondary historical references. Referenced v0.89 idea files are absent from the inspected public checkout; their original text was not verified. Do not invent normative mappings or compression guarantees. |
| ADR 0008 / v0.8 loop | failure → hypothesis → mutation → experiment → evaluation → record → indexing | Observation/problem → hypothesis → candidate → separately admitted experiment → measurement/disposition → retained record/lineage. Indexing is persistence, not activation. |

## Contract compatibility

Existing canonical inputs remain [evaluation_plan.v1](../../../../adl-spec/schemas/v0.8/evaluation_plan.v1.json),
[mutation.v1](../../../../adl-spec/schemas/v0.8/mutation.v1.json),
[experiment_record.v1](../../../../adl-spec/schemas/v0.8/experiment_record.v1.schema.json)
and [canonical_evidence_view.v1](../../../../adl-spec/schemas/v0.8/canonical_evidence_view.v1.schema.json).
Their original semantics, IDs and bytes remain unchanged. Closed schemas reject
unknown fields; v1 GHB adds an explicit versioned envelope/profile with typed
references, not extra undeclared v0.8 properties.

An adapter MUST bind source version/digest, declared mapping and loss/unknown
fields. Existing mutation surfaces such as evaluation/guardrail/provider changes
are not permission for those changes in the first profile. Existing `adopt`
means a historical evaluation decision; its v1 adapter may recommend activation
but MUST NOT manufacture activation, causal-control or installed uptake receipts.
A legacy numeric delta is not a posterior; missing provenance is an explicit gap.
No adapter may invent experiment admission from a recorded experiment result.

The [frozen v0.8 distribution](../../../../tools/godel_contract_distribution/README.md)
remains historical distribution, including its intentional source snapshot.
It does not automatically load a new GHB v1 contract. #1415 should keep one
editable canonical copy of new schemas, stable public raw schema IDs and reuse
existing public-contract export machinery. No new engine/crate or duplicate
editable schema mirror is implied. Genuine owner-side loading/validation is
required before installed-consumption acceptance; fixture callbacks, no-op
consumers and source-path smoke checks are insufficient. A missing real reader
requires bounded Runtime owner work, not a claim of parity.

## Consequences and acceptance boundary

The explicit names make authority and causal ordering inspectable while allowing
existing owners to implement their own storage/recovery protocol. They introduce
no production algorithm and do not upgrade any installed package. Profile and
schema acceptance require review; implementation conformance requires exact
source/package, real operation, negative and recovery evidence. G0–G5 levels in
the historical research packet remain proposals, not assigned certifications.
The narrower proposed facets in v1 do not discard later Gate A/B or WP10 scope.

Runtime #239's delivered audit distinguishes the CLI fixture from existing real
learning/Birthday owners. Preserve those owners and Runtime #112's delivered
repair. Reconcile learning/memory/continuity through their native owners rather
than promising a new global transaction or replacement identity.

Proposed decision acceptance is left to the owning review process. No ADR
acceptance, installed migration, fleet rollout, release or empirical improvement
is claimed by this source document.
