# V3-01 Design

Issue: #161

## Objective

Establish the immutable product, command, state, output, safety, and parity contracts that every later issue implements.

## Scope

The public command tree, versioned requests/results, exit taxonomy, canonical state fields, card projections, topology ownership, review, publication linkage, finish, cleanup, migration, and supported-platform matrix.

## Dependencies

- No child dependency; setup issue #146 and umbrella readiness only

## Architecture Decisions

- `V3-D01`

## Deliverables

- A versioned contract manifest, retained-v2 invariant register, versioned normalized parity/import schema, importer retention policy, command/help golden packet, explicit unsupported behavior register, the retained `.csdlc/evidence/73/official-cli-source-baseline.json` manifest and portable `git ls-tree` verification contract, the measured `csdlc-v3/contracts/state-size-baseline.v1.json` artifact and locked recomputation lane, versioned JSON envelope and schema-evolution policy, reviewer-principal and independence mechanism, per-card/per-phase field optionality table and optional-value placeholder, `PublicationLinkage::{Closing, PartOf}` contract with normalized qualified issue identity and relation grammar, state-size guard, PVF subprocess command-allowance policy, `pr watch` timeout/poll policy, and a versioned field/operation capability matrix covering normal authoring, post-review correction, invalidation, recovery provenance, audit evidence, and next valid operations. The state-size guard includes measured warning/block thresholds and headroom evidence. Output filtering includes a versioned supported-`jq` subset manifest with explicit unsupported syntax and diagnostics. The contract also pins the exact candidate `cargo-deny` release used from the construction spike onward. V3-02 may recommend changing that candidate only through the same reviewed stop/go architecture-revision path used for any failed spike threshold; it cannot silently substitute a release. V3-01 also freezes a candidate dependency manifest naming every previously open YAML, JSON Schema, middleware, template, and file-locking crate with exact version/features and a pre-spike direct-dependency count. V3-02 cannot begin while a production dependency slot remains unnamed or the candidate set already exceeds 30.

## Owned Paths

- `csdlc-v3/contracts/**`
- `csdlc-v3/schemas/**`
- `csdlc-v3/tests/contracts/**`
- `.csdlc/issues/161/**`
- `.csdlc/prepared/issues/161/**`
- `.csdlc/prepared/issues/161/validate-outcome.rb`
- `.csdlc/evidence/161/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. Every public command and output mode has a versioned contract.
2. Every retained v2 invariant maps to one owner issue and proof lane.
3. Exact review, GitHub truth, topology ownership, atomic state, and cleanup boundaries cannot be weakened by later implementation choices.
4. Unknown or intentionally changed v2 behavior is explicit and reviewed.
5. The importer remains available until the later of all v2-origin issues reaching terminal state or the operator-approved rollback window expiring.
6. Output filtering and templating have one approved in-process implementation boundary and cannot invoke a shell or external formatter.
7. Reviewer independence is structurally checked where identity is bindable; policy-only identity cannot silently satisfy publication.
8. Closing and non-closing publication are disjoint typed modes; `PartOf` cannot close or terminally complete its parent issue, and split-repository linkage is qualified in both modes.
9. Every mutable authoritative field has exactly one matrix owner and at least one typed authoring path; every supported invalidation/recovery state has a valid typed next operation. Operator authority may gate that operation but cannot replace its command, transition, target state, or audit contract.
10. Command help, kernel authorization, doctor findings, and tests are generated from or mechanically checked against the same capability matrix so scattered phase allowlists cannot silently diverge.
11. The state-size warning precedes the mutation block, initial block capacity is at least ten times the largest deterministic v2 baseline bundle, warning is fixed at 80 percent of that block, and neither path silently drops audit evidence.
12. V3-01 approval is blocked until the state-size artifact identifies the actual largest v2 bundle at `f1c01499`, records every measured blob and total, and passes the locked recomputation case; no unmeasured adequacy claim is allowed.
13. If that measurement makes the 10x block impractical for atomic state or operator latency, V3-01 stops and returns to architecture review for a versioned retention/compaction decision; it may neither lower the factor nor proceed with an unbounded aggregate.
14. The same gate proves the complete V3-16 review/recover/card-family canary fits below 50 percent of the block using maximum schema-valid event sizes, so embedded audit growth is represented rather than inferred from typical v2 history.
15. `--jq` accepts only the frozen supported subset; unsupported syntax fails with a typed usage error rather than partial or external execution.
16. The retained `adl.external_source_baseline.v1` manifest passes the VPP's repository-relative `upstream-source-baseline` lane before V3-02 can start; every cited blob must match the pinned `cli/cli` tree object exactly.

## PVF Lanes

- `v3-01-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/161/validate-outcome.rb`.
- `v3-01-focused-rust`: Run the focused C-SDLC v3 implementation tests owned by this work package. Command: `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --all-targets`.
- `v3-01-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Schema validation, golden command-tree comparison, invariant-to-issue coverage, publication-linkage truth tables for same-repository and split-repository inputs, capability-matrix completeness and uniqueness, recovery-path reachability, duplicate/omission checks, and independent contract review.

## Authority Boundary

- Issue V3-01 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Rust implementation, dependency selection beyond constraints, live state mutation, child command implementation, or v2 behavior changes.

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- An invariant lacks an owner, a command requires unresolved product policy, or contract approval would silently change v2 authority.

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-01`
