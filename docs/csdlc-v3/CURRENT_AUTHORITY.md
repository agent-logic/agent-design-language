# Current C-SDLC authority

C-SDLC v3 is operational after #505 / PR #591. Operational authority requires
validation of the canonical `csdlc-v3/operator/authority-selector.json`, its
native authenticated receipt, terminal reconciliation and Git objects against
`origin/main`. Missing or stale proof suspends authority; prose and manifests
cannot grant it. Use `.adl/bin/native-v3/csdlc` and the declared typed requests.

Native `issue`, `edit`, `validate`, `doctor` and `bind` own local preparation;
`github-issue` and `github-pr` own remote operations; `review` precedes `publish`;
`finish` establishes terminal truth and `clean` separately guards cleanup.
Authorized exact-head PR merge uses the native `github-pr` `pull_request_merge`
request documented in [Native pull-request merge](PULL_REQUEST_MERGE.md). It
requires authenticated review/policy/check admission and durable reconciliation;
`finish --observe-github` then observes terminal truth without a second mutation.
`proof` and `install` are guarded native operational routes; their typed request
and authority checks determine whether an invocation may mutate. `shadow` and
`soak` retain historical inspection contracts with execution retired. Explicit
`local` construction inspection is non-operational; operational routes fail closed
when their authority context is unavailable and do not fall back to it.
V2 is permitted only for an explicitly authorized rollback or bounded transition
remediation, never as an automatic fallback for missing v3 proof.

## Command discovery and result interpretation

The command contract built from `v3-command-manifest.json` is available through
`csdlc --contract`; `csdlc <command> --describe` returns that command's descriptor.
These discovery responses describe inputs, effects and result schema; they do not
grant authority or execute the described route. Verify discovery against the
binary selected for the invocation. A source change or isolated candidate build
does not establish that the active installed binary has been upgraded.

JSON command results retain their owner-specific fields and add `/envelope` with
schema `csdlc.v3.command_result.v1`. Successful descriptor discovery returns its
contract directly; help remains text. Read process outcome, owner status,
authority and effect outcome separately: exit success or a `ready` classification
alone is not mutation, publication or terminal authority. An unknown effect or
unreported evidence invalidation must not be interpreted as proof of no effect.
Use the underlying owner result and durable receipts for lifecycle decisions.

The observation routes `doctor`, `validate`, `eligibility`, `schedule` and
`shepherd` inspect without creating a lifecycle lock or replaying interrupted
transactions. Recovery-required state must remain visible for an explicit
mutation/recovery route. This diagnostic guarantee does not extend to every
command grouped with local preparation, such as `issue`, `edit` or `bind`.

## Candidate issue intent interface

The [#869 issue intent guide](INTENT_COMMANDS.md) documents the candidate
ordinary commands and shared advanced request contract. Existing native request
routes remain available. Candidate source and isolated validation do not upgrade
the active operator binary or authorize sprint activation.

## Historical evidence

The [original pre-cutover changeover notice](TOOLING_CHANGEOVER_NOTICE.md) is
immutable historical evidence of the warning issued before #505. Its statements
about the then-current v2 route are not current operator instructions. The
[construction readiness notice](CUTOVER_READINESS_NOTICE.md) and retained
full-replacement denominator likewise retain their source-time meaning.
Future authority changes require a new explicit operator-reviewed decision and
notification; they are not authorized by these historical notices.

## Focused fitness proof

Run `cargo test --manifest-path csdlc-v3/Cargo.toml --test command_manifest`.
The authority-surface checks validate canonical selector/receipt truth, current
policy and metadata, and actual CLI help. Negative cases introduce obsolete
policy/help wording and require rejection. Historical records are excluded from
the current-routing scan and checked for preservation in the issue review.
PVF: deterministic local contract proof; small CPU and local Git/subprocess;
required for this authority-guidance change, no network or cloud execution.
