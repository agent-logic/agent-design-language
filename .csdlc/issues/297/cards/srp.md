# Structured Review Prompt

Template: 1.0.0

Issue: 297

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/src/projection_recovery.rs
csdlc-v2/src/projection_cleanup.rs
csdlc-v2/src/lib.rs
csdlc-v2/tests/gate5.rs
csdlc-v2/tests/archived_projection_cleanup.rs
.csdlc/issues/297
.csdlc/evidence/297/bridge-r3
.csdlc/evidence/297/noether-300-routing-bridge-gap.md

## Prompts

- Review every acceptance criterion with code, security, test, and evidence coverage, emphasizing crash consistency, immutable receipt ordering, inode ownership, symlink/hardlink and rename races, exact cleanup authority, topology/CAS enforcement, no evidence loss, and subsequent ordinary-commit behavior.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Reviewer inspection was limited to the assigned #297 bridge r3 scope; #300 remains responsible for bridge-fed integration matrix proof after #297 is terminal and ancestral.
- Reviewer independently ran the focused bridge test from csdlc-v2 and the full archived_projection_cleanup test; two initial command-shape attempts were non-proving and classified as invocation limits.

## Review Result

Revision: Some("git-blake3:3b8bc5a8afc777730edb0a29919a2292df427079:a683ff42ffde9edce06af5110963d049b7bf6bea55457835465abb30f86c1b40")

Reviewer: Some("fresh-session:0d7a9b8e-bc56-45ab-a650-45d0668a5dc8")

Result: pass
