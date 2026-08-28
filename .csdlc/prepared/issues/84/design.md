# Issue 84 Design — Live Unity Observatory Runtime v3 integration

## Goal

Bind the approved Unity Observatory to the stable Runtime v3 API and WSS contract with authenticated controls, explicit failure states, and native Editor proof.

## Required Outcome

The Unity client consumes authentic Runtime v3 state, issues only authorized controls, reconnects safely, and retains positive and negative native Editor evidence without a parallel transport.

## Ownership

- `demos/v0.91.6/unity-observatory`
- `adl/tools/validate_v092_unity_observatory_live.sh`
- `docs/milestones/v0.92.1/evidence/observatory/unity`

## Dependencies

- Terminal reviewed #251 TLS 1.2 authority
- Terminal #122 public exposure
- Terminal evidence inputs #340 and #256
- Sprint 8 umbrella #536

## Safety Boundary

- This issue owns only the listed result and paths.
- All external mutations and private material remain governed by the operator constraints.
- Validation and exact-head review precede publication.

## Non-Goals

- HTML redesign
- Runtime API implementation
- TLS authority
- Provider integration
- AWS work
- Player build
