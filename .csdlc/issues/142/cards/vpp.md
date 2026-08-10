# Validation Planning Prompt

Template: 1.0.0

Issue: 142

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/142/design.md

Diagram: .csdlc/prepared/issues/142/diagram.mmd

## Selected Lanes

[
  {
    "lane": "three-voter-runtime-contract",
    "proof_role": "Prove real process launch, three-voter quorum, single-Observatory coherent projection, snapshot eligibility, fencing, activation, stale-node denial, bounds, restart, and teardown with nonzero focused tests.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6",
      "AC-9",
      "AC-10",
      "AC-11",
      "AC-12",
      "AC-13",
      "AC-14",
      "AC-17"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 3600,
    "budget_tokens": 16000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_runtime_operational",
      "--no-tests=fail"
    ],
    "parallel_group": "local-contract",
    "defer_reason": "The issue-owned production integration target is an implementation deliverable and must exist before execution validation."
  },
  {
    "lane": "phase-a-wuji-three",
    "proof_role": "Run exactly three Wuji voters and one polis Observatory, prove convergence, governed action, one-voter failure behavior, recovery, and complete resource release.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-14",
      "AC-15",
      "AC-16"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 3600,
    "budget_tokens": 16000,
    "argv": [
      "bash",
      "adl/tools/run_v092_distributed_runtime_observatory_demo.sh",
      "--phase",
      "wuji-three"
    ],
    "parallel_group": "exclusive-live-serial",
    "defer_reason": "Runs only after reviewed implementation exists; its cleanup receipt is the hard prerequisite for Phase B."
  },
  {
    "lane": "phase-b-wuji-aws-recovery",
    "proof_role": "After Phase A cleanup only, run one Wuji and two AZ-separated AWS voters with pinned private local models on all three nodes; govern-admit the distinct AWS shepherd, saturate inference while proving consensus timing, commit a Wuji-proposed snapshot boundary, prove both AWS indices and identical canonical per-voter snapshots, serially compact/restart both AWS voters from snapshot roots, asymmetrically partition live Wuji with no harness bridge, elect an AWS consensus leader with both AWS votes, prove old Observatory lease TTL expiry and deny stale reads plus premature takeover, durably fence and activate owner then shepherd, acquire one Observatory lease, continue mutation, heal and prove Wuji demotion/synchronization, re-partition Wuji, stop the opposite-AZ AWS voter while the shepherd host survives and prove one-of-three halt, then tear down all resources.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10",
      "AC-11",
      "AC-12",
      "AC-13",
      "AC-14",
      "AC-15",
      "AC-16"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 7200,
    "budget_tokens": 32000,
    "argv": [
      "bash",
      "adl/tools/run_v092_distributed_runtime_observatory_demo.sh",
      "--phase",
      "wuji-aws-three-recovery"
    ],
    "parallel_group": "exclusive-live-serial",
    "defer_reason": "Requires a proven Phase A cleanup receipt, verified agent-logic-admin business-account identity, and reviewed private AWS connectivity."
  },
  {
    "lane": "exact-plan-runtime-and-receipt-review",
    "proof_role": "Validate serial non-overlap, exact argv/source/config, shepherd and model identity, native snapshot/committed-prefix provenance, durable quorum transitions, Observatory lease, redaction, AWS account/AZ identity, local-model teardown, merge-compatible receipt topology, protected-source drift denial, Gemini disposition, and independent implementation-review provenance.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10",
      "AC-11",
      "AC-12",
      "AC-13",
      "AC-14",
      "AC-15",
      "AC-16",
      "AC-18",
      "AC-19"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 8000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/142/validate-proof-receipt.rb"
    ],
    "parallel_group": "post-demo",
    "defer_reason": "The issue-owned receipt validator is authored and independently reviewed with the implementation."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 21600

Tokens: 100000

## Commands

- `cargo nextest run --manifest-path adl-runtime/Cargo.toml --test distributed_runtime_operational --no-tests=fail`
- `bash adl/tools/run_v092_distributed_runtime_observatory_demo.sh --phase wuji-three`
- `bash adl/tools/run_v092_distributed_runtime_observatory_demo.sh --phase wuji-aws-three-recovery`
- `ruby .csdlc/prepared/issues/142/validate-proof-receipt.rb`

## Failure Semantics

Fail closed on singleton substitution, duplicate identity or state, authority drift, incoherent Observatory cut, public/plaintext exposure, wrong AWS account, phase overlap, incomplete cleanup, invalid exact proof, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
