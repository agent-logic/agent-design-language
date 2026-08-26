# #274 Observatory Quorum Serving-Eligibility Authority

## Decision and boundary

#274 is slice 205.c. It implements only the quorum-leased Observatory
serving-eligibility lifecycle over the terminal #272 durable foundation. The
coordination parent #205 owns no product code. #274 consumes only the sealed
authenticated projection delivered by terminal #350 and extended by terminal
#358; it cannot manufacture
authority from local configuration, cached booleans, caller DTOs, raw tokens,
permits, clocks, digests, quorum lists, or deadlines.

Production ownership is intentionally disjoint from #273:

- `adl-runtime/src/distributed/observatory_serving_eligibility.rs`
- `adl-runtime/tests/distributed_observatory_serving_eligibility.rs`
- `.csdlc/issues/274`
- `.csdlc/prepared/issues/274`
- `.csdlc/evidence/274`
- `adl/tools/check_coverage_impact.sh` (one explicit source-to-focused-test mapping only)
- `adl/tools/test_check_coverage_impact.sh` (mapping contract regression only)
- `adl/tools/run_pr_fast_coverage_lane.sh` (exact focused expression feature routing only)
- `adl/tools/test_run_pr_fast_coverage_lane.sh` (exact runner regression only)

#274 does not own or modify `serving_authority.rs`, the future Shepherd-specific
module or test, `authority_store_adapters.rs`, or any #205 parent surface.
`adl-runtime/src/distributed/mod.rs` is a shared registration surface and is not
in #274's independent product allowlist. If Rust module registration cannot be
avoided, implementation is serialized after #273 is terminal and ancestral;
the eventual #274 implementation packet may add only the one Observatory
module declaration after rebasing onto that terminal merge.

## Authority inputs

The prerequisite chain #191, #199, #200, #201, #202, #203, #272, #273, #350,
#356, and #358 must have
canonical merged terminal caches whose merge SHAs are ancestors of the exact
implementation base. The only authority-bearing production entrypoint accepts
`&PublishedAuthorityResult` and `&VerifiedServingAuthorityCut` and immediately
calls terminal #350's sealed
`verify_observatory_authority_projection(authority, cut)`. The returned
`VerifiedObservatoryAuthorityProjection` is private input to the state machine;
no constructor or parallel caller-supplied field list is permitted. This binds
the authenticated trust domain, polis, lineage, operation, committed log index,
foundation generation/OwnerCommit/fence/lease/state/result/receipt, durably
revalidated old/joint quorum basis, signer eligibility, authenticated
transition action and predecessor reference, full canonical finalization time,
and full inclusive committed deadline. Any verification error or missing, pending,
mismatched, rolled-back, corrupt, expired, or nonancestral authority fails
closed before state mutation.

Terminal #356 exposes only borrowed/copy accessors for the already-redacted
projection fields. Terminal #358 adds verifier-sealed accessors for the
authenticated action, predecessor operation reference, deadline seconds/nanos/
uncertainty, and finalization seconds/nanos/uncertainty. #274 uses only those
accessors for transition selection, predecessor matching, replay keys,
monotonic log/fence checks, canonical-time expiry, receipts, and its redacted
projection. It does not receive raw lineage, operation, OwnerCommit, lease,
membership, quorum basis, artifact material, or caller-selected action.

The Observatory lifecycle is a deterministic state machine:

1. `Ineligible`: no current committed quorum lease authorizes serving.
2. `Eligible`: one exact committed quorum lease and fence authorize serving.
3. `Revoked`: explicit committed revocation permanently rejects that lease.
4. `Expired`: the committed lease deadline has passed under the declared
   deterministic time input and cannot be renewed or revived.

The state machine never accepts an action argument. It dispatches exclusively
on terminal #358's authenticated `transition_action()`. Authenticated Acquire
requires no predecessor and records eligibility only from a successfully
verified projection. Authenticated Renew requires its predecessor reference to
equal the current eligible operation, the same lineage, and a strictly newer
committed log index/fence. Authenticated Transfer requires that same exact
predecessor match and atomically denies it before the successor becomes
eligible. Authenticated Revoke requires the exact current predecessor and is
monotone. #358 already rejects invalid action/predecessor cardinality and self
predecessors before #274 sees a projection.

Expiry is observation, not caller authority: the caller supplies only a bounded
canonical observation `(seconds, nanos)` and #274 delegates comparison to the
sealed projection's `is_expired_at`. The authenticated full deadline remains
inside #350/#358; equality remains eligible and only a lexicographically later
`(seconds, nanos)` expires. Uncertainty is retained in authenticated receipts
for deterministic replay but cannot widen or narrow expiry. Replay of the same
verified operation returns the exact prior result; caller action selection,
conflicting replay, stale fence, wrong lineage/predecessor, expired deadline, or
an independently valid but mismatched authority/cut pair fails closed.

