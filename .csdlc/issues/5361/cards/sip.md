# Structured Intent Prompt

Template: 1.0.0

Issue: 5361

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Accept Runtime v3 at an exact revision after all parity, consumer, operational, rollback, and workcell proof dependencies converge.

## Required Outcome

Runtime v3 is installed through its stable owner path, operationally proven, rollback-capable, reviewed, and consumable by ADL v2 and v0.92 without relying on Runtime v2 implementation paths.

## Scope

- Runtime v3 integrated acceptance and retained proof synthesis
- Parity-A through Parity-D evidence consumption
- ADL v2 adapter and provider/tool consumer proof
- guardian, secure access, Observatory, rollback, and operations proof
- live multi-agent workcell output-contract consumption

## Authority

- #5361 owns Runtime v3 acceptance synthesis
- #5591, #5592, #5589, and #5590 own parity implementation
- #5501 owns live workcell proof
- #5384 consumes completed #5361 evidence
- typed C-SDLC v2 records and exact-revision review govern acceptance

## Assumptions

- none

## Operator Constraints

- Use only the typed C-SDLC v2 lifecycle and installed owner binaries on the bound #5361 branch; never edit tracked issue work on main
- Keep this preparation revision limited to #5361 cards, dependency design, diagram, validation contract, and typed request records; do not perform Runtime product implementation or acceptance execution in this lane
- When execution begins after the declared dependencies integrate, satisfy every AC-1 through AC-7 obligation with exact-revision proof; do not defer, weaken, omit, or replace required parity, consumer, operational, rollback, review, or quality work with fixture-only evidence
- Do not depend on Runtime v2 implementation paths, hard-code network addresses, or permit non-HTTPS access
- Do not use raw gh or AWS; unsupported GPU, remote-provider, and deployment claims remain explicit out-of-scope non-claims unless later reviewed evidence brings them into scope
