# v0.93 Decisions

## Status

Forward-planning decisions. These are accepted as planning boundaries for the
v0.93 allocation, but they are not implementation closeout decisions.

## Decision Log

| ID | Decision | Status | Rationale | Impact |
| --- | --- | --- | --- | --- |
| D-01 | v0.93 owns constitutional citizenship, social cognition, and polis governance. | Accepted for planning | v0.91 owns moral trace and v0.92 owns identity/birthday, so governance and social cognition need their own later layer. | Keeps moral evidence, identity, ToM, reputation, standing, and governance from collapsing into one milestone. |
| D-02 | A human is not the citizen; the CSM identity is the citizen. | Accepted for planning | Human input can be allowed, but citizen action requires identity binding, Freedom Gate mediation, signed trace, and temporal anchoring. | Prevents operator action from masquerading as citizen conduct. |
| D-03 | Constitutional review consumes moral trace; it does not redefine moral trace. | Accepted for planning | v0.91 should define moral events, trace, metrics, and trajectory review. | Reduces duplicate schemas and keeps v0.93 focused on policy interpretation. |
| D-04 | Standing changes must be evidence-based and appealable. | Accepted for planning | Citizenship without review transparency risks arbitrary punishment. | Requires challenge, appeal, evidence, and restoration surfaces. |
| D-05 | Governance evidence must be privacy-preserving. | Accepted for planning | Reviewers need evidence, but not raw private state. | Forces redacted projection and evidence-reference design. |
| D-06 | Economics and payment rails are not v0.93 scope unless later planning creates a narrow bridge. | Accepted for planning | Economics has its own planned lane and should not swamp constitutional governance. | Prevents contract-market work from hiding inside governance language. |
| D-07 | Theory of Mind is private social cognition, not public reputation or standing. | Accepted for planning | ToM is useful only if it remains evidence-grounded, bounded, and distinct from governance verdicts. | Requires signed update events, redacted projections, conflict/decay semantics, and explicit authority before ToM can influence review. |
| D-08 | v0.93 owns the first ToM, reputation-boundary, and shared-social-memory planning pass. | Accepted for planning | v0.90.3 now supplies standing/access, v0.91 supplies moral trace, and v0.92 supplies identity/birthday evidence. | Moves ToM out of stale late-roadmap targeting while preventing premature v0.90.3 implementation claims. |
| D-09 | v0.93 owns an explicit six-WP enterprise-security tranche. | Accepted for planning | Governance without zero trust, policy enforcement, key/secrets lifecycle, audit/compliance evidence, isolation/data governance, and security operations would not be credible for an inhabited polis. | Adds WP-S1 through WP-S6 as required v0.93 planning work while avoiding external certification claims. |
| D-10 | v0.93 owns upstream delegation as part of delegation/upstream-delegation/IAM governance. | Accepted for planning | Upstream cognition changes authority, identity, provenance, cost, and verification boundaries, so it belongs with governed delegation rather than runtime acceleration or C-SDLC. | Adds governed escalation to polis services, trusted external polis boundaries, and frontier cognition providers without treating providers as sovereign actors outside trace and policy. |

## Open Questions

- Which governed-tool authority surfaces will be available by v0.93?
- Does v0.93 need a narrow economics/governance bridge, or should economics
  remain fully separate?
- What is the minimum social-contract representation that is useful without
  overclaiming complete constitutional authority?
- What is the minimum shared social memory packet that is useful without leaking
  private ToM or private citizen state?
- Which constitutional review demo should become the flagship proof surface?
- Which enterprise-security demo should become the flagship proof surface:
  zero-trust denial, key rotation/revocation, incident/audit evidence,
  isolation leakage prevention, or an integrated governance-security scenario?

## Exit Criteria

- Later v0.93 WP planning keeps each accepted boundary intact or records a
  deliberate supersession.
- Any production-law, personhood, payment, economics, external certification, or
  production-compliance expansion is captured as an explicit decision before
  implementation.

## Metadata

Planning template set: 1.1.0. Target: v0.93. Authoring issue: #1047; current reconciliation: #922 in v0.92.2. Accountable planning role: milestone owner; named implementation owners are assigned before opening.

## Purpose

Separate operator-selected scheduling from proposed design and unresolved execution choices.

## How To Use

Original D-01 through D-10 remain planning boundaries, not proof of implementation. The new register below supersedes conflicting scheduling and destination text.

## Current Decision Register

| ID | Decision | Status / owner | Resolution gate |
|---|---|---|---|
| D-11 | Repo split opens v0.93 after v0.92.2 acceptance/closure | Operator selected | WP-01/RD-11 |
| D-12 | One public ADL and six private repositories; CodeFriend software separate from website | Operator boundary; software name proposed | RD-02 confirms exact names and people |
| D-13 | Runtime v4 belongs in v0.93 | Required by explicit operator direction on 2026-09-16 | RV-01 through RV-11 completed and independently qualified before release |
| D-14 | Retain existing governance and six security tracks | First-pass assumption; scope/capacity review pending | Before opening issue wave |
| D-15 | Customer-scale CodeFriend deployment | Required by explicit operator direction on 2026-09-16; launch audience/environment/limits resolved before execution | CF-01 and #922 final reconciliation; actual publication needs explicit authorization |
| D-16 | Licenses, registries, supported versions, history/issue migration, evidence retention | Proposed owners in migration plan; named owners and policy unresolved | RD-02 before extraction |
| D-17 | Runtime v4 review defects | Proposed corrections in feature doc; not accepted implementation design | RV-01 before RV-02 |
| D-18 | ATE, OCI, economics, optional modernization and cloud/account operations | Retain existing separate routing; no automatic admission | #922 final reconciliation |

Runtime v4 is mandatory in v0.93. RV-01 through RV-11 are required, including native, process and WASM adapters, provider integration and installed recovery qualification. Native generation recovery is an intermediate dependency gate, not a reduced release scope. Design review resolves implementation choices without deferring required outcomes. No calendar estimate is claimed.

CodeFriend Beta 1 must launch in v0.93. CF-07 is mandatory: an authorized live product, tester admission, end-to-end verification and retained rollback. Planning, a preview or withheld execution authorization cannot satisfy release completion. #1047 prepares this obligation and does not itself launch the service.


## Operator-confirmed additions — #922 / 2026-09-22

- D-19: CodeFriend versioned templates and per-kind style guides are required v0.93 scope. CT-01–CT-10 must complete before CF-05 launch qualification; use the supplied REV 01 brand source and full supported artifact catalog.
- D-20: Citizen reproduction/migration is required v0.93 scope. CM-01–CM-04 provide the bounded identity/lineage/custody outcome and feed INTEGRATE. This replaces the old unresolved placement question.
- D-21: #922 may draft/update now by operator instruction; final #921 evidence, predecessor acceptance, inter-milestone break and explicit v0.93 opening remain required. No permission for new product implementation or live launch is implied.
