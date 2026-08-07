# Issue 5903 design: claim-free Sprint 4 execution readiness

## Decision

Apply the generic readiness contract merged by #5901 to the already-approved
Sprint 4 packet. Move every serialization-gate record from path-only SPP
`affected_areas` into the same SPP's typed `replan_triggers`, while retaining an
exact gate manifest for parity validation. Preserve the approved product
designs, issue order, validation lanes, and live dependency gates.

## Scope

- Normalize the SPP owned-path collections for #5825, #5826, #5827, #5828,
  #5829, #5830, #5831, #5833, and #5834 through typed edits.
- Preserve every removed serialization gate in the owning issue's typed
  `replan_triggers` and prove exact manifest parity.
- Normalize umbrella #5857 to own only its execution packet and evidence paths.
- Replace retired claim/reacquire instructions in the Sprint 4 prompt and
  execution packet with typed branch/worktree binding authority.
- Retain exact typed edit requests and a focused readiness validator.

## Invariants

- No Sprint 4 product source is created or changed.
- The nine-child denominator and dependency order do not change.
- Git branch/worktree topology remains lifecycle ownership authority.
- #5857 stays open; this repair closes only #5903.

## Validation

- Current `csdlc-doctor` built from the merged #5901 source passes #5857 and
  all nine children with `ready=true`; the retained report records provenance.
- A focused validator force-builds and digests `csdlc-doctor` from the current
  source, then checks path-only SPP ownership and exact serialization-gate
  parity across the merged baseline, retained manifest, and current cards.
- The same explicitly nondeterministic live-proof lane queries the legacy
  tracker for prerequisite closure, rejects retired claim text, enforces the
  exact issue denominator, and applies an exact-base changed-path allowlist
  proving no Sprint 4 product path changed.
