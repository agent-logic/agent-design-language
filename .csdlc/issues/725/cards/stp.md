# Structured Task Prompt

Template: 1.0.0

Issue: 725

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Remove v2 authority and build dependencies from C-SDLC v3 only.

## Deliverables

- native authority implementation
- updated focused tests
- no-v2 canary evidence
- migration documentation

## Acceptance

1. AC-1: Native v3 authority selector and receipt require no v2 source.
2. AC-2: Operational selector reads are v3-native.
3. AC-3: Fail-closed exact-head digest and remote reconciliation remain.
4. AC-4: Proof install and shadow tests prove native authority.
5. AC-5: Fresh no-v2 worktree compile and tests pass.
6. AC-6: Migration path is documented.

## Dependencies

- V3-F cutover is complete
- PR 726 failure is evidence only and not a branch dependency

## Inputs

- csdlc-v3/Cargo.toml
- csdlc-v3/src/commands/remote/mod.rs
- csdlc-v3/src/commands/proof.rs
- csdlc-v3 tests
- PR 726 standalone failure

## Non Goals

- unrelated gh-style UI work in issue 724
- merge or closeout
- caller-controlled authority switching
