# C-SDLC removal plan — #1064

Proposed September 17, 2026. Work stays in this session. The isolated implementation is under qualification; PR publication is pending. No shared binary activation has occurred in this work.

## Target

One task record, one working branch, relevant validation, one independent implementation review, and one delivery flow. Routine local bookkeeping needs no approval ceremony. Remove machinery rather than wrap it in another interface.

## Remove and consolidate

| Remove | Replace with |
| --- | --- |
| Native and semantic records both owning local edits | One authoritative record and atomic version-checked local commit |
| Local-edit reservation, outcome and reconciliation receipts | Internal crash-safe storage; no external-effect protocol for local field changes |
| Six separately maintained cards and their completion gates | Sections or generated views of the one task record |
| Tracked projection commits and durable projection acknowledgements | Disposable generated views; regeneration does not advance business state |
| Hand-built proof paths, digests and review receipt envelopes | Tool-owned bookkeeping derived from actual validation and review judgment |
| Multiple receipts for the same external result | One operation identity with request and observed outcome |
| Repeated readiness and authority scans between adjacent steps | One admitted command context, rechecked at the mutation boundary |
| Full re-review after wording or generated-file changes | Preserve review; review only changed code or acceptance scope |
| Sprint reviews repeating child reviews and tests | Aggregate child results; inspect integration changes and unresolved findings |
| Separate finish and cleanup ceremonies | One closeout flow with safe cleanup and actionable exceptions |
| Several recovery owners for one operation | Resume the retained operation through its ordinary command |

Historical evidence remains immutable and readable. Retaining history does not require generating more receipts or retaining multiple active writers.

## Review and checkpoint budget

Two delivery checkpoints for routine issue work:

1. Candidate ready: relevant validation and one independent review cover the candidate.
2. External delivery: exact target/candidate, required CI and operator authorization match; record the observed result once.

No mandatory independent reviews of issue creation, preparation, individual cards, generated receipts, publication wording or unchanged child issues. Design review is exceptional, for a concrete unresolved architectural decision.

A finding requires correction, relevant tests and focused review of that correction. It does not restart preparation or the entire review. Source or acceptance changes still require review coverage; metadata and view changes preserve applicable evidence.

Keep explicit authorization for merge, shared activation, destructive action and ambiguous recovery where required. These are real external-action boundaries, not local bookkeeping checkpoints.

## Durable record budget

- One task record containing scope, plan, binding, validation references, review disposition and delivery state.
- One current validation result per relevant candidate/input set; raw logs are attachments.
- One current review record incorporating correction dispositions; prior reviews remain history.
- One operation entry per actual external mutation, including uncertainty and reconciliation.
- No lifecycle receipt for status, projection regeneration, local wording edits or another owner acknowledging the same result.

An internal atomic-storage journal is not an additional lifecycle authority or operator checkpoint.

## Execution sequence

1. Finish the tooling issue census and compare retained evidence from the last demonstrably working installation with the current candidate. Reproduce failures on isolated copies.
2. Consolidate all local amendments into the semantic commit path. Remove redundant writers and make projections non-authoritative.
3. Consolidate evidence records, derive review bookkeeping and invalidate only evidence affected by a change. Update skills, manuals and repository rules together to remove redundant reviews and card gates.
4. Unify preview/execution admission and the external operation recovery path. Complete publication, closeout and cleanup, including authentic older records.
5. Qualify one complete candidate and publish one PR. One independent candidate review, with focused correction review as needed. No cross-session implementation dispatch or shared-binary refresh during development.

These are implementation slices within #1064, not new issues, approval gates or another sprint. Activation remains separately authorized.

## Acceptance

Measure baseline and candidate on the same copied records and deterministic transport journeys: commands, manual inputs, durable records, reviews, repeated Git/authority reads, retries and elapsed time. Include failures and denominators.

- Zero manually reconstructed receipt/digest/context envelopes.
- One normal implementation review; focused correction review only for changed code or scope.
- Zero external-effect reservations for ordinary local edits.
- Zero mandatory card/projection checkpoints or generated-state commit loops.
- Metadata/view changes preserve applicable validation and review.
- Complete prepare/bind/implement/validate/review/correct/publish/authorized-merge/close/clean journeys.
- Finite recovery after crashes, stale requests and remote success with a lost response; no duplicate external effects.
- Authentic older records retain facts and continue through the same public flow.
- Preserve authorization, exact-candidate review, nonzero tests, credential hygiene and dirty-work protection.
- Target at least 50% fewer avoidable commands/retries on the matched defect corpus; separate necessary test/CI/external wait time.

