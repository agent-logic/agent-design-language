# Rust Resilience Owner-Boundary Refactoring

RUST-01 is one behavior-preserving refactoring slice over the current `adl/src/resilience.rs` ownership surface. It extracts cohesive resilience owners into an explicit module family while preserving supported APIs, failure taxonomy, retries, timeouts, cancellation, traces, and focused behavioral proof.

The goal is a narrower change and validation-impact surface, not file splitting for appearance and not a mandatory line-count reduction. The issue records the exact pre/post module and validation denominators, keeps every test PVF-classified, and stops if the work requires behavior changes or expands into unrelated Rust surfaces.

Repository-wide refactoring, Runtime v4, aesthetic cleanup, and arbitrary LoC targets are excluded.

## Source recommendation dispositions

RUST-01 deliberately owns only the resilience owner-boundary slice. The other independently finishable recommendations from the retained Rust review remain outside this issue and are routed to the v0.93 planning intake rather than hidden inside one refactoring issue:

| Recommendation | Disposition | Target | Rationale |
|---|---|---|---|
| Tracing consolidation | Deferred | v0.93 planning intake | Cross-cutting observability behavior needs its own denominator and compatibility proof. |
| Enum derive normalization | Deferred | v0.93 planning intake | Mechanical derive work is independent of resilience ownership and should carry its own API and serialization review. |
| HTTP middleware consolidation | Deferred | v0.93 planning intake | Transport middleware has separate retry, timeout, and compatibility behavior. |
| Secret hygiene hardening | Deferred | v0.93 planning intake | Security-sensitive handling requires a dedicated threat and redaction proof boundary. |
| Canonical JSON signing | Deferred | v0.93 planning intake | Signing canonicalization is an authority and compatibility change, not a resilience refactor. |
| Streaming substrate consolidation | Deferred | v0.93 planning intake | Streaming ownership spans Runtime and transport surfaces and requires a separate migration contract. |

These are explicit planning dispositions, not completion claims or newly created issues.
