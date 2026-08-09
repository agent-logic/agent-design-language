# C-SDLC v3 Rust Plan Review Record

Status: Initial findings incorporated; exact-revision verification pending

Issue: #73

Initial reviewed revision: `13aad5fd8039661f1bbbcaff703ee8d50f17c330`

Reviewed paths:

- `.adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.md`
- `.adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.mmd`

## Reviewer Identity And Evidence

| Reviewer | Provider-asserted model | Initial result | Evidence |
| --- | --- | --- | --- |
| Gemini | `gemini-3.1-pro-preview` | Request changes | `.csdlc/evidence/73/provider-reviews/initial-gemini-result.json` |
| Claude | `claude-sonnet-4-6` | Request changes | `.csdlc/evidence/73/provider-reviews/initial-claude-sonnet-result.json` |
| Claude diagnostic | `claude-opus-5` | HTTP 200 with empty text; not accepted as review | `.csdlc/evidence/73/provider-reviews/initial-claude-opus-result.json` |
| Claude diagnostic retry | `claude-opus-5` | HTTP 200 with empty text; not accepted as review | `.csdlc/evidence/73/provider-reviews/initial-claude-opus-r2-result.json` |

Model identity is provider-asserted. Provider reachability and successful text
extraction do not make model findings lifecycle authority.

## Gemini Review

### G-01: V2 writer revocation is missing

- Severity: P0
- Disposition: Incorporated
- Change: The migration now archives exact v2 state, removes the canonical v2
  index, writes a durable `migrated_to_v3` writer fence, requires v3 to observe
  the fence before mutation, updates supported v2 tools to reject fenced writes,
  and makes CI reject reintroduced v2 authority.

### G-02: Remote intent commit is absent from the transaction sequence

- Severity: P1
- Disposition: Incorporated
- Change: The plan now separates durable pre-network intent commit from
  post-readback state reconciliation and requires crash-resumable intents.

### G-03: Remote and terminal issue is overloaded

- Severity: P1
- Disposition: Incorporated
- Change: The former V3-13 is split into GitHub read-only observation, PR
  mutation/foreground watch, and finish/cleanup issues.

### G-04: OS signals do not reach structured cancellation

- Severity: P2
- Disposition: Incorporated
- Change: Root signal handling, cancellation-token propagation, task joining,
  OS-child termination, and bounded output drain are explicit.

### G-05: Cancellation lacks a distinct exit code

- Severity: P2
- Disposition: Incorporated
- Change: Exit 130 is reserved for interrupted/cancelled invocation outcomes.

### G-06: Cleanup path identity is underspecified

- Severity: P3
- Disposition: Incorporated
- Change: Cleanup requires canonical path equality with the verified Git
  worktree root and rejects prefix or relative-path matching.

## Claude Review

### C-01: Dependency graph lacks repository-to-adapter stabilization

- Severity: P0
- Disposition: Incorporated
- Change: V3-04 has a reviewed adapter-interface checkpoint; V3-05 consumes it
  through fakes; V3-09 waits for the V3-05 repository observation contract.

### C-02: Windows commit guarantees are unresolved

- Severity: P0
- Disposition: Incorporated
- Change: V3-08 must prove a per-platform synchronization and replacement
  matrix. Windows mutation remains fail-closed read-only if equivalent
  durability is not proven; the plan does not silently weaken its claim.

### C-03: Lazy application fields have ambiguous cell types

- Severity: P1
- Disposition: Incorporated
- Change: The `App` design now classifies sync and async lazy fields explicitly,
  caches typed initialization results, and makes a sync-to-async change an
  architecture revision.

### C-04: Cancellation propagation is underdefined

- Severity: P1
- Disposition: Incorporated
- Change: One root `CancellationToken`, signal wiring, `JoinSet` drain order,
  cancellation-aware waits, and OS-child termination are now specified.

### C-05: Importer schema, retention, and unsupported-field behavior are vague

- Severity: P1
- Disposition: Incorporated
- Change: V3-01 owns a versioned normalized import schema and retention policy;
  unsupported fields block v3 mutation pending reviewed field dispositions.

### C-06: Quantified size and effort claims lack methodology

- Severity: P2
- Disposition: Incorporated
- Change: Source counting excludes generated expansion under a declared tool
  profile, spike extrapolation must be explicit, confidence is lowered, and
  every proposed issue has an engineer-week planning range.

### C-07: Octocrab capability gaps are deferred

- Severity: P2
- Disposition: Incorporated
- Change: V3-02 inventories every required GitHub operation and reopens the
  dependency decision if more than three require raw requests. Every raw
  endpoint requires typed structures, an API reference, and fixtures.

### C-08: Local-command, PVF, and remote-operation issues are too broad

- Severity: P2
- Disposition: Incorporated
- Change: Local issue/bind work is separated from card/doctor work; PVF planning
  is separated from execution/evidence; remote work is split into three issues.
  The plan now contains 18 implementation issues plus deferred retirement.

## Final Verification

The final plan must receive fresh Claude and Gemini reviews over the same exact
revision. Verification is complete only when both return no undispositioned
P0/P1 findings and this record names that revision and evidence.
