# v0.92 WP-18C Umbrella #110 Session Handoff

## Canonical Work

- Umbrella: [#110](https://github.com/agent-logic/agent-design-language/issues/110), `[v0.92][WP-18C][umbrella] Establish Observatory as the living Polis interface` (open).
- Active child in this worktree: [#112](https://github.com/agent-logic/agent-design-language/issues/112), WP-18C.02, `Govern Layer 8 identity, authority, refusal, and audit` (open, typed phase `bound`).
- Completed upstream children: #111 is closed and merged through PR #228 at merge commit `5dab282aa6b730efd057f0502dacd462d30cc1d0`; #113 is closed.
- Remaining downstream children: #114, #115, #116, and #117 are open. #122 is explicitly deferred beyond this v0.92 execution wave and is not a critical-path gate.

## Bound Context

- Worktree: `/Volumes/FastWork/adl-worktrees/adl-issue-112-layer8-authority-preparation-v2`
- Branch: `codex/112-layer8-authority-preparation-v2`
- Current HEAD: `804b84d9dfcd91096bb95e39f42b4cfd2d312050` (`chore(csdlc): bind issue 112 execution`)
- Remote relation observed during handoff: ahead of `origin/main` by 15 commits and behind by 1 commit.
- PR for this branch: none.
- #112 merge/closeout: not merged, not closed, and not published.

## Completed Work

The current uncommitted candidate:

- adds a Runtime-kernel-owned Layer 8 authority model with action-specific capabilities, policy intersection, bounded refusals, replay resistance, and a redacted hash-chained audit store;
- wires authority checks into the merged #111 production conversation boundary before reservation/provider dispatch;
- adds the narrow Runtime API authorization wrapper and compatibility exports;
- adds focused authority, Runtime API, production conversation, and Observatory presentation tests/contracts;
- updates the issue #112 typed planning surfaces from preparation lanes to the minimum post-#111 product lanes; and
- adds the feature contract `docs/milestones/v0.92/features/LAYER8_CONVERSATION_AUTHORITY.md`.

No implementation commit exists after the bind commit. Treat every product change below as preserved but unreviewed work in progress.

## Validation Actually Run

- Focused production conversation-boundary test: passed, 1 selected test / 1 passed.
- Focused Runtime API integration target `layer8_authority_runtime_api`: passed, 2 selected tests / 2 passed.
- An attempted exact `--locked` API run exposed the repository's pre-existing stale `adl/Cargo.lock`; the lockfile was restored and is not part of this candidate.

Not yet proven in this session:

- the complete `layer8_authority` contract target;
- the real-browser Observatory authority-state target;
- the complete declared required lane set at one exact revision;
- independent exact-head review;
- PR checks, publication, merge, or terminal closeout.

## Unresolved Work And Gates

- Review the full uncommitted implementation for correctness and fit with merged #111 before committing.
- Complete and run the four required issue-owned VPP lanes: production conversation boundary, authority contract, Runtime API integration, and real-browser Observatory UI. Run no optional, soak, cloud, or broad jobs.
- Obtain independent exact-head review and resolve every actionable finding before publication.
- Publish only after current review truth exists, then require only the minimum declared green PR checks before merge.
- #114 is downstream of #112. #115 and #116 follow their live declared dependencies. #117 is the final integrated child after #111-#116 are terminal.
- Do not treat #83, #122, tooling issue #213, or any deferred work as a #112 execution gate.

## Dirty And Uncommitted Paths

Tracked modifications:

- `.csdlc/issues/112/audit.jsonl`
- `.csdlc/issues/112/cards/sip.values.json`
- `.csdlc/issues/112/cards/sor.values.json`
- `.csdlc/issues/112/cards/spp.md`
- `.csdlc/issues/112/cards/spp.values.json`
- `.csdlc/issues/112/cards/srp.values.json`
- `.csdlc/issues/112/cards/stp.values.json`
- `.csdlc/issues/112/cards/vpp.md`
- `.csdlc/issues/112/cards/vpp.values.json`
- `.csdlc/issues/112/index.json`
- `adl-runtime-kernel/src/bin/adl-runtime-kernel.rs`
- `adl-runtime-kernel/src/control.rs`
- `adl-runtime-kernel/src/lib.rs`
- `adl-runtime-kernel/tests/conversation_sessions.rs`
- `adl-runtime/src/lib.rs`
- `adl/src/csm_runtime_api.rs`

Untracked paths:

- `.csdlc/locks/112.lock`
- `.csdlc/prepared/issues/112/replace-affected-areas-post-gate-request.json`
- `.csdlc/prepared/issues/112/replace-validation-lanes-execution-request.json`
- `adl-runtime-kernel/src/layer8_authority.rs`
- `adl-runtime/src/layer8_authority.rs`
- `adl-runtime/tests/layer8_authority.rs`
- `adl/tests/layer8_authority_runtime_api.rs`
- `adl/tools/validate_layer8_authority_observatory_ui.sh`
- `docs/milestones/v0.92/features/LAYER8_CONVERSATION_AUTHORITY.md`
- `.adl/docs/TBD/V092_WP18C_UMBRELLA_110_SESSION_HANDOFF.md` (this handoff)

## Exact Next Action

Resume in this exact worktree without resetting or rebasing. First inspect the complete uncommitted diff against `804b84d9d` and reconcile any remaining implementation/test mismatch with the merged #111 contract. Then run only the four required #112 VPP lanes, fix failures, commit the coherent candidate, and commission an independent review of that exact clean head. Do not open a PR before that review passes.

## Non-Goals And Ownership Boundaries

- Do not mutate umbrella #110 or sibling child lifecycle state from this child worktree.
- Do not work on #83, #113, #114-#117, #122, or #213 here.
- Do not restart unrelated services, run optional CI, launch cloud resources, or perform soak/broad test jobs.
- Do not merge or close #112 without reviewed exact-head evidence and required-only green PR checks.
- Runtime remains the sole authority; browser state, caller claims, provider output, and agent self-report must not grant or widen authority.
- Preserve all current dirty work. Do not discard, reset, or overwrite it during recovery.
