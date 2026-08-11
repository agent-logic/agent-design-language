# Structured Planning Prompt

Template: 1.0.0

Issue: 208

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

After PR #197 merges, bind #208 and implement the complete production Guardian-to-live-kernel path, durable channel succession, sealed participant registry, rollback/discard semantics and exact fifty-six-case/sixty-four-subassertion proof; run Runtime/kernel tests and Clippy plus diff and producer serially, resolve independent review through subagents, then validate and publish a ready unmerged PR.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "After #191 merges ancestrally, bind #208 and freeze the production Guardian/kernel init, private TLS, stable channel epoch, certificate succession, participant registry, journal, stream, root and receipt interfaces.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement validated configuration and startup in both production binaries, construct the opaque client in Guardian initialization, establish the supervised-kernel private session before readiness, and inject only that capability into the production polis runtime.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement the sealed complete live-participant registry, two-phase quiesce and signed export, partial-quiesce rollback/resume, isolated stage/validate, nonactivated validated-target discard, durable replay/restart/succession and filesystem/bounds safety.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run serially the exact Runtime test, kernel test, Runtime Clippy, kernel library/binary Clippy, diff hygiene and producer; require fifty-six cases and byte-for-byte parity with the eight-row sixty-four-subassertion map and its SHA-256.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Obtain fresh independent exact-head subagent review and route every actionable finding to a separate resolution subagent; rerun S4 after every source change until the reviewed source is clean.",
    "acceptance_ids": [
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "After clean review, run the distinct immutable validator, publish a ready PR closing #208, shepherd hosted CI, and wait for operator review and merge authorization.",
    "acceptance_ids": [
      "AC-9"
    ],
    "status": "pending"
  }
]

## Invariants

- No production readiness is reported until Guardian and kernel share the validated private session and the complete sealed live-participant registry exists
- Process restart preserves the durable logical channel epoch; certificate succession changes transport credentials without losing or duplicating an accepted operation
- Only TLS 1.3 mTLS authenticates control frames, while the manifest signing key signs content only and no bearer or public identity gains continuity authority
- Every participant prepared for quiesce either contributes to the committed bundle or has an exact reconciled resume receipt before admission reopens
- Source remains quiesced after bundle commit until exact resume or downstream fence; every nonactivated Staging or Validated target remains exactly discardable
- No caller path, normal-build mock, synthetic snapshot, cached bool, opaque receipt, or redacted evidence creates distributed authority

## Risks

- A listener/client library could pass tests without either production binary reaching it
- A one-way participant hook could leave early participants quiesced after a later failure
- Boot or certificate generation could accidentally strand an accepted operation after reply loss
- The live assembly could omit a state owner while a synthetic aggregate snapshot appears green
- Validated pre-fence target material could survive cancellation without receipt-bound cleanup
- Large frames or hostile roots could allocate without bounds, traverse paths, race opened handles, or leave residue
- The issue could drift into #210 transport, #204/#211 distributed policy, or final cloud qualification

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/208/design.md

Digest: 4f9fc720ef1b19b81a662a7ecb7669c98297567f5d474af30b9014551354c45c

## Diagram

.csdlc/prepared/issues/208/diagram.mmd

Digest: ec8c267a8722c9caf32a4efb94afa9b0e6af7dc73f74c4ea9e9b8b4c97b0ee08

## Stop Conditions

- Issue #191 / PR #197 is not externally reviewed, merged, and ancestral
- Production Guardian initialization and kernel startup cannot establish the private session without an additional narrowly tracked prerequisite
- The live assembly cannot enumerate every stateful continuity participant and prove registry completeness before readiness
- Any participant lacks a safe receipt-bound resume path after partial quiesce
- Certificate succession cannot reconcile old accepted work without accepting stale new work
- Any normal-build caller can inject a mock, caller path, synthetic checkpoint, participant, or raw authority
- Implementation expands into remote transfer, migration/recovery decisions, public Observatory/API, models, AWS, or live qualification
- Any focused proof or independent review has an unresolved actionable finding

## Handoff

Proceed only after doctor readiness.
