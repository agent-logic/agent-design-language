# v0.91.8 Baseline And Ownership

Status: pinned architecture baseline. This packet identifies the exact source
trees and ownership boundaries used by WP-02. It does not authorize cutover or
deletion and does not claim that every line in an incumbent tree is live.

## Revision

- Git revision: `19c2b6e2ad18bddc75db9231643a54b2a446ce72`
- Measurement scope: tracked Rust files under the four source roots below
- Line rule: physical lines reported by `wc -l`, with files sorted by path
- Integrity rule: the Git tree object is the authoritative content digest

| Source root | Git tree | Rust files | Physical lines | Architecture disposition |
| --- | --- | ---: | ---: | --- |
| `adl/src` | `caeef78124a301b687b6c4461482a4566e4ee210` | 560 | 314,162 | Incumbent mixed ADL surface; classify by reachability and replacement owner before deletion. |
| `adl-runtime/src` | `ca21937d159bf2220060b6cf6b7fe7908778f4fe` | 20 | 12,251 | Transitional Runtime v3 compatibility/evidence surface; consolidate only after live parity. |
| `adl-runtime-kernel/src` | `99e2abb0839bdd42612a0270bd1d931b84eadea5` | 24 | 12,209 | Canonical Runtime v3 process and ingress owner. |
| `csdlc-v2/src` | `64917798a4015a2becf83e75cfe48beadb5af879` | 38 | 14,149 | Independent C-SDLC v2 lifecycle owner; outside ADL core and Runtime ownership. |

These counts are inventory facts, not deletion denominators by themselves.
Architecture ownership is closed at the product and source-root boundaries in
this packet. WP-13 may count a line as deletion-eligible only after its later
file-level manifest assigns the capability to an accepted replacement,
retained owner, boundary/non-claim, or explicit blocker. That later eligibility
classification refines deletion scope; it does not reopen product ownership.
Generated output, Cargo caches, worktree-local files, and untracked artifacts
are excluded.

## Approved Ownership Boundary

| Product | Owns | Must not absorb |
| --- | --- | --- |
| ADL v2 | Language primitives, deterministic compilation, portable plan/record contracts, thin CLI, generation selection | Runtime supervision/provider execution or C-SDLC lifecycle state |
| Runtime v3 | Bounded execution, supervision, provider/tool ports, recovery, live runtime state | ADL language semantics or C-SDLC cards and publication authority |
| C-SDLC v2 | Issue records, cards, claims, review, publication, shepherding, terminal closeout | ADL compilation or Runtime execution behavior |

`adl-runtime-kernel` is the canonical Runtime v3 process. `adl-runtime` remains
transitional until #5591, #5592, #5589, and #5590 establish reviewed live
parity and #5361 accepts the result. Neither tree is deletion-eligible from
planning prose alone.

## Budget Rule

The milestone target remains a 90 percent reduction of the reviewed replaced
incumbent ADL surface, with 80 percent as the fail-closed minimum. The
denominator is the sum of only those baseline rows or files later classified
as replaced and deletion-eligible. Runtime v3 source and test ceilings require
a separate reproducible implementation/test classification. The current owner
report, `bash adl/tools/report_runtime_v3_loc.sh`, observes 12,209 physical
lines under `adl-runtime-kernel/src`: above its 10,000 challenge target, below
its 20,000 exception ceiling, and therefore `reviewed_exception_required`.
This packet does not silently convert that exception posture into a new target.

Machine-readable parity is retained in
[`baseline_and_ownership_v0.91.8.json`](baseline_and_ownership_v0.91.8.json).
