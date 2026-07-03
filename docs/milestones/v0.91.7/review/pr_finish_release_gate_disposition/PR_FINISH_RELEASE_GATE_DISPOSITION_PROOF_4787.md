# PR Finish Release-Gate Disposition Proof (#4787)

## Summary

`pr finish` now accepts a first-class `--release-gate-disposition <repo-relative-path>` argument for validation-manager profiles that require `release_gate_review` disposition before PR publication.

## Contract

A release-gate disposition is publishable only when it is:

- repo-relative
- tracked or staged for publication
- valid YAML/JSON
- tied to the same issue number as the finish command
- explicitly covers every release-gate matched surface reported by the validation manager
- includes a non-blocking disposition/decision
- includes reviewer/review-mode, focused-validation, and residual-CI proof fields
- free of machine-local absolute path markers

Missing, malformed, blocked, rejected, or unrelated disposition files fail closed.

## Validation

Focused validation run locally:

- `cargo test --manifest-path adl/Cargo.toml --bin adl parse_finish_args_requires_title_and_accepts_finish_flags -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml --bin adl release_gate_disposition_ -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish adl_pr_finish_ -- --nocapture`

## Non-claims

This issue does not make every validation-manager lane publishable. The broader registered multi-command publication surface remains routed through `#4815` where applicable.
