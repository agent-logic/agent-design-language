# Review assessments and source support

Issue #1120 separates an actionable defect candidate from a positive observation
or an unresolved question. This document describes the in-progress contract;
installed-product and live quality acceptance remain separate requirements.

## Classification

A defect candidate identifies observed behavior, expected behavior, a concrete
trigger, impact, and proposed remediation or verification. Severity describes
the possible defect's impact, not the importance of a safeguard that already
works. Only defect candidates enter repair and test-proposal priority lists.

Positive observations describe supported behavior without assigning defect
severity. Unresolved questions retain uncertainty, missing context, and scope
limitations. Both remain in the authoritative assessment set and report counts;
neither becomes a repair task merely because it discusses important behavior.

Four valid lanes may complete with no defect candidates. That is successful
execution, not proof that the repository is defect-free. Malformed assessments
or invalid source support produce an explicit incomplete result, preserving
valid evidence from other lanes and independently supported siblings within the
same lane. `assessment_gaps` retains the original item index, safely bounded
summary, and rejection reason. Every citation required by an assessment must
validate; one matching citation cannot rescue its unsupported siblings. A gap
keeps the run incomplete and unavailable for successful Journey publication. They must not become a successful empty review.

## Source citations

Provider lane contract `codefriend.review_lane.v3` requires an admitted
`evidence_id` and an exact `quote`. Models do not calculate byte offsets. The
owner requires exactly one occurrence in the claimed immutable source object,
including overlapping occurrences. It derives zero-based half-open UTF-8 byte
spans internally and revalidates the source and quote digests. Legacy provider
offset fields may be decoded but never select or disambiguate a match.

No trimming, normalization, fuzzy search, cross-file matching, or arbitrary first
match is allowed. Invented, ambiguous, unavailable, or wrong-file quotes become
explicit unverified gaps, never findings. Retained `VerifiedCitation` spans and
digests still undergo the original strict validation. Historical lane-v2 input
prompt bytes and supported records remain readable without rewriting evidence.

Provider JSON may be bare, or wrapped in exactly one whole-response plain or
`json` Markdown fence with outer whitespace. Prose, multiple blocks, trailing
content, malformed JSON, and unknown fields remain rejected. Raw provider
responses are retained unchanged. Line annotations are never cited source bytes.

Support is bounded to four citations per assessment and 2 KiB per quote, with
64 KiB of quote bytes per lane and 256 KiB per review. There are at most 100
assessments per lane. Duplicate references count toward resource budgets before
rejection. Existing smaller ingress and transport limits remain authoritative.
Resource failures are explicit; successful output must not silently truncate
assessments. These limits do not introduce an elapsed-time cutoff for a review.

A matching excerpt proves where text came from. It does not prove that the text
supports the model's interpretation, that a trigger is possible, or that the
severity is justified. Those remain subjects of independent semantic review.

## Identity and compatibility

New `codefriend.contracts.v3` runs bind the complete assessment set in their
canonical identity. Assessment identities bind original admission, lane,
classification and support before the run identity is derived, avoiding a
circular dependency. The assessment set is absent from historical serialized
v1 and privacy-omission v2 records; their bytes and identities remain unchanged.

A defect's logical identity is distinct from its assessment receipt. Its lane,
source paths, observed and expected behavior, and trigger identify the logical
defect across reviews. Severity, explanation and proposed remedy can change
without making the previous defect appear resolved and a new one appear added.
The complete assessment receipt still binds all supplied content and citations.
Source excerpt positions and receipt changes remain provenance evidence;
baseline defect deltas do not certify semantic equivalence of different quotes.

New lane/input/result contracts use an explicit generation. Consumers must
validate compatible generation pairs and canonically derive actionable finding
projections. Source coverage remains independent from execution status: a
successful run can still have privacy omissions or unresolved context.

Historical findings retain their original, unclassified semantics. They are not
rewritten into verified defect candidates. An unsupported comparison across
assessment generations is not comparable; removing old observations from a new
defect list must not be reported as repaired source defects.

Embedded-record validation establishes snapshot integrity. Active operations
still require the original Store's consent, expiry and deletion checks. New
assessment identities cannot renew or replace that authority.

## Qualification

Deterministic tests cover source-location validation, classification routing,
resource bounds, legacy byte parity and canonical readback. They do not establish
model quality. The coupled PVF manifest records that distinction.

Live qualification uses newly captured request bytes for the changed prompt,
the original authorized scopes, and the existing cumulative model-spend ledger.
An independent reviewer adjudicates every returned defect candidate. Original
responses and failed attempts remain retained; there is no automatic provider
retry or replacement of historical findings. The complete Sprint 10 journey
matrix remains required.

### Semantic acceptance boundary

Qualification audits every defect candidate against the immutable admitted
source, including whether its cited excerpt supports its specific claim. An
excerpt from a valid file is insufficient when the finding describes behavior
in another file. The audit separately checks the concrete trigger, expected
behavior, impact and severity; partial support for a narrower problem does not
validate a broader original claim.

Record each candidate as supported, unsupported, or unresolved with the
supporting source and missing context. Keep a narrowly supported subclaim
explicitly qualified rather than accepting the original finding as a whole.
Audit positive observations and unresolved questions for correct routing too:
their absence from repair lists must not remove them from the report.

Keep the original response and its disposition separate. Do not feed audit
answers into a repeated live review or use previous failed outputs as provider
fixtures. A fresh run must use the accepted installed candidate and independently
captured requests, with failed or uncertain calls included in the spend ledger.
