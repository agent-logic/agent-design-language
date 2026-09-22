# Native recovery after concurrent merge (#1129)

## Diagnosis

The retained #1127 report attributed the stuck operation to a ready/merge race.
The ready call did refuse `intent_publication_remote_identity_mismatch` after
PR #1128 merged. However, the semantic pending operation was an earlier
`record_issue_mutation` for `issue_edit`, with unresolved
`github_mutation_not_reconciled` evidence. Its native intent was present.
The ready refusal did not create that pending operation.

The semantic staged packet retained the resolved edit, including preserved
fields. Recovery recomputed an operation digest from those expanded fields
instead of the original edit request. It looked for a different native intent
and failed `github_mutation_intent_unreadable`.

The bounded repair preserves the original request in new staged packets. Recovery
uses the packet's retained operation digest to load the typed native intent,
checks its intent digest and exact original-or-resolved request identity, and
then reconciles the original operation. Legacy expanded packets remain supported.
Missing, malformed, mismatched or altered intent evidence remains a refusal;
absence never becomes proof that a write did not happen.

## Validation contract

- `cargo test --offline --manifest-path csdlc-v3/Cargo.toml --test installed_intent_commands issue1129_`
  runs the real installed command journey with isolated Git repositories and
  synthetic authenticated transport. It injects external merge during PR
  readback, both with and without an earlier edit whose readback failed.
  Ready refusal must preserve the semantic/native reservation inventory.
  Recovery checks its preview token, reconciles the edit, and permits ordinary
  `finish` without another remote write.
- `cargo test --offline --manifest-path csdlc-v3/Cargo.toml --lib issue1129_`
  proves legacy expanded and original packet identity, and rejects changed
  target, path traversal, wrong intent digest, corrupt bytes and missing intent.
- The existing `installed_remote_recover` tests cover retry bounds, receipt
  identity and recovery after native completion or interrupted reservation.

PVF classification: deterministic installed integration plus local identity
contract; local CPU/Git/filesystem and synthetic transport; required issue
regression gate. No live GitHub mutation, paid provider call, Runtime deployment,
or release qualification is claimed. Stdout/stderr and credential redaction
checks remain in the installed fixture's shared success assertions.

The production #1127 journal and shared installed binary were not modified by
this implementation. Its live recovery remains subject to authenticated remote
readback; a later conflicting edit must not be overwritten or guessed away.
