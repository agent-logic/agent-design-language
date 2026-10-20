# ADL review guide

Review the exact candidate and its declared issue scope against the current
public repository boundary. Begin with [README.md](README.md),
[the repository map](docs/REPOSITORIES.md), [AGENTS.md](AGENTS.md) and the changed
package's manifest and validation entrypoints.

External contributors use the [public contribution route](CONTRIBUTING.md#external-contributors).
The native publication and closeout duties below belong to maintainers and managed
agent sessions; contributors provide changes and honest validation evidence.

## Scope and evidence

Public ADL owns language/contracts, portable presentation and installed-product
routing. Runtime, CodeFriend and Cognitive-SDLC source and product acceptance belong
to their owners. Do not require retired monorepo source or sibling builds as proof
of a public-only change. Conversely, public CI does not qualify a private owner.

Record the repository, base/head, changed surfaces and review limitations. Read
source and focused tests; do not infer behavior solely from names, prose, a source
copy or a successful process exit. Preserve failed, deferred and unexecuted proof.
Historical review packets remain immutable dated evidence, not current readiness.

## Review questions

- Does the change satisfy the issue without unrelated implementation or policy changes?
- Are deterministic ordering, schema/version compatibility and explicit side effects preserved?
- Do public commands preserve owner selection, argument/environment/stream behavior,
  and refusal when an installation or required identity is missing?
- Are artifact provenance, credentials and trust boundaries enforced by the actual
  contract rather than a path, filename or user-visible success message?
- Are input sizes bounded, paths normalized safely, and error, cancellation and
  partial-failure state transitions explicit for the changed behavior?
- Do tests exercise failures and meaningful behavior at the declared PVF lane?
- Do README, agent and onboarding instructions name files and commands that exist
  in this repository? Are external dependencies versioned and owner-qualified?
- Are lifecycle review, CI, merge, native finish, installed qualification and
  release claims separately supported?

## Validation

For public CLI changes, inspect and use `bash tools/public_adl/run_cli_ci.sh`.
For other public packages, select the current manifest and relevant focused lane.
For documentation-only changes, use link/path/command-source checks and
`git diff --check`; broad owner builds are not necessary. State exactly what ran,
what was inspected, and what remains deferred. Existing test receipts are historical
or reused evidence unless the candidate was actually tested.

For changes affecting coverage, inspect the current [public coverage inventory](tools/public_adl/coverage_inventory.json)
and [coverage gate](tools/public_adl/coverage.py). Review denominator and exclusion
changes, current thresholds and CI/local parity. Documentation-only work does not
require executing the coverage lane.

## Findings and outcome

Report actionable findings first: severity, exact file/line, concrete failure,
impact and bounded correction. Distinguish confirmed defects from questions and
unsupported claims. Avoid duplicate findings or style-only scope expansion.

An independent reviewer supplies exact-head evidence for native `review`; authors
fix actionable findings before `publish`. Changes after review require current
review evidence. Publication, merge, native `finish` and cleanup follow
[the default workflow](docs/default_workflow.md). Review approval alone grants no
merge, deployment, paid execution or release authority.

Earlier versions of this guide and linked milestone reviews remain available in
Git history. Preserve their dated findings and accounting rather than rewriting
past acceptance to match the new repository layout.
