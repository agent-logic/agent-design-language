# Issue #862 local command decomposition inventory

## Baseline

- Revision: `b0311902d77668d6f126d501fffe4c752232af87`
- Source: `csdlc-v3/src/commands/local/mod.rs`
- SHA-256: `d2cbdb97a2ebb6915151cc7bc4586550ead48d07be8ec1e8f4d82ddc2614ea54`
- Lines: `3256`
- Named top-level items found by the bounded inventory: `94`
- Existing sibling production modules: `intent.rs` only.
- Recursive production source before decomposition: `3607` lines (`mod.rs`
  plus the pre-existing `intent.rs`).

## Baseline responsibility map

1. Public request/result contracts and route enums.
2. Non-authoritative planning, registry parsing, contract validation, and route planning.
3. Lifecycle-state inspection and fixture initialization.
4. Operational context and authority validation.
5. Mutation locking, durable journals, staging, completion, and recovery.
6. Git worktree registration, creation, and bound-checkout verification.
7. Issue initialization and bind activation.
8. Typed card edit, template rendering, structure validation, and semantic projections.
9. Schedule, shepherd, doctor, eligibility, and operational route dispatch.
10. Atomic persistence, digest calculation, and filesystem error mapping.

## Invariants

- Public exports and serde shapes remain unchanged.
- Route names, result fields, error codes, paths, digests, idempotency, recovery, and fail-closed behavior remain unchanged.
- Dependency direction flows from route orchestration into cohesive owners and shared storage primitives; no new command-domain cycle is accepted.
- Focused local/native contract suites must pass with nonzero denominators.

## Implemented owners

- `planning.rs`: typed contract, registry planning, non-authoritative route planning.
- `lifecycle.rs`: lifecycle observation and fixture state.
- `transactions.rs`: durable mutation journal and recovery.
- `worktree.rs`: Git registration and bound-checkout verification.
- `binding.rs`: bind policy and activation.
- `cards.rs`: typed card editing, rendering, structure validation.
- `context.rs`: authority/root/CAS validation.
- `issue.rs`: create-only issue-state initialization.
- `routing.rs`: operational dispatch and route reports.
- `storage.rs`: atomic persistence and lifecycle digests.
- `intent.rs`: retained native intent and semantic-storage bridges.

## Dependency direction

The production module graph is ordered as follows. Every dependency points to
an earlier layer; lateral and upward command-domain dependencies are rejected
by `tests/local_module_decomposition.rs`.

1. `planning`, `storage`
2. `lifecycle`, `worktree`
3. `transactions`
4. `cards`, `context`
5. `binding`, `issue`
6. `intent`
7. `routing`

## Focused structural proof classification

| Test surface | PVF lane | Proof role | Determinism | Resources | Release gate |
| --- | --- | --- | --- | --- | --- |
| `tests/local_module_decomposition.rs` | tooling | Enforces a thin facade, explicit dependencies, the reviewed acyclic direction, and one production owner for each named responsibility. | Deterministic source contract | Local Rust compiler and repository bytes only | Required for #862 |

## Candidate inventory

- `local/mod.rs`: `341` lines, SHA-256
  `3638df71c9b8e5372b2504eff14be15fbb2114aac90a7b8093a70297c7fbd468`.
- Recursive production source after decomposition: `3790` lines across twelve
  files. The explicit imports and module responsibility documentation account
  for the modest recursive increase; the public facade is 2915 lines smaller.
- No production module uses `use super::*`; dependencies are declared by owner.

## Route and boundary proof

The exact candidate passed the following nonzero deterministic suites:

| Suite | Passed | Principal contract |
| --- | ---: | --- |
| `local_module_decomposition` | 2 | Thin facade, explicit acyclic dependency graph, one responsibility owner |
| `local_commands` | 36 | Issue/bind/edit/validate/doctor/schedule/shepherd/eligibility, cards, authority, CAS, digests, topology and negatives |
| `operational_cli_commands` | 19 | Installed command and retired-writer boundaries |
| `transactions` | 45 | Journals, recovery, CAS, idempotency, semantic-store boundaries |
| `foundation` | 13 | Repository context and read-only compatibility contracts |
| `proof_worktree_binding` | 1 | Installed legacy binding-writer retirement |
| `terminal_cleanup_cutover_commands` | 35 | Terminal, cleanup, cutover and worktree boundary compatibility |

Total: `151` passed, `0` failed, `0` ignored, `0` filtered out. Strict
all-target clippy and formatting also pass. This table records local proof only;
CI and independent review remain separate gates.

The broader `run_owner_validation_lane.sh csdlc` is not claimed as passing. It
passed its first 22 authority tests, command guidance, wrapper migration,
editor guidance, active-reference scan and metric backfill, then stopped in the
unchanged `test_card_prompt.sh`: that baseline test requires prompt-template
set `1.0.3`, while both this issue's baseline and `origin/main` select `1.0.5`.
Neither file is changed by #862; the mismatch requires separate ownership.
