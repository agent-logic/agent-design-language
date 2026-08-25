# Rust Resilience Owner-Boundary Refactoring

RUST-01 is one behavior-preserving refactoring slice over the current `adl/src/resilience.rs` ownership surface. It extracts cohesive resilience owners into an explicit module family while preserving supported APIs, failure taxonomy, retries, timeouts, cancellation, traces, and focused behavioral proof.

The goal is a narrower change and validation-impact surface, not file splitting for appearance and not a mandatory line-count reduction. The issue records the exact pre/post module and validation denominators, keeps every test PVF-classified, and stops if the work requires behavior changes or expands into unrelated Rust surfaces.

Repository-wide refactoring, Runtime v4, aesthetic cleanup, and arbitrary LoC targets are excluded.
