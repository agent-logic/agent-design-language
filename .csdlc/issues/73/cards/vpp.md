# Validation Planning Prompt

Template: 1.0.0

Issue: 73

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.md

Diagram: .adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.mmd

## Selected Lanes

[
  {
    "lane": "planning-diff-hygiene",
    "proof_role": "Reject whitespace and patch hygiene errors in the planning packet.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "planning-specification-completeness",
    "proof_role": "Count all 19 issue specifications and every required specification field, and reject unbalanced code fences.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "awk",
      "BEGIN{s=o=sc=ng=d=dl=ac=v=st=f=0} /^### V3-(0[1-9]|10A|10B|11A|11B|12|13|14|15|16|R01):/{s++} /^\\*\\*Objective:\\*\\*/{o++} /^\\*\\*Scope:\\*\\*/{sc++} /^\\*\\*Non-goals:\\*\\*/{ng++} /^\\*\\*Dependencies:\\*\\*/{d++} /^\\*\\*Deliverables:\\*\\*/{dl++} /^\\*\\*Acceptance criteria:\\*\\*/{ac++} /^\\*\\*Validation proof:\\*\\*/{v++} /^\\*\\*Stop conditions:\\*\\*/{st++} /^```/{f++} END{exit !(s==19&&o==19&&sc==19&&ng==19&&d==19&&dl==19&&ac==19&&v==19&&st==19&&f%2==0)}",
      ".adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.md"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "independent-model-review-receipts",
    "proof_role": "Verify both retained provider receipts completed successfully, bind the reviewed revision in their request IDs, and contain PASS decisions.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 1000,
    "argv": [
      "jq",
      "-e",
      "-s",
      "length == 2 and all(.[]; (.final_status == \"ok\") and (.request_id | endswith(\"17041ed7\")) and (.output_text | contains(\"DECISION: PASS\"))) and ([.[].route.provider] | sort == [\"anthropic\", \"gemini\"])",
      ".csdlc/evidence/73/provider-reviews/post-pre-pr-final-gemini-result.json",
      ".csdlc/evidence/73/provider-reviews/post-pre-pr-final-claude-sonnet-result.json"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "reviewed-scope-stability",
    "proof_role": "Prove the architecture and diagram are byte-unchanged from the exact revision passed by both providers.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--quiet",
      "17041ed7da93d2b4f9c6978053daedeb3b8c1c27",
      "--",
      ".adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.md",
      ".adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.mmd"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "local-link-existence",
    "proof_role": "Verify the architecture's repository-local comparative-document link resolves.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 10,
    "budget_tokens": 1000,
    "argv": [
      "test",
      "-f",
      ".adl/docs/TBD/CSDLC_V3_GH_INSPIRED_ARCHITECTURE.md"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "upstream-source-paths",
    "proof_role": "Verify every cited official cli/cli source path exists at the pinned upstream revision.",
    "acceptance_ids": [
      "AC-1",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "-C",
      "/Users/daniel/git/cli",
      "ls-tree",
      "--name-only",
      "-r",
      "9fc0f70e0ef97446de9166febce546e955675bc3",
      "--",
      "cmd/gh/main.go",
      "internal/ghcmd/cmd.go",
      "pkg/cmd/root/root.go",
      "pkg/cmdutil/factory.go",
      "pkg/cmd/factory/default.go",
      "pkg/cmd/issue/list/list.go",
      "pkg/iostreams/iostreams.go",
      "pkg/cmdutil/errors.go",
      "pkg/httpmock/registry.go",
      "cmd/gen-docs/main.go"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "diagram-render",
    "proof_role": "Render the exact Mermaid dependency graph into retained issue evidence.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "mmdc",
      "-i",
      ".adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.mmd",
      "-o",
      ".csdlc/evidence/73/architecture.svg"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `git diff --check`
- `awk BEGIN{s=o=sc=ng=d=dl=ac=v=st=f=0} /^### V3-(0[1-9]|10A|10B|11A|11B|12|13|14|15|16|R01):/{s++} /^\*\*Objective:\*\*/{o++} /^\*\*Scope:\*\*/{sc++} /^\*\*Non-goals:\*\*/{ng++} /^\*\*Dependencies:\*\*/{d++} /^\*\*Deliverables:\*\*/{dl++} /^\*\*Acceptance criteria:\*\*/{ac++} /^\*\*Validation proof:\*\*/{v++} /^\*\*Stop conditions:\*\*/{st++} /^```/{f++} END{exit !(s==19&&o==19&&sc==19&&ng==19&&d==19&&dl==19&&ac==19&&v==19&&st==19&&f%2==0)} .adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.md`
- `jq -e -s length == 2 and all(.[]; (.final_status == "ok") and (.request_id | endswith("17041ed7")) and (.output_text | contains("DECISION: PASS"))) and ([.[].route.provider] | sort == ["anthropic", "gemini"]) .csdlc/evidence/73/provider-reviews/post-pre-pr-final-gemini-result.json .csdlc/evidence/73/provider-reviews/post-pre-pr-final-claude-sonnet-result.json`
- `git diff --quiet 17041ed7da93d2b4f9c6978053daedeb3b8c1c27 -- .adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.md .adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.mmd`
- `test -f .adl/docs/TBD/CSDLC_V3_GH_INSPIRED_ARCHITECTURE.md`
- `git -C /Users/daniel/git/cli ls-tree --name-only -r 9fc0f70e0ef97446de9166febce546e955675bc3 -- cmd/gh/main.go internal/ghcmd/cmd.go pkg/cmd/root/root.go pkg/cmdutil/factory.go pkg/cmd/factory/default.go pkg/cmd/issue/list/list.go pkg/iostreams/iostreams.go pkg/cmdutil/errors.go pkg/httpmock/registry.go cmd/gen-docs/main.go`
- `mmdc -i .adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.mmd -o .csdlc/evidence/73/architecture.svg`

## Failure Semantics

Fail closed on missing issue-plan detail, stale or mismatched review revisions, undispositioned actionable findings, invalid source references, or any expansion into implementation.

## Handoff

Retain typed evidence before convergence.
