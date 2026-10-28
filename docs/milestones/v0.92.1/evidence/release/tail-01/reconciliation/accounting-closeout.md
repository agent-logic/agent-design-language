# Issue517 accounting closeout

All245 historical non-proving rows have exact source identities and explicit accounting dispositions in `ownership.json`: zero unowned accounting rows and no pending accounting issue creations. The original393-row assessment is preserved. Issue517 owns the docs quality-gate decision; implementation remains outside its scope.

The WBS wording used by the PR #752 accounting review permitted required checks to pass or exceptions to be explicitly owned and dispositioned, and retained closed predecessor work without reopening. The current WBS requires every required proving lane to pass; an owned exception alone does not waive a required failing lane. This accounting closeout establishes documented dispositions, not release-gate satisfaction. The admission already maps retained criteria to accepted successor issues. The corrected reconciliation joins those mappings to observed closed issues, merged PR revisions, retained closed-out index records or native terminal receipts. Missing cross-references do not establish new implementation obligations.

PR750 merged at `f61deb36d5eccb4a3e391510bbea4bb6f2d51216`, adding11 native terminal states and11 receipts. Its reviewed current closed+linked-merged-PR denominator has `missing_count=0`. This addresses that terminal denominator, not independent execution of every predecessor criterion. Exact750 artifacts are catalogued in `terminal-closeout-750.json`.

Two special joins remain explicit:511 was absorbed into512, whose merged PR719 and native terminal supply delivery authority;497 is closed without a closing-PR reference and is outside750's closed+linked-merged denominator. Its separately observed merged PR613 and explicit624 sidecar amendment supply accepted successor disposition, not an invented terminal receipt.523 has native terminal proof;526 remains the open later-stage ceremony owner.

The proposed four domain follow-ups for189 rows were withdrawn. Those rows retain their original classifications and accepted successor lineage. The six rows tied to independently confirmed native defects name existing749 and new751; code and tests belong to those separate issues.

A completed accounting decision is distinct from direct criterion execution proof and release authorization. No historical result is rewritten, no requirement is waived, and `release_authorized` remains false. The docs decision can report complete accounting while retaining exact evidence limits and separately owned code findings.
