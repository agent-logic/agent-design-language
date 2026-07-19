# WP-19 Publication-Safe Manifest

Status: replacement_preflight_required

Audience: external_reviewer

Issue: #4646

Last audited: 2026-07-19

## Authoritative Corpus

`REVIEW_CORPUS.v1.txt` is the sole candidate allowlist for the replacement
external review. The same file controls publication auditing, path-existence
validation, reviewer scope, and digest computation. It must be frozen at a new
exact revision before dispatch. The dispatch receipt is deliberately excluded.

## Allowed Internal-Review Inputs

- `docs/reviews/v0.91.7/internal-review-4645/FINDINGS_REGISTER.md`
- `docs/reviews/v0.91.7/internal-review-4645/SPECIALIST_LANE_RESULTS.md`
- `docs/reviews/v0.91.7/internal-review-4645/PUBLICATION_DISPOSITION_5571.md`
- `docs/reviews/v0.91.7/internal-review-4645/redaction-audit-5571/`
- `docs/milestones/v0.91.7/review/V0917_WP18_INTERNAL_REVIEW_4645.md`

These synthesized documents contain issue identifiers, repo-relative evidence
references, finding descriptions, dispositions, and explicit non-claims. A
deterministic scan found no secret values, private keys, raw provider output,
private URLs, or absolute user-home paths in the allowlisted files. Security
terms such as `credentials`, `API keys`, and `secret exposure` describe review
findings; they do not include credential values.

The source portion of the corpus intentionally contains redaction-test
fixtures. `adl/src/provider/http_family/tests.rs` uses the public AWS
documentation example access-key identifier `AKIAIOSFODNN7EXAMPLE`, and
`adl/src/csm_runtime_api.rs` contains synthetic `/Users/example/` plus literal
path-prefix markers used to prove sanitization. These are test inputs, not
operator credentials or machine-local evidence.

## Explicit Exclusions

Do not send or require these WP-18 directories:

- `docs/reviews/v0.91.7/internal-review-4645/packet/`
- `docs/reviews/v0.91.7/internal-review-4645/live-state/`
- `docs/reviews/v0.91.7/internal-review-4645/validation/`

The retained WP-18 manifest classifies that raw packet as `local_only` with
`publication_allowed: false`. The validation summary also records absolute
machine-local build paths. This WP-19 allowlist does not upgrade or override
that policy.

## Boundary

The completed #5571 audit permits sharing only its retained public disposition
and redaction records plus the other paths listed in `REVIEW_CORPUS.v1.txt`.
It does not approve publication of every already-retained WP-18 artifact. The
replacement corpus still requires final preflight at its exact target revision.
