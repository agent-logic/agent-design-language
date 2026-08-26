# WP-25 Internal Review Design

## Outcome And Authority

Issue `agent-logic/agent-design-language#313` prepares and, only after every
entry gate passes, executes the v0.92 internal milestone review. The review is
findings-first evidence. It does not remediate findings, authorize external
review, approve release, or replace typed C-SDLC lifecycle authority.

The review target is one clean, immutable revision descended from all required
predecessors. The packet records repository identity, base, exact target SHA,
source manifest, exclusions, unknowns, redactions, reviewer identities, lane
outputs, finding provenance, synthesis, validation, and independent meta-review.

## Entry Gate

Execution remains blocked until all of the following are true:

- canonical WP-23 issue `#312` is merged, typed-terminal, reconciled,
  ancestral to the candidate, and has no active registered worktree;
- canonical WP-24 issue `#10` is terminal, reconciled, ancestral, and clean;
- WP-24A is explicitly deferred to v0.92.1 and is not a WP-25 execution gate;
- canonical issue `#313` continues to record WP-24A / `#342` as deferred to
  v0.92.1 and non-blocking, and the typed cards reproduce that current truth
  without claiming that the deferred work is terminal;
- the Sprint 6 graph in `#307` agrees with the issue identities above;
- the candidate checkout is clean and its exact SHA is frozen before packet
  generation.

Preparation may create and validate the issue cards, design, lane contract,
and validator plan before these gates close. It must not bind execution or run
the review early.

## Packet Contract

The canonical packet root is `docs/reviews/v0.92/internal-review-5846/`, with a
milestone entrypoint at
`docs/milestones/v0.92/review/V092_INTERNAL_REVIEW_5846.md`. The retained
packet contains at least:

- `README.md` for authority, status, and stop boundaries;
- `PACKET_MANIFEST.md` plus a machine-readable manifest and digests;
- `LIVE_STATE.md` for issue, PR, lifecycle, ancestry, and worktree truth;
- `SPECIALIST_LANE_RESULTS.md` with reviewer identity and completion status;
- `PROOF_REGISTER.json` binding each specialist report, reviewer, inspected
  denominator, method, limitations, finding count, target SHA, and digest;
- `FINDINGS_REGISTER.md` with stable IDs and provenance;
- `SYNTHESIS.md` preserving duplicates and disagreements;
- `VALIDATION.md` with exact commands, denominators, and limitations;
- an independent review-quality or meta-review record.

Included, excluded, unknown, local-only, generated, vendored, private, and
redacted surfaces are explicit. Host-absolute paths, credentials, private raw
state, and unredacted provider prompts are forbidden in publishable artifacts.

## Nine Specialist Lanes

Run all nine lanes independently against the same manifest and exact SHA:

1. correctness and behavioral defects;
2. architecture, boundaries, coupling, and state models;
3. tests, PVF classification, CI truth, and proof gaps;
4. security, privacy, trust boundaries, and secret handling;
5. dependencies, toolchains, lockfiles, and supply-chain risk;
6. documentation, claims, links, and release-truth consistency;
7. lifecycle, issue/PR identity, typed records, and retained evidence;
8. demos, integrations, platform proof, and operational readiness;
9. release and publication boundaries, redaction, and non-claims.

Zero findings is acceptable only when the lane records reviewer-authored scope,
evidence sampled, commands or inspection method, limitations, and an explicit
defensible zero-finding conclusion. Missing or incomplete lanes block review.

## Finding And Synthesis Contract

Every finding records a stable ID, originating lane and reviewer, severity,
exact evidence, violated invariant or failure mode, reproduction or proof gap,
affected scope, proposed owner route, duplicate links, disagreement state, and
open disposition. Synthesis may deduplicate presentation but never erase
provenance or disagreement. Findings remain open inputs for WP-27; WP-25 does
not fix them.

## Validation And Meta-Review

The issue-owned validator fails closed on wrong repository or SHA, dependency
drift, missing lane, missing reviewer identity, invalid finding schema, broken
evidence link, untracked duplicate/disagreement, digest mismatch, secret-like
content, private or absolute path leakage, or an unsupported readiness claim.

After packet validation, a bounded independent meta-review checks coverage,
severity calibration, evidence quality, actionability, disagreement retention,
redaction, and claim boundaries. Any actionable meta-review finding returns the
packet to correction and requires revalidation at the same exact SHA, or a full
refresh if source identity changes.

## Rollback

Withdraw the packet from milestone authority while preserving reviewer outputs
as immutable evidence. Correct the affected lane, manifest, schema, synthesis,
or redaction surface; rerun all declared validation and meta-review gates; and
republish only when identity and digests are current.

## Non-Goals

- Product or documentation remediation.
- External-review dispatch or rewriting an external review.
- One automatically created issue per finding.
- Release approval, ceremony, deployment, publication, or submission.
- Treating closed issues, receipts, articles, podcasts, CI green, or zero
  findings as product acceptance without the required exact-revision evidence.
