# Structured Output Record

Template: 1.0.0

Issue: 661

Repository: agent-logic/agent-design-language

Card: sor

Status: complete

## Summary

Routed resident Shepherd conversation turns through the configured Shepherd operation adapter, projected generated provider responses into the existing conversation reply contract, and preserved explicit provider failure.

## Artifacts

- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/ingress.rs
- adl-runtime-kernel/src/shepherd.rs
- adl-runtime-kernel/src/conversation_sessions_tests.rs

## Execution

- Recognize resident Shepherd recipient ids from the canonical population and submit ShepherdRequest work to the Shepherd adapter.
- Project only structurally valid, correlation-bound ShepherdResponse payloads as public conversation replies.
- Carry the conversation recipient id inside the governed Shepherd request so public output remains bound to the addressed recipient.
- Replace the hardcoded Shepherd acknowledgement assertion with deterministic generated-output and provider-failure proof.

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/661/validate-focused.sh"
    ],
    "purpose": "Prove at exact substantive head c71d93f4c that fresh Shepherd turns return generated provider output, retain correlation, and surface provider failure without fallback",
    "outcome": "passed",
    "evidence_ref": "local:c71d93f4ca6afc13265df0678418149c6bbef42f:1-passed-0-failed"
  }
]

## Integration

merged

## Publication

Publication: closed

Merge: merged

## Closeout

complete

## Follow Ups

- none
