# Structured Output Record

Template: 1.0.0

Issue: 277

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the #277 Runtime conversation continuity layer for durable watermarks, conversation-scoped monotonic attempt-local idempotency, replay decisions, ambiguous-dispatch outcomes, and receipts without absorbing #278, #114 parent, #115, or #270/#276 authority.

## Artifacts

- adl-runtime-kernel/src/conversation_continuity.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/tests/conversation_continuity.rs
- .csdlc/evidence/277
- .csdlc/prepared/issues/277
- .csdlc/issues/277
- adl-runtime-kernel/src/conversation_continuity.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/tests/conversation_continuity.rs
- .csdlc/evidence/277
- .csdlc/prepared/issues/277
- .csdlc/issues/277
- adl-runtime-kernel/src/conversation_continuity.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/tests/conversation_continuity.rs
- .csdlc/evidence/277
- .csdlc/prepared/issues/277
- .csdlc/issues/277

## Execution

- Added adl-runtime-kernel::conversation_continuity as a small continuity store layered on the #276 ConversationJournal foundation.
- Persisted sender watermarks and recipient acknowledgement watermarks as additive journal events while consuming, not redefining, #270 acknowledgement trust.
- Persisted attempt-local idempotency outcomes across restart, including duplicate completed suppression, duplicate ambiguous suppression, and retryable pre-dispatch outcomes.
- Persisted delivery, response, acknowledgement receipt references and replay decisions with owner/high-watermark evidence.
- Added focused Runtime kernel tests for restart reconstruction, idempotency, ambiguous dispatch, retryable pre-dispatch state, stale acknowledgement watermark refusal, replay ownership, receipt reconstruction, and deletion filtering.
- Added adl-runtime-kernel::conversation_continuity as a small continuity store layered on the #276 ConversationJournal foundation.
- Persisted sender watermarks and recipient acknowledgement watermarks as additive journal events while consuming, not redefining, #270 acknowledgement trust.
- Persisted attempt-local idempotency outcomes across restart with admission scoped by conversation_id plus idempotency_key, preventing one conversation from suppressing another conversation using the same key.
- Persisted duplicate completed suppression, duplicate ambiguous suppression, and retryable pre-dispatch outcomes across restart.
- Persisted delivery, response, acknowledgement receipt references and replay decisions with owner/high-watermark evidence.
- Added focused Runtime kernel tests for restart reconstruction, conversation-scoped idempotency, ambiguous dispatch, retryable pre-dispatch state, stale acknowledgement watermark refusal, replay ownership, receipt reconstruction, and deletion filtering.
- Added adl-runtime-kernel::conversation_continuity as a small continuity store layered on the #276 ConversationJournal foundation.
- Persisted sender watermarks and recipient acknowledgement watermarks as additive journal events while consuming, not redefining, #270 acknowledgement trust.
- Persisted attempt-local idempotency outcomes across restart with admission scoped by conversation_id plus idempotency_key, preventing one conversation from suppressing another conversation using the same key.
- Made attempt snapshot replay monotonic so stale PreDispatchRetryable records cannot downgrade prior Completed or DispatchedAmbiguous outcomes for the same conversation-scoped idempotency key.
- Persisted duplicate completed suppression, duplicate ambiguous suppression, and retryable pre-dispatch outcomes across restart.
- Persisted delivery, response, acknowledgement receipt references and replay decisions with owner/high-watermark evidence.
- Added focused Runtime kernel tests for restart reconstruction, conversation-scoped idempotency, monotonic attempt replay, ambiguous dispatch, retryable pre-dispatch state, stale acknowledgement watermark refusal, replay ownership, receipt reconstruction, and deletion filtering.

## Validation

[
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/277/validate_preparation_bundle.py"
    ],
    "purpose": "Run the issue-owned lifecycle/scope/dependency validator after bind and implementation.",
    "outcome": "passed",
    "evidence_ref": "issue-277-preparation-validator.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "conversation_continuity",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict Clippy for the focused #277 test target.",
    "outcome": "passed",
    "evidence_ref": "runtime-kernel-conversation-continuity-clippy.log"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--check"
    ],
    "purpose": "Run rustfmt check for adl-runtime-kernel.",
    "outcome": "passed",
    "evidence_ref": "runtime-kernel-conversation-continuity-fmt.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "conversation_continuity"
    ],
    "purpose": "Run the focused #277 integration test target.",
    "outcome": "passed",
    "evidence_ref": "runtime-kernel-conversation-continuity-tests.log"
  },
  {
    "command": [
      "python3 .csdlc/prepared/issues/277/validate_preparation_bundle.py",
      "cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --check",
      "cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test conversation_continuity",
      "cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --test conversation_continuity -- -D warnings",
      "git diff --check"
    ],
    "purpose": "Focused #277 post-review proof for bounded dependency/scope validation, formatting, 8-test conversation continuity behavior including scoped idempotency, strict Clippy, and diff whitespace hygiene.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/277/issue-277-preparation-validator.log; .csdlc/evidence/277/runtime-kernel-conversation-continuity-fmt.log; .csdlc/evidence/277/runtime-kernel-conversation-continuity-tests.log; .csdlc/evidence/277/runtime-kernel-conversation-continuity-clippy.log; git diff --check PASS"
  },
  {
    "command": [
      "python3 .csdlc/prepared/issues/277/validate_preparation_bundle.py",
      "cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --check",
      "cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test conversation_continuity",
      "cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --test conversation_continuity -- -D warnings",
      "git diff --check"
    ],
    "purpose": "Post-base-refresh #277 proof after merging current origin/main with no touched-path collision; reprove dependency/scope validation, formatting, 8-test conversation continuity behavior, strict Clippy, and diff whitespace hygiene.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/277/issue-277-preparation-validator.log; .csdlc/evidence/277/runtime-kernel-conversation-continuity-fmt.log; .csdlc/evidence/277/runtime-kernel-conversation-continuity-tests.log; .csdlc/evidence/277/runtime-kernel-conversation-continuity-clippy.log; git diff --check PASS"
  },
  {
    "command": [
      "python3 .csdlc/prepared/issues/277/validate_preparation_bundle.py",
      "cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --check",
      "cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test conversation_continuity",
      "cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --test conversation_continuity -- -D warnings",
      "git diff --check"
    ],
    "purpose": "Focused #277 proof after monotonic attempt replay remediation: dependency/scope validation, formatting, 10-test conversation continuity behavior including scoped idempotency and downgrade refusal, strict Clippy, and diff whitespace hygiene.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/277/issue-277-preparation-validator.log; .csdlc/evidence/277/runtime-kernel-conversation-continuity-fmt.log; .csdlc/evidence/277/runtime-kernel-conversation-continuity-tests.log; .csdlc/evidence/277/runtime-kernel-conversation-continuity-clippy.log; git diff --check PASS"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
