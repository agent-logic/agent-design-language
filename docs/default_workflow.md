# Native maintainer issue workflow

This is the internal maintainer/managed-agent workflow. External contributors
use the [public fork-and-PR route](../CONTRIBUTING.md#external-contributors),
without installing private lifecycle tooling. Maintainers retain responsibility
for native admission, review and delivery of accepted contributions.

Use Cognitive-SDLC's authenticated native owner for tracked issue work. Select the
exact executable returned by successful admission for this repository and use its
matching [package manuals](csdlc-v3/man/README.md). The presence of source, a symlink
or a downloaded package alone grants no authority. Missing or stale admission
fails closed; retired v1 wrappers and automatic v2 fallback are not alternatives.

1. Read the issue, [AGENTS.md](../AGENTS.md), current cards and relevant source.
   Check primary checkout status and registered worktrees; keep primary `main` clean.
2. Inspect the selected executable's `--help` and command `--describe`. Prepare an
   issue-specific six-card plan with native `prepare ISSUE --plan PLAN.json`.
   Preparation inputs belong in resolved Git metadata, not primary tracked files.
3. Use native `status ISSUE`, `validate ISSUE` and `bind ISSUE`. Implement only in
   the registered worktree with an issue-bound session goal.
4. Use native `edit` for semantic card changes. Run the issue's focused validation
   and keep planned versus executed evidence explicit.
5. Obtain independent exact-head review; native `review` precedes `publish`.
   Use the covered native GitHub owner for issue/PR writes. Required check and
   policy admission precedes any authorized merge.
6. After merged delivery, native `finish` records terminal truth. Native `clean`
   separately previews and removes only the exact registered worktree.

The durable card sequence is `SIP -> STP -> SPP -> VPP -> SRP -> SOR`. Cards are
projections of typed state, not an alternate place to edit that state. Unknown
remote outcomes require authenticated readback and the native recovery route;
do not replay a write or hand-edit receipts.

Read [issue intent commands](csdlc-v3/INTENT_COMMANDS.md) alongside the selected
package contract. If prose and the executable differ, record the mismatch and
resolve the authority boundary. Do not build removed `csdlc-v3/` source in ADL.
Authoring and tests for that implementation belong to
[Cognitive-SDLC](https://github.com/agent-logic/cognitive-sdlc).

Historical cutover and rollback records retain their original scope. Any bounded
rollback exception requires its explicit authorization. See
[current authority](csdlc-v3/CURRENT_AUTHORITY.md) and
[checkout coordination](tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md).
