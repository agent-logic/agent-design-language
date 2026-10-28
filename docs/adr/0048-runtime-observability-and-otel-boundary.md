# ADR 0048: Runtime Observability And OTel Boundary

- Status: Accepted
- Date: 2026-07-06
- Accepted in: v0.91.7
- Related issues: #4718, #4682, #4989
- Related ADRs: ADR 0011, ADR 0038
- Source evidence:
  - `docs/milestones/v0.91.7/review/observability_4718/INTEGRATED_LOGGING_OTEL_PROOF_4718.md`
  - `docs/milestones/v0.91.7/WBS_v0.91.7.md`
  - `docs/milestones/v0.91.7/V092_HANDOFF_v0.91.7.md`
  - `docs/milestones/v0.91.7/review/runtime/SOAK2_FEATURE_LIST_MATRIX_4843.md`

## Context

v0.91.7 treats logging and observability as required polis/runtime
infrastructure. Machine-readable command output, human `adl_event` logs,
redaction, and OTel-compatible mapping need a boundary that prevents logs from
being mistaken for product readiness.

## Decision

ADL should preserve the stdout/stderr observability contract and treat
OTel-compatible mapping as an integration boundary. Runtime or Observatory
claims may consume logging proof only when the relevant runtime/soak consumer
has retained evidence.

## Consequences

- JSON tools remain parse-safe.
- Human logs remain useful for diagnosis.
- Runtime readiness must consume observability proof rather than merely cite it.

## Alternatives Considered

### Treat component logging proof as runtime readiness

Rejected. Runtime readiness requires integrated consumption evidence.

## Validation Notes

Future observability changes should prove parse-safe JSON, stderr `adl_event`
behavior, redaction hygiene, and runtime/consumer handoff.

## Non-Claims

- This ADR does not claim full OpenTelemetry export is complete.
- This ADR does not claim Unity or runtime consumers are complete without their
  own proof.
