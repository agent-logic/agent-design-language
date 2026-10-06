# Issue 1145 local implementation and publication boundary

## Result

Everyone selection is implemented on `codex/1145-observatory-everyone-selection`.
Source candidate: `25af9600b26ca96d83c1e3fdbe82108cef8f73be`.
Local tests and independent source review pass. Native proof admission and PR publication remain blocked. No CI, merge, live Runtime, provider, installed acceptance, or deployment is claimed.

## Behavior

- Everyone selects explicit communication-eligible IDs from the selected endpoint's current local roster; Clear and named deselection buttons allow reductions.
- Complete paginated HTTP `AgentPopulationFeed` pages are checked for revision, cursor, scope, count, duplicates, and continuation consistency. Same-revision page requests omit the successor event cursor. Runtime's `population_complete` flag is not local paging completeness.
- Roster newcomers do not join a prepared selection. Missing or ineligible selected agents remain visible and block sending until explicitly removed or reselected. Runtime identity changes clear selection.
- Loading Everyone blocks old-selection sends. Manual changes, Clear, roster updates, or polis changes invalidate pending selection results.
- Nine or more eligible agents are shown without truncation; the existing eight-recipient limit blocks sending until reduced. Authorization and Runtime policy remain required.
- Named buttons, live selection counts, native keyboard multi-select, and a full-width mobile communication view support desktop and mobile use. Existing per-agent outcome handling remains.

## Local validation

All commands ran in the bound issue worktree and passed:

```sh
node --test demos/html-observatory/tests/governed_room_selection.test.mjs
node adl/tools/validate_v092_governed_room_observatory.mjs
node demos/html-observatory/tests/accessibility_responsive.test.mjs
node --test demos/html-observatory/tests/conversation_sessions.test.mjs demos/html-observatory/tests/polis_identity.test.mjs
node demos/html-observatory/tests/governed_room_selection.browser.mjs
git diff --check
```

The browser test requires Playwright resolution (this host used bundled dependencies via `NODE_PATH`) and installed Chrome. Chrome launch required sandbox escalation. Every HTTP request is intercepted and WebSocket is replaced by a fixture; no live provider or Runtime sends occur. Desktop 1280x900 and mobile 390x844 pass. Screenshots were inspected; an initial mobile navigation-width defect was repaired and guarded with element viewport bounds. The final browser run used the reviewed source candidate.

PVF: runtime lane, deterministic local UI contract proof, small CPU/memory resources, required issue acceptance proof; not release or installed qualification.

## Independent review

Reviewer `/root/review_1145` returned PASS with no remaining actionable findings for exact source candidate `25af9600b26ca96d83c1e3fdbe82108cef8f73be`; independently reran four selection tests. Browser results are implementer-run fixture evidence.

Resolved findings cover policy eligibility for configured agents, complete roster enumeration, the actual HTTP response shape, successor versus pagination cursor semantics, deselection/loading races, Send while loading, runtime incarnation resets, and local versus global population completeness. Producer contract traced through `adl-runtime-kernel/src/control.rs` (`agent_roster_page`, `agent_roster_handler`) and `control/feeds.rs` (`AgentPopulationFeed`).

## Native publication blocker

`csdlc proof 1145 --json` refused with `intent_validator_execution_unsupported`; exact output is retained in `NATIVE_PROOF_REFUSAL.json`. The declared validator is `manual-review observatory-everyone-selection`, backed by the concrete local tests and independent review above. Native `csdlc-v3/src/commands/proof/intent.rs` only executes Cargo validators; non-Cargo declarations are preparation-only. No unrelated Cargo test was substituted.

The native SOR implementation amendment succeeded and records `worktree_only` / publication blocked. Applying the final SRP review result through the native review-class amendment refused with `intent_amendment_policy_rejected`; output is retained in `NATIVE_REVIEW_CARD_REFUSAL.json`. The SRP was not hand-edited to bypass that gate. Independent review evidence is retained here pending native proof admission. Publication metadata now describes the implementation and includes `Closes #1145`.

Next owner action: provide an admitted native proof-completion route for this JavaScript/browser validation; rerun proof, finalize review cards, obtain current exact-head native review, then publish through the native owner. Fixing that lifecycle owner is a separate bounded tooling change. This worktree and session goal remain open; issue 1145 is not complete or merged.
