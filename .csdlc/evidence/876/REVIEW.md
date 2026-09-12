# Independent review — #876

Reviewer: review_836. Exact source: e0dc293dca0114426e0fa267ef632899ff64cf70.

No actionable production-code finding was reported. The review covered profile
expansion/admission ordering, constructor effects and credential references,
recursive credential recognition, production dispatch/concurrency/LKG proof,
and provider compatibility.

## P2 — Publication cards retain template placeholders

The native values accepted preparation fields such as execution_summary and
review_summary, but the templates consume summary and individual execution,
validation and findings fields. Rendered SRP/SOR therefore remained incomplete.
This finding blocks publication until corrected and independently confirmed.

Correction: populated every rendered SRP/SOR field through native semantic
edits, used not_collected for missing telemetry, preserved worktree-only and
pending-CI truth, and retained the finding here. Production source unchanged.
Metadata-only exact-head review is pending.

### Bounded tooling reproduction retained

At source e0dc293dca, `cards/sor.values.json` contained a concrete
`execution_summary`, while `cards/sor.md` still rendered `<summary>`.
Likewise `srp.values.json` contained `review_summary`, while its rendered card
retained `<findings_status>` and `<recommended_outcome>`. The successful native
validate receipt at generation8 (`proof-validate.json`, digest79b8040e...) reported
`six_card_validation_passed`. The same issue is reproducible read-only with
`git show e0dc293dca:.csdlc/issues/876/cards/sor.values.json` and the corresponding
rendered Markdown. This is unknown semantic-field acceptance plus incomplete
render validation, not evidence of completed card authoring. The correction uses
the actual template fields. No lifecycle-tooling source was edited in #876 and
no new issue was created from this note.

## Metadata re-review — PASS

review_836 independently approved exact
`892242a33de94d47d4847839c01b0c5cbb7f137a`: P2 resolved, concrete validation
commands and uncollected telemetry truthful, pending CI/merge preserved, source
unchanged. No actionable findings remain. Native approval fields now record
that observed result; this update changes records only.

## Completion audit P2 — Malformed declared endpoint silently selects fallback

After publication at 06c0dcd, the completion audit added a numeric endpoint case
through the production loader. `config.endpoint: 123` under an Ollama profile
was accepted: adapter string accessors interpreted it as absent, permitting CLI
fallback. The previous independent reviews and proof above remain accurate
records of their scope, but did not include this case.

The new initial-loader case fails on the published source with `invalid initial
definition accepted` (numeric-endpoint-audit.log). Correction checks the existing
explicit endpoint/model/vendor/reference/shadow string fields before expansion,
rejecting non-string values while retaining existing empty-string/default rules. Opaque future adapter config remains
extensible. The real watcher/dispatch fixture now rejects a numeric endpoint,
asserts unchanged generation/digest and performs retained endpoint/model dispatch
without invoking the malformed replacement. Expanded-invalid-endpoint coverage
also remains in the loader matrix. Focused proof and renewed review are required
before updating the PR; prior CI does not approve this correction.

## Correction source review — PASS; whitespace claim correction

review_836 approved exact99e34042c9abb9fb58cfea5d5f150da651401ea1 production
changes: type guard correct, opaque-config extensibility preserved, and actual
watcher/LKG proof sufficient. A metadata finding noted that the aggregate
`git diff --check 06c0dcd..99e340` returns2 for terminal blank lines in three
retained raw logs. Those raw logs remain unchanged. SOR now identifies the
executed passing check as the scoped source/doc diff check and explicitly
discloses the aggregate raw-log whitespace. No source proof was rerun for this
wording correction; metadata-only final confirmation remains the publication gate.

## User review at 9495c0e — Bedrock alias and URL credentials (P2)

- Supported Bedrock `expected-account-sha256` alias with a 64-character public
  hash was rejected as an opaque credential while `expected_account_sha256`
  was exempt. Both aliases must receive identical string/64-hex validation and
  compatibility proof; adapter precedence remains unchanged.
- Endpoint/base URL `user:password@` userinfo and query
  `?api_key=sk-fixture` bypassed string credential checks and generic HTTP
  scheme/host validation. Reject embedded credentials and decoded credential
  query names before activation; prove actual watcher last-known-good retention.

The user reviewed exact `9495c0e94e325c4e9d24050ab2c7213eb91b4f51`;
10 focused tests, formatting and scoped diff checks were green, with no external
changes. Those checks did not establish the missing cases above. Prior proof and
findings remain preserved. This repair uses synthetic fixture values only.
