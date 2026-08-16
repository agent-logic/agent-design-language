# Validation Planning Prompt

Template: 1.0.0

Issue: 210

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/210/design.md

Diagram: .csdlc/prepared/issues/210/diagram.mmd

## Selected Lanes

[
  {
    "lane": "continuity-transfer",
    "proof_role": "Prove byte-for-byte parity with continuity-transfer-acceptance-map.json sha256=2929794678966f233f8caf4df3131d9188cac3e5107fc0190cee9dd4fd1d71cd, including the canonical ordered 45-case manifest with exact pass result and unique marker for every case, acceptance_count=8, subassertion_count=84, every case has at least one directly mapped subassertion, including exact_retry_cached, wrong_polis_denied, wrong_domain_denied, and generic_send_denied, AC-3 conflicting_duplicate_denied membership, and the explicit machine assertion that #210 has no activation or deletion authority. The map covers every AC and all 45 cases, including live route/membership/certificate/boot drift, canonical framing/final/range bounds, signed catalog entry/chunk/range expectations, bounded queues/readers/stages/transfers/journals/caches, crash-reconcilable bytes/verifier/prefix order, cleanup after expiry/cancel/restart, #208 live zero residue, and strict proof sequencing. Reject missing, extra, duplicate, reordered, renamed, wrongly mapped, wrongly marked, nonpassing, or authority-widening case/subassertion evidence.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 24000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_continuity_transfer",
      "--no-tests=fail"
    ],
    "parallel_group": "210-serial-01-runtime",
    "defer_reason": "Deferred until #202, then #199, then #203 are independently reviewed, merged, and ancestral; after all three merges #210 must resync to the resulting exact origin/main and pass typed csdlc-validate issue plus csdlc-doctor before bind. Fail closed on missing target, zero tests, map digest/count/order/result/marker drift, caller-substitutable authority/expectations, any case/subassertion mismatch, #208 effect-ownership widening, or #204 decision-ownership widening."
  },
  {
    "lane": "continuity-transfer-clippy",
    "proof_role": "Only after the focused proof passes, reject warnings and API misuse across the transfer session, sealed #201/#208 ports, signed incremental verifier, cleanup requests, and focused target.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 10000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_continuity_transfer",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "210-serial-02-clippy",
    "defer_reason": "Deferred until the exact serial prerequisite and post-merge resync gate passes, the owned focused target exists, and serial test proof passes; fail closed on warnings, missing target, missing source, or authority-boundary drift."
  },
  {
    "lane": "continuity-transfer-diff-hygiene",
    "proof_role": "Only after tests and Clippy, load recorded execution_base_revision and proving_source_revision, require exact Git objects and base ancestry, check whitespace and EOF over the complete base..source range, and reject dirty protected paths. Working-tree-only diff output is insufficient.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/210/verify-diff-hygiene.rb"
    ],
    "parallel_group": "210-serial-03-diff",
    "defer_reason": "Deferred until the serial prerequisite/resync gate passes and exact execution base/source truth plus the verifier exist; fail closed on absent/non-object revisions, nonancestry, any base..source whitespace/EOF diagnostic, dirty protected source, working-tree-only evidence, or uncoordinated #205/#210 distributed/mod.rs landing overlap."
  },
  {
    "lane": "continuity-transfer-producer",
    "proof_role": "Only after tests, Clippy, and exact-range diff pass, independently load and hash the protected acceptance map, require exact ordered 45-case result/marker manifest and 8-row/84-subassertion parity, prove the corrected AC-3 conflict mapping and absence of #210 activation/deletion authority, and produce exact Git execution-base/source, command, stream, timing, protected-digest, signed expectation, resource, cleanup, and result evidence for independent review.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 10000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/210/produce-proof-receipt.rb"
    ],
    "parallel_group": "210-serial-04-producer",
    "defer_reason": "Deferred until the serial prerequisite/resync gate and all prior serial lanes pass and the exact producer exists; fail closed on dirty protected source, wrong map digest/count/order/result/marker, missing/extra/duplicate/nonpassing evidence, authority widening, wrong signed expectations, wrong cleanup authority, or nonzero status."
  },
  {
    "lane": "continuity-transfer-receipt",
    "proof_role": "Only after producer completion, immutable evidence introduction, and fresh independent review of the exact proving source, independently validate execution-base-to-reviewed-source diff hygiene, protected digests, commands, the canonical ordered 45-case result/marker manifest, all 8 acceptance rows and 84 unique subassertions, corrected AC-3 membership, #210 nonactivation/nondeletion authority, strict Clippy, signed expectation and cleanup bindings, review provenance, evidence immutability, and squash-merge-safe ancestry.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 5000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/210/validate-proof-receipt.rb"
    ],
    "parallel_group": "210-serial-06-validator",
    "defer_reason": "Deferred until the serial prerequisite/resync gate, producer, typed finalize, immutable evidence, fresh exact-head review, and any required post-#205 shared-mod resync exist; it must not overlap any prior lane or review and fails on any source/base/map/case/subassertion/order/result/marker/authority/command/review/ancestry drift."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo nextest run --locked --manifest-path adl-runtime/Cargo.toml --test distributed_continuity_transfer --no-tests=fail`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_continuity_transfer -- -D warnings`
- `ruby .csdlc/prepared/issues/210/verify-diff-hygiene.rb`
- `ruby .csdlc/prepared/issues/210/produce-proof-receipt.rb`
- `ruby .csdlc/prepared/issues/210/validate-proof-receipt.rb`

## Failure Semantics

Fail closed on invalid token/session/route/certificate/boot/cut, generic or confused dispatch, wrong source/target/lineage/polis/domain, bad frame/order/digest/manifest, bound overflow, deadline, cancellation ambiguity, prefix/result drift, rollback, disk-full, unsafe path, residue, evidence leak, source drift or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
