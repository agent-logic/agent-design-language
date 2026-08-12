# Structured Output Record

Template: 1.0.0

Issue: 199

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented governed promotion, removal, non-voting enrollment, fresh-node rejoin, concrete authority parity, durable checkpoint/publication, crash recovery, and bounded exact retries. Retained v19 proof binds twelve behavior cases and nine production assertions to the exact protected source revision.

## Artifacts

- .csdlc/evidence/199/v19/execution-proof.json
- .csdlc/prepared/issues/199/produce-proof-receipt.rb
- .csdlc/prepared/issues/199/validate-proof-receipt.rb

## Execution

- Sealed phase completion behind the GovernedMembershipRuntime owner and bound exact stable maps and target membership before Raft effects
- Staged local membership and authority projections until durable publication and repaired projections on exact published retries
- Journaled enrollment before external activation and retained a bounded durable multi-operation result cache
- Classified failed membership changes using the applied OpenRaft configuration, preserving ambiguous joint outcomes and retrying proven no-effect attempts
- Proved real removal, separate enrollment, fresh-node rejoin promotion, parity publication, and immediate crash recovery
- Exercised fail-closed restart and exact retry across enrollment, exclusion, learner, joint/final history, local reconcile, parity, checkpoint, durable publication, and visible-view boundaries
- Dropped and reopened the lock-backed MembershipCoordinator after each injected real-node boundary, and resumed partially journaled joint/final history idempotently
- Persisted candidate stable Raft ID, old/target registry digests, and the collision-checked enrollment target registry atomically with publication
- Placed stable-map crash hooks immediately after target-map construction and before MembershipState event preparation, distinct from local-projection hooks
- Separated older retained enrollment cache hits from current published projection repair; current retries re-observe #202 and validate exact NonVoting identity
- Required exact existing MembershipState voter identity, AuthorityMembership stable map, and live OpenRaft voter parity before enrollment journaling or #202 effects
- Rejected conflicting candidate identity and stale durable registry mappings before enrollment journal or #202 activation
- Preflighted the complete MembershipState Join and required candidate identity absence before journal/#202 effects; rejected duplicate durable registry entries
- Serialized durable enrollment publication and local visibility repair, blocking all later membership operations until the exact projection is installed
- Serialized durable publication and local projection visibility repair for enrollment, promotion, and removal; required exact durable registry equality with current authority.

## Validation

[
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/199/produce-proof-receipt.rb"
    ],
    "purpose": "Produce exact v19 protected-source evidence with twelve behavior cases and nine production assertions",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/199/v19/execution-proof.json"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/199/validate-proof-receipt.rb"
    ],
    "purpose": "Validate exact argv, denominators, protected source, immutable introduction, and current-main ancestry",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/199/v19/execution-proof.json"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject warnings across the production Runtime library",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/199/v19/clippy-lib.stderr.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_membership_transition",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject warnings across the exact public transition target",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/199/v19/clippy-integration.stderr.log"
  }
]

## Integration

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
