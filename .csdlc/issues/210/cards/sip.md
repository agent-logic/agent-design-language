# Structured Intent Prompt

Template: 1.0.0

Issue: 210

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Move one real #208 continuity bundle between the exact authorized source and target voters with bounded resumable authenticated streaming.

## Required Outcome

A closed typed service on the existing authenticated polis connection streams and incrementally verifies exact bundle bytes into the target's isolated #208 stage, survives partition/restart/reply loss, and aborts with live zero-residue proof without generic transport or whole-bundle allocation.

## Scope

- Opaque #201 transfer authority bound to exact source, target, route cut, lineage, bundle, deadline, and limits
- Closed typed continuity-transfer service on #191 authenticated current-voter sessions
- Bounded canonical frames, backpressure, accepted-prefix durability, resume, replay, and conflict handling
- Incremental SnapshotCatalog and whole-content verification without whole-bundle allocation
- Opaque #208 source-bundle reader and target isolated-stage writer integration
- Exact abort, cleanup, zero-residue, crash, restart, rollback, path, and evidence proof

## Authority

- Only a private finalized #201 transfer payload plus the current #191 route and #203 certificate cut creates a transfer session
- Transfer capability has no generic send API and cannot dispatch Raft, unknown, or public messages
- Source and target filesystem access occurs only through opaque #208 handles; no caller path or synthetic bundle is accepted
- Transfer completion creates possession evidence only and never creates fencing, activation, OwnerCommit, serving, or cloud authority
- Normal builds expose no injectable mock transport, verifier, bundle reader, or stage writer

## Assumptions

- none

## Operator Constraints

- Do not bind or edit product source until #191, #201, #203, and #208 are externally reviewed, merged, and ancestral
- Keep #210 limited to bundle data transport and verification; #204 owns migration/recovery policy
- Resolve every review finding through a subagent and obtain fresh exact-head review before publication
- Open a ready PR for visibility but never merge before operator review and authorization
- No AWS use and no lifecycle closeout
