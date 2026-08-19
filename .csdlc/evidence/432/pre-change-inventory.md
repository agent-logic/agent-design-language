# Issue 432 pre-change migration inventory

Inventory source: `git ls-files .adl` at preparation head `40608478b8912a2bde5c971cac337fda82f36a4c`.

Disposition for every entry: remove from the Git index while preserving the
ignored working-tree file. No entry is copied into another tracked location
except the worktree-policy authority, whose non-sensitive two-field contract is
copied to `adl/config/worktree-policy.json`.

1. `.adl/TBD/OPUS_REVIEW_RUNBOOK.md`
2. `.adl/docs/TBD/ADL_RUNTIME_SEMANTIC_ABI_AND_TRANSFER_PLAN.md`
3. `.adl/docs/TBD/AGENT_LOGIC_ACCOUNT_REPO_MIGRATION_PLAN.md`
4. `.adl/docs/TBD/CSDLC_V3_GH_INSPIRED_ARCHITECTURE.md`
5. `.adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.md`
6. `.adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.mmd`
7. `.adl/docs/TBD/CSDLC_V3_RUST_PLAN_REVIEW.md`
8. `.adl/docs/TBD/PODCAST_STUDIO_NEXT_WEEK_LAUNCH_PLAN_5702.md`
9. `.adl/docs/TBD/RUNTIME_TLS_ACIP_SIMPLIFICATION_GEMINI_REVIEW.md`
10. `.adl/docs/TBD/RUNTIME_TLS_ACIP_SIMPLIFICATION_REVIEW.md`
11. `.adl/docs/TBD/V092_SPRINT_5854_DEMO_PUBLICATION_SESSION_PROMPT.md`
12. `.adl/docs/TBD/V092_SPRINT_5855_RUNTIME_OBSERVATORY_SESSION_PROMPT.md`
13. `.adl/docs/TBD/V092_SPRINT_5856_QUALITY_RELEASE_SESSION_PROMPT.md`
14. `.adl/docs/TBD/V092_SPRINT_5857_BIRTHDAY_CORE_SESSION_PROMPT.md`
15. `.adl/docs/TBD/V092_SPRINT_5858_FOUNDATION_SESSION_PROMPT.md`
16. `.adl/docs/TBD/V092_SPRINT_5862_DISTRIBUTED_GUARDIAN_SESSION_PROMPT.md`
17. `.adl/docs/TBD/VOCECHAT_REFERENCE_EVALUATION.md`
18. `.adl/docs/TBD/session-handoffs/ISSUE_84_UNITY_OBSERVATORY_SESSION_HANDOFF.md`
19. `.adl/docs/TBD/session-handoffs/README.md`
20. `.adl/docs/TBD/session-handoffs/V092_PLANNING_AND_RECOVERY_SESSION_HANDOFF.md`
21. `.adl/docs/TBD/session-handoffs/V092_SPRINT_5_BIRTHDAY_CONTINUITY_SESSION_HANDOFF.md`
22. `.adl/docs/TBD/session-handoffs/V092_WP18C_UMBRELLA_110_SESSION_HANDOFF.md`
23. `.adl/docs/TBD/session-handoffs/V092_WP_04_16_RUNTIME_REMEDIATION_SESSION_HANDOFF.md`
24. `.adl/live-provider-probe-disposition.md`
25. `.adl/provider-adapter-focused-tests.log`
26. `.adl/v0.91.7/tasks/issue-4630__v0-91-7-wp-03-consume-c-sdlc-integration-control-plane-truth/vpp.md`
27. `.adl/worktree-policy.json`

Active authority references found before migration were limited to
`AGENTS.md`, `csdlc-v2/src/lifecycle.rs`, and `csdlc-v2/tests/gate2.rs`.
They now resolve only `adl/config/worktree-policy.json`. Historical `.csdlc`
records remain immutable provenance and are not runtime or policy authority.