Component tests and green CI alone do not prove these outcomes. Do not restore old state blindly after new remote effects; reconcile them first.

## Diagnosis evidence and limits

The initial GitHub census retrieved 550 issues, open and closed. Title matching identified 169 tooling-related candidates, including planning and documentation; this is not a count of defects. Full classification is unfinished.

Nineteen recent defects form repeated chains:

- Admission/correction: #984, #1003, #1046, #1048.
- Generated-state obstruction: #1013, #1039.
- Existing-record admission: #981, #1029, #1036, #1044.
- Interruption and external reconciliation: #975, #980, #1016, #1018, #1033, #1041.
- Coordination completion: #1006, #1028, #1061.

These support a failure of stages composing into complete journeys. They do not establish the exact deployment responsible for the operator's yesterday/today difference. That attribution requires retained installation and invocation evidence.

A separate claim that the primary repo-stable ADL binary still dates from September 11 and lacks CodeFriend is false of the binary inspected September 17: its modification time is September 17 05:07 local, and both top-level help and `adl codefriend --help` expose CodeFriend commands. This is command-availability evidence, not full product qualification or proof of its earlier state.

## Consultation disposition

Claude Opus and Gemini were consulted through the documented Rust provider
adapter with the removal plan and bounded source excerpts. These are design
consultations, not candidate approval. Claude's initial large request returned
HTTP 200 without usable text; a focused diagnostic and a bounded source
consultation subsequently returned usable text. The cause of the first empty
response remains undetermined.

Adopt the shared recommendation: one authoritative task record, disposable
views, and no local-edit external-effect journal. Make readers independent of
projection acknowledgements before stopping acknowledgement writes. Qualify
ordinary-command recovery after a committed edit, authentic older-record
continuation, and remote success with a lost response.

Do not equate a self-declared version in a generated file with authentication,
or delete unknown staging files. Keep path confinement, source dirty-work
protection, authority checks at mutation, exact-version admission, and external
operation identity. Observational commands remain read-only. Source assertions
from the consultations require verification: Gemini's claims about strict
unknown-field rejection on TypedReviewReceipt and absent fallback policy did
not match the inspected source. Existing atomic file replacement also needs
qualification before treating Claude's atomicity concern as a new defect.

## Current implementation boundary

Implemented in the isolated candidate, pending full qualification:

- Ordinary card, validator and publication amendments use one semantic local
  commit rather than native edit plus external-effect reservation/attachment.
- Review accepts the independent judgment and derives bookkeeping, retaining
  judgment and receipt in one file while reading legacy two-file records.
- New proof records bind candidate inputs separately from PR presentation.
  Title/body/draft changes reuse current proof and review; scope and validator
  changes do not. Older records retain conservative input-version checking.
- Semantic admission and proof no longer depend on generated state/card bytes.
  Authoritative inputs still pass renderer validation, and dirty source remains
  guarded. Inspection remains read-only.
- The operator manual describes compact review input and metadata preservation.

Active application and copied-record conversion writers no longer create
projection acknowledgements. Historical storage records remain readable; view
readback derives health from actual bytes without advancing task state.

The matched deterministic delivery rehearsal completed with 17 baseline commands
versus 12 candidate commands, two reviews versus one, and one failed invocation
versus none. It retained three intended external effects in both runs. See
`JOURNEY_COMPARISON.json` for denominators and timings. These are synthetic
transport journeys, not real GitHub delivery or authentic-record continuation.

The authenticated archived generation-9 conversion rehearsal passed: seven
record roles, twelve scenarios, and thirty durability-boundary sides. This
establishes conversion/recovery qualification, not the full delivery continuation
of an authentic older record.

Validator edits now validate declarations and semantic authority without requiring
a clean source tree. Proof execution still admits candidate bytes and refuses dirty
source. Claude's focused consultation recommended this separation; source inspection
confirmed the original rejection came from candidate-byte admission.

Still required: finish generated-state and ordinary-command recovery coverage,
prove authentic-record continuation through delivery/cleanup, complete final
checks, independent candidate review and PR publication.
Shared installation and merge remain outside this implementation boundary.
