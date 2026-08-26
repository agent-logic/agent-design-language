# Structured Intent Prompt

Template: 1.0.0

Issue: 201

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Establish the deterministic quorum-committed authority command, opaque voter-endorsement, durable result/checkpoint, and legacy-command closure protocol that #199 and #200 consume without any leader, caller, harness, or replica-local clock self-authorizing authority.

## Required Outcome

Every published authority-operation token is deterministically derived from an exact committed intent, canonical quorum-attested time, and a strict quorum of opaque current-voter endorsements; its private operation-specific view retains the exact bounded store-native signed artifact bytes plus digest and operation binding for sealed #199/#200/#203 consumers, its result and retry state are externally checkpointed, legacy direct authority commands fail closed, and no downstream membership or concrete-store side effect is claimed here.

## Scope

- Canonical prepare-and-finalize committed authority protocol
- Opaque local VoterEndorsementAuthority bound to exact current voter identity
- Canonical quorum-attested time token for deterministic replicated apply
- Durable protocol journal, exact retry cache, and external ConsensusCheckpointAuthority
- Opaque VerifiedAuthorityOperation token for downstream #199 and #200
- Legacy direct PolisCommand authority closure or versioned fail-closed replay
- Focused three-voter positive, restart, replay, rollback, and fault proof

## Authority

- Only concrete MembershipState and AuthorityMembership plus opaque distinct current-voter endorsements authorize finalization
- The leader, caller, runner, model, Shepherd, harness, local history, and replica-local clock are never quorum authority
- Raw signing keys, caller-produced endorsements, caller-selected voter sets, caller-substituted artifact bytes, and digest-only artifact reconstruction are rejected
- The private-field VerifiedAuthorityOperation retains exact committed operation-specific signed bytes, digest, intent, and operation binding; sealed #199/#200/#203 consumers retain their existing borrowed exact-artifact view
- A separate sealed #210-only ContinuityTransferGrantProjection is available only for the continuity-transfer variant and binds exact source, target, route/membership/certificate/boot cuts, signed bundle/catalog bytes and digests, bounded entry/chunk/range commitments, deadline, and cleanup identity; wrong-variant or wrong-consumer access is denied
- Neither private view exposes a constructor, replacement setter, generic payload conversion, raw endorsement, signing operation, migration decision, fencing, activation, serving, or concrete store effect
- #201 emits opaque verified projections but performs no OpenRaft membership or downstream authority-store side effect
- Governed membership belongs to #199, reconciliation publication to #200, concrete existing-store application to #203, transfer execution to #210, and kernel stage/read/discard effects to #208
- Guardian/API/WSS, models, AWS, live demonstrations, and #142 terminal delivery remain out of scope

## Assumptions

- none

## Operator Constraints

- Do not bind or edit product source until PR #197 is externally reviewed, merged, and ancestral
- Use typed C-SDLC v2 lifecycle commands and an issue-bound worktree
- Keep #201 bounded to the core protocol; route governed membership to #199 and concrete-store reconciliation to #200
- Run fresh independent exact-head review before publication
- Open a ready PR for visibility but never merge before operator review and authorization
- No AWS use and no lifecycle closeout
