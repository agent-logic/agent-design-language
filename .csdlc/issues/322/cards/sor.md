# Structured Output Record

Template: 1.0.0

Issue: 322

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Repair #5913 adl-review compatibility routing, PR-fast CI selector routing, fenced-code-safe review contract validation, and long-fence delimiter handling.

## Artifacts

- adl/src/cli/mod.rs
- adl/tools/test_adl_review_compatibility.sh
- adl/tools/run_pr_fast_test_lane.sh
- adl/tools/test_run_pr_fast_test_lane.sh
- .csdlc/evidence/5913/ci-fix-pr-fast-routing.log
- adl/src/cli/mod.rs
- adl/tools/test_adl_review_compatibility.sh
- .csdlc/evidence/5913/review-finding-fix-fenced-contract.log
- adl/src/cli/mod.rs
- adl/tools/test_adl_review_compatibility.sh
- .csdlc/evidence/5913/review-finding-fix-long-fence-contract.log

## Execution

- Route adl-review verify-repo-contract to a direct markdown repository-review contract verifier instead of the removed v1 tooling multiplexer.
- Route adl-review code-review fixture mode to the deterministic CodeBuddy/CodeFriend showcase smoke path and validator, rejecting provider-backed backends for this bounded issue.
- Update the focused compatibility regression so it no longer uses adl tooling as an oracle and covers good/bad contract packets, deterministic fixture smoke, narrowed help output, and hidden stale command diagnostics without v1/multiplexer wording.
- Remove the stale `binary_id(adl::bin/adl-session)` selector from the CLI-family PR-fast test lane because `adl-session` is no longer a declared binary.
- Update the PR-fast selector contract test expectations so the generated focused filter matches the current declared CLI/process binary surface.
- Strip fenced code blocks before verifying repository-review contract sections, fields, and finding markers.
- Add a fenced-code-only negative fixture proving required review markers inside examples do not satisfy the contract verifier.
- Track Markdown fence character and opening delimiter length when hiding fenced code blocks before repo-review contract validation.
- Require a closing fence to use the same marker character, at least the opening delimiter length, and only trailing whitespace.
- Add a long-fence negative fixture proving required review markers inside a four-backtick block do not satisfy the contract verifier.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_adl_review_compatibility.sh"
    ],
    "purpose": "Initial focused compatibility regression for repaired adl-review read-only compatibility surface and fail-closed command boundaries",
    "outcome": "passed",
    "evidence_ref": "adl-review-compatibility.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl/Cargo.toml",
      "--bin",
      "adl-review",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Initial strict relevant Rust lint for repaired adl-review dispatch changes",
    "outcome": "passed",
    "evidence_ref": "strict-clippy-adl-review.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_adl_review_compatibility.sh"
    ],
    "purpose": "Focused compatibility regression after hidden stale-command diagnostic wording fix",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5913/review-finding-fix-compatibility.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl/Cargo.toml",
      "--bin",
      "adl-review",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Strict relevant Rust lint after hidden stale-command diagnostic wording fix",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5913/review-finding-fix-clippy.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_run_pr_fast_test_lane.sh",
      "+",
      "bash",
      "adl/tools/run_pr_fast_test_lane.sh",
      "--base",
      "5a1d3bfda7108bede1572cbd9dc9e2af19d9eedb",
      "--head",
      "HEAD",
      "--print-plan",
      "+",
      "cargo",
      "nextest",
      "list",
      "--workspace",
      "-E",
      "test(/^cli::/) or binary_id(adl::bin/adl-process)",
      "+",
      "bash",
      "adl/tools/test_adl_review_compatibility.sh"
    ],
    "purpose": "Post-publication CI-fix validation for stale PR-fast nextest binary_id routing plus original adl-review compatibility smoke.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5913/ci-fix-pr-fast-routing.log"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl/Cargo.toml",
      "--",
      "--check",
      "+",
      "bash",
      "adl/tools/test_adl_review_compatibility.sh",
      "+",
      "bash",
      "adl/tools/test_run_pr_fast_test_lane.sh",
      "+",
      "cargo",
      "clippy",
      "--manifest-path",
      "adl/Cargo.toml",
      "--bin",
      "adl-review",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Focused validation for fenced-code-safe repo-review contract verification, preserved adl-review compatibility behavior, PR-fast selector contract, formatting, and strict relevant Rust lint.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5913/review-finding-fix-fenced-contract.log"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl/Cargo.toml",
      "--",
      "--check",
      "+",
      "bash",
      "adl/tools/test_adl_review_compatibility.sh",
      "+",
      "bash",
      "adl/tools/test_run_pr_fast_test_lane.sh",
      "+",
      "cargo",
      "clippy",
      "--manifest-path",
      "adl/Cargo.toml",
      "--bin",
      "adl-review",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Focused validation for long-fence-safe repo-review contract verification, preserved adl-review compatibility behavior, PR-fast selector contract, formatting, and strict relevant Rust lint.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5913/review-finding-fix-long-fence-contract.log"
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
