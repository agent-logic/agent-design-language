# v0.92 Session Handoff Recovery Index

This directory preserves the active task state recovered before retiring large
or duplicated Codex task histories. Each file is intended to bootstrap a
brand-new, non-cloned Codex task.

| Area | Sprint or umbrella | Active issue | Handoff |
|---|---|---:|---|
| Observatory / living Polis interface | WP-18C umbrella `#110` | `#112` | [V092_WP18C_UMBRELLA_110_SESSION_HANDOFF.md](V092_WP18C_UMBRELLA_110_SESSION_HANDOFF.md) |
| Distributed Runtime membership remediation | Corporate WP-04.16 umbrella `#142` | `#199` | [V092_WP_04_16_RUNTIME_REMEDIATION_SESSION_HANDOFF.md](V092_WP_04_16_RUNTIME_REMEDIATION_SESSION_HANDOFF.md) |
| Sprint 5 birthday continuity | Sprint `#5854`, WP-18 `#5836` | `#237` | [V092_SPRINT_5_BIRTHDAY_CONTINUITY_SESSION_HANDOFF.md](V092_SPRINT_5_BIRTHDAY_CONTINUITY_SESSION_HANDOFF.md) |
| Planning, recovery, runner policy, and storage diagnosis | Cross-sprint coordination | none | [V092_PLANNING_AND_RECOVERY_SESSION_HANDOFF.md](V092_PLANNING_AND_RECOVERY_SESSION_HANDOFF.md) |

## Recovery Rules

- Resume from a new task; do not clone the source task.
- Enter the exact worktree named by the selected handoff.
- Preserve all listed dirty and untracked paths.
- Recheck live GitHub and branch state before acting because handoffs are
  point-in-time records.
- Do not write tracked issue work on `main`.
- Do not run optional CI. Use at most one paid 16-core runner per issue for the
  required proving job.
- Never use or inspect `/private/tmp`.

## Additional Preserved Reference

The VoceChat product/reference evaluation is retained one directory above as
[`VOCECHAT_REFERENCE_EVALUATION.md`](../VOCECHAT_REFERENCE_EVALUATION.md).
It is not an implementation handoff and was not sent to Observatory tasks.
