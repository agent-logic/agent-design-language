# v0.92.2 documentation and review handoff

**Draft for review; acceptance pending #916.** The operator authorized #917 documentation work under the approved deferral of incomplete #915 qualification to v0.93 before CodeFriend Beta1 launch. This packet supports that review; it is not a release approval or authorization to send private material externally.

## Candidate and scope

Historical tested ADL implementation snapshot: `2b047c8dc34e133ee1353ec265507a17c523e06f` (merged #1137/#1138). Historical tested website snapshot: `a45e339c13b24716edbd3fadf29dddff36ffe02e` (merged CodeFriend.ai #18). The inherited #916 evidence checkpoint is `e119223cd1`, merged into this documentation branch. It includes the earlier test-only control-readiness correction and subsequent quality-evidence updates; it is not an accepted installed release candidate. The source revisions below remain the historical test baselines, not a claim about current main. [Manifest](HANDOFF_MANIFEST.json) binds each inspected document's bytes and the exact full parent revision, plus all 69 canonical task identities.

Review the current [quality decision](../issue-916/QUALITY_DECISION.json), [task ledger](../issue-916/TASK_LEDGER.json), [24-prerequisite assessment](../issue-916/PREREQUISITE_ACCEPTANCE.json) and [post-merge checks](../issue-916/POSTMERGE_VERIFICATION.json). The canonical [document inventory](../../CANONICAL_DOC_INVENTORY_v0.92.2.md) includes every milestone Markdown document recursively, beyond the original hand-picked planning list. The manifest binds that full set plus required machine-readable contracts and evidence, including the latest Sprint 10 checkpoint, repaired-candidate merge readback and 84-artifact producer hash audit. Historical study documents are retained unchanged; their statistics were not recomputed.

## What is established

- Merged ADL and website integration has 154 affected native tests and 141 website tests passing, with no failures or skips. The earlier 396-test native census belongs to its earlier source revision; do not add these overlapping denominators together.
- #916 planning parity passes 69 task identities and 298 negative fixtures. Five historical Runtime criteria replay against exact retained evidence; those results do not qualify a new integrated installed candidate.
- The repaired PAIR discovery endpoints answer successfully. No new inference or failover run is claimed. The historical PAIR experiment decision remains REPAIR; MLX has bounded smoke proof without review-speed superiority; speculative decoding remains repair/inconclusive.
- #912 and #913 satisfied the operator-accepted article-draft and revision-4 private PDF handoffs. Further editing and publication are separate work.
- Podcast #671 and live pilot #875 are now v0.93. Preserve all 69 original identities; distinguish explicit deferral from execution failure or completed pilot proof.

- All 84 indexed prerequisite evidence artifacts resolve at their declared revisions/paths and match retained SHA-256 values. This proves indexed-byte availability only, not source-specific acceptance or final candidate freshness.

## What is not established

The original #915 qualification remains incomplete: zero of twelve journey cells accepted, with twenty-four obligation rows not fully established. The operator approved [deferral to v0.93 before CodeFriend Beta1 launch](../issue-916/SPRINT10_DEFERRAL.json). That disposition does not pass #916 or authorize launch. The successor qualification work must deliver independent installed-candidate proof for both repositories, both website modes, invitation-only access/user isolation, actual approved provider review, second-run behavior and rendered exports. Source tests, endpoint discovery, issue closure and a merged patch do not establish deployment or that qualification. The final installed binary identities, accepted manifest and #916 passing decision are absent. No external review should issue a release verdict from this draft. OBS-S3/#910 retains its separate deployment obligation. The #945 packet records acceptance of all twelve ARCH-ADR decisions, satisfying that specific #925 obligation without establishing product or release acceptance.

## Fresh-reader walkthrough

From the ADL repository root, using Python 3.9 or later:

```sh
python3 docs/milestones/v0.92.2/evidence/issue-917/validate_handoff.py
python3 docs/milestones/v0.92.2/evidence/issue-917/validate_handoff.py --self-test
python3 docs/milestones/v0.92.2/validate_planning.py --self-test
python3 docs/milestones/v0.92.2/adr/issue-911/validate_packet.py --self-test
python3 docs/milestones/v0.92.2/adr/issue-945/validate_packet.py --self-test
```

The handoff validator checks exact document digests, local linked-path availability, the 69 unique task mapping, 24 prerequisite rows, declared source identities and the pending-acceptance guard. It does not execute commands found in documents or contact external links. Planning validation reports its nonzero negative-fixture count. Per-target results are in #916's source-bound evidence. The component commands below are reconstructed from those target lists and repository manifests; they are not a retained shell transcript. Rerun affected tests when the candidate changes, rather than treating this draft as a permanent green receipt.

Read the manifest and verify its document inventory. Follow the quality decision and task ledger, sample a claimed pass and a not-proven row, then check release notes and feature requirements against that evidence. Required paths must resolve; private raw evidence must be made available before accepting a claim that requires it. Record P1/P2 findings before summary and identify the exact file, line, source revision, consequence and bounded correction. An independent walkthrough remains required before final handoff acceptance.

## Component reproduction

Use separate disposable checkouts with the required private repository access. Do not change a shared checkout to reproduce these results. These commands build local test artifacts and may download locked dependencies; they do not install a shared binary, deploy, or run paid qualification.

For native tests, use the ADL source revision `e240a1b804968f8e96be7ea6e5dbfbc2ac51ea83` recorded in [post-merge verification](../issue-916/POSTMERGE_VERIFICATION.json), including the unmerged #916 test-readiness correction. From that checkout's root, use Cargo/Rust matching its checked-in toolchain (where declared), default features, and the following explicit integration targets. Keep build output local to this disposable checkout. Tests require permission to bind local sockets. This is the 154-test target set; it is not the earlier 396-test census.

```sh
git rev-parse HEAD
CARGO_TARGET_DIR="$PWD/adl/target" cargo test --locked --manifest-path adl/Cargo.toml \
  --test codefriend_agent --test codefriend_agent_publication \
  --test codefriend_agent_receipt --test codefriend_baseline_repeat \
  --test codefriend_evidence --test codefriend_integration \
  --test codefriend_journey --test codefriend_journey_v2 \
  --test codefriend_review_assessments --test codefriend_server \
  --test codefriend_update_cycle
```

For website tests, use `agent-logic/codefriend.ai` at `a45e339c13b24716edbd3fadf29dddff36ffe02e`, Node.js 24 or later and npm. From that checkout's root:

```sh
git rev-parse HEAD
npm ci --ignore-scripts
npm test
```

Confirm the printed source revisions before running. Expected retained totals are 154 native and 141 website tests, with zero failures and zero ignored/skipped tests. Compare actual nonzero denominators and results, not elapsed times or log hashes (logs include environment-dependent paths/timings). These recipes have been checked against the source manifests and recorded target lists; they were not rerun as part of this docs-only checkpoint. Raw historical logs and precise historical host/toolchain details require the authorized evidence custodian. A different toolchain or platform is a new observation, not an exact replay claim.

## Access and evidence custody

Repository access is required for both private repositories; the operator grants it through the normal account process. Do not put tokens in URLs, command lines, reports or this packet. Tracked `.csdlc/evidence/864` creation records are available in the ADL checkout. Protected Runtime archives and review receipts remain in Git-local C-SDLC retention; an authorized custodian must supply the exact digest-bound records for raw replay. #916 recovered the #852 archive from retained closeout storage without modifying its original bytes. Private manuscript prose/PDF and retained statistical raw data are not copied into this handoff; their sanitized identity records define the limits of what can be reviewed here. If access is missing, mark the affected obligation not proven—never infer success or publish private bytes to make links work.

## Finding rubric and remaining steps

P1: false release/candidate acceptance, privacy/provenance breach, or missing required qualification presented as passed. P2: meaningful command/path drift, unsupported feature claim, missing required evidence or incorrect deferral routing. P3: clarity that impedes review without changing acceptance truth. Distinguish actual failure, missing evidence, historical limitation and recommendation.

Close documented findings, obtain independent documentation review and a fresh-reader walkthrough, then refresh this packet against #916's accepted decision. Publication finalization, internal/external release review, remediation and ceremony remain #918–#925 work in their declared order. This packet sends no external message and authorizes no deployment, provider spending or release.

## Prior-review checks applied to this milestone

Read Claude's [first](../../repository-decomposition/CLAUDE_REVIEW_1.md) and [corrected-candidate](../../repository-decomposition/CLAUDE_REVIEW_2.md) reviews. F10 remains resolved: target v3 independence is separate from the known retained v2-to-resilience dependency, and extraction cannot strand rollback support. The reviews are retained planning evidence, not permission to extract repositories.

Earlier Claude/external-review lessons were applied here: include nested packets, reconcile accepted decisions against proposal history, distinguish source-time tracker/custody observations from current state, keep immutable evidence separate from live claims, and supply actual reproduction commands. This pass corrected the #671/#875 scope drift, SIM pilot gate, twelve-decision #945 acceptance, merged website #18 status, manuscript custody date, and old #1064 candidate status. Older milestones were inspected only as review context and were not edited.

## Package metadata coverage

The [Cargo review](CARGO_MANIFEST_REVIEW.md) and [27-manifest audit](CARGO_MANIFEST_AUDIT.json) cover the 27 Cargo manifests present at that historical audit, with 23 local dependency references checked. This handoff checkout has 29 tracked Cargo manifests: the later #916 and #917 validation-only crates are outside that historical audit. Main package versions remain 0.92.1 and independent components keep their declared versions. Release metadata selection belongs to #918; this handoff does not label existing binaries as a v0.92.2 release.

## Current qualification handoff

PR #1146 is merged as recorded in [the authenticated repair readback](../issue-916/QUALIFICATION_REPAIR_MERGE.json). The prior [Sprint 10 checkpoint](../issue-916/SPRINT10_CLOSEOUT_REFRESH.json) observed it OPEN and remains immutable historical evidence. The original qualification remains incomplete. Its original deferral named v0.93; the current successor milestone is v0.93.1 before Beta1 launch. Preserve the historical failed runs, incomplete export warnings, and unknown ADL second-run outcome with its retained reservation: no replay or changed request identity to bypass uncertainty. The approved disposition permits documentation preparation but does not establish #916 acceptance. Successors are [#1148](https://github.com/agent-logic/agent-design-language/issues/1148) for citation correctness, [#1149](https://github.com/agent-logic/agent-design-language/issues/1149) for interrupted-request recovery, and [#1150](https://github.com/agent-logic/agent-design-language/issues/1150) for independent qualification after both corrections. All three are now v0.93.1 obligations required before Beta1 launch. The original v0.93 deferral record remains historical evidence. Current G02 and G03 dispositions are described in [the gap analysis](../issue-916/GAP_ANALYSIS.md).

[Producer byte verification](../issue-916/PRODUCER_HASH_RECHECK.json) checks 84 references across 24 prerequisite rows. A reader can verify the retained hashes but must still distinguish historical producer results from final qualification. This refresh executes no product/provider tests and makes no claim about current deployed software. After the #916 owner records its accepted scope and decision, refresh the accepted source/binary/artifact identities, affected document hashes and fresh-reader review before final handoff.

## Native documentation proof adapter

The issue-local `validation/Cargo.toml` runs the existing handoff and planning Python validators unchanged for the native Cargo proof route. It requires their successful nonzero fixture counts and original 69-task/24-prerequisite contract; the document count is read from the current manifest. The earlier 27-manifest audit is historical and excludes the subsequently added #916 and #917 validation-only crates. This adapter changes no product or owner binary and grants no qualification or release acceptance.

## Draft predecessor checkpoint

This draft includes unmerged #916 ancestry for review. The [producer scope audit](../issue-916/PRODUCER_SCOPE_AUDIT.md) retains G03 evidence and contract gaps; neither this checkpoint nor its publication establishes passing release quality. #915 and Sprint 10/#936 are closed by explicit incomplete/deferred disposition, as recorded in the [deferral record](../issue-916/SPRINT10_DEFERRAL.json). Original qualification remains incomplete. No auto-merge or downstream final acceptance is authorized.

## September 23 predecessor and routing observation

The [#916 decision at `be804668a896cff4b770c38b8224c9d31ec7850c`](https://github.com/agent-logic/agent-design-language/blob/be804668a896cff4b770c38b8224c9d31ec7850c/docs/milestones/v0.92.2/evidence/issue-916/QUALITY_DECISION.json) routes #1148, #1149 and #1150 to **v0.93.1** following the #922 split. Authenticated issue readback confirms all three remain open in that milestone. This supersedes current routing prose above wherever it repeats the original v0.93 scope; it does not alter the retained deferral record.

PR #1151 was still open and draft at that revision; its quality decision remains `not_proven`, with no accepted release candidate or installed binary digests. This observation does not replace the inherited `e119223cd13ebbb6a07c0349ba6772f8ee2ecece` evidence checkpoint or qualify its results as current. Final handoff acceptance still requires the accepted #916 output and a refreshed inventory and independent review. Broader milestone overlays are being repaired separately under #1160; this handoff does not claim those unmerged repairs are present.

The internal review packet is published in [PR #1153](https://github.com/agent-logic/agent-design-language/pull/1153) at `6fc19987dcb9aaa347e61e6185f4ae1718e5ec6c`. Its nine lanes report 27 findings grouped into [#1159](https://github.com/agent-logic/agent-design-language/issues/1159), [#1160](https://github.com/agent-logic/agent-design-language/issues/1160), [#1161](https://github.com/agent-logic/agent-design-language/issues/1161), and [#1162](https://github.com/agent-logic/agent-design-language/issues/1162). Publication of that review does not establish remediation completion, merged fixes, final re-review or release acceptance.
