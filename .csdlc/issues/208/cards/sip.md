# Structured Intent Prompt

Template: 1.0.0

Issue: 208

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Connect the production Guardian and supervised kernel through a private authenticated continuity channel that drives the complete live kernel participant set and reconciles restart, reply loss, rollback, and target cleanup.

## Required Outcome

The production Guardian initializes an opaque client and reaches a production kernel private listener before distributed-runtime readiness; the listener drives real admission and every sealed live continuity participant, exports a signed checkpoint, stages and validates isolated targets, resumes partial quiesce or discards every nonactivated target with exact receipts, and preserves one durable replay namespace across process restart and controlled certificate succession.

## Scope

- Production Guardian configuration, client construction, supervised-kernel session establishment, and polis-runtime capability injection
- Production kernel configuration, private loopback TLS 1.3 mutual-auth listener, startup, and closed dispatch
- Strict canonical codec, durable logical channel epoch, certificate succession, journals, replay, results, and restart reconciliation
- Complete sealed registry of real live admission, recorder/prefix, reasoning, governance, and operation-state participants
- Two-phase quiesce, signed checkpoint export, receipt-bound rollback/resume, isolated target validation, and validated pre-fence discard
- Exact focused proof, sixty-four boundary subassertions, separate Runtime/kernel Clippy, diff hygiene, immutable receipts, and independent review

## Authority

- TLS 1.3 mutual authentication is the sole request-channel authority; there is no application signing claim, bearer authority, or unsigned self-authenticating payload
- The accepted operation binds a stable logical Guardian/kernel identity and durable channel epoch; process boot identifiers are diagnostic and certificate rotation uses an explicit persisted succession schedule
- The kernel continuity manifest authority signs checkpoint content only and cannot authenticate the private channel or authorize transfer, migration, ownership, activation, or serving
- The private listener is loopback-only, absent from public Axum/OpenAPI, and denies agent, voter, Shepherd, Observatory, public-control, and distributed-authority identities
- Normal builds construct the client and participant registry only from validated production initialization; caller paths, mocks, synthetic checkpoints, injected traits, or omitted live participants fail closed
- SourceContinuityEffectPort alone performs quiesce/checkpoint/resume and returns opaque SourceQuiesceReceipt, SourceCheckpointHandle, and SourceResumeReceipt; downstream callers supply an already-verified decision but never live handles or paths
- ContinuityBundleSourcePort is issued only from an exact committed SourceCheckpointHandle and exposes bounded expected-range reads plus the exact signed manifest/catalog projection; it exposes no path or raw file handle
- TargetContinuityEffectPort owns stage/verify/activate effects and returns opaque TargetStageHandle, TargetPossessionEvidence, and TargetActivationReceipt; every write revalidates signature/key generation, entry order/schema/range/digest and chunk index/range/digest/predecessor before effect
- At stage creation #208 separately mints TargetCleanupPermit bound to the exact stage/root/channel generation and content commitments; transfer expiry/cancellation removes move authority but not this discard-only permit, which remains valid until verified TargetDiscardReceipt or TargetActivationReceipt
- #210 may request #208 stage/verify/discard effects but returns only VerifiedTransferPossession and never deletes or activates; #204 alone owns the executor/control-operation adapter and migration decision that invokes #208 source resume, target activate, or target discard
- #208 retains every kernel/filesystem effect and the cleanup owner; its effect receipts create no transfer, migration, fencing, ownership, activation-decision, or serving authority
- #210 owns remote transfer protocol, #204 owns migration decisions, #211 owns recovery decisions, and #142 owns final live qualification

## Assumptions

- none

## Operator Constraints

- Do not bind or edit product source until #191 / PR #197 is externally reviewed, merged, and ancestral
- Do not close #208 with library-only modules; both production binaries, Guardian initialization, polis-runtime injection, kernel startup, and every declared live participant must be covered
- Resolve every review finding through a subagent and obtain fresh exact-head review before publication
- Open a ready PR for visibility but never merge before operator review and authorization
- No public continuity route, AWS use, live cloud qualification, or lifecycle closeout
