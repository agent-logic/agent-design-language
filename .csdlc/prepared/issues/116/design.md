# #116 design: operator attention inbox and intervention workflow

## Issue identity

- Issue: #116
- Title: `[v0.92][WP-18C.06] Build the operator attention inbox and intervention workflow`
- Version: v0.92
- Parent: #110

## Dependency posture

#116 starts after terminal #111, #112, #114, #115, #265, #270, #271, #276, #277, and #278 truth. It consumes those surfaces; it does not reopen their authority, durable history, acknowledgement, transcript restore, or receipt semantics.

## Scope

#116 owns the operator attention inbox contract and its bounded intervention workflow:

1. Typed attention request lifecycle and policy-visible state.
2. Runtime-side queue semantics for source identity, reason, priority, expiry, correlation, grouping, deduplication, quiet mode, and bounded retention.
3. Observatory inbox/read state/filter/deep-link UI for request visibility and explicit outcomes.
4. Governed response routing for acknowledge, reply, defer, resolve, and refuse without implying approval unless a separate authority action exists.
5. Restart/reconnect and overload/spoofing/stale-request proof.

## Non-goals

- Do not implement generic system alerts or unrelated notification providers.
- Do not bypass Layer 8 authority or fabricate sender identity/urgency.
- Do not change #270 trusted acknowledgement API, #271 Observatory delivery-state integration, #276/#277/#278 durable journal/receipt foundations, or #114 parent coordination truth except as read-only dependencies.
- Do not implement #117 final qualification or #279/#280/#281/#282 proof bundles.

## Proposed implementation seams

The implementation should first locate the current runtime/Observatory conversation surfaces introduced by #114 and #271, then add the smallest cohesive seams:

- a runtime attention-request model/store/queue seam with deterministic identifiers and lifecycle transitions;
- an Observatory-facing projection/query/action surface for inbox views and explicit operator outcomes;
- focused tests covering spoof denial, rate limit/dedup/grouping behavior, expiry, restart/reconnect persistence, and response routing boundaries.

The exact product paths should be finalized after binding by inspecting current main. The validation packet intentionally names semantic lanes instead of precommitting to stale source paths.

## Validation plan

- `issue-116-preparation-validator`: prove the preparation packet stays scoped to #116 and observes terminal dependency caches.
- focused runtime tests for attention queue lifecycle, dedup/rate/expiry/restart behavior.
- focused Observatory tests for inbox visibility, filters, unread/read state, deep links, and explicit outcomes.
- strict formatting/clippy for touched Rust targets.
- fresh exact-head review before publication.

## Risks and controls

- Risk: inbox requests become an unbounded alert channel. Control: rate limits, grouping, deduplication, quiet modes, bounded retention, and validation proof.
- Risk: an agent fabricates another source or urgency. Control: bind source identity to existing Layer 8/runtime authority evidence; tests must deny spoofed identity and urgency escalation.
- Risk: operator response accidentally becomes authority approval. Control: response states are communication outcomes only; explicit authority actions remain separate.
- Risk: duplicate notifications after restart/reconnect. Control: stable request identifiers and durable lifecycle replay proof.
