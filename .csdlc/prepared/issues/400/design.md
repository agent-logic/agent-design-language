# Issue #400 design: implemented-phase card recovery truth repairs

## Problem

Implemented issues can receive exact review findings that require generated card
truth to be corrected before publication. The current typed `csdlc-edit`
surface blocks two #117-reproduced repairs:

- SPP plan-step truth cannot record completed or in-progress post-implementation
  status because replacement steps are constrained to pending-only values.
- STP dependencies cannot be repaired during the recovery epoch because
  `replace_planning_collection` only authorizes selected planning fields.

This leaves implemented work unable to truthfully pass review without raw
Markdown edits or lifecycle reset.

## Scope

Implement a narrow typed recovery route for implemented-phase card truth drift:

- allow bounded SPP plan-step status repair for post-implementation truth;
- allow bounded STP dependency repair when review evidence identifies omitted
  dependencies;
- preserve generation/digest CAS, append-only audit history, and recovery-epoch
  fail-closed behavior.

## Non-goals

- No raw generated-card editing.
- No review/publication guard weakening.
- No generic lifecycle reset.
- No #117 product implementation.

## Validation plan

Use focused `csdlc-v2` tests covering positive #117-style repairs and negative
phase/CAS/history cases, plus format and strict Clippy for the touched Rust
surface.
