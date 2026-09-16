# Issue #907 remote owner decomposition inventory

## Baseline

- Source revision: `40b756aeea5eaac10b5dc4940614ded13ae21275`.
- `csdlc-v3/src/commands/remote/mod.rs`: 4,046 lines, SHA-256 `29d4dc90b0c1287e58ef4681c7d7f46442d032818d43ec1d47c0ff63f28985b9`.
- Recursive remote source: 9,816 lines: 6,020 production lines and 3,796 retained test lines.
- Existing siblings retained: `intent.rs` (886 lines), `merge.rs` (885), `merge_linkage.rs` (203), `tests.rs` (2,852), and `tests/merge_cases.rs` (944).
- The facade owned public models, review/publication admission, canonical authority verification, operational routing, mutation staging and execution, authenticated transport, durable storage, reconciliation, and delivery classification in one file.

## Candidate

The production owner is 6,194 lines across cohesive modules. The small increase over the baseline comes from explicit module boundaries, imports, and module documentation. The two retained test files remain byte-for-byte unchanged.

| Module | Lines | Responsibility |
| --- | ---: | --- |
| `mod.rs` | 48 | Stable public re-exports and test-only compatibility aliases |
| `model.rs` | 547 | Public request, receipt, result, merge, and dispatch contracts |
| `support.rs` | 159 | Stable digests, shared validation, Git-control resolution, and adapter constants |
| `storage.rs` | 289 | Durable intent, receipt, and recovery storage |
| `authority.rs` | 119 | Canonical selector loading and authority verification |
| `delivery.rs` | 380 | Verified review-to-publication delivery and terminal classification |
| `merge_linkage.rs` | 213 | Qualified issue linkage and merge observation query |
| `transport.rs` | 865 | Credential-scoped GitHub transport, response validation, and reconciliation |
| `publication.rs` | 867 | Review, publication, and authenticated PR-readback admission |
| `merge.rs` | 828 | Merge eligibility, durable intent, dispatch, and replay |
| `mutation.rs` | 888 | Typed mutation staging, admission, and orchestration |
| `routing.rs` | 91 | Operational route dispatch after authority admission |
| `intent.rs` | 900 | Retained intent observation, recovery, and high-level orchestration |

No production module exceeds 1,000 lines. `mod.rs` contains no behavior implementation.

## Route and owner mapping

| Route or contract | Production owner |
| --- | --- |
| `review`, `publish`, `pr-state`, remote receipt loading | `publication.rs` |
| Operational `github`, `github-issue`, `github-pr` dispatch | `routing.rs` |
| Mutation admission, staging, execution, and replay | `mutation.rs` |
| GitHub invocation, credential scoping, response validation, authenticated readback | `transport.rs` |
| Merge admission, intent, execution, and replay | `merge.rs` |
| Closing and part-of linkage policy | `merge_linkage.rs` |
| Canonical v3 selector and exact-review authority | `authority.rs` |
| Durable mutation intent, receipt, and recovery paths | `storage.rs` |
| Public serialized contracts and defaults | `model.rs` |
| Review-to-publication delivery classification | `delivery.rs` |
| Pending operation observation and explicit recovery routes | `intent.rs` |

## Dependency direction

`remote_module_decomposition.rs` enforces this strict rank order and rejects lateral or upward production dependencies:

1. `model`
2. `support`
3. `authority`, `delivery`, `merge_linkage`, `storage`
4. `publication`, `transport`
5. `merge`
6. `mutation`
7. `routing`
8. `intent`

The guard also requires module documentation, rejects implicit `use super::*` production dependencies, limits every production module to 1,000 lines, limits the facade to 100 lines, and binds one representative responsibility symbol to each owner.

## Local proof

| Proof | Result | PVF classification |
| --- | --- | --- |
| `cargo test --manifest-path csdlc-v3/Cargo.toml --test remote_module_decomposition` | 3 passed | contract/architecture guard with alternate-import negative fixtures; deterministic; bounded CPU/disk; required gate |
| `cargo test --manifest-path csdlc-v3/Cargo.toml --lib commands::remote` | 65 passed | unit and controlled fake remote transport; deterministic; bounded CPU/disk; required gate |
| `cargo test --manifest-path csdlc-v3/Cargo.toml --test remote_publication_commands` | 13 passed | CLI/contract integration; deterministic; bounded CPU/disk; required gate |
| `cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands` | 19 passed | operational CLI integration; deterministic; bounded CPU/disk; required gate |
| `cargo test --manifest-path csdlc-v3/Cargo.toml --test terminal_cleanup_cutover_commands` | 35 passed | terminal/cutover regression; deterministic; bounded CPU/disk; support gate |

No live remote write was used. Required CI remains integration proof for the independently reviewed candidate.
