# v0.92 Feature: Memory Grounding, Capability Envelope, and Witnesses

## Metadata

- Feature Name: Memory Grounding, Capability Envelope, and Witnesses
- Milestone Target: `v0.92`
- Status: implementation in progress; WP-12 capability-envelope slice implemented by `#5829`
- Related issues: `#3377`, `#3434`, `#5825`, `#5826`, `#5829`
- Planning template set: `docs/templates/planning/1.0.0`

## Template Rules

This feature doc remains the bounded contract for memory, capability,
witnesses, and receipts. The WP-12 capability-envelope slice is implemented by
`#5829`; witness and receipt slices remain separate downstream work and are not
claimed here.

## Status

Mixed implementation feature contract for `v0.92`: the capability-envelope
runtime contract and fixtures are implemented, while later witness and receipt
work remains planned.

Related readiness issue: `#3377`.

## Purpose

Bind first-birthday identity claims to witnessed memory references, capability
envelopes, and citizen-facing evidence rather than to vocabulary alone.

## Context

The first birthday needs grounding in memory and capability, but reviewers
should not need raw private-state access to verify it.

## Coverage / Ownership

v0.92 owns redaction-safe memory references, capability envelope shape,
witness set, and receipt surface for the birthday packet.

## Overview

The feature binds identity claims to witnessed artifacts, bounded capabilities,
and reviewable receipts.

## Design

The design should use references and projections for memory grounding,
capability envelopes for provider/model/tool/skill/authority limits, and
witness/receipt records for review.

## Execution Flow

1. Resolve allowed memory-grounding references.
2. Build capability envelope.
3. Attach witness set and receipt.
4. Include all surfaces in the birthday packet.

## Determinism and Constraints

Memory grounding must not expose raw private state. Capability envelopes must
name limits and authority context rather than imply unlimited capacity.

## Integration Points

- Identity and continuity records.
- ObsMem/trace baseline.
- Governed tool evidence where applicable.
- Birthday review packet.

## Implemented Capability Envelope Input

Issue `#4761` supplies the pre-`v0.92` capability envelope input at
`.csdlc/evidence/4761/capability-envelope/envelope.v1.json`, with exact source
inventory in `.csdlc/evidence/4761/capability-envelope/inputs.v1.json`,
fail-closed validation in
`.csdlc/evidence/4761/capability-envelope/validation.v1.log`, and unsupported
claims in `.csdlc/evidence/4761/capability-envelope/non-claims.v1.md`.

Birthday packets may consume that envelope as the provider, model, tool, skill,
authority, and limit context surface. They must not treat the envelope as
birthday execution, Memory Palace completion, credentialed remote-provider
deployment, production citizenship, governance completion, or raw private-state
authority.

## WP-12 Runtime Capability Envelope

Issue `#5829` adds the versioned runtime contract at
`adl-runtime-kernel/src/capability_envelope.rs`. The contract consumes two
separately checked Birthday authorities:

- an accepted WP-08 `BirthdayCandidate` whose canonical digest is current; and
- a WP-09 `BirthdayIdentityRecord` whose schema and canonical record digest are
  current and whose stable name, identity root, and continuity head agree with
  the Birthday candidate.

The untrusted envelope input is evaluated against a separately provisioned
policy. The policy pins exact evidence path, content digest, source revision,
and issue provenance; allowed provider/model pairs, tools, skills, and grants;
required denials and unsupported claims; and explicit resource ceilings for
prompt/output tokens, tool calls, skill invocations, timeout, and recurrence.
The envelope describes capability context only. It neither grants authority nor
proves provider, model, tool, or skill invocation.

Canonical sorting and exact deduplication make equivalent inputs byte-stable.
Case-folded identifier collisions, unknown evidence, stale revisions,
undeclared provider/model/tool/skill selections, authority escalation,
grant/denial conflicts, omitted or zero limits, limit escalation, missing
provenance, and omitted non-claims fail closed. Every serialized contract uses
`deny_unknown_fields`. Paths must remain normalized and repository-relative;
secret-like material and private, home, host, or traversal paths are rejected.
Rejected untrusted identifiers are represented only by stable SHA-256
fingerprints in serializable and debug diagnostics; raw evidence, provider,
model, tool, skill, grant, denial, and unsupported-claim values are never
echoed. Path checks are lexical and platform-independent, including Windows
drive/UNC forms and leading private, home, users, or user namespaces.

`validate_capability_envelope` is the exported consumption boundary. It checks
the packet digest and reconstructs the complete expected envelope from the
original Birthday authorities plus the provisioned policy, so a caller cannot
make a forged packet acceptable merely by recomputing its digest.

## Validation

Validation includes required memory-reference fields, redaction checks,
capability-envelope checks, witness/receipt fixtures, and private-state denial
cases. The focused `capability_envelope` integration target covers deterministic
positive construction and comprehensive negative evidence, authorization,
limit, provenance, parsing, privacy, portability, and packet-forgery cases.

## Source Inputs

- `docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md`
- `docs/milestones/v0.92/README.md`
- `docs/milestones/v0.92/WBS_v0.92.md`
- `docs/planning/ROADMAP_RUNTIME_V2_AND_BIRTHDAY_BOUNDARY.md`
- `docs/planning/ADL_FEATURE_LIST.md`
- `#3377`

## Scope

This feature should establish:

- memory grounding tied to witnessed artifacts
- capability envelopes covering provider, model, tool, skill, authority, and
  limit context at birth
- birth witnesses and citizen-facing receipt surfaces
- redaction-safe review posture for grounded memory and witnessed capability
  claims
- clear separation between witnessed capability context and later reputation or
  governance judgment

## Acceptance Criteria

- Memory-grounding references are reviewable and redaction-safe.
- Capability envelope records provider, model, tool, skill, authority, denial,
  limit, provenance, and unsupported-claim context.
- Witness and receipt surfaces exist.
- Birthday packet can cite these surfaces without exposing raw private state.

## Risks

- Reviewers may ask for raw memory. Mitigation: provide witnessed references
  and redacted projections.
- Capability envelopes may overclaim. Mitigation: require explicit limits.

## Future Work

Later milestones can expand memory palace, reputation, economics, and richer
witness authority once governance work lands.

## Notes

This feature is the practical evidence bridge between identity and review.

## Non-goals

- raw private-state exposure
- unconstrained memory-palace implementation
- production contract-market or payments work

## Completion Target

`v0.92`
