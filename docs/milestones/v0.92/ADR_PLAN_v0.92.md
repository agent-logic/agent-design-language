# v0.92 ADR Plan

## Status

Forward ADR planning for `v0.92`.

This plan does not accept any ADR by itself. It identifies candidate
architecture decisions that WP-01 and the v0.92 review tail should confirm,
split, draft, or explicitly defer.

## Purpose

v0.92 introduces the first true Gödel-agent birthday. That milestone changes
architecture boundaries around identity, continuity, memory grounding,
cognitive profiles, binary ACIP communication, and the governance handoff.
Those boundaries should not live only in feature prose.

The ADR goal is to make the durable decisions reviewable before the milestone
claims completion.

## Existing Baseline

Accepted ADRs currently live in `docs/adr/` and run through ADR 0058. ADR
0051 remains a deferred disposition rather than an accepted architecture
decision; ADR 0058 consumes its Memory Palace obligation.

Relevant inherited decisions:

- ADR 0009: bounded cognitive system architecture.
- ADR 0010: chronosense as a first-class substrate.
- ADR 0011 and ADR 0012: long-lived runtime and bounded CSM run architecture.
- ADR 0013: citizen-state continuity substrate.
- ADR 0016: moral evidence and cognitive-being substrate.
- ADR 0017: secure local agent comms and A2A boundary.
- ADR 0019: Theory of Mind foundation.
- ADR 0028: C-SDLC tracked workflow state and signed trace boundary.

v0.92 should cite and refine these records rather than rewriting them
casually.

## Candidate ADR Set

| Candidate | Title | Disposition | Primary boundary | Source WPs |
| --- | --- | --- | --- | --- |
| ADR 0059 | First True Birthday Evidence Boundary | Proposed | Birthday candidate validation is structural; trusted authority and freshness remain separate. | WP-08 |
| ADR 0060 | Stable Identity, Name, And Continuity Record Boundary | Proposed | Stable identity and bounded continuity use digest-bound records and explicit ambiguity. | WP-09, WP-10 |
| ADR 0061 | Memory Grounding And Capability Envelope Boundary | Proposed | Memory and capability inputs remain bounded, referenced, and fail closed. | WP-11, WP-12 |
| ADR 0062 | Witness And Birthday Receipt Authority Boundary | Proposed | Exact-candidate signed witnesses and redacted receipts use opaque runtime trust policy. | WP-15 |
| ADR 0063 | ACP Cognitive Profile Evidence Boundary | Proposed | Cognitive profiles are runtime-authority-bound projections, not identity or rights authority. | WP-13 |
| ADR 0064 | Adaptive Learning DAG Governance Boundary | Proposed | Adaptive graph mutations are authority-bound, bounded, replayable, and fail closed. | WP-13A |
| ADR 0065 | ACIP Schema Catalog And Governed Projection Boundary | Proposed | #283 reconciles the stale #5832 receipt against exact replacement terminal authority from #209 / PR #215 with non-empty machine-readable validation. | WP-14, WP-18C |
| ADR 0066 | Distributed Guardian Membership, Authority, And Fencing Boundary | Deferred | #284 retains terminal and partial Guardian evidence, but two-voter AWS/model-health proof and #142 completion remain residual gaps. | WP-04, WP-18C |
| ADR 0067 | Runtime Transport And TLS Stack Boundary | Proposed | Runtime transport uses one Rustls-backed trust model with explicit production certificate input. | WP-03, WP-04, WP-14 |
| ADR 0068 | Birthday-To-Governance Handoff Boundary | Deferred | #285 retains terminal WP-19 handoff evidence, but WP-18/#5836 birthday proof is not terminal and no ADR acceptance is claimed. | WP-18, WP-19, WP-18C |
| ADR 0069 | Observatory Governed Runtime Consumer Boundary | Deferred | #286 records #84/WP-18A Unity Runtime consumer proof as open; #117/#271/#282 are partial inputs only. | WP-18A, WP-18C |
| ADR 0070 | Cross-Polis Continuity Transfer Planning Boundary | Proposed | Copying is not continuity; operational migration remains deferred. | WP-17 |
| ADR 0071 | Provider-Neutral Multi-Agent Proof Boundary | Deferred | #287 records #341/WP-18B as open with no derived terminal cache; #283-#286 are supporting-only inputs. | WP-18B, WP-18C |

## Authoring Policy

- Candidate ADRs should be drafted only from landed feature work, tests,
  fixtures, demos, review findings, and milestone docs.
- Candidate ADRs remain proposed until human review accepts them.
- ADR 0068 remains deferred until terminal birthday proof exists; the v0.93
  planning handoff evidence remains useful input but does not grant citizenship,
  does not grant standing, does not assign rights or duties, and does not accept
  or implement v0.93 governance.
- Accepted ADRs should live in `docs/adr/`.
- Candidate/provenance copies should live in `docs/architecture/adr/` if the
  milestone follows the existing ADR promotion pattern.
- ADRs must preserve non-claims for legal personhood, production citizenship,
  completed v0.93 governance, production transport security, and signed trace
  closure unless those are explicitly implemented and reviewed.

## WP Integration

- WP-01 should confirm this plan is still complete when opening the issue wave.
- Feature WPs should record decision implications in their SRP/SOR or review
  notes.
- WP-16 should prepare the ADR packet for review if implementation produced the
  expected architecture decisions.
- WP-17 and WP-18 should review the ADR packet alongside code, docs, tests,
  demos, and release evidence.
- WP-19 should fix, defer, or route ADR findings.
- WP-22 should not close v0.92 with missing ADRs for accepted architectural
  boundaries.

## Acceptance Criteria

- Every v0.92 architecture boundary that must survive the milestone has an ADR
  candidate, an explicit deferral, or an accepted existing ADR reference.
- Candidate ADRs cite source evidence and keep proposed/accepted status clear.
- ADR 0059 through ADR 0071 are authored as Proposed or Deferred candidates
  before v0.92 closeout.
- No ADR claims the first birthday proves personhood, production citizenship,
  completed governance, production transport security, or signed trace closure.

## Notes

This ADR plan is intentionally forward-looking. It should become stricter after
v0.92 WP-01 opens the final issue wave and after the first implementation WPs
produce evidence.
