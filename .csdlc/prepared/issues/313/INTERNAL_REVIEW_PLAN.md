# WP-25 / #313 Internal Review Preparation Plan

Status: `design_approved_ready_for_binding`

Canonical issue: `agent-logic/agent-design-language#313`

This plan follows the full internal-review pattern established by the v0.91.2
corrective review packet and the v0.91.8 WP-18 packets. The typed six-card issue
record is initialized and its design is approved. Execution still stops before
specialist review, publication, remediation, or external dispatch until the
typed worktree binding is recorded.

## Current Gate Truth

- WP-23 / `#312`: merged through PR `#469`, typed-terminal, ancestral, and its
  exact registered worktree is cleaned.
- WP-24 / `#10`: merged, typed-terminal, ancestral, and clean.
- WP-24A: moved to v0.92.1 by operator disposition; not a WP-25 blocker.
- `#313` records WP-24A / `#342` as deferred to v0.92.1 and explicitly
  non-blocking. Preserve that live wording in the initialized typed six-card
  bundle; reject any regression to the legacy three-dependency contract.

## Preparation Sequence

1. Preserve the completed fresh typed design approval.
2. Validate the initialized six-card inputs from current `#313` truth so
   WP-24A remains deferred to v0.92.1 and the active dependency set is exactly
   WP-23 plus WP-24.
3. Confirm every active entry gate is terminal, reconciled, ancestral, and
   clean before review execution.
4. Resolve the installed v2 generation, validate the six-card bundle, run
   doctor, and preserve the registered FastWork issue worktree binding.
5. Maintain the issue-bound session goal before review execution.

## Review Execution Sequence

1. Capture live issue, PR, typed lifecycle, terminal receipt, ancestry, branch,
   and worktree truth for every active predecessor.
2. Freeze one clean `agent-logic/agent-design-language` exact target SHA.
3. Build a bounded repository packet with explicit included, excluded, unknown,
   generated, vendored, local-only, private, and redacted surfaces.
4. Record a machine-readable manifest and digest every packet object.
5. Run the nine specialist lanes defined in `design.md`, all against the same
   packet and SHA.
6. Build the stable findings register, retaining reviewer provenance,
   duplicates, disagreements, severity rationale, reproduction or proof gaps,
   owner routes, and open dispositions.
7. Synthesize findings-first without fixing findings or claiming readiness.
8. Run packet schema, identity, digest, evidence-link, secret, private-path,
   redaction, and negative validation.
9. Run an independent meta-review of coverage and review quality. Correct only
   packet-quality findings inside WP-25; route product findings to WP-27.
10. Record exact-head review truth through typed `csdlc-review`, then publish
    only through the typed lifecycle if all gates pass.

## Required Nine Lanes

Build the common bounded source packet first with `repo-packet-builder`. Run
the lanes in waves that respect the available agent slots; concurrency changes
latency only and never permits different packet digests or target SHAs.

| Lane | Review route | Required focus |
| --- | --- | --- |
| Correctness | `repo-review-code` | Behavioral defects, regressions, API misuse, state transitions |
| Architecture | `repo-architecture-review` | Boundaries, layering, coupling, runtime and authority models |
| Tests / PVF / CI | `repo-review-tests` | Denominators, assertions, determinism, lane truth, proof gaps |
| Security / privacy | `repo-review-security` plus `redaction-and-evidence-auditor` | Trust boundaries, secrets, injection, permissions, redaction |
| Dependencies | `repo-dependency-review` | Manifests, lockfiles, toolchains, provenance, supply-chain risk |
| Docs / claims | `repo-review-docs` | Commands, links, current behavior, release claims, non-claims |
| Lifecycle / evidence | `records-hygiene` | Issue/PR identity, cards, receipts, ancestry, stale evidence |
| Demos / integration | `demo-operator` with the declared proof register | Demo truth, platform proof, operational and integration gaps |
| Release / publication | `release-evidence` plus `review-readiness-cleanup` | Publication safety, launch assets, residual risk, authority |

After all nine immutable outputs exist, use `repo-review-synthesis` for the
findings register and `review-quality-evaluator` for the independent
meta-review. A lane may use a fallback reviewer only when the report records
the substitution and retains the same scope and output contract.

## Planned Packet

The execution packet will be rooted at
`docs/reviews/v0.92/internal-review-5846/` to preserve the issue-owned path in
the migrated source contract. The milestone entrypoint remains
`docs/milestones/v0.92/review/V092_INTERNAL_REVIEW_5846.md`.

Minimum artifacts:

- `README.md`
- `PACKET_MANIFEST.md`
- machine-readable manifest and object digests
- `LIVE_STATE.md`
- `SPECIALIST_LANE_RESULTS.md`
- `PROOF_REGISTER.json` mapping every lane to its exact target SHA, reviewer,
  report digest, inspected denominator, commands or method, limitations, and
  finding count
- `FINDINGS_REGISTER.md`
- `SYNTHESIS.md`
- `VALIDATION.md`
- independent meta-review record

Each specialist also writes one immutable report beneath `specialists/`; the
summary never substitutes for those reviewer-authored reports.

## Stop Boundary

This preparation has initialized and design-approved `#313`, but it does not
run the specialist reviews before typed worktree binding. It does not remediate
product findings or claim internal-review or release readiness.
