---
name: csdlc-v2-validate
description: Execute declared PVF validation DAGs with typed evidence.
---
Use `csdlc-schedule` for read-only classification. For routine issue execution, use `csdlc-validate --root <worktree> finalize --request <shared-git-request>` so execution, passing validation, and `Implemented` are one atomic state transition. Keep the one-shot request at the Git-common path `.git/csdlc-v2/requests/<issue>.json` and overwrite it. Do not embed shell command strings or treat skipped/pending proof as passed.

Do not call deleted ADL structured-prompt shell wrappers such as
`adl/tools/validate_structured_prompt.sh`. Bootstrap/review flows that need card
validation must initialize or load typed v2 issue state, then run
`csdlc-validate`.

Rust validation lanes must declare their Cargo target boundary explicitly.
Use forms such as `cargo test --manifest-path csdlc-v2/Cargo.toml --lib
schema::tests` or `cargo test --manifest-path csdlc-v2/Cargo.toml --test
gate2`. A bare `cargo test --manifest-path ...` is a truthful broad lane; a
trailing free substring such as `cargo test ... schema` is not a named target
and is rejected because it can fan out into unrelated integration binaries.
