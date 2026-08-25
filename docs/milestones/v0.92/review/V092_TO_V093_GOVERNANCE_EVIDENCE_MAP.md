# v0.92 To v0.93 Governance Evidence Map

## Status

WP-19 evidence map for issue `#5839`.

This map is a bounded handoff surface. It names what the accepted v0.93 planning
package may consume from v0.92 identity and birthday work, and it names what
remains blocked or forbidden. It does not grant citizenship, does not grant
standing, does not assign rights or duties, does not complete v0.93 governance,
and does not accept ADR 0068.

## v0.93 Allocation Authority

The accepting consumer is the accepted v0.93 planning allocation, not a concrete
opened v0.93 implementation issue. The current authority is:

- `docs/milestones/v0.93/DECISIONS_v0.93.md`, D-01: v0.93 owns
  constitutional citizenship, social cognition, and polis governance,
  `Accepted for planning`.
- `docs/milestones/v0.93/CONSTITUTIONAL_CITIZENSHIP_AND_POLIS_GOVERNANCE_PLAN_v0.93.md`,
  which allocates constitutional citizenship, rights/duties, standing,
  constitutional review, Theory of Mind, shared social memory, delegation/IAM,
  polis governance, and governance/security evidence to v0.93.
- `docs/milestones/v0.93/WP_ISSUE_WAVE_v0.93.yaml`, which is still a
  candidate issue wave. That means this map may prepare downstream consumption,
  but must not claim opened v0.93 implementation ownership.

## Evidence Map

| v0.92 evidence source | Accepted state or blocker | Allowed v0.93 use | Forbidden inference | Redaction posture | Unresolved decision | Accepting consumer |
| --- | --- | --- | --- | --- | --- | --- |
| Birthday review packet: `docs/milestones/v0.92/review/FIRST_BIRTHDAY_REVIEW_PACKET_v0.92.md` | Accepted WP-16 review-packet source on current main. | Use as the reviewer-facing inventory of identity, continuity, capability, cognitive-profile, witness, receipt, and claim-boundary evidence expected from v0.92. | Must not infer citizenship, personhood, standing, rights, duties, or governance approval from the existence of a birthday packet. | Consume evidence references and summaries only; do not expose private raw state. | v0.93 must define its own constitutional-review packet shape before using these rows in governance findings. | v0.93 constitutional citizenship and polis-governance planning package. |
| Birthday evidence manifest: `docs/milestones/v0.92/review/first-birthday-review-evidence.v1.json` | Accepted WP-16 machine-readable evidence manifest on current main. | Use exact evidence entries to bind future governance-review inputs to source files and terminal states. | Must not treat a missing or blocked row as implicit approval. | Use digest/path/status fields and governed projections; do not import local-only artifacts as public evidence. | v0.93 must decide which evidence classes become required governance-review inputs. | v0.93 reviewer-facing constitutional evidence planning. |
| Cross-polis continuity transfer: `docs/milestones/v0.92/features/CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md` and `.csdlc/evidence/5835/dependency-authority.json` | WP-17 is merged per operator update and current source contains the continuity-transfer contract. | Use movement semantics to distinguish portable references, local-only state, continuity ambiguity, and fail-closed migration posture. | Must not infer that a copy, relocation, or migration grants citizenship or preserves standing. | Consume portable references and redaction labels; keep local-only machine state local. | v0.93 must define governance consequences for ambiguous or failed continuity transfer. | v0.93 standing, appeal, and cross-polis governance planning. |
| Rejected continuity-transfer matrix: `.csdlc/evidence/5835/rejected-transfer-matrix.json` | Accepted WP-17 negative matrix on current main. | Use rejected cases as future governance negative fixtures for copy, relocation, missing authority, and evidence gaps. | Must not reclassify rejected transfer cases as birth, citizenship, or governance admission. | Negative-case labels and reasons are consumable; private source material remains non-public unless separately redacted. | v0.93 must define appeal/review treatment for rejected or ambiguous transfers. | v0.93 standing transition and constitutional review planning. |
| Runtime first-birthday demo: merged issue #427 and `docs/milestones/v0.92/features/FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md` | Landed bounded local engineering demo; external publication remains operator-gated. | Use the declared demo contract and retained engineering evidence as planning context. | Must not claim the birthday demo is public, governance-complete, or externally publishable from this map. | Use documented packet fields and governed projections; do not expose private runtime artifacts. | v0.93 must define its own governance acceptance rather than inheriting publication authority. | v0.93 flagship constitutional-review demo planning. |
| v0.93 governance allocation: `docs/milestones/v0.93/DECISIONS_v0.93.md` and `docs/milestones/v0.93/CONSTITUTIONAL_CITIZENSHIP_AND_POLIS_GOVERNANCE_PLAN_v0.93.md` | Accepted for planning; concrete implementation issue wave remains candidate. | Names the downstream consumer and separates v0.92 identity evidence from v0.93 governance interpretation. | Must not treat planning allocation as completed governance implementation or production constitutional authority. | Public planning docs are consumable; private governance evidence requirements must be designed by v0.93. | v0.93 WP-01 must open concrete implementation issues before execution credit. | v0.93 planning promotion and issue-wave readiness. |

## Forbidden Governance Claims

This map rejects the following claims:

- v0.92 birthday evidence grants citizenship.
- v0.92 continuity evidence grants standing.
- v0.92 handoff prose assigns rights or duties.
- v0.92 review packets complete v0.93 governance.
- ADR 0068 is accepted architecture.
- The candidate v0.93 issue wave is an opened implementation wave.

## Rollback Boundary

If v0.93 allocation changes before this handoff is consumed, rollback is
limited to this map and the ADR-plan row. Rollback must preserve accepted v0.92
evidence, retain rejected cases as evidence, and never fabricate governance
authority.
