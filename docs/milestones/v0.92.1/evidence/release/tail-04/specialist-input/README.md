# Independent specialist input contract

These files are reviewer-authored inputs to `assemble-review.rb`. The assembler
must not invent observations, acceptance dispositions, reviewer identities, or
test results.

Each `<lane>.json` uses schema
`adl.v0921.internal_review_specialist_input.v1` and contains:

- `lane`, exact `candidate_sha`, `status: completed`, `reviewer`,
  `completed_at`, and a concrete `review_method`;
- the exact `denominator_refs` assigned to that lane;
- exactly one reviewer-authored observation per reference, with `ref`,
  `conclusion` (`verified_no_gap` or `finding`), a concrete `detail`, and a
  `review_basis` object whose `kind` is `candidate_path`,
  `acceptance_mapping`, `live_state`, `command`, or `retained_proof` and whose
  `subject` names the inspected surface;
- the exact canonical `finding_ids` emitted by that lane;
- for acceptance references, terminal `implementation_disposition`
  (`implemented`, `partial`, `missing`, or `not_applicable`) and terminal
  `proof_disposition` (`proved`, `partial`, `missing`, or `not_applicable`);
- for the tests lane, at least three distinct replayable `test_invocations`
  with argv arrays, zero exit status, retained output and digest, explicit
  success markers, and exact candidate-bound command-artifact digests, plus a
  truthful `execution_scope` that distinguishes executed proof from static
  review. Replay must reproduce success and the declared markers; byte-identical
  Cargo timing output is not required.

Reviewer scripts may enumerate rows, resolve candidate blobs, run deterministic
checks, and emit row-specific decisions. They may not default unexamined rows
to a passing conclusion, reuse the removed boilerplate disposition, or claim a
command proved surfaces it did not exercise. A missing or incomplete input is a
hard assembler failure.

Validate each completed lane before handoff:

```bash
ruby docs/milestones/v0.92.1/evidence/release/tail-04/validate-specialist-input.rb \
  docs/milestones/v0.92.1/evidence/release/tail-04/specialist-input/<lane>.json
```
