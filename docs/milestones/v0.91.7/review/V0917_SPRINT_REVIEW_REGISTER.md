# v0.91.7 Sprint Review Register

Status: active_review_register

Last updated: 2026-07-07

Issue: #4972

## Purpose

This register is the canonical v0.91.7 sprint-review status list. It records
what has been reviewed, what findings remain open, and what must happen before
the milestone can claim release readiness.

It does not close any WP by itself. A WP counts as release-ready only when its
implementation/proof work is complete, reviewed, remediated, and truthfully
closed out in the issue/card/PR surfaces.

## Current Summary

- WP-01, WP-02, and WP-04 have tracked review packets and are closed.
- WP-05 is closed and review-remediated by `#4932`; the local-agent proof
  artifact now agrees with the fail-closed provider/model identity guard.
- WP-06 is closed and review-remediated by `#4936`; selected build-throughput
  work and remote-builder follow-ons have landed and are recorded in the WP-06
  packet.
- WP-07 has substantial runtime review artifacts and many closed runtime
  issues, but CSM survival/post-blocker issues remain open before final release
  readiness.
- WP-08 has a runtime AWS/signal packet and is waiting on final review/closeout
  reconciliation rather than missing child implementation proof. WP-09 through
  WP-20 are not yet release-review-clean.
- WP-21 is closed; WP-22 and WP-23 remain open for next-milestone review and
  release ceremony.
- Tools sprint #4806 is closed and review-remediated by #4961 for stale child
  card truth, tracked review evidence, and remaining sprint-conductor raw-`gh`
  helper paths. #4950 remains a visible watcher closeout-state residual.

## Review Status Table

| WP | Umbrella | Status | Review Packet | Findings / Remediation | Next Action |
| --- | ---: | --- | --- | --- | --- |
| WP-01 | #4628 | closed | `docs/milestones/v0.91.7/review/V0917_WP01_PLANNING_PROMOTION_4628.md` | No active finding recorded in this register. | Keep as source truth for release-tail review. |
| WP-02 | #4629 | closed | `docs/milestones/v0.91.7/review/V0917_WP02_V0916_CLOSEOUT_TRUTH_CONSUMPTION_4661.md` | Child cleanup `#4661`-`#4665` and `#4699` are closed. | Keep as source truth for release-tail review. |
| WP-03 | #4630 | closed; review finding remains | `docs/milestones/v0.91.7/review/V0917_WP03_REVIEW_4972.md`; remediation packet `docs/milestones/v0.91.7/review/V0917_WP03_REVIEW_REMEDIATION_4953.md` | #4953 is closed and repaired the merged-PR/no-PR shepherd classifier confusion, stale #4713 residue, and WP-03 label discoverability. Current review still finds #4630 shepherd output reports `merged_needs_closeout`; its local readiness also fails in this review worktree because that worktree lacks the #4630 source prompt even though the root checkout retains it. Cross-cutting resilience/workflow issue #4780 is now closed via PR #5008. | Resolve the #4630 closeout-state and worktree-local readiness drift, or record it as an approved release-tail blocker before WP-03 is consumed as clean. |
| WP-04 | #4631 | closed | `docs/milestones/v0.91.7/review/V0917_WP04_CLOSEOUT_4631.md`; `docs/milestones/v0.91.7/review/V0917_WP04_CLOSEOUT_REMEDIATION_4747.md` | Remediation issue `#4747` is closed. | Keep metrics limitations visible; do not treat unknown metrics as zero. |
| WP-05 | #4632 | closed; review-remediated | `docs/milestones/v0.91.7/review/V0917_WP05_SCHEDULER_PROVIDER_LOCAL_AGENT_CLOSEOUT_4632.md` | #4932 is closed and repaired the stale `#4675` local-agent artifact after `#4849`; provider route and model suitability now both select Gemini while local Gemma remains shadow-only. | Keep as source truth unless new findings appear. |
| WP-06 | #4633 | closed; review-remediated | `docs/milestones/v0.91.7/review/V0917_WP06_BUILD_THROUGHPUT_VALIDATION_COST_REDUCTION_4633.md` | #4936 is closed and repaired review-truth records. The WP-06 packet now records the selected sprint lane plus reconciled remote-builder follow-ons: `#4837`, `#4838`, `#4879`, `#4680`, and `#4679`. | Keep build-throughput residual non-claims visible; paid AWS lanes remain explicit operator-triggered paths. |
| WP-07 | #4634 | closed umbrella; open CSM survival follow-ons | runtime review packets under `docs/milestones/v0.91.7/review/runtime/` plus `docs/milestones/v0.91.7/review/observability_4718/` | Runtime/OTel/Soak artifacts exist; open issues include `#4906`, `#4910`, `#4911`, `#4918`, `#4919`, `#4921`, `#4922`, `#4929`, and `#4933`. | Finish CSM survival/post-blocker issues and run final WP-07 review. |
| WP-08 | #4635 | open; implementation packet exists | `docs/milestones/v0.91.7/review/V0917_WP08_RUNTIME_AWS_SIGNAL_OPERATIONS_4635.md` | Runtime AWS/signal child issues `#4684`-`#4688`, `#4913`, and `#4915` are closed with retained proof. Adjacent cross-cutting resilience dependency `#4782` is also closed and consumed as related AWS/remote-builder durability truth, not as a WP-08 child. Final WP-08 review/closeout truth still needs reconciliation before release-ready claim. | Run final WP-08 review/closeout reconciliation and keep live AWS proof boundaries visible. |
| WP-09 | #4636 | open | Unity review artifacts under `docs/milestones/v0.91.7/review/unity_observatory_*` | Umbrella and child issues `#4689`-`#4691` remain open. | Finish Observatory/demo proof and review as WP-09. |
| WP-10 | #4637 | open | none yet | Curiosity `#4692` and constructability `#4693` remain open. | Implement and review curiosity/constructability proof. |
| WP-11 | #4638 | open | none yet | Reasoning graph, loops, skill standard, AEE/ObsMem, and Godel snapshot issues remain open. | Implement and review all WP-11 runtime/cognitive surfaces. |
| WP-12 | #4639 | open | none yet | Security/protocol issues `#4656`-`#4660`, `#4914`, `#4917`, and `#4920` remain open. | Implement and review security/protocol surfaces. |
| WP-13 | #4640 | open | none yet | Affect, Godel, economics, guild, CodeFriend, and publication issues remain open. | Implement and review WP-13 surfaces. |
| WP-14 | #4641 | open | none yet | Launch/birthday handoff children `#4758`-`#4763` remain open. | Finish launch and v0.92 birthday handoff proof. |
| WP-15 | #4642 | open | none yet | Demo matrix / proof coverage not yet review-clean. | Execute after implementation WPs are sufficiently proven. |
| WP-16 | #4643 | open | none yet | Quality gate not yet review-clean. | Execute after implementation/demo evidence is current. |
| WP-17 | #4644 | open | none yet | Docs/adoption review pass not yet complete. | Run docs alignment after WPs stabilize. |
| WP-18 | #4645 | open | none yet | Internal review not yet run for v0.91.7. | Run after WP-17. |
| WP-19 | #4646 | open | none yet | External review not yet run for v0.91.7. | Run after WP-18 remediation is ready. |
| WP-20 | #4647 | open | none yet | Review remediation not yet started. | Fix findings from WP-18/WP-19. |
| WP-21 | #4648 | closed | none found as a review packet | Next milestone planning closed early relative to open implementation WPs; consume cautiously. | Recheck during WP-22. |
| WP-22 | #4649 | open | none yet | Next milestone review pass not yet complete. | Review v0.92 planning after WP-21/WP-20 truth is stable. |
| WP-23 | #4650 | open | none yet | Release ceremony not yet complete. | Run only after all required review/remediation gates are clean or explicitly blocked with operator approval. |

