# Issue #112 Layer 8 Conversation Authority Design

## Purpose

Issue #112 adds a Runtime-owned Layer 8 authorization boundary for governed
human-agent conversation actions. The Runtime must authenticate the caller,
intersect action-specific capability with current agent and Polis policy, reject
replay and scope widening, durably record a redacted decision, and only then
allow sequence reservation, provider execution, or delivery.

This is a preparation design. It grants no product implementation, execution
binding, publication, merge, or GitHub authority.

## Serial Gates

Execution has exactly one serial gate:

- #111 must be closed by a merged PR and ancestral to the execution base.

Preparation may reach typed pre-bind readiness while #111 is open. Binding,
product edits, and product validation remain unavailable until #111 is
terminal, merged, ancestral, and ownership-compatible. Closed #83 is preserved
source material and explicitly is not a dependency.

## Runtime Authority Contract

The authority module owns typed contracts for:

- a stable Layer 8 principal derived only from authenticated, unexpired,
  non-revoked Runtime evidence bound to one Polis and credential generation;
- separate least-privilege capabilities for discovery, new contact,
  continuation, attachment, and exact multi-recipient addressing;
- intersection with current agent and Polis policy, where no input may widen
  another;
- exact conversation, action, attachment, recipient-set, replay, and policy
  bindings;
- bounded public refusal reasons that reveal no private policy, credentials,
  content, provider payload, secrets, or private cognition;
- canonical redacted, hash-chained audit records with restart integrity.

Unknown identity, expiry, rotation, revocation, stale capability epoch,
recipient substitution, recipient-set widening, implicit broadcast,
cross-Polis use, policy unavailability, replay ambiguity, audit corruption, and
audit write failure all fail closed.

## Runtime API Integration

After #111 passes, `adl/src/csm_runtime_api.rs` may invoke the Layer 8
authority decision before sequence reservation, provider execution, and
delivery. The integration consumes authenticated Runtime identity and the
canonical conversation identifiers provided by the merged gated contracts. It
does not transfer session, ingress, UI, provider, or delivery ownership into
issue #112.

Role projections remain bounded:

- operator: own request identity plus bounded decision, refusal, retry,
  correlation, recipient, conversation, and outcome fields;
- recipient agent: authorized sender, action, conversation, correlation, and
  bounded delivery metadata only;
- reviewer: redacted authority inputs, digests, chain linkage, decision, and
  outcome evidence;
- public: no principal, conversation, recipient, policy, capability, or audit
  detail unless separately approved as aggregate availability data.

## Issue-Owned Product Surface

After #111 passes, issue #112 owns these exact product targets:

- `adl-runtime/src/layer8_authority.rs`
- `adl-runtime/src/lib.rs` for the module export only
- `adl-runtime/tests/layer8_authority.rs`
- `adl/src/csm_runtime_api.rs` for the narrow pre-delivery invocation only
- `adl/tests/layer8_authority_runtime_api.rs`
- `adl/tools/validate_layer8_authority_observatory_ui.sh`
- `docs/milestones/v0.92/features/LAYER8_CONVERSATION_AUTHORITY.md`

The focused product validation targets are exact and deferred:

- `cargo nextest run --locked --manifest-path adl-runtime/Cargo.toml --test layer8_authority --no-tests=fail --status-level all`
- `cargo nextest run --locked --manifest-path adl/Cargo.toml --test layer8_authority_runtime_api --no-tests=fail --status-level all`
- `bash adl/tools/validate_layer8_authority_observatory_ui.sh`

The Rust targets must select and pass nonzero tests at the implementation
revision, and the browser target must prove authorized, refused, stale or
revoked, and disclosure-safe Observatory states. No preparation-text check may
stand in for any product target.

## Preparation Boundary

This fresh packet owns only `.csdlc/issues/112` and
`.csdlc/prepared/issues/112`. The earlier worktree at
`/Volumes/FastWork/adl-worktrees/adl-issue-112-layer8-authority-preparation`
is preserved unchanged as non-authoritative historical evidence. This packet
does not import its lifecycle state, generation, approval, or readiness.

## Stop Conditions

Stop before binding or product edits when #111 is not terminal, merged, and
ancestral; when its merged contract changes the declared ownership or
API boundary; when authorization cannot precede sequence reservation and
provider execution; when redaction requires retaining forbidden content; or
when any requested action would mutate another issue or widen issue #112.

## Rollback

Product rollback removes only issue #112 authority integration and leaves
governed conversation actions unavailable. It must not restore an unguarded
delivery path, bypass audit, treat browser state as authority, or discard
retained audit evidence.
