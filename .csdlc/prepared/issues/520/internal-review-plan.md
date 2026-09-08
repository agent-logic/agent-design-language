# v0.92.1 final-candidate internal review plan

Status: `prepared_not_started`

Issue #520 produces one findings-first register for one immutable v0.92.1
candidate. Review execution is gated on the reviewed merge of #519. This plan
does not freeze a candidate or claim review work has begun.

## Exact denominator

At execution time, retain full base and candidate SHAs. The candidate is the
exact TAIL-03/#519 reviewed merge on `main`. The base is not operator-entered:
it is derived as the sole parent of the immutable WP-01 authority merge,
issue #480 / PR #527. The run manifest records all three SHAs and the validator
recomputes that parent relationship. Build and retain these complete,
machine-readable inventories:

1. every tracked path in `base...candidate`, classified as production code,
   test/proof, documentation, lifecycle/evidence, generated, vendored, or
   other;
2. a pagination-complete live GitHub snapshot of every issue assigned to
   v0.92.1 and every associated PR, plus an inventory that matches that
   snapshot exactly, including
   open, closed, merged, deferred, duplicate, and superseded dispositions;
3. every v0.92.1 acceptance surface and planned work package, mapped to its
   implementation, proof, documentation, or explicit unresolved gap;
4. every canonical release document, demo claim, provider/cloud boundary, and
   retained release-tail artifact cited by #519.

The live snapshot must record completion of cursor pagination and must not
declare a query cap. It retains the exact GraphQL query, query digest, every
page, terminal `hasNextPage: false`, and a digest-bound raw response. The
validator re-queries GitHub and rejects a captured roster that differs from
live authority. Planned-ID mappings are independently derived from the
immutable WP-01 final creation receipt at the candidate revision. No global
cap, sample, search-result truncation, or representative subset may
reduce those inventories. Generated or vendored surfaces may be dispositioned
as such, but they remain counted. Any omission, unclassified row, zero-test
lane, or missing evidence is a review finding or blocker—not a pass.

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

Stop if #519 is not merged and reviewed, the candidate changes, an inventory is
incomplete, a mandatory lane is empty/stale, a finding lacks exact evidence, or
packet validation fails. #520 does not fix product findings, perform external
review, approve release, merge, deploy, restart Runtime, or invoke paid cloud
or provider operations.
