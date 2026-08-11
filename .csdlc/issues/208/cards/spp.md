# Structured Planning Prompt

Template: 1.0.0

Issue: 208

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

After PR #197 merges, bind #208 and implement the complete production Guardian-to-live-kernel path, durable channel succession, sealed participant registry, rollback/discard semantics, and the #208-owned target activation effect and retained receipt callable only from #204's finalized decision; prove the exact ordered fifty-six-case denominator, sixty-four boundary subassertions, and twelve cleanup/expiry/cancel/restart/activation lifecycle subassertions, run Runtime/kernel tests and Clippy plus exact-range diff and producer serially, resolve independent review through subagents, then validate and publish a ready unmerged PR.

## Plan

Revision 7

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
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement validated configuration and startup in both production binaries, construct sealed role-specific #204 migration and #210 transfer capabilities in Guardian initialization, establish the supervised-kernel private session before readiness, and inject only those capabilities into the production polis runtime.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Resolve review findings with durable five-participant prepare/resume receipts and RecoveryRequired reconciliation, restart-persistent source quiesce, crash-safe target stage/validate/discard/activate terminals, atomic channel/journal/certificate succession, hard effect caps and deadlines, descriptor-anchored filesystem effects, and kernel verification of signed #204 decisions.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run serially both exact focused suites and both strict Clippy lanes, then exact diff hygiene and a fresh producer whose 56 cases emit assertion-bound canonical behavior receipts while retaining exact 64-boundary and 12-lifecycle parity.",
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
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Obtain fresh independent exact-head subagent review from a reviewer distinct from this resolution owner, fix every actionable finding through a separate resolution owner, and rerun S4 after every source change until clean.",
    "acceptance_ids": [
      "AC-9"
    ],
    "status": "in_progress"
  },
  {
    "id": "S6",
    "action": "After clean review, run the current-main-aware immutable validator, publish a ready PR closing #208, shepherd hosted CI, and wait for operator review and merge authorization.",
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
- Every RFC 8785 request is rejected before dispatch on duplicate keys, noncanonical encoding, unknown fields, NaN or infinity, trailing bytes, decode/re-encode mismatch, or exporter mismatch, with exact tracked subassertion markers
- Every participant prepared for quiesce either contributes to the committed bundle or has an exact reconciled SourceResumeReceipt before admission reopens, including after reply loss or process restart
- Source remains quiesced after SourceCheckpointHandle commit until exact SourceResumeReceipt or downstream fence; every nonactivated TargetStageHandle remains exactly discardable
- ContinuityBundleSourcePort reads only signed expected ranges from one exact SourceCheckpointHandle and TargetContinuityEffectPort revalidates signed key generation, catalog entry order/schema/range/digest, and chunk index/range/digest/predecessor before any bytes are written
- TargetPossessionEvidence exists only after exact complete validation; #210 wraps it with TargetStageHandle as VerifiedTransferPossession but gains no deletion or activation authority
- TargetCleanupPermit is a separate discard-only #208 authority bound to the exact stage and survives #210 transfer expiry, cancellation, reply loss, and process restart until verified TargetDiscardReceipt or TargetActivationReceipt
- #204 alone owns the activation decision and private decision adapter; #208 alone performs source resume, target activate, target discard, and every other kernel/filesystem effect
- #208 activation revalidates the exact finalized #204 decision and stage/possession/cleanup/route/membership/certificate/boot/lineage bindings, atomically consumes TargetCleanupPermit, durably records the activated generation, and returns a retained exact-retry TargetActivationReceipt
- A TargetStageHandle with a TargetActivationReceipt is never discardable; cleanup or discard after activation is denied across retry and restart
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

Digest: f8477289e87099b41863e14bfc6ab265892fce8becfa5603e6b129684c8c90e6

## Diagram

.csdlc/prepared/issues/208/diagram.mmd

Digest: 5ed91caca4e4c26c0e39066bde9cdb2ca36632a1345982113b649d42abda7c49

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
