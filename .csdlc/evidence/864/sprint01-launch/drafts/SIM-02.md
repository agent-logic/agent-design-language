# [v0.92.2][C-SDLC v3][SIM-02] Make the installed command contract consistent and discoverable

## One complete result

One current descriptor-backed command contract makes installed help, accepted input schemas, effect classifications, result envelopes, source routing, current documentation and conformance tests agree. Ordinary operational errors cannot silently select historical construction semantics.

Dependency: SIM-01 merged and proven. Coordinates under SIM-UMBRELLA; hands its descriptor, provenance and result contract to SIM-03. This is executable contract reconciliation, not a documentation-only inventory.

## Current evidence and owned paths

The launch primary installed binary's `--help` omitted `release-preflight`, while `csdlc-v3/src/main.rs` and `docs/csdlc-v3/v3-command-manifest.json` include it. Source dispatches a typed guarded `rollback` alias not listed in root help or the 26-command manifest. These are observed source/installed discrepancies, not proof of a new runtime failure; resolve installed provenance before attributing cause.

Own `csdlc-v3/src/main.rs`, `csdlc-v3/src/commands/mod.rs`, route descriptor/schema glue adjacent to it, `docs/csdlc-v3/v3-command-manifest.json`, relevant current schemas in `docs/csdlc-v3/`, `csdlc-v3/tests/command_manifest.rs`, `csdlc-v3/tests/operational_cli_commands.rs` and `csdlc-v3/tests/proof_parity_install_commands.rs`. Update only contract-relevant guidance in `csdlc-v3/AGENTS.md`, `csdlc-v3/README.md`, and `docs/csdlc-v3/CURRENT_AUTHORITY.md`. Coordinate installation/provenance checks with `adl/tools/install_owner_binaries.sh`; do not replace the live binary. Avoid taking over #861's complete man-page deliverable or #862's decomposition.

## Acceptance

1. The descriptor covers every frozen current command and explicit alias disposition in the SIM-03 command inventory. Installed candidate help, route recognition, required inputs, effect class and result schema derive from or are exhaustively checked against that single contract. No command disappears through a denominator change.
2. Cover all 26 manifest commands plus guarded `rollback`. Distinguish currently operational routes, read-only helpers, historical proof/construction and administrative routes. Put retained construction/import/simulation under explicit proof/administrative discovery; preserved historical artifacts do not advertise alternate operational authority.
3. Operational authority, context, parse, stale request or discovery failures fail closed. They cannot call a construction-report fallback and present it as successful operational work. An explicit historical/proof invocation remains available only under its declared non-operational contract. Prove the distinction at public installed entrypoints, including genuine non-primary worktrees.
4. A versioned result envelope consistently represents command, issue or explicit issue-not-applicable, status, authority status, issue version, effects, findings and allowed next operations. `blocked`, failed, expected no-op and recovery-required remain distinct; output never reports observation after a mutation. Preserve exit/status semantics and honest partial outcomes.
5. Isolated installation records source revision, build identity and installed digest. Help/schema/result checks invoke that executable rather than a different target-directory binary. Demonstrate mismatch rejection using stale installed-help/provenance and omitted-route fixtures; retain baseline and corrected observations.
6. Extend SIM-01's journey corpus with install/help/schema/prepared-start, draft-to-ready/uncertain-remote response and terminal-readback/result/cleanup cases. Existing typed review, authority, cleanup and publication linkage checks remain enforced. stdout/stderr separation, secret redaction and compatibility logging pass.

## Validation and PVF

Required deterministic local CPU/Rust/Git contract and installed-public-journey proof; fake authenticated remote transport only. Run focused command-manifest, operational CLI and proof/parity installation tests, plus fixture negatives for omitted descriptor, stale installed binary, help/schema/effect mismatch and forbidden fallback. `cargo test --manifest-path csdlc-v3/Cargo.toml --test command_manifest`, `--test operational_cli_commands`, `--test proof_parity_install_commands`, and focused terminal/remote tests when their contract changes; run Cargo formatting. Classify each new fixture's role, determinism/resources and required issue/SIM-07 release-gate status. Passing text comparison alone is insufficient: actual installed routes and negative operational errors must execute.

## Exclusions and stop conditions

No replacement intent CLI implementation owned by SIM-03, semantic transaction migration owned by SIM-04, broad manuals/decomposition, historical evidence rewriting or live activation. Stop on authority ambiguity, ownership collision, incompatible contract not explicitly classified, or missing installed proof. Record unresolved source-versus-installed provenance honestly; do not rebuild the active binary to erase baseline evidence.

## Authority and execution boundary

This is one implementation task in the first v0.92.2 C-SDLC simplification sprint. Use native C-SDLC v3, the current selector/receipt guards, an issue-bound FastWork worktree and an issue-bound session goal. Re-resolve current source and active owners before edits. Root main stays inspection-only. Resolve overlap with CSDLC-MAN/#861, CSDLC-DECOMPOSE/#862, CSDLC-REMOTE and other SIM workers before shared-path edits. Preserve all historical evidence bytes.

This issue authorizes implementation and isolated candidate proof, not coordinated live writer activation, state conversion, replacement of the active operator binary, real GitHub effects, Runtime/provider shutdown, paid cloud/provider execution or a second live writer. The breaking replacement activates only through the separately authorized transition/pilot after SIM-06/07/08. Installation for proof means an isolated fixture-local candidate built from the exact reviewed source. Never work around an authority or stale-review failure.

## Source contract

- `docs/milestones/v0.92.2/cognitive-sdlc/C_SDLC_V3_SIMPLIFICATION_PLAN.md`
- `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`
- `docs/milestones/v0.92.2/ATOMIC_TASK_CONTRACTS_v0.92.2.md`
- `csdlc-v3/AGENTS.md` and root `AGENTS.md`

Launch inspection baseline: `ace209ad9c701a855d165da817164ff60a755203`. This is source inspection, not execution proof or a promise that defects survive to the implementation baseline. Retain source/candidate revisions, installed binary digest/provenance and clean fixture identity with every run.
