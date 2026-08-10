# Issue 144 Design: Trusted Cognitive Authority And Full Revision Lineage

## Outcome And Sources

Repair the merged WP-13 cognitive-profile contract so profile policy and evidence derive from provisioned cryptographic authority, every predecessor through genesis is verified, and authority rotation is explicit and governed. The implementation consumes the existing Runtime v3 signing and governance APIs; it must not introduce caller-self-authorizing policy material.

## Owned Paths

- `adl-runtime-kernel/src/cognitive_profile.rs`
- `adl-runtime-kernel/tests/cognitive_profile.rs`
- `adl-runtime-kernel/tests/fixtures/cognitive_profile`
- `docs/milestones/v0.92/features/ACP_COGNITIVE_PROFILES_v0.92.md`
- `.csdlc/prepared/issues/144/produce-native-receipt.rb`
- `.csdlc/prepared/issues/144/validate-native-receipts.rb`
- `.csdlc/evidence/144`
- `.github/workflows/wp13-authority-repair.yml`

## Read-Only Inputs

- `adl-runtime-kernel/src/governance.rs`
- `adl-runtime-kernel/src/identity.rs`
- `adl-runtime-kernel/src/continuity.rs`
- Merged PR #139 and legacy issue #5830 records
- Every other source, sibling issue, Sprint 3 path, global workflow, and closeout surface

## Contract

A provisioned cognitive authority pins a verifying key, authority identifier, monotonic authority epoch, canonical policy digest, and governed evidence digest. Profile creation and update require a signed authority statement over those exact values. The profile retains the verified authority context digest. Rotation requires a signed transition from the currently trusted authority context to the new context, monotonically advances the epoch, and cannot rewrite earlier history.

Revision validation consumes the complete ordered predecessor chain. It recomputes every profile, canonical input, public projection, authority context, and predecessor link through genesis; truncated, substituted, rehashed, or syntactically valid forged ancestors fail closed. The current profile may reference only the verified terminal predecessor.

## Serialization Gates

This corrective issue is a mandatory predecessor of legacy issue #5831 publication and downstream Sprint 4 work. It does not edit #5831 while executing. The existing WP-13 product paths are serially owned by this post-merge defect until its qualified PR merges.

## Validation

The exact `cognitive_profile` target must prove trusted creation, same-authority update, governed rotation, complete multi-revision replay, self-authorized policy/evidence rejection, deep-chain substitution/truncation/rehash rejection, stale/wrong-key/wrong-epoch rotation rejection, privacy, and unchanged nonclaims. Native Linux and macOS jobs must execute the exact nonzero target at candidate HEAD, retain sanitized digest-bound receipts, and pass independent cross-platform semantic-equivalence validation.

## Rollback

Revert only issue #144 changes. Never restore self-authorizing policy acceptance or weaken full-chain verification to preserve compatibility. Keep legacy #5830 evidence immutable and truthfully identify it as superseded by this corrective authority contract.

## Non-Goals

No adaptive-learning implementation, autonomous retraining, broad governance redesign, global CI change, Sprint 3 work, diagnosis, personhood, standing, rights, citizenship, consciousness, or final Birthday completion.
