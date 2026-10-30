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
Retained construction and proof routes do not independently authorize mutation.
V2 is permitted only for an explicitly authorized rollback or bounded transition
remediation, never as an automatic fallback for missing v3 proof.

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
