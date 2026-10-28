# PR 1273 provider review and bounded corrections

Reviewed source: `fbfa90bdb4b1a3b237d8297e1feca6f0ab4d58c7`, base
`6a891db6d2ec7b59cfd369078bb9dd8204fd101c`. Claude Opus 5 and Gemini
3.1 Pro Preview independently reviewed source packets at the operator's request.
The packets contained the full changed-path inventory, 51 selected implementation
diffs and validation records; a second packet supplied the complete retirement
validator and CI files. These are bounded source reviews, not a full deleted-file
audit, functional qualification, or native lifecycle authority.

## Confirmed corrections

- Claude identified that an unlisted production file named `tests.rs` could be
  silently excluded from coverage. A synthetic LLVM report reproduced a passing
  full gate with that uncovered file omitted. Coverage now excludes only explicit
  `test_sources` in the inventory, rejects unlisted test-like paths, and rejects
  exclusions overlapping owned production sources. The inventory names 18 existing
  tracked test files/helpers across the five public packages and CLI.
- Claude identified missing diagnostics when nightly coverage result JSON was
  malformed. The report now records a blocker and exits unsuccessfully for invalid
  JSON, missing fields, invalid denominators, or malformed containers. The original
  producer evidence remains available; no malformed result is treated as passing.
- Claude identified conditional invocations that could silently skip missing
  guardrails. Both referenced scripts are already retired C-SDLC owner scripts;
  their dead optional invocations were removed from public CI, without restoring
  private owner implementation or claiming equivalent native lifecycle checks.

Focused proof: ten deterministic coverage tests pass; the CI contract passes its
success, threshold-failure, missing-result, invalid-JSON, zero-denominator,
missing-field and invalid-container report cases. A bounded independent subagent
review verified the changes and exact exclusion inventory. Its malformed-container
finding was corrected and the reviewer confirmed closure. No runtime, compiler,
package source, retirement denominator or rollback archive changed.

## Remaining review observations and dispositions

- Gemini retracted its missing `CONTRACT_REQUIRED` finding after receiving
  `ci_policy.py`: contracts are always selected and the aggregate requires success.
  Claude's equivalent conditional concern and empty-base concern are contradicted
  by the same source: manual selection takes the full baseline and PR selection
  refuses missing revision pairs.
- Provider concerns about PR-controlled CI policy are repository-governance
  limitations, not proof that this PR bypasses its checks. All eight checks passed
  at the reviewed head. This review does not establish branch-protection settings.
- Retirement validation is local with retained offline custody evidence. It is not
  a hosted CI custody audit; hosted checks and the local guard remain distinct.
- The execution overlay is a bounded retirement identity/deletion guard. It does
  not approve all unrelated additions to the repository. No broader whole-tree
  approval is claimed.
- Owner selectors explicitly require a previously qualified installation that
  remains trusted. They do not claim protection against an attacker controlling
  that prefix or the caller's environment; root ownership is not their contract.
- Windows execution is outside the Runtime selector's supported host targets.
  Exact lowercase archive identifiers are intentional contract requirements.
- Historical excluded workspace crates, documentation-only selection, formatting
  sensitivity in CI contract tests and label-triggered cancellation remain bounded
  review observations, not evidence of a reproduced current execution failure.
- Archive paths and members are bound to retained custody identities; the guard
  reads regular members rather than extracting them. Python TarInfo.isfile does
  not accept hardlinks. No archive traversal defect was demonstrated.

The initial adapter calls to Claude returned empty output and are not reviews;
subsequent direct authenticated calls returned complete text with `end_turn`.
Gemini's truncated responses were retained and completed separately. Original
responses, requests, hashes and corrections remain in the issue's external
provider-review evidence directory. No provider's response is treated as authority.

Native publication reconciliation, finish and cleanup remain outstanding. The PR
was opened as a draft using the operator's explicit transport override. Earlier
qualification records retain their exact revision boundaries; the corrective
commit requires its own hosted CI result before merge readiness can be claimed.
