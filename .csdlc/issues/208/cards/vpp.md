# Validation Planning Prompt

Template: 1.0.0

Issue: 208

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/208/design.md

Diagram: .csdlc/prepared/issues/208/diagram.mmd

## Selected Lanes

[
  {
    "lane": "guardian-continuity-production",
    "proof_role": "Prove production Guardian initialization, supervised-kernel private-session readiness, durable channel restart/certificate succession, public-route absence, sealed #210 client ports, and the client half of the exact fifty-six cases plus eight-row sixty-four-subassertion map, including the domain row's exact RFC 8785 markers.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1500,
    "budget_tokens": 10000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "kernel_continuity_client",
      "--no-tests=fail"
    ],
    "parallel_group": "208-serial-01-runtime-test",
    "defer_reason": "Deferred until #191 merges and the complete production client target exists; fail closed on missing target, zero tests, library-only reachability, generic/caller-constructible #210 ports, or missing, extra, duplicate, reordered, nonpassing case/result/marker/subassertion."
  },
  {
    "lane": "kernel-continuity-production",
    "proof_role": "After the Guardian lane, prove production kernel startup, sealed complete live-participant quiesce/export and rollback, isolated validation/discard, signed expected entry/chunk/range revalidation before #210 stage writes, discard-only cleanup survival after transfer expiry/cancel, filesystem/bounds safety, and the server half of all fifty-six cases and sixty-four mapped subassertions.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1500,
    "budget_tokens": 10000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "kernel_continuity_control",
      "--no-tests=fail"
    ],
    "parallel_group": "208-serial-02-kernel-test",
    "defer_reason": "Deferred until the private listener, participant registry, sealed #210 ports, and focused kernel target exist; fail closed on missing target, synthetic participants, caller descriptors, cleanup-authority expiry with transfer authority, zero tests, or any case/map parity drift."
  },
  {
    "lane": "guardian-continuity-clippy",
    "proof_role": "After both focused tests, reject warnings and API misuse across the Runtime library, Guardian production binary, client integration test, and sealed #210 client projections.",
    "acceptance_ids": [
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "--bin",
      "adl-runtime-guardian",
      "--test",
      "kernel_continuity_client",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "208-serial-03-runtime-clippy",
    "defer_reason": "Deferred until the owned Runtime targets exist; fail closed on warnings, missing target, or missing production source."
  },
  {
    "lane": "kernel-continuity-clippy",
    "proof_role": "After Runtime Clippy, reject warnings and API misuse across the kernel library, production kernel binary, integration test, expected-descriptor verifier, and persistent cleanup permit.",
    "acceptance_ids": [
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "--bin",
      "adl-runtime-kernel",
      "--test",
      "kernel_continuity_control",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "208-serial-04-kernel-clippy",
    "defer_reason": "Deferred until the owned kernel targets exist; fail closed on warnings, missing target, or missing production source."
  },
  {
    "lane": "continuity-diff-hygiene",
    "proof_role": "After all source-changing validation, load the recorded execution base and proving source revisions, require both exact Git objects and base ancestry, and run diff whitespace/EOF hygiene over the complete base..source range plus reject dirty protected paths. A working-tree-only diff is insufficient.",
    "acceptance_ids": [
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/208/verify-diff-hygiene.rb"
    ],
    "parallel_group": "208-serial-05-diff",
    "defer_reason": "Deferred until implementation records exact execution_base_revision and proving_source_revision and creates the verifier; fail closed on absent/non-object revisions, nonancestry, any base..source whitespace or EOF diagnostic, dirty protected source, or working-tree-only evidence."
  },
  {
    "lane": "guardian-kernel-continuity-producer",
    "proof_role": "After tests, both Clippy packages and exact-range diff hygiene pass, produce exact Git/source/base/argv/stream/timing/protected-digest evidence for fifty-six cases and byte-for-byte parity with continuity-boundary-subassertion-map.json sha256=cc7a0f9cb8e09840bb977f88a8d1721e0f04348beefca2cfbb6a33a6b4b15ef0, boundary_row_count=8, subassertion_count=64, including exact RFC 8785 and sealed-port/cleanup proof.",
    "acceptance_ids": [
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 8000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/208/produce-proof-receipt.rb"
    ],
    "parallel_group": "208-serial-06-producer",
    "defer_reason": "Deferred until prior serial lanes pass and the exact producer exists; fail closed on dirty protected source, wrong base/source diff proof, map digest/count/order drift, missing/extra/duplicate/nonpassing evidence, or nonzero status."
  },
  {
    "lane": "guardian-kernel-continuity-receipt",
    "proof_role": "Only after producer completion and fresh independent exact-head review, independently validate execution-base-to-reviewed-source diff hygiene, exact protected source, argv, fifty-six cases, eight rows, sixty-four mapped subassertions, both Clippy lanes, immutable evidence introduction, review provenance, and squash-merge-safe proof truth.",
    "acceptance_ids": [
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/208/validate-proof-receipt.rb"
    ],
    "parallel_group": "208-serial-08-validator",
    "defer_reason": "Deferred until producer, typed finalize, immutable evidence, and fresh review exist; fail closed until exact source, execution base, map digest/count/order, cases, subassertions, commands, diff range, and review are bound."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo nextest run --locked --manifest-path adl-runtime/Cargo.toml --test kernel_continuity_client --no-tests=fail`
- `cargo nextest run --locked --manifest-path adl-runtime-kernel/Cargo.toml --test kernel_continuity_control --no-tests=fail`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --lib --bin adl-runtime-guardian --test kernel_continuity_client -- -D warnings`
- `cargo clippy --locked --manifest-path adl-runtime-kernel/Cargo.toml --lib --bin adl-runtime-kernel --test kernel_continuity_control -- -D warnings`
- `ruby .csdlc/prepared/issues/208/verify-diff-hygiene.rb`
- `ruby .csdlc/prepared/issues/208/produce-proof-receipt.rb`
- `ruby .csdlc/prepared/issues/208/validate-proof-receipt.rb`

## Failure Semantics

Fail closed on incomplete production wiring, unsafe listener/root/TLS, noncanonical frame, wrong identity/domain/polis/node/epoch/generation/prefix, replay/conflict/reorder, incomplete participant registry, partial quiesce without reconciled resume, stale certificate new work, synthetic or corrupt snapshot, activated-target discard, capacity/path/size/open-handle/residue drift, public-route access, evidence/source drift, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
