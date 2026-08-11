# WP-18C.06 Operator Attention Inbox Design

## Boundary

Issue #116 adds a governed path for agents and Runtime governance to request Layer 8 attention. Runtime remains authoritative for identity, authorization, lifecycle, ordering, retention, and receipts; the Observatory is a projection and interaction client. This preparation does not implement code, mutate #83, bind execution, or authorize publication.

## Contract

`AttentionRequestV1` carries `request_id`, authenticated `source_principal_id`, `source_agent_id`, `polis_id`, `reason_class`, source-proposed priority, policy-derived effective priority, `conversation_id`, `correlation_id`, optional governed work reference, creation time, expiry time, lifecycle state, deduplication key, and schema version. Free-form display text is bounded and public-safe; credentials, private cognition, raw policy inputs, and fabricated authority claims are rejected.

Runtime verifies the signed source envelope through #112 authority, resolves source identity instead of trusting payload identity, clamps or rejects priority through policy, and records the decision. An agent cannot claim another identity, an operator-only reason class, emergency priority, or approval authority. Operator responses are authorized anew and return through #111 canonical conversation ingress. `reply`, `acknowledge`, `defer`, `resolve`, and `refuse` are attention outcomes only; none grants approval or governance authority unless a separate explicit #112 authority action succeeds.

Lifecycle: `submitted -> queued -> acknowledged|deferred|resolved|refused|expired|cancelled`. A deferred request returns to queued only at its governed wake time and before expiry. Terminal requests are immutable apart from retention/redaction processing. Every transition carries actor, prior state, next state, correlation, decision reason, monotonic sequence, and receipt digest.

## Queue, Deduplication, Expiry, And Limits

- Queue order is effective priority, then earliest expiry, then durable enqueue sequence. Client timestamps never order authority.
- Deduplication identity is policy-versioned `(polis_id, source_principal_id, reason_class, correlation_id, governed_work_ref)`; repeated active submissions update bounded occurrence metadata and do not emit another notification.
- Expiry is enforced by Runtime time authority on enqueue, read, transition, restart, and reconnect. Expired requests cannot be acknowledged or replied to.
- Retention is bounded separately for active and terminal records; pruning preserves required redacted audit receipts under #112/#114 policy.
- Per-principal token buckets, per-Polis aggregate limits, maximum active depth, payload-size limits, and reason-class quotas fail closed. High urgency does not bypass quotas; a separately authorized policy escalation may reserve capacity.
- Quiet mode suppresses client notification delivery, not durable queueing or expiry. Grouping changes presentation only and never merges distinct authorization or correlation identities.
- Overload admits only policy-reserved classes, durably accounts rejection/grouping, and never silently drops accepted actionable requests.

## Authorization And Projection

Creation requires #112 contact capability for the authenticated source and target operator scope. Listing and transition endpoints re-authorize every request and response. The Observatory receives only public-safe source label, reason class, effective priority, age/expiry, lifecycle state, unread projection, and authorized deep links. Browser cache is non-authoritative and cleared or hidden on revocation.

Unread state is a per-authorized-principal projection over durable transition sequence, not a mutable browser counter. Notification preferences may narrow delivery channels but cannot suppress audit, alter queue priority, or expand visibility. Deep links use opaque stable identifiers and re-authorize at resolution time.

## Exact Future Write Ownership

Issue #116 owns these new implementation surfaces:

- `adl-runtime/src/operator_attention.rs`: typed lifecycle, queue policy, deduplication, expiry, rate limits, durable recovery adapter, and unit tests.
- `adl-runtime/schemas/operator-attention/v1/`: request, transition, projection, and receipt schemas plus catalog entry.
- `adl-runtime/tests/operator_attention.rs`: Runtime/API integration, overload, spoof, restart, reconnect, and recovery proof.
- `demos/html-observatory/operator-attention.js`: inbox state, filters, deep links, transitions, reconnect, and notification preferences.
- `demos/html-observatory/index.html` and `styles.css`: the bounded inbox view and accessible interaction controls.
- `adl/tools/test_v092_operator_attention_inbox.sh`: deterministic browser/contract proof runner.
- `docs/milestones/v0.92/features/OPERATOR_ATTENTION_INBOX_v0.92.md`: implemented contract and evidence boundary.

Narrow integration edits are limited to `adl-runtime/src/lib.rs`, `adl-runtime/src/runtime_api.rs`, `demos/html-observatory/app.js`, and `demos/html-observatory/runtime-v3.config.json`. Any need to edit a predecessor-owned internal surface triggers replan.

## Pairwise Overlap And Ownership

- #111 owns canonical conversations, turns, delivery outcomes, and governed reply ingress. #116 stores only conversation/correlation references and calls exported #111 interfaces; it does not define a second message protocol.
- #112 owns Layer 8 principals, capabilities, refusal taxonomy, revocation, and audit authority. #116 supplies attention-specific policy inputs and projections; it does not authenticate identities or interpret browser authority.
- #114 owns durable conversation history, receipts, retention primitives, migration, and recovery. #116 owns attention records and composes exported durability primitives; it does not alter conversation-history semantics.
- #110 owns umbrella reconciliation and execution order. #116 closes only its own implementation issue after later publication and merge authority.
- #83 owns the first HTML roster/chat vertical slice. #116 may consume its stable Observatory integration after it lands, but this preparation and future implementation must not mutate #83 state or absorb its ownership.

## Serial Gates

Execution binding is prohibited until #111, #112, and #114 are terminal, merged changes are ancestral to the chosen base, and their exported contracts are inspected. If names or paths differ from this preparation, update SPP/VPP through typed editors before implementation. #117 remains downstream and does not gate #116 preparation.

## Negative Proof Matrix

- Overload: per-source burst, distributed multi-source flood, high-priority abuse, full active queue, quiet mode, and notification fanout remain bounded and accounted.
- Spoofing: substituted source, forged urgency, operator-only reason class, cross-Polis request, replayed envelope, revoked principal, and stale capability fail closed.
- Lifecycle: duplicate active request groups once; terminal replay is idempotent; expired/deferred requests cannot transition illegally; reply never implies approval.
- Restart/reconnect: accepted requests and unread cursors recover without loss, reordering, duplicate rows, or duplicate notifications.
- Recovery: partial write, corrupt record, stale sequence, policy-version drift, expired-on-restart, and retention pruning quarantine or fail safely with durable evidence.
- Projection: revoked reads, stale browser cache, unauthorized deep links, private text, secret-shaped fields, and cross-principal unread state do not leak.

## PVF And Handoff

Execution must use focused deterministic Runtime unit/schema lanes, Runtime API and durable-restart integration lanes, adversarial overload/authorization lanes, and browser accessibility/responsive/reconnect proof. Live provider, AWS, push-vendor, and public deployment work are deferred non-goals. Handoff is to typed `csdlc-bind` only after serial gates are proven; then create an issue-bound goal before implementation.
