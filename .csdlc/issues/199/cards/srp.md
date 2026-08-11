# Structured Review Prompt

Template: 1.0.0

Issue: 199

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

Exact membership coordinator and narrow PolisRuntime/MembershipState/AuthorityMembership integration, real secure OpenRaft transition tests, issue proof/evidence, typed issue truth, and absence of #200 or operational side effects.

## Prompts

- Does every operation require coarse AuthorityOperationKind::Membership plus an exact sealed issue-local EnrollNonVoting, PromoteVoter, or RemoveVoter artifact, with wrong coarse kind and wrong discriminator denied separately?
- Does the coordinator consume the sealed #201 artifact accessor and invoke only current governed #202 factory ports without staging, mutating, or constructing #202 private authority state?
- Does enrollment journal the exact external #202 admission operation and generation receipt, re-observe it after restart, and bind promotion to both the local published generation and the still-current external generation?
- Does removal journal and re-observe the exact #202 exclusion operation and generation receipt while local parity remains fail closed until OpenRaft final membership and all local state agree?
- Does the coordinator use standard OpenRaft learner and membership APIs while proving exact committed joint and final configurations and stable collision-free Raft ids?
- Can restart or leadership change reconcile before and after every governed #202 call, external observation, OpenRaft effect, local checkpoint, result, and publication step without duplicate effects or atomic cross-authority claims?
- Does governed rejoin ignore retained voting state until new sealed enrollment and promotion operations, current identity and certificate, catch-up, joint and final commitment, and parity complete?
- Does proof bind the exact thirty-six cases, wrong-discriminator subassertion, real secure nodes, protected-source drift, immutable evidence, strict Clippy, required hosted CI, and exact merge topology?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
