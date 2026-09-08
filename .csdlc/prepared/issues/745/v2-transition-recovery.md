# Issue #745: authorized typed v2 recovery

The operator explicitly authorized "use v2 if needed, get it done, record the defect" in Planning #7 after native-v3 issue creation failed. This is bounded transition recovery; native v3 remains default authority.

Defect #744: https://github.com/agent-logic/agent-design-language/issues/744
ADR issue #745: https://github.com/agent-logic/agent-design-language/issues/745

The native attempt returned github_mutation_reconciliation_pending with underlying github_mutation_reconciliation_unavailable. Explicit approved token-file selection then produced authenticated github_mutation_not_reconciled. The native implementation persists intent before credential/process success and exposes no proven-nondispatch retry route. See csdlc-v3/src/adapters/mod.rs and csdlc-v3/src/commands/remote/mod.rs at source baseline bf617859982f8b9613737344400626eb90768930.

Retained native intent digest: 8dbec9dac7cebe523aff76b76932358788bbb6e3139a3dd2ef0d34c6dad0f6cf. It remains unresolved in the Git-common native-v3 intent store. Do not replay it following v2 creation; #744 owns its reviewed repair.

Typed v2 created #744 using operation key planning7-v0921-defect-20260908 and #745 using planning7-v0921-adr-20260908, both with authenticated readback. No raw GitHub mutation, intent deletion, credential disclosure, or generation-selector change occurred.

The operator identified preparation files written in the primary checkout. All task-owned preparation/draft files were preserved byte-for-byte in an isolated bootstrap checkout and removed from primary main. The primary checkout was verified clean. Initialization and binding then ran from the isolated bootstrap, and implementation runs only in the #745 bound FastWork worktree.

Validation inputs live under the prepared issue directory because typed finalize replaces the generated evidence directory. Runtime/cloud tests and architecture acceptance are outside this documentation scope.