## Quorum and overlap safety

Quorum evidence and the committed deadline are consumed exclusively through the
#350 verifier; #274 never receives or counts caller-provided peers, thresholds,
membership, signer IDs, or time claims. One trust-domain/polis lineage
has at most one eligible Observatory quorum lease per committed generation.
Transfer cannot expose overlapping incompatible quorums: the predecessor is
denied at the same committed transition that makes the successor eligible.
Partitioned, minority, stale, or locally retained evidence cannot acquire,
renew, transfer, or restore eligibility.

## Receipts and projection

Every accepted transition returns a deterministic receipt binding the
verifier-authenticated action and predecessor reference,
the opaque operation/lineage references from the sealed projection, prior and
resulting state digests, committed log index, foundation generation/fence,
authority-result digest, and coarse outcome. The public projection contains
only schema/version, #350's opaque keyed references, coarse eligibility state,
generation/fence/log index, transition/result digests, and an `expires_after`
boolean derived from canonical time. It exposes no raw token, permit, signature,
key, peer identity, endpoint, address, filesystem path, exact private deadline,
OwnerCommit, lease ID, foundation state/receipt digest, or provider material.

## Proof plan

The focused target `adl-runtime/tests/distributed_observatory_serving_eligibility.rs`
will prove positive acquire/renew/transfer/revoke/expiry flows and negative
fixtures for every #350 verifier mismatch, every #358 action/predecessor shape,
caller action injection attempts, independently valid A/B x A/B
cross-pairs, stale log/fence, wrong lineage, replay conflict, inclusive expiry
boundary, revoked revival, superseded transfer, overlapping eligibility attempts,
same-second nanos before/equal/after expiry, restart from durable Observatory
state, corrupt/unknown state, and redaction.
Tests must demonstrate that no public #274 API accepts quorum, membership,
deadline, OwnerCommit, lease, or naked digest fields. Zero tests, ignored tests,
or missing negative cases are
non-proving.

The repository coverage-impact gate is also part of the repaired acceptance
surface. `adl/tools/check_coverage_impact.sh` must map only
`adl-runtime/src/distributed/observatory_serving_eligibility.rs` to the exact
union `binary_id(adl-runtime::distributed_observatory_serving_eligibility) or
(binary_id(adl-runtime) and
test(/^distributed::observatory_serving_eligibility::tests::/))`, covering the
authentic integration binary and its meaningful module unit tests; it must not add a
basename fallback, broaden another source mapping, or weaken the 80 percent
threshold. `adl/tools/test_check_coverage_impact.sh` must prove the exact mapping
and preserve fail-closed behavior for an unrelated unmapped production Rust
source. The observed integration-only selector ran 4/4 tests but measured only
230/360 lines (63.89 percent) because it could not execute the meaningful module
unit tests. Because both targets are feature-gated, `run_pr_fast_coverage_lane.sh`
may route only the exact union to the existing `internal-test-fixtures` path and
may constrain Cargo to `--lib --test distributed_observatory_serving_eligibility`;
its contract test must prove that exact command without changing other filters.
The mapping and runner must use the same union without hidden rewriting.
Meaningful focused cases must raise this module's measured line coverage
to at least 80 percent; test-only padding, ignored tests, or a broad package
filter are non-proving.

The pre-bind validator proves packet identity, exact terminal-cache ancestry,
the disjoint allowlist, forbidden-path exclusions, and the serial registration
gate. It does not prove product behavior or authorize bind.

## Serial gates

1. Bootstrap this six-card packet at exact current `origin/main`, which must
   include terminal #350 merge `5bff0099858f005bcc045b0aa7548be4892a2acb`
   terminal #356 merge `abadc7c4501b9a26ae841206704bdfd1fee8508f`,
   and terminal #358 merge `cd0feef31240b95d344c5ae9b774325506586a5d`.
2. Obtain a new #119-compliant `fresh-session:<UUID>` design review that
   explicitly checks disjointness from #273 and the shared registration gate.
3. Approve and bind only after the new review passes and typed doctor validates
   terminal #350/#356/#358 cache truth and ancestry.
4. `distributed/mod.rs` may receive exactly one additive
   `pub mod observatory_serving_eligibility;` declaration; no other hunk is
   permitted. #273 is already terminal and ancestral.
5. Product implementation, exact-head review, publication, CI, and finish use
   the normal typed lifecycle; #275 remains out of scope.

## Non-goals

No Shepherd lifecycle (#273), durable foundation changes (#272), parent #205
implementation, #203 registry changes, UI, process launch, listener enforcement,
HTTP/WSS or transport wiring, migration #204, projection-v1 UI, cloud deployment,
cloud qualification, provider action, or paid runner.
