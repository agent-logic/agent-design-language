# v0.93.2 successor closeout plan — ADL #1243

## Status and source boundary

This is a proposed closeout plan, not execution of v0.93.2 or release authority. It completes the planning surface of ADL #1242, whose accepted final roster is still required before acceptance of this plan. ADL #1223 was merged through PR #1273 at `791e44216659a14eeb95c8a05f79e965709b3421`; asynchronous native closeout is separate from source delivery.

Reuse the existing [#1259 planning package](https://github.com/agent-logic/agent-design-language/pull/1264) at `0ba1106676c7c3a815e91336ae32942b5c51ba49`, particularly EXECUTION_PLAN_v0.93.2.json and RELEASE_PLAN_v0.93.2.md. The #1242 preparation at `0e5d72768f` proposes 67 retained identities from 68: only PY-01 moves to the next milestone, whose version is not activated. Affect and humour remain outside v0.93.2. These are planning identities, not a count of opened issues or delivered results.

Preserve CodeFriend, the full GHB program, Runtime v4/plugins, governance/ToM/relationships/delegation/guilds, enterprise security, citizen migration/reproduction, demonstrations, integration and qualification. Runtime activation and citizen continuity are shared GHB dependencies, not duplicate engines. Reuse CodeFriend #92 and Runtime #112, not replacement issues.

## Owner roles and ordered work packages

Named owners are assigned at execution admission; the roles below are accountable planning roles, not fabricated staffing commitments. Existing v0.93.2 TAIL identities are reused. Within a phase, independent repository evidence collection may run in parallel; a downstream decision cannot claim its predecessor accepted prematurely.

| Existing identity | Accountable role | Inputs and dependencies | Required result and evidence |
| --- | --- | --- | --- |
| INTEGRATE | Integration owner | Delivered retained product WPs and their exact contracts | One compatible source/artifact lockset; consumer and rollback compatibility matrix; unresolved results remain visible. |
| QUALIFY | Qualification owner | INTEGRATE; all required product/demo gates, including GHB Gate A, Gate B and WP12 | Installed product evidence at the lockset, actual test/demo denominators and failures, supported platforms, provenance and recovery evidence. PY-01 is excluded only after #1242 reconciliation is accepted. |
| TAIL-01 | Quality owner | QUALIFY and complete retained roster | Per-WP accepted/deferred/failed disposition with evidence and rationale; no aggregate green claim hiding missing entries. |
| TAIL-02 | Documentation owner | TAIL-01 and versioned product interfaces | Readable docs/review packet, onboarding/build commands per owning repo, known limitations and disclosure inventory. |
| TAIL-03 | Release preparation owner | TAIL-02 and candidate lockset | Exact versions, artifact identities/integrity, release notes, compatibility and rollback catalog; separate public/private inventory and permissions. Preparation does not publish. |
| TAIL-04 | Independent internal reviewer | TAIL-03 frozen packet | Findings linked to exact source/artifact identities, severity and acceptance impact. Reuse unchanged proof; review changed evidence only. |
| TAIL-05 | External review coordinator | TAIL-04 dispositions and disclosure-cleared packet | Independent review evidence and covered scope; record unavailable or omitted review honestly. Provider cost or disclosure requires its applicable authority. |
| TAIL-06 | Remediation owner | TAIL-04/05 findings | Every finding fixed and verified or explicitly deferred with owner, target and rationale. Requalify only affected dependencies; no automatic acceptance of unresolved release blockers. |
| TAIL-07 | Successor planning owner | TAIL-06 accepted dispositions | Next milestone scope and residual register; no dropped or silently reassigned findings. |
| TAIL-08 | Successor closeout planner | TAIL-07 accepted package | Successor quality/review/release sequence with owner roles, evidence and decision boundaries, following this plan. |
| TAIL-09 | Independent planning reviewer | TAIL-07/08 | Bounded consistency review of scope, dependency graph, residual routing and acceptance criteria; findings resolved before final decision. |
| TAIL-10 | Release owner and operator | TAIL-01 through09 accepted, explicit release authority | Authorized release identity and destination readback, final issue/PR disposition, next handoff and asynchronous native terminal reconciliation/cleanup. |

## CodeFriend launch compatibility

v0.93.1 is required to deliver a ready-to-deploy CodeFriend candidate and wait-list mechanism; it is not proof of public launch. CodeFriend #92 remains the existing later launch route. Public deployment requires an explicit exact candidate, audience, environment and budget decision. Preserve the accepted report/provenance/verification contract, private repository handling and waiting-list behavior. Changes must identify compatibility and rollback evidence for affected consumers. Do not repeat the older release-plan sentence that assigns Beta 1 public launch to v0.93.1.

## Evidence and deferral contract

For each result retain repository, issue/PR, reviewed source head, artifact identity, installed qualification where required, proof selector and actual outcome, reviewer and disposition. An artifact merely being available is not verified integrity or installed acceptance. Keep original failures and subsequent fixes distinct. For deferred findings record severity, affected consumers, reason, owner role, destination issue/milestone, and explicit acceptance decision; an unknown destination remains an unresolved handoff item. Preserve #875/#1145 routes, #915 adverse evidence and #1148/#1149 provenance; reuse #1150 proof and completed #671 rather than allocating duplicates.

Reuse unchanged proof at its exact identity. Re-run a focused check only when changed behavior or evidence invalidates it. Documentation changes require links, scope/dependency checks and diff hygiene; they do not warrant product rebuilds. CI execution and local preflight are recorded separately.

## Acceptance checklist for this planning issue

- #1242 final roster and #1241 findings dispositions are accepted and reconciled with this plan; currently pending.
- Every existing successor closeout identity has a role, predecessor and observable evidence requirement (table above).
- CodeFriend launch and GHB gates are preserved; Python and affect/humour deferrals remain explicit.
- Independent review of this exact planning change is recorded; no release outcome is inferred from review of a plan.
- Native publication and issue acceptance remain distinct from this draft preparation.

## Current v0.93.1 handoff and non-claims

This plan belongs to the v0.93.1 Sprint 4 preparation work, but describes v0.93.2 closeout. It neither closes v0.93.1 nor opens the successor. Final v0.93.1 lockset, #1232 quality disposition, #1234 documentation packet, #1236 publication packet and #1241 remediation decisions remain the owning issues' evidence. Native closeout may run asynchronously without holding unrelated source delivery, but pending receipts are not reported as complete. No deployment, paid execution, tag, release, new issue wave, shared binary change or broad qualification run is performed by this document.
