# GWS Project Setup And Onboarding

## Purpose

Provide the minimum bounded setup path for an ADL project that wants to use the
retained Google Drive sync or context-mirror surface.

## Inputs To Gather

- one bounded Drive root folder ID
- one bounded Drive seed folder ID when using the context mirror
- the repository-relative source file, target name, and Drive folder path when
  syncing one file
- the project's GitHub issue/PR workflow entrypoints

## Required Environment Surface

The current retained Drive surface uses these environment inputs:

- `ADL_GWS_LIVE_MODE`
- `ADL_GWS_WRITE_APPROVAL` only when a bounded execute-mode Drive write is
  intentionally allowed
- `ADL_GWS_RECURSIVE_SYNC` when recursive context mirroring is intended
- one operator-approved external credential source such as `ADL_GWS_TOKEN` or
  `ADL_GWS_CREDENTIALS_FILE`, following the runbook's ordered credential list

Drive folder IDs are command-line scope inputs rather than environment
variables.

Recommended initial posture:

- `ADL_GWS_LIVE_MODE=dry_run`

Do not start a project in execute mode by default.
Do not set `ADL_GWS_WRITE_APPROVAL` by default; add it only immediately before
an intentionally bounded live write.

Auth is required for live bounded use, not for fixture-first or dry-run-only
adoption proof.

## Onboarding Steps

1. Confirm the project actually needs bounded Drive file sync or context
   mirroring.
2. Confirm GitHub remains the canonical planning and repo-change authority.
3. Decide whether the project is adopting the Drive surface in:
   - fixture-first / dry-run posture, or
   - live bounded posture
4. If live bounded posture is intended, follow the approved credential-source
   order in
   [`ADL_GOOGLE_DRIVE_CONTEXT_MIRROR_RUNBOOK.md`](../../../../tooling/ADL_GOOGLE_DRIVE_CONTEXT_MIRROR_RUNBOOK.md);
   keep credentials outside the repository.
5. Bind the narrow Drive scope with `--root-folder` for a single-file sync, or
   `--drive-root-folder-id` and `--drive-seed-folder-id` for a context mirror.
6. Run the retained native Drive sync in dry-run mode first.
7. Run the retained context mirror only after its folder scope is explicit.
8. Review the generated reports before considering any bounded live write.
9. Save the resulting proof artifacts with the project packet.

## First Commands To Prefer

Use the bounded demo/report surfaces currently present in ADL. The first
command is a local dry-run demonstration; the production context-mirror
command and credential contract live in the linked runbook.

- `ADL_GWS_LIVE_MODE=dry_run cargo run --manifest-path adl/Cargo.toml --bin demo-adl-gws-native-drive-sync -- --root-folder demo-root --folder-path docs/seed`
- `ADL_GWS_LIVE_MODE=dry_run cargo run --manifest-path adl/Cargo.toml --bin adl-gws-context-mirror -- --drive-root-folder-id demo-root --drive-seed-folder-id demo-seed`

Use dry-run mode first unless there is a clear reason to attempt execute mode.

The earlier `demo_v0912_gws_live_safety_package`,
`demo_v0912_gws_live_capability_execution_surface`, and
`demo_v0912_gws_live_content_card_roundtrip` entrypoints were retired by
[#309](https://github.com/agent-logic/agent-design-language/issues/309) through
[PR #460](https://github.com/agent-logic/agent-design-language/pull/460). Their
retained reports are historical evidence, not runnable command guidance. Do
not restore those entrypoints to replay the old packet.

## What A Healthy First Run Looks Like

Healthy does not always mean proving live writes. A healthy first run may be:

- producing a deterministic in-memory dry-run report
- skipped with a truthful auth/scope/tooling reason
- completing a bounded Drive write with exact content readback when execute
  mode was explicitly authorized

That is acceptable if the recorded reason is explicit and reviewable.

## Project Handoff Expectation

The operator taking over the project should be able to answer:

- which Drive root and seed folder IDs are allowed
- which repository-relative source and destination paths are in scope
- whether the Drive surface is dry-run only or execute-capable
- which external credential source label and least-privilege Drive scopes were
  used, without recording credential contents
- whether every reported Drive result completed exact content readback
- what still must go through GitHub issue/PR controls
