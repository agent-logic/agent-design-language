# Completed publication transport recovery (#1142)

A completed semantic Publish can coexist with a retained PR-update transport
whose receipt is missing. Earlier recovery returned the semantic completion
before authenticating and settling that transport. Recovery now binds the exact
retained request to its completed semantic operation and reconciles the remote
PR title, body, operation marker, number and head. It records the native transport
receipt without reopening semantic history or sending another update.

From the bound issue worktree, after coordinating installation of the reviewed
owner binary with active sessions:

```sh
csdlc recover 1140 --json
csdlc recover 1140 --execute --preview <preview_digest> --json
csdlc status 1140 --json
```

Use the selected installed `csdlc` path and the exact fresh `preview_digest` from
the first command. Do not use a Cargo target binary as the operational owner.
Require an authenticated transport receipt, `performed_mutation: false`, and an
empty `pending_remote` inventory before the merge owner resumes its ordinary
native merge route. A stale preview must be refreshed, not bypassed. This repair
does not itself merge a PR, finish an issue, replay providers or deploy.

Only an exact current match settles an orphaned PR update. Changed metadata,
a different head, missing target, malformed or ambiguous response, and unavailable
readback remain blocked with retained evidence intact. Legacy update intents do
not retain authenticated prior metadata; a mismatch cannot distinguish an absent
update from a later author's change and therefore cannot authorize a retry.
Existing supported authenticated-absence recovery retains its one-attempt budget;
this transport-only path grants no new retry and does not reset spent budgets.

Validation uses deterministic installed-owner fixtures with synthetic GitHub
transport, local Git and local-only resources. The focused regressions cover
successful transport reconciliation, byte-identical semantic history, ordinary
replay, subsequent native merge and refusal cases. Existing authenticated-absence
and durable-receipt recovery regressions cover the unchanged retry guard. These
are tooling proofs, not live recovery or deployment evidence.
