# v0.92 Work Package Execution Readiness

The issue wave is open. A row marked `prepared next` has satisfied milestone
ordering but still requires issue-specific design approval, claim binding, and
owner validation before implementation.

| Work | Ready after | Current readiness |
| --- | --- | --- |
| WP-01 | v0.91.8 release and planning prerequisites | Active under #5817 |
| WP-01B | WP-01 publication | Prepared next; issue #5818 |
| WP-02 | WP-01, WP-01B, reviewed #5815 plan and organization readiness | Dependency-gated; issue #5819 |
| WP-02A | WP-02 | Dependency-gated; issue #5801 |
| WP-03 | WP-02A | Dependency-gated; issue #5820 |
| WP-04 | WP-03 | Dependency-gated; issue #5821 |
| WP-05 | WP-02A | Dependency-gated; issue #5822 |
| WP-06 | WP-02A | Dependency-gated; issue #5823 |
| WP-07 | WP-01, WP-05 | Dependency-gated; issue #5824 |
| WP-08 through WP-19 | Dependencies in WBS and issue-wave YAML | Open and card-initialized; not execution-authorized by WP-01 |
| WP-18A | Explicit operator scope disposition | Removed from v0.92 completion; backlog #84 owns Unity work, with #122 in v0.92.1 and #251 in backlog |
| WP-20, WP-21, WP-21A | Integrated feature/demo proof | Open and card-initialized; release-tail work |
| WP-22 | WP-21A reviewed merge is present and ancestral; all implementation inputs are available | Corrective authority is issue #467, which supersedes #311 release-credit semantics; #310's merge is the required input, while its terminal reconciliation and cleanup remain asynchronous and non-gating |
| WP-23 / #312 | Merged WP-22 authority available as documentation input | Active documentation review; it may record either a blocked or passing WP-22 result and never waits for asynchronous closeout |
| WP-25 / #313 | WP-23 merged, plus WP-24 and WP-24A inputs required by the WBS and issue wave | Proceeds after those three content dependencies; it does not depend on WP-22 closeout |
| WP-26 through WP-30 | Preceding release-tail stage merged, in the order declared by the WBS and issue-wave YAML | Stage-gated; administrative issue closeout is asynchronous and non-gating |

## Start Rule

Before any child implementation starts, its owner must verify live dependencies,
complete issue-specific design approval, bind a dedicated worktree and claim,
create the issue-bound goal, and run only the focused proof declared by its VPP.
WP-01 does not grant product completion or merge authority to child issues.

## WP-22A Corrective Hydration

WP-22A #467 resolves the quality-gate evidence ledger with zero blockers. Downstream work depends on merged implementation and its own stage gates, never asynchronous issue closeout. AEE-020 is a downstream release-tail outcome, not a circular prerequisite to WP-22.
