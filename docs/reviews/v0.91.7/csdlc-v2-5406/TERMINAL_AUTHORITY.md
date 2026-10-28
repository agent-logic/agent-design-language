# C-SDLC v2 Terminal Authority

Issue: #5406

## Finding

Post-merge closeout previously changed only the issue worktree. The primary
checkout therefore retained an earlier active claim, while the shared receipt
was not written until prune and contained only the issue index. Exact SRP and
SOR evidence was absent from that receipt.

## Resolution

C-SDLC v2 now retains one digest-bound terminal bundle containing the complete
closed issue record and all six typed card values. The bundle is written
atomically below the Git common directory during closeout, including on an
idempotent retry.

A valid terminal bundle suppresses only the matching earlier claim: repository,
issue, and initialization digest must agree. Tampered, conflicting, incomplete,
or foreign bundles fail closed.

The typed `reconcile-terminal` closeout operation imports the retained bundle
onto a dedicated closeout branch. That branch is the mechanism for making
post-merge terminal truth portable in the tracked `.csdlc/issues/<issue>`
projection; no direct primary-checkout or Markdown edit is authorized.

Receipt authority is locked and checked before legacy normalization can mutate
local state. Reconciliation stages the retained design and diagram bytes into
issue-local `retained/` paths in the same atomic directory transaction as the
cards and index, so concurrent changes to earlier authored paths cannot alter
the terminal projection.

## Related Typed Repairs

- `csdlc-bind --amend-request` adds collision-checked paths to a live claim.
- `update_plan_step` advances SPP execution status without rewriting a card.
- `replace_validation_lanes` corrects VPP proof-role mappings under lifecycle
  guards and cross-card validation.

Gate 10D2 remains `v1_sunset`; no v1 command or retained v1 residue is restored.
