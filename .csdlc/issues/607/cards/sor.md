# Structured Output Record

Template: 1.0.0

Issue: 607

Repository: agent-logic/agent-design-language

Card: sor

Status: draft

## Summary

Implemented and twice live-qualified the restart-safe warm two-node AWS Polis at 31a90eccf; Runtime readiness was 5.53s and 5.22s, both GPU nodes retained two models in 113.93s and 112.43s, and all six resident ACC tool cycles passed in both runs.

## Artifacts

- adl/tools/run_issue607_warm_polis.sh
- adl/tools/issue607_probe_runtime.py
- adl/tools/issue607_qualify_warm_polis.sh
- adl/tools/issue607_validate_saved_plan.sh
- adl/tools/test_issue607_warm_polis.sh
- infra/aws/runtime/gpu-proof
- docs/operations/cloud/aws/shepherd-gpu-proof/README.md
- .csdlc/evidence/607/local-validation-resume-b7b1ebd95.json
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime/src/bin/adl-runtime-lifecycle-soak.rs
- adl-runtime/src/guardian.rs
- adl/tools/run_issue607_warm_polis.sh
- adl/tools/issue607_probe_runtime.py
- adl/tools/issue607_qualify_warm_polis.sh
- adl/tools/issue607_validate_saved_plan.sh
- adl/tools/test_issue607_warm_polis.sh
- adl/tools/validate_v092_runtime_guardian_lifecycle.sh
- infra/aws/runtime/gpu-proof
- docs/operations/cloud/aws/shepherd-gpu-proof/README.md
- .csdlc/evidence/607/aws-warm-qualification-31a90eccf.json

## Execution

- Separated retained warm storage, disposable preparation, and disposable compute ownership.
- Loaded exact preparation output keys and made zero, one, and two existing AMI continuation create only missing images.
- Made sealed snapshots and terminal preparation completion idempotent at their bounded seams.
- Allowed an artifact generation to continue only under a clean descendant controller and recorded both identities in launch evidence.
- Waited indefinitely for healthy AWS transitions while failing immediately on API and terminal-state errors.
- Bound resume to consumed authorization, source, plans, Terraform state, owner, images, ledger, campaign, and terminal checkpoint.
- Bound destructive recovery to exact authorization, ledger, owner, campaign, generation, and owner-filtered AWS discovery.
- Made cost-ledger initialization atomic and rejected duplicate, malformed, arithmetically inconsistent, or wrong-input preparation entries.
- Prevented destructive recovery when a terminal preparation result exists and allowed only exact retained artifacts in residue proof.
- Separated retained warm storage, disposable preparation, and disposable compute ownership under exact Terraform plans and single-use campaign authorization.
- Prepared immutable Runtime and GPU AMIs plus integrity-bound persistent EBS volumes once, with no launch-time build, package installation, Git access, model download, or mutable dependency resolution.
- Kept Guardian and Runtime alive independently of qualification failures and retried external continuity readiness instead of terminating on transient Ollama unavailability.
- Added a private Runtime-to-GPU Ollama path exposed to the existing model test through a Runtime-local loopback bridge while keeping Ollama non-public.
- Executed two clean AWS launches with authenticated Runtime HTTPS and WSS, two resident GPU models, real Shepherd inference, and six governed ACC tool cycles.
- Removed contradictory pre-detach stop behavior for terminate-on-shutdown instances and made live EC2 resource APIs authoritative over stale deleted instance and volume entries in the AWS tagging index.
- Reconciled both run states to zero disposable compute residue while retaining the two warm volumes and four inexpensive recovery snapshots.
- Recorded a cumulative conservative campaign cost of USD 16.694191 under the authorized USD 20 ceiling.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_issue607_warm_polis.sh",
      "all"
    ],
    "purpose": "Prove bounded no-paid resume, exact destructive recovery identity, strict atomic cost reconciliation, controller ancestry, checkpoint reconciliation, and artifact reuse.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/607/local-validation-resume-b7b1ebd95.json"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject diff hygiene defects.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/607/local-validation-resume-b7b1ebd95.json"
  },
  {
    "command": [
      "bash",
      "-n",
      "adl/tools/run_issue607_warm_polis.sh",
      "adl/tools/test_issue607_warm_polis.sh"
    ],
    "purpose": "Prove shell parse validity.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/607/local-validation-resume-b7b1ebd95.json"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_issue607_warm_polis.sh",
      "all"
    ],
    "purpose": "Prove the final no-paid controller, Terraform, permanent Guardian, immutable preparation, launch, teardown, residue, and cost-envelope contracts after live remediation.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/607/aws-warm-qualification-31a90eccf.json"
  },
  {
    "command": [
      "bash",
      "adl/tools/run_issue607_warm_polis.sh",
      "launch",
      "--commit",
      "7be87dd22260d30a7966d1b129123e84bb761074",
      "--run-id",
      "adl-issue607-e8925c1dc8b0-launch-1-retry-1",
      "--storage-id",
      "adl-issue607-warm-v6",
      "--ordinal",
      "1",
      "--execute"
    ],
    "purpose": "Prove the first clean warm AWS launch, real two-model Shepherd inference, six resident ACC tool cycles, Guardian resilience assertions, and zero final live compute residue.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/607/aws-warm-qualification-31a90eccf.json"
  },
  {
    "command": [
      "bash",
      "adl/tools/run_issue607_warm_polis.sh",
      "launch",
      "--commit",
      "7be87dd22260d30a7966d1b129123e84bb761074",
      "--run-id",
      "adl-issue607-e8925c1dc8b0-launch-2",
      "--storage-id",
      "adl-issue607-warm-v6",
      "--ordinal",
      "2",
      "--execute"
    ],
    "purpose": "Prove repeatable clean warm AWS startup with the same sealed AMIs and volumes, real two-model and six-agent ACC qualification, and zero final live compute residue.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/607/aws-warm-qualification-31a90eccf.json"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "guardian",
      "--lib"
    ],
    "purpose": "Prove the Guardian retry and supervision behavior changed by this issue; 22 focused tests passed with zero failures.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/607/aws-warm-qualification-31a90eccf.json"
  }
]

## Integration

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
