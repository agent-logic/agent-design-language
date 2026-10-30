---
name: csdlc-v2-shepherd
description: Classify scheduler, validation, review, publication, and readiness next actions.
---
C-SDLC v2 remains the live lifecycle authority only until explicit V3-F/#505
cutover. Before that cutover, `csdlc-v3/**` is non-authoritative construction
evidence and must not mutate lifecycle state or replace v2 shepherding.

Invoke read-only `csdlc-shepherd`. It recommends typed next work and never acquires authority or mutates lifecycle state.
