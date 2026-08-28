# Tooling Issue: Bound Worktree Missing Prepared Helper

Issue: #578

Observed during #578 execution:

- The approved preparation bundle named
  `.csdlc/prepared/issues/578/reviewer-selection-smoke.sh` in the VPP
  validation lane.
- After `csdlc-bind` created
  `/Volumes/FastWork/adl-worktrees/adl-issue-578-glm-5-3-flash-provider-profile`,
  the bound worktree contained the issue cards and retained design files but
  did not contain the prepared smoke helper.
- Running the declared command failed before validation:

```text
bash: .csdlc/prepared/issues/578/reviewer-selection-smoke.sh: No such file or directory
```

Local disposition:

- Re-created the helper under the bound #578 prepared bundle.
- Re-ran the declared smoke command successfully.
- Treated this as lifecycle-tooling materialization evidence, not as provider
  code failure.

Follow-up candidate:

- `csdlc-bind` should either materialize issue-local prepared validation helpers
  that are referenced by VPP/SRP lanes, or the preparation/doctor route should
  reject helper paths that will not exist in the bound execution worktree.
