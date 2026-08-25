# WP-26 External Review Intake Validation

## Scope

- Canonical issue: #314
- Legacy C-SDLC issue/evidence id: 5847
- Worktree: `/Volumes/FastWork/adl-worktrees/adl-issue-314-wp26-external-review-intake`
- Branch: `codex/314-wp26-external-review-intake`
- Intake head: `c666ff3d411e6062d5cc5750d2f88be9efbcd673`

## Source Preservation

All three received PDFs were copied byte-for-byte into
`docs/reviews/v0.92/external-review-5847`.

| Retained report | SHA-256 |
| --- | --- |
| `adl-v0.92-documentation-review-findings-received-2026-08-24.pdf` | `70bbb48b271580a4e63eeedae250a8e017fd2cd0549a7ad0ea7117fa758c6f63` |
| `adl-v0.92-code-review-birthday-activation-received-2026-08-24.pdf` | `abecfd57ad64b838116779daa2da26ba0ce8ec7ebd2d08a3039cdf9a398b105e` |
| `adl-v0.92-code-review-birthday-activation-copy-1-received-2026-08-24.pdf` | `abecfd57ad64b838116779daa2da26ba0ce8ec7ebd2d08a3039cdf9a398b105e` |

The two code-review PDFs are byte-identical and are retained as separate
source reports because both were received.

## Findings Inventory

- Source finding occurrences: 10
- Unique findings after separate deduplication: 7
- Source severity counts: P1 = 1, P2 = 4, P3 = 5
- Unique severity counts: P1 = 1, P2 = 3, P3 = 3
- Complete source-instance index:
  `docs/reviews/v0.92/external-review-5847/findings-index.json`
- Deduplicated #315 routing index:
  `docs/reviews/v0.92/external-review-5847/cross-report-deduplication-index.json`

## Routing Update

Live issue metadata for #315, #316, and #471 was read on 2026-08-24 before the
local routing update. Per operator direction, #471 is recorded as a remediation
child route under #315 / WP-27. #314 retains intake/provenance ownership only,
and #316 / WP-28 remains independent planning work rather than a remediation
owner for this external-review packet.

No live GitHub issue body, metadata, merge state, or closure state was mutated
for #315, #316, or #471.

## Local Render And Extraction

Rendered and extracted in repo-local `tmp/pdfs/314-wp26-external-review`:

- PNG render outputs: 12
- Extracted text outputs: 3

Poppler emitted fontconfig cache warnings because its bundled cache directory
was not writable, but PNG outputs were produced and visually inspected for the
finding tables.

The temporary render/extract directory was removed after inspection; the
durable retained artifacts are the source PDFs, JSON indexes, and this evidence
note.

## Commands Run

```text
PATH="/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2:$PATH" .adl/bin/csdlc-v2/csdlc-bind --root . --request .git/csdlc-v2/requests/314-wp26-bind.json
ruby .csdlc/prepared/issues/5847/validate-external-review.rb packet
ruby .csdlc/prepared/issues/5847/validate-external-review.rb report
ruby -c .csdlc/prepared/issues/5847/validate-external-review.rb
git diff --check
python3 -m json.tool docs/reviews/v0.92/external-review-5847/findings-index.json
python3 -m json.tool docs/reviews/v0.92/external-review-5847/source-report-manifest.json
cmp -s /Users/daniel/Downloads/ADL_v0.92_Documentation_Review_Findings.pdf docs/reviews/v0.92/external-review-5847/adl-v0.92-documentation-review-findings-received-2026-08-24.pdf
cmp -s /Users/daniel/Downloads/ADL_v0.92_Code_Review_Birthday_Activation.pdf docs/reviews/v0.92/external-review-5847/adl-v0.92-code-review-birthday-activation-received-2026-08-24.pdf
cmp -s /Users/daniel/Downloads/ADL_v0.92_Code_Review_Birthday_Activation_1.pdf docs/reviews/v0.92/external-review-5847/adl-v0.92-code-review-birthday-activation-copy-1-received-2026-08-24.pdf
```

## Results

- `csdlc-bind`: passed; created the bound #314 intake worktree.
- Packet validator: `PASS: external review packet identity`
- Report validator: `PASS: external review report identity`
- Ruby syntax check: `Syntax OK`
- JSON syntax checks: passed
- `git diff --check`: passed
- Byte-for-byte comparisons: passed

## Readiness And Blockers

Ready for #315 handoff: yes.

Ready for release, merge, publication, or #314 closeout: no.

Preserved blockers:

- The documentation report verdict is `BLOCKED` because no dispatch, exact head
  SHA, or recomputed corpus digest accompanied the request.
- The source reports do not establish an exact reviewed target revision.
- The code review reports were operator-requested and outside the WP-23/#312
  documentation packet.
- The code review reports did not run builds, tests, failpoint injection, cargo
  audit, git operations, or runtime assertions.
