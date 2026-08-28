---
name: csdlc-v2-shepherd
description: Classify scheduler, validation, review, publication, and readiness next actions.
---
Invoke read-only `csdlc-shepherd`. It recommends typed next work and never acquires authority or mutates lifecycle state.

## C-SDLC v3 transition boundary

C-SDLC v3 is construction evidence only until an explicit operator-reviewed
V3-F cutover changes root authority. Continue using this v2 shepherd route for
live next-action classification; v3 projections do not acquire scheduling or
lifecycle authority before cutover.
