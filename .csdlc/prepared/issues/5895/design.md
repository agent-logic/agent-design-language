# Issue 5895 design: retire stale `csdlc-migrate` installer authority

Status: design approved for preparation.

## Decision

Treat this as a verification-first installer correctness repair. Current `main`
already deletes the `csdlc-migrate` binary and does not list it in the active
coexistence manifest. Execution must first reconstruct the current authoritative
binary inventory. If every acceptance criterion is already satisfied by the
#5861/#5896 line, the truthful outcome is an evidence-backed no-code closure;
otherwise the implementation removes only the proven stale expectation and adds
one focused regression guard.

## Authority boundaries

- Issue authority remains
  `danielbaustin/agent-design-language#5895`.
- Code publication targets `agent-logic/agent-design-language`; its PR body
  must use `Closes danielbaustin/agent-design-language#5895` so GitHub closes
  the authoritative old-repository issue when the new-repository PR merges.
- This is split issue/code publication authority, not repository cutover or
  issue migration.
- `csdlc-v2/operator/generation-selector.json`, the active coexistence inventory,
  and installed provenance are authoritative. Historical Gate 10 evidence is
  immutable and must not be rewritten.
- The retired binary must never be restored, renamed, wrapped, or replaced.

## Current-state flow

1. Enumerate Cargo binary declarations, coexistence/install inventories,
   selector truth, installer scripts, focused proof tests, and current operator
   documentation.
2. Classify each `csdlc-migrate` occurrence as active authority, regression
   guard, historical evidence, or issue text.
3. If active authority remains, delete that expectation and update the smallest
   coupled proof surface. If none remains, retain an evidence packet proving the
   issue was already resolved.
4. Build and install the declared set into a clean stable v2 generation
   directory, resolve v2, and compare installed provenance to the exact source
   revision.
5. Run one claim-free `csdlc-issue create -> csdlc-validate -> csdlc-bind`
   canary using the installed binaries.

## Invariants

- No v1 wrapper, `csdlc-migrate`, claim, lease, heartbeat, or compatibility shim.
- No broad product test suite; validation stays on installer, selector,
  provenance, retired-surface, and one lifecycle canary.
- No historical artifact churn.
- A no-code disposition is allowed only when exact current-main evidence proves
  every acceptance criterion.

## Failure behavior

Fail closed if the declared binary set and installed set differ, selector
provenance is stale, the canary uses a non-installed binary, or any active
`csdlc-migrate` route remains.

## Expected execution order

Execute before #5883 because both can touch installer/coexistence proof. Rebase
#5883 after this issue settles. #5881 is logically independent but should rebase
before changing shared operator documentation.
