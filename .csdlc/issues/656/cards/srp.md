# Structured Review Prompt

Template: 1.0.0

Issue: 656

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl/tools/install_runtime_v3_generation.sh
adl/tools/runtime_v3_generation.py
adl/src/cli/csm_runtime_v3_cmd.rs
adl/tools/test_runtime_v3_generation_install.sh
adl/tests/csm_runtime_v3_generation.rs

## Prompts

- Can an incomplete set become current?
- Does the receipt bind exact activated files?
- Do launchd and Runtime-init agree?
- Is preflight before mutation?
- Is rollback limited to verified generations?

## Findings

[
  {
    "id": "F-656-1",
    "severity": "p1",
    "summary": "Rust preflight does not reject non-executable generation artifacts before service mutation.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-656-2",
    "severity": "p1",
    "summary": "Authoritative SOR does not retain exact-final-SHA validation truth.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-656-3",
    "severity": "p2",
    "summary": "Generation corruption can prevent the governed stop path from stopping a running service.",
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

- Review ran on macOS; Linux systemd behavior was inspected but not executed.
- No live Runtime or service-manager mutation was performed.

## Review Result

Revision: Some("git-blake3:d0327eb6f913efab2cacba8b12322c8fd183abee:e24a087e13afe0ac9d62f87fa1db59de4fdadd90efc1f97248658ba9f22d8f17")

Reviewer: Some("fresh-session:ab7be61c-d12e-4f81-8b72-596202f1c850")

Result: changes_required
