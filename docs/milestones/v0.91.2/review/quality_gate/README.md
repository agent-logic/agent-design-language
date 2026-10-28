# v0.91.2 Quality Gate Packet

> Historical v0.91.2 quality walkthrough. #1223 retired demo_v0912_quality_gate.sh and its companion test because their required legacy coverage plan/full-run routes are retired. The original record, recipes and assertions below are preserved as history; they are not current executable commands or evidence of passing coverage. Current public validation is defined by .github/workflows/ci.yaml and tools/public_adl/run_ci.sh; coverage semantics are documented in tools/public_adl/COVERAGE.md.

## Purpose

This directory holds the tracked `WP-18` quality-gate packet for `v0.91.2`.

It exists to make the milestone's quality and coverage posture reviewable
without pretending that a packet or contract test is the same thing as final
release approval.

## Primary Surfaces

- [QUALITY_GATE_PACKET_v0.91.2.md](QUALITY_GATE_PACKET_v0.91.2.md)
- [QUALITY_GATE_v0.91.2.md](../../QUALITY_GATE_v0.91.2.md)
- `bash adl/tools/demo_v0912_quality_gate.sh`

## Boundaries

- This packet does not declare the milestone released.
- This packet does not convert contract checks into full release coverage
  evidence.
- Later Sprint 4 review, remediation, release evidence, and ceremony work still
  own the final closeout decision.
