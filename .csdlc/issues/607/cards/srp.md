# Structured Review Prompt

Template: 1.0.0

Issue: 607

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/607
.csdlc/prepared/issues/607/design.md
.csdlc/prepared/issues/607/diagram.mmd
adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
adl-runtime/src/bin/adl-runtime-lifecycle-soak.rs
adl-runtime/src/guardian.rs
adl/tools/issue607_probe_runtime.py
adl/tools/issue607_qualify_warm_polis.sh
adl/tools/issue607_validate_saved_plan.sh
adl/tools/run_issue607_warm_polis.sh
adl/tools/test_issue607_warm_polis.sh
adl/tools/validate_v092_runtime_guardian_lifecycle.sh
docs/operations/cloud/aws/shepherd-gpu-proof/README.md
infra/aws/runtime/gpu-proof

## Prompts

- Can normal launch reach any compiler package manager Git mutable download or model pull path?
- Can Terraform destroy or a trap delete the persistent warm volumes?
- Are timing denominators complete and comparable?
- Can stale or cross-AZ volume content activate?
- Are #605 SSH private-Ollama IAM and cleanup invariants preserved?

## Findings

[
  {
    "id": "F-607-1",
    "severity": "p1",
    "summary": "The qualification path reports PASS even when the GPU guest exceeds the 30-second local-ready limit and controller-observed service readiness exceeds 120 seconds.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-607-2",
    "severity": "p1",
    "summary": "The paid-action controller accounts per storage generation and only records successful attempts, so retries and earlier campaigns are not proven to remain inside one issue-wide 20 USD envelope.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-607-3",
    "severity": "p2",
    "summary": "The corrected warm-volume teardown behavior has focused static proof but has not been exercised by a successful live AWS qualification run.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:7a622c99bd54d99fe0c28349e5ccbf7d683b6838:46acff62ac090a8731675357c71b9b0531c397f03bc7139ad532312948e7eb7a")

Reviewer: Some("fresh-session:9bf1aebb-3fcb-45fd-90dd-b1087f6feb00")

Result: changes_required