## Tools Sprint Review Records

| Sprint | Status | Review / Remediation Packet | Findings / Residuals | Next Action |
| --- | --- | --- | --- | --- |
| Repo-native workflow stabilization | #4806 closed; review-remediated | `docs/milestones/v0.91.7/review/V0917_TOOLS_SPRINT_4806_REVIEW_REMEDIATION_4961.md` | #4961 is closed and repaired stale child card truth, tracked release-visible review evidence, remaining sprint-conductor raw-`gh` helper paths, and owner-binary fallback wording. #4950 remains open for watcher `closeout_needed` ambiguity. | Keep #4950 visible until resolved. |
| Resilience integration mini-sprint | #4778 ready for umbrella publication | `docs/milestones/v0.91.7/review/V0917_RESILIENCE_INTEGRATION_MINI_SPRINT_4778.md` | #4780, #4781, #4782, #4783, and #4784 are closed with retained workflow, provider/model, AWS/remote, runtime middleware, and failure-injection proof. PR #5014 merged with `adl-ci` and `adl-coverage` green. | Publish and close out #4778 umbrella truth. |

## WP-05 Repair Record

Review found that the retained local-agent delegation artifact still combined a
ChatGPT provider route with Gemini model-suitability selection. That no longer
regenerated under the current scheduler because `#4849` correctly added
fail-closed provider/model identity validation.

This issue repairs the WP-05 proof surface by:

- updating `adl/tests/fixtures/scheduler/local_agent_delegation_readiness_inputs_v1.json`
  so the eligible provider route is `google/gemini-2.5-flash`;
- marking the previous ChatGPT route ineligible for this proof because it is
  not the cheapest validated outcome for the task;
- regenerating
  `docs/milestones/v0.91.7/review/provider/artifacts/local_agent_delegation_readiness_plan_4675.json`;
- preserving local Gemma as `shadow_only` advisory delegation, with no
  autonomous execution, repo mutation, closeout, or merge authority.

## Non-Claims

- This register does not claim v0.91.7 is release-ready.
- This register does not close any WP or child issue.
- This register does not claim WP-07 or later open WP findings are fixed.
- This register does not claim live provider invocation, live local model
  quality, or autonomous multi-agent authority from WP-05 scheduler artifacts.
