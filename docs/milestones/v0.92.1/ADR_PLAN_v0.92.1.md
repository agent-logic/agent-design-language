# v0.92.1 ADR Plan

| ADR candidate | Owner | Required evidence |
| --- | --- | --- |
| Corporate critical-asset and redacted evidence boundary | CORP-01/02 | Counsel boundary, asset schedule, public/private evidence split |
| C-SDLC v3 single binary and application context | V3-01/03/04 | Reviewed command and dependency contract |
| C-SDLC v3 state commit point and transaction durability | V3-06/08 | Canonical state, fault injection, platform matrix |
| C-SDLC v3 branch/worktree ownership without claims | V3-10A | Ownership and recovery proof |
| C-SDLC v3 exact review and publication linkage | V3-12 | Staleness, independence, `closing | part_of` proof |
| C-SDLC v3 writer-fenced cutover | V3-16 | Single-writer migration and rollback evidence |
| Distributed Runtime qualification topology | DRT-01 | Three-voter, shepherd, Observatory, fault-controller contract |
| Distributed Runtime fencing and continuity | DRT-04 | Quorum, stale-owner fencing, halt, snapshot, healing evidence |
| Distributed evidence and replay contract | DRT-06/07 | Producer receipts, coherent cut, replay digest, redaction |

ADRs remain proposed until their owning issue validates the decision. Release and ceremony mechanics do not require ADRs.
