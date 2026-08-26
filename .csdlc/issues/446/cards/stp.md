# Structured Task Prompt

Template: 1.0.0

Issue: 446

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Wire existing UTS, ACC, Freedom Gate, and governed-executor components into actual long-lived resident cycles.

## Deliverables

- Typed authority contract
- Runtime proposal service
- Injected dispatcher
- Terminal receipts
- Focused tests

## Acceptance

1. Authorized proposal executes through a governed read-only adapter with receipt
2. Authority mismatch and unauthorized tools deny without actuation
3. Malformed or multiple proposals deny
4. Compiler, gate, and adapter failures emit denials
5. Receipts bind resident, proposal digest, ACC, gate, outcome, cycle, and lineage
6. Fixture dispatch remains test-only
7. Integration proof originates in a Runtime provider response

## Dependencies

- UTS-to-ACC compiler
- Freedom Gate
- Governed executor
- #5347 retirement boundary

## Inputs

- adl/src/lib.rs
- adl/src/long_lived_agent.rs
- adl/src/long_lived_agent/types.rs
- adl/src/uts_acc_compiler.rs
- adl/src/uts_acc_compiler/core.rs
- adl/src/governed_executor_parts/logic.rs
- adl-runtime/src/resident_agent.rs
- .csdlc/prepared/issues/446/validate_issue446.sh

## Non Goals

- AWS qualification
- Issue #269
- Arbitrary process/network/filesystem authority
- Restoring demo binaries
- Broad provider rewrite
