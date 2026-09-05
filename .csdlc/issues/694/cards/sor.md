# Structured Output Record

Template: 1.0.0

Issue: 694

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented #694 Runtime-authoritative conversation history restoration for the HTML Observatory. Runtime now serves complete conversation_history.v1 turns for operator outbound and agent reply halves from production conversation history, the Observatory requests history on fresh authenticated connection/recipient selection and deduplicates restored/live turns by stable role/base-turn identity, and focused proof covers replay bounds, history request schema, fresh-state reload, privacy/adversarial behavior, and production Runtime history ordering.

## Artifacts

- adl-runtime-kernel/src/control.rs
- demos/html-observatory/app.js
- adl/tools/validate_v092_observatory_transcript_history.mjs
- adl/tools/test_issue694_conversation_history_reload.sh
- .csdlc/issues/694
- .csdlc/prepared/issues/694

## Execution

- Added Runtime control conversation-history serialization for ordered operator outbound and agent reply turns using the existing production conversation history store.
- Added authenticated Observatory WSS conversation-history request handling and bounded page-size behavior without live Runtime mutation or new cloud/provider scope.
- Wired the HTML Observatory to request Runtime history on fresh state, recipient selection, and authenticated reconnect, then restore turns without depending on browser-local transcript state.
- Hardened transcript rendering deduplication so live turns and restored history variants share stable operator/agent base-turn render keys.
- Extended the #694 validator and shell proof to cover request schema, invalid request rejection, role/base-turn dedupe, focused Observatory contract/security tests, and the exact Runtime production-history regression.

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Verify Git patch whitespace hygiene for the #694 issue worktree.",
    "outcome": "passed",
    "evidence_ref": "issue694-diff-hygiene.log"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--",
      "--check"
    ],
    "purpose": "Verify Rust formatting for the touched Runtime kernel package.",
    "outcome": "passed",
    "evidence_ref": "issue694-fmt-check.log"
  },
  {
    "command": [
      "node",
      "demos/html-observatory/tests/security_privacy_adversarial.test.mjs"
    ],
    "purpose": "Run existing HTML Observatory security/privacy/adversarial tests after history restoration wiring.",
    "outcome": "passed",
    "evidence_ref": "issue694-observatory-security-privacy.log"
  },
  {
    "command": [
      "node",
      "demos/html-observatory/tests/conversation_sessions.test.mjs"
    ],
    "purpose": "Run existing HTML Observatory conversation-session contract tests after history restoration wiring.",
    "outcome": "passed",
    "evidence_ref": "issue694-observatory-session-contract.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_issue694_conversation_history_reload.sh"
    ],
    "purpose": "Run the issue-owned #694 proof script covering HTML Observatory history replay bounds plus exact Runtime production conversation-history ordering.",
    "outcome": "passed",
    "evidence_ref": "issue694-transcript-history-proof.log"
  }
]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
