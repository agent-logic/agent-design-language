# Structured Output Record

Template: 1.0.0

Issue: 5778

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Preserved the reviewed finish implementation and repaired the current-main Runtime API formatting regression tracked by #5783.

## Artifacts

- adl-runtime/src/runtime_api.rs
- .csdlc/evidence/5778/post-finalize-remediation.md

## Execution

- Merged current main so local validation matches GitHub's pull-request merge tree.
- Applied current stable rustfmt to the Runtime API endpoint inventory introduced by #5781.
- Retained the #5778 finish implementation unchanged while routing the separate defect through #5783.

## Validation

[
  {
    "command": [
      "cargo",
      "+stable",
      "fmt",
      "--all",
      "--",
      "--check"
    ],
    "purpose": "Reproduce and prove the repository formatter gate on the exact current-main merge tree from the adl workspace.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5778/post-finalize-remediation.md"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "../adl-runtime/Cargo.toml",
      "runtime_api_contract_advertises_only_served_routes",
      "--locked"
    ],
    "purpose": "Confirm the formatted endpoint inventory retains the Runtime API contract introduced by #5781.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5778/post-finalize-remediation.md"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
