# Issue 311 Design: v0.92 Quality Gate

Status: preparation-ready; execution is serialized after canonical #310 is terminal, reconciled, ancestral, and cleaned.

## Authority And Migration Boundary

Canonical issue #311 replaces legacy `danielbaustin/agent-design-language#5842`
for active WP-22 execution. Legacy #5842 and its typed record remain immutable
provenance. Current-repository issue and PR truth comes from
`agent-logic/agent-design-language`; legacy predecessor records are consumed only
where no canonical successor exists. No row may combine identities from the two
repositories without an explicit migration mapping.

The gate is governed by `docs/milestones/v0.92/QUALITY_GATE_v0.92.md`,
`docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md`,
`docs/milestones/v0.92/features/README.md`, the WBS, issue-wave YAML, demo
matrix, checklist, and canonical typed terminal caches. Planning status is not
delivery evidence.

## Outcome Contract

Build one machine-readable completion matrix with exactly one row for every
indexed v0.92 product feature and every supporting critical path required by
the milestone gate. Each accepted row binds:

- stable row identity and feature/critical-path source;
- owner issue and repository;
- exact implementation paths and candidate/reviewed/merge revisions;
- PR and closing linkage;
- focused positive, negative, integration, and platform evidence;
- typed canonical generation, digest, terminal cache, and merge ancestry;
- claim boundary, residual risk, and disposition.

Rows that are planned, open, unknown, fixture-only, receipt-only, demo-only,
synthetic, provider-substituted, stale-reviewed, non-ancestral, platform-
unproven, or backed only by self-asserted JSON remain blockers. Digest syntax or
file existence alone is never proof. The validator must independently parse
typed terminal caches, resolve Git objects and ancestry, and re-observe GitHub
issue/PR/check truth.

The gate produces a findings-first blocker report. It does not repair product
features, waive incomplete work, or authorize WP-23/WP-25 while blockers remain.

## Dependency And Execution Sequence

1. Require canonical #310 terminal/reconciled/ancestral/clean before binding
   execution. Re-observe #309 terminal merge `5b3657582fea2109f000623bb121b7998185ac0a`
   and all legacy WP-04/WP-05/WP-06/WP-07/WP-13A predecessors plus canonical
   #308/WP-20 through their correct repository authorities.
2. Pin the exact gate base/candidate SHA and enumerate the feature index plus
   supporting critical-path denominator. Missing, duplicate, or extra rows fail.
3. Resolve every row to exact implementation, review, validation, integration,
   platform, GitHub, Git, and typed-terminal evidence.
4. Execute forged-evidence negatives for every prohibited class, including
   stale heads, non-ancestral merges, fabricated checks, malformed caches,
   cross-repository identity substitution, and missing platform proof.
5. Emit schema-valid matrix, gate record, and blocker report. The quality gate
   passes only with zero blocking rows.
6. Obtain one fresh exact-head independent review of validator behavior and all
   dispositions before publication.

## Owned Paths

- `docs/milestones/v0.92/QUALITY_GATE_v0.92.md`
- `docs/milestones/v0.92/WP_EXECUTION_READINESS_v0.92.md`
- `docs/reviews/v0.92/quality-gate-311/`
- `.csdlc/evidence/311/`
- `.csdlc/prepared/issues/311/validate-quality-gate.rb`
- `.csdlc/prepared/issues/311/test-validate-quality-gate.rb`

The issue-local design, diagram, and six typed cards are lifecycle artifacts.
All other repository content and all dependency worktrees are read-only.

## Serialization

- #310 -> #311: execution binding waits for exact terminal reconciliation and
  merge ancestry.
- #311 -> #312/WP-23 and the later review tail: downstream work remains blocked
  unless the retained gate result is `passed` at the exact merged #311 head.
- A blocker report is a valid WP-22 result but does not unlock downstream work.

## Validation

The positive validator regenerates the denominator, parses every row and typed
terminal envelope, resolves Git identities/ancestry, and verifies live GitHub
state and required checks. The negative suite mutates one authority dimension
at a time and requires deterministic rejection. Docs/YAML/schema and diff
hygiene cover the exact changed candidate. No paid runner is required; platform
claims consume already-retained provenance rather than launching new compute.

## Rollback

Revert the quality-gate documents, validator, and retained packet as one unit.
Preserve the rejected packet for audit, regenerate from current live and typed
truth, and never downgrade semantic checks to string or digest existence.

## Non-Goals

- Product remediation, issue closure, or evidence invention.
- Crediting fixtures, demo mode, synthetic success, or substituted providers.
- WP-23 documentation alignment, WP-25 review execution, release approval, or
  milestone ceremony.
- Mutating #310, dependency records, external repositories, or cloud resources.
