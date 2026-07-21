# Incumbent v1 Coverage Map

The machine-authoritative map is the `required_behaviors`, per-case
`behaviors`, and `coverage` sections of `corpus.yaml`.

| Surface | Cases | Proof |
|---|---|---|
| CLI identity | `cli-help`, `cli-version` | help/version output and repeated byte stability |
| six primitives | `six-primitives-plan` | provider, tool, agent, task, workflow, and run resolve together |
| projections | `graph-json`, `prompt-projection` | typed graph JSON and rendered prompts |
| graph ordering | `fork-join-ordering`, `map-order-a/b`, `branch-order-a/b`, `sequential-order-a/b` | fork/join order, map/branch equivalence, sequential difference |
| parser and schema failures | `invalid-argument`, `malformed-yaml`, `schema-error` | nonzero exits and stable diagnostics |
| reference failures | `unknown-provider`, `unknown-agent`, `unknown-task`, `unknown-tool`, `unknown-workflow`, `unknown-run-reference`, `missing-state` | reference-specific nonzero exits and diagnostics |
| graph failure | `dependency-cycle` | state-derived cycle rejection |
| local execution | `local-mock-run` | credential-free deterministic local run |
| signing | `ed25519-sign-verify-tamper` | fixed Ed25519 sign, verify, then tamper rejection |

All 25 cases run three times. The retained report therefore covers 75
independent incumbent process executions.
