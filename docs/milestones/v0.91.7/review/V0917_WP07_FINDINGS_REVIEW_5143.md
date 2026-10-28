# WP-07 Findings Review

Status: review_findings_record
Issue: #5143
WP: WP-07
Umbrella: #4634
Date: 2026-07-10

## Scope

This document records WP-07 review findings that were previously spread across
runtime, observability, Soak 2, and CSM coherence artifacts. It is a findings
index, not a new proof run and not a release-readiness pass.

Source note: some fine-grained review dispositions live in operational SRP/SOR
cards under `.adl/` in the primary checkout rather than in this issue worktree.
When a finding cites those card sources, the evidence column also includes a
merged PR or tracked review packet for portable review context.

## Findings

| ID | Severity | Status | Finding | Evidence | Disposition |
| --- | --- | --- | --- | --- | --- |
| WP07-F-001 | P2 | fixed | OTel sink/status write failures could be silent in quiet mode when `ADL_OBSERVABILITY_STDERR=0` and OTel monitoring was configured. This could hide runtime logging failure from both stderr and durable compatibility-log review. | Operational SRP/SOR cards for `#4634` in the primary checkout; merged PR [#4889](https://github.com/danielbaustin/agent-design-language/pull/4889) | Fixed in `adl/src/cli/observability.rs` by writing OTel sink failures to the compatibility log when `ADL_OBSERVABILITY_LOG` is configured, even when stderr is suppressed. Regression proof recorded: `cargo test --manifest-path adl/Cargo.toml observability::tests::otel_sink_failure_is_durable_when_stderr_is_suppressed --lib`. |
| WP07-F-002 | P2 | fixed | The #4718 observability proof harness wrote misleading retained artifact refs when invoked with a custom `OUT_DIR`; generated event samples still pointed at default generated paths. | `docs/milestones/v0.91.7/review/observability_4718/bounded_codex_review_4718.md` | Fixed before PR publication by deriving provider artifact refs, proof-summary paths, and event-sample paths from the requested `OUT_DIR`; the focused contract check asserts custom-output `<tmp>` refs. |
| WP07-F-003 | P2 | fixed | The #4718 proof summary used loose event-field matching, so a field such as `subcommand=doctor` could be counted as a real command and weaken retained command evidence. | `docs/milestones/v0.91.7/review/observability_4718/bounded_codex_review_4718.md` | Fixed before PR publication by parsing whitespace-delimited event key/value fields before building observed-command and observed-result sets; regenerated proof no longer records `doctor` as a command. |
| WP07-F-004 | boundary | integrated_proven with non-claims | Logging/OTel is now retained as integrated-proven within its stated local boundary, after earlier issue-local proof and Soak 2 consumption. That proof must not be inflated into production collector/exporter or hosted telemetry claims. | `docs/milestones/v0.91.7/review/observability_4718/INTEGRATED_LOGGING_OTEL_PROOF_4718.md`; `docs/milestones/v0.91.7/review/runtime/soak2_4682/soak2_execution_status_4682.json`; `docs/milestones/v0.91.7/review/runtime/WP07_PRE_V092_RUNTIME_COHERENCE_DISPOSITION_4845.md`; `docs/milestones/v0.91.7/review/runtime/FINAL_CSM_RUNTIME_COHERENCE_GATE_4906.md` | Preserve the proof boundary: stdout/stderr separation, redaction, event samples, daemon trace/span/service fields, local `ADL_OTEL_LOG` JSONL export, and `ADL_OTEL_STATUS` monitor status are proven; production collector, hosted backend, and broad release readiness are not claimed from this row. |
| WP07-F-005 | blocker | blocked_with_evidence | Final CSM runtime coherence remains blocked with evidence rather than passed. Unity/WP-09 live consumption, WP-12 protocol/security activation, and the v0.92 capability envelope still require retained integrated proof or explicit operator-approved non-claims/defer decisions. | `docs/milestones/v0.91.7/review/runtime/WP07_PRE_V092_RUNTIME_COHERENCE_DISPOSITION_4845.md`; `docs/milestones/v0.91.7/review/runtime/FINAL_CSM_RUNTIME_COHERENCE_GATE_4906.md`; `docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md` | Keep the `#4906` blocking rows visible. Consume WP-07 as closed umbrella/runtime progress, not as clean release readiness, until a follow-on disposition closes or explicitly defers the blocking rows. |

## Retained Proof And Non-Claims

The #4718 retained proof establishes parse-safe JSON stdout for the scoped
`pr.sh doctor 4718 --json --allow-open-pr-wave` path, human-oriented
`adl_event` observability on stderr and compatibility log, redaction/path
hygiene, and an OTel-compatible mapping boundary.

It does not claim:

- production OpenTelemetry collector coverage;
- OTLP exporter wiring;
- hosted telemetry service integration;
- Unity editor execution;
- final WP-07 release readiness.

## Review Conclusion

WP-07 has fixed code-review findings, including the runtime logging bug in
WP07-F-001, and it has retained integrated logging/OTel proof within a bounded
local observability contract. The remaining WP-07 finding state is not "no
findings"; it is "fixed/proven findings plus blocked release-readiness rows."
Release consumption must preserve that distinction.
