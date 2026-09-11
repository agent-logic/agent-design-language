# v0.92.1 final-candidate internal review plan

Status: `review_repair_in_progress`

Issue #520 produces one findings-first register for one immutable v0.92.1
candidate. This is a complete rerun of the internal review after the remediation
wave. Both reviewed dependency merges are present in the frozen candidate
`fb6cbc7f619daa54f901fd2d12f480add682ace3`. The second review found 14
product defects and routed all of them to #814-#821 under #522. Independent
exact-head review of the first assembled packet then found six packet-method
defects; those are accepted and must be repaired before publication.

## Current entry state

The C-SDLC issue is bound to `codex/520-internal-review` at
`/Volumes/FastWork/adl-worktrees/adl-issue-520-internal-review`. Execution may
is active at the frozen post-gate candidate. #718 and #758 are merged and closed
by their declared PRs. The assembler may consume only completed, independently
attributed specialist inputs; it must never synthesize reviewer conclusions.
All 176 acceptance rows require terminal implementation and proof dispositions,
and every retained evidence locator must resolve to the cited line. No paid
provider, cloud, deployment, or Runtime operation is authorized or required by
this internal review.

## Exact denominator

At execution time, fetch and retain full base, dependency-merge, and candidate
SHAs. The candidate is the exact `origin/main` revision observed immediately
after both #718/PR #809 and #758/PR #805 are merged and closed. Both dependency
merge commits must be ancestors of that candidate. The base is not
operator-entered: it is derived as the sole parent of the immutable WP-01
authority merge, issue #480 / PR #527. The run manifest records these SHAs and
the validator recomputes their ancestry. Build and retain these complete,
machine-readable inventories:

1. every tracked path in `base...candidate`, classified as production code,
   test/proof, documentation, lifecycle/evidence, generated, vendored, or
   other;
2. a pagination-complete live GitHub snapshot of every issue assigned to
   v0.92.1, every fully paginated closing PR reference for those issues, and
   every PR assigned to the milestone, plus inventories that match that
   snapshot exactly, including open, closed, merged, abandoned, stacked,
   deferred, duplicate, and superseded dispositions;
3. every v0.92.1 acceptance surface and planned work package, mapped to its
   implementation, proof, documentation, or explicit unresolved gap;
4. every canonical release document, demo claim, provider/cloud boundary, and
   retained release-tail artifact cited by the release handoff.

The live snapshot must record completion of independent issue, per-issue
closing-reference, and repository pull-request cursor pagination and must not
declare a query cap. It retains the exact GraphQL queries, combined query
digest, every page, terminal `hasNextPage: false` for every connection, and a
digest-bound raw response. The validator re-queries GitHub and rejects a
captured issue or milestone-PR roster that differs from live authority.
Planned-ID mappings are independently derived from the
immutable WP-01 final creation receipt at the candidate revision. No global
cap, sample, search-result truncation, or representative subset may
reduce those inventories. Generated or vendored surfaces may be dispositioned
as such, but they remain counted. Any omission, unclassified row, zero-test
lane, or missing evidence is a review finding or blocker—not a pass.

The execution specification itself is read and digest-checked from the frozen
candidate. Every acceptance row retains the exact canonical criterion content
and its digest; IDs alone are insufficient. Every issue inventory row must
match live title, state, and closing-PR associations exactly.

## Mandatory review coverage

- Review every changed production-code file for behavior, errors, recovery,
  concurrency, configuration, security, and integration regressions.
- Review every changed or added test/proof file for whether it exercises the
  claimed production behavior, meaningful test counts, negative cases, and
  non-vacuous assertions.
- Review all canonical v0.92.1 planning, feature, release, README, REVIEW,
  handoff, and evidence-index documents for implementation and status truth.
- Reconcile the full milestone issue/PR roster with typed lifecycle records,
  exact-head reviews, merge ancestry, and retained evidence.
- Review architecture, dependencies, supply chain, credentials/redaction,
  demos, Runtime/provider/cloud boundaries, and publication claims wherever
  the inventory shows an applicable surface.
- Map every acceptance criterion to concrete implementation and proof. Missing
  or merely documentary fulfillment is a finding.

Specialist lanes may run in parallel over disjoint assignments. Each assignment
records its exact path/issue rows, reviewer identity, candidate SHA, result,
limitations, and raw findings. A lane may not pass with an empty assignment or
by citing CI alone.

Every lane report is content-addressed in the packet manifest, contains one
evidenced observation for every assigned denominator reference, and contributes
its complete raw finding set to synthesis. The validator rejects a synthesized
register that differs from the raw lane union.

## Execution handoff

Run these internal lanes over the single immutable candidate. Lanes may execute
in parallel only after the denominator and assignment ledger are frozen:

- packet and denominator: `repo-packet-builder`;
- behavior/correctness: `repo-review-code`;
- test and proof quality: `repo-review-tests`;
- trust boundaries and redaction: `repo-review-security`;
- documentation and lifecycle truth: `repo-review-docs`;
- architecture and dependencies: `repo-architecture-review` and
  `repo-dependency-review`;
- findings union: `repo-review-synthesis`;
- final packet checks: `review-quality-evaluator` and
  `redaction-and-evidence-auditor`.

The execution operator must first:

1. fetch `origin/main`; verify #718/PR #809 and #758/PR #805 are each merged and
   closed, and freeze the exact fetched `origin/main` SHA;
2. record the exact base, both dependency merge SHAs, frozen candidate,
   issue/PR roster, and execution specification digests in `run_manifest.json`;
3. generate the complete denominator inventories and reject any empty mandatory
   assignment;
4. dispatch the disjoint lane assignments with the candidate SHA and retained
   denominator references;
5. synthesize every raw finding, then run the production validator in all four
   modes: `denominator`, `findings`, `integrity`, and `all`.

The repository's hosted Opus/API review runbook is not an implicit dependency
of #520. It may supply additional evidence only after separate operator
authorization; its absence cannot weaken or skip any mandatory internal lane.

## Finding and synthesis contract

Each finding has a stable ID, P0-P3 severity, exact candidate SHA, concrete
file/line or command evidence, affected acceptance surface, impact, source
lane, status, and disposition owner. Preserve disagreements and raw-finding
provenance through deduplication. Zero findings is valid only after every row
in every denominator has an evidenced disposition.

The synthesis must explicitly answer:

- what was implemented completely;
- what is partial, inert, unreachable, documentation-only, or unproven;
- what tests exercise real behavior versus helpers or fixtures;
- what claims exceed retained evidence;
- what findings block release, route to #522, or require an explicit owned
  deferral.

## Retained packet

Retain under `docs/milestones/v0.92.1/evidence/release/tail-04/`: the run
manifest; base/candidate identity; complete inventories; assignment ledger;
specialist reports; raw and synthesized findings; acceptance-coverage matrix;
live-state snapshot; proof register; validation results; redaction audit;
quality evaluation; and a digest manifest.

## Gates

Stop if #718 or #758 is not merged and closed by its declared PR, either merge
commit is not ancestral to the frozen candidate, `origin/main` changes before
the denominator is frozen, an inventory is incomplete, a mandatory lane is
empty/stale, a finding lacks exact evidence, or packet validation fails. #520
does not fix product findings, perform external review, approve release, merge,
deploy, restart Runtime, or invoke paid cloud or provider operations.
