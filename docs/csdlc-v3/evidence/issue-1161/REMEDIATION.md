# Issue 1161 — Group A recovery and operator contracts

This is the bounded remediation packet for the #919 internal review, under #921.
It does not claim that the full milestone review, release, or deployment is complete.

## Finding dispositions and proving surfaces

| Finding | Correction | Focused proof |
| --- | --- | --- |
| ARCH-001 | Installed recover previews and activates retained issue/creation journal commits after authenticating retained identity and current authority. Creation recovery requires explicit operation selection; business effects are not redispatched. Supported recovery-adoption identity is preserved. | `semantic_journal_recovery`: retained crash windows, corruption, no creation dispatch; creation protocol adoption unit regression |
| ARCH-002 | Bind activation verifies the complete file/directory image. Interrupted target copies can be rebuilt only from an intact retained source and matching partial image. Conflicts preserve source, backup and journal. | `semantic_local_proof`: interrupted partial copy and activation; conflicting file, changed source and symlink refusal |
| ARCH-003 | Pending Finish/FinishWithoutPr reuses the retained original decision and authenticates current remote terminal state before completing missing state/receipt. Conflicting state refuses. | `semantic_journal_recovery`: reservation/state crash windows, merged finish replay, changed remote and conflicting local state |
| SEC-001 | Operational curl places `-q` first, disabling ambient curl configuration; credential config remains private. | Adapter subprocess regression with synthetic credentials, isolated CURL_HOME, real curl and negative-control sentinel |
| SEC-002 | Redact raw stdout/stderr before truncating either stream. | Adapter cutoff regressions including truncation inside a synthetic credential |
| DOC-002 | Semantic no-PR disposition omits unsupported caller timestamps. | Installed operator example accepted; either timestamp injection rejected without mutation |
| DOC-003 | Manual uses semantic plan/edit shapes and bind-before-edit ordering. | Installed owner executes documented edit and disposition; generated manual parity |
| DOC-004 | Shared schema describes validator declarations and replacement, while documenting native contextual admission and planning-only non-Cargo declarations. | Operator/schema regressions and separately executed JSON Schema positive/negative cases |
| TEST-TRANS-001 | Writer parks after the first actual commit until reader acknowledges generation 2; subsequent reads race remaining commits. | `transactions::tests::semantic_readers_observe_only_coherent_commit_boundaries` |

CODE-003 was withdrawn by the review coordinator and is not a repair claim.

## Validation and PVF classification

All new Rust regressions are deterministic tooling/owner-binary tests, required
for this issue's integration gate, using isolated local Git/files/processes and
synthetic remote/credential fixtures. They require no live provider, AWS server,
or credential disclosure. Crash injection is debug-only. The curl regression
requires the ordinary local curl executable and includes a negative control.

The planned native proof includes `installed_intent_commands`,
`installed_recovery_scope`, `transactions`, `semantic_terminal_cleanup`,
`semantic_journal_recovery`, `semantic_local_proof`, `operator_man_pages`,
`adapters::` unit tests, and the creation adoption unit regression. Native
candidate-bound receipts record actual executions; this list is not a claim
that a future CI run passed.

The JSON Schema regression at
`csdlc-v3/tests/support/validate_intent_content_schema.py` uses pinned
`jsonschema==4.26.0` in an isolated local environment. This is local supplemental
proof; current CI does not install or run that Python dependency. Native Rust
operator tests cover command acceptance and schema references independently.
Generated manual parity uses `python3 docs/csdlc-v3/man/render.py --check`.

Before final native proof, 90 integration tests passed (7 journal recovery,
17 local proof, 56 transactions, 7 operator manual, 2 terminal cleanup,
1 installed recovery scope), followed by 17 adapter unit tests and 1 creation
adoption unit test. No zero-test invocation is counted as proof.

## Review and remaining boundaries

Independent review caught two implementation defects during remediation:
creation activation originally ignored authenticated recovery adoption, and the
first reader handshake could pass on only pre/post-write observations. Both were
corrected and independently re-reviewed. Independent documentation review also
required including existing supported review/cleanup recovery dispositions in
the schema; final review evidence must cover the corrected complete envelope.

No shared operational binary is replaced by these tests. Main remains unchanged.
PR checks, exact-head independent review, and native proof/review/publication
remain distinct evidence. No merge or deployment is authorized by this packet.

The direct complete installed-command suite passed111 tests with1existing
explicitly ignored baseline-comparison test, zero failures (364.40seconds).
The original native full-suite run exceeded its300second bound while progressing.
Required native proof therefore uses explicit substring batches retaining all111
enabled tests; overlap is not counted twice. The baseline-comparison test remains
explicitly non-proving because its separate isolated baseline was not supplied.
An intermediate batch run was deliberately cancelled with confirmed cleanup to
correct fixture-only unused-helper warnings; neither incomplete run is a PASS.
Clippy all-targets with warnings denied passed after the scoped fixture annotation.
