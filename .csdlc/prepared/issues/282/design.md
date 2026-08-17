# #282 design — exact-revision production Polis interface qualification

## Purpose

#282 assembles the final evidence packet for #117 without changing Runtime or Observatory behavior. It consumes terminal child evidence from #279, #280, and #281 and produces a reviewable, exact-revision qualification packet for release decision-makers.

## Inputs

- #279 terminal cache: accessibility, keyboard/focus, semantics, contrast, reduced-motion, screen-reader, and responsive UX proof.
- #280 terminal cache: large-Polis performance, long transcript, bounded latency/memory/DOM/stream resources, reconnect/restart/backpressure/offline/version-mismatch recovery proof.
- #281 terminal cache: security, privacy, TLS/origin, key/token handling, XSS/content rendering, replay, confused-deputy, stale-data, denial, and redaction proof.
- Parent contract: #117 coordinates production Polis interface qualification and #110 remains the WP-18C umbrella.

## Deliverable shape

The issue-owned implementation should add a bounded qualification packet under `.csdlc/evidence/282/` and the lifecycle cards for #282. The packet must:

1. Name one exact integrated candidate revision on current `origin/main`.
2. Index #279, #280, and #281 terminal envelopes, PRs, merge SHAs, reviewed heads, and proof artifacts.
3. Include an operator runbook for reading and rerunning the qualification evidence without credentials or cloud deployment.
4. Synthesize independent product, architecture, and security review outcomes.
5. Record residual risks, non-claims, and explicit deferred gates.

## Acceptance mapping

- AC-1: Terminal caches for #279, #280, and #281 validate with `canonical_match=true`.
- AC-2: The qualification artifact references exact SHAs and does not summarize from stale branches.
- AC-3: The runbook is executable with local/read-only commands only.
- AC-4: The packet does not claim cloud/public deployment, Unity live proof, Runtime authority changes, or new product authority.
- AC-5: Independent product/architecture/security review outcomes are retained and no unresolved actionable findings remain.

## Non-goals

- Implementing fixes discovered by #279, #280, or #281.
- Runtime authority, API, storage, or browser UI changes.
- Cloud/public deployment.
- Unity feature implementation or provider credential proof.
- Editing #117 or #110 parent issue bodies.

## Validation

The preparation validator for #282 must verify:

- #279, #280, and #281 terminal cache validation passes.
- The cached terminal envelopes for #279, #280, and #281 record merged PR disposition, closed-by-merged-PR issue state, and non-empty merge SHAs.
- The local #282 design and diagram still match the decomposed #117.d scope.
- The diagram preserves the required dependency topology from #279/#280/#281 into #282 and from #282 through review synthesis to #117/#110.
- The design explicitly rejects implementation and cloud scope.

Implementation validation should add a second artifact validator that verifies the final qualification packet has exact SHA references and the required review/runbook sections.
