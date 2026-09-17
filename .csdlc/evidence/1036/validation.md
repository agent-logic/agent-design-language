# Issue #1036 validation evidence

PR #1042 implements bound legacy adoption and retains the writer fence through interrupted recovery. The corrective change also recognizes authenticated repository-scoped issue-creation receipts during legacy admission, preserving their exact intent/receipt linkage and all retained bytes.

## Qualification history

The initial candidate at `9b248b5baa021befdfb93bfebdb57bbb43901df3` passed 59 installed journey tests and the conversion release gate (seven roles, 12 scenarios, 30 fault cases). It nevertheless failed preparation of authentic copied #874 and #898 records in a mixed remote census. That candidate was rejected for operational promotion. Removing the remote intents/mutations from a disposable diagnostic copy removed the failure; this diagnostic was not accepted as qualification.

Commit `d084c20be99f35741c993dfe53eef80409f9adce` corrects the issue-creation receipt classification and adds positive byte-preservation and six tamper regressions. Main was merged with both installed-test imports preserved. Qualification must be rerun against the corrected exact executable; earlier candidate results do not establish corrected-candidate acceptance.

The corrected receipt candidate at `b41d4c940ab6dacf2024f10aeff4938cc7fae592` passed 60 installed journey tests, the conversion release gate, and authentic copied #874/#898 prepare/rebuild/status/validate with all 927 retained record files present. Exact-head review then found overlapping adoption callers could release a shared guardian prematurely. The final correction retains a stable per-issue client lock through activation, projection, release and rejected-attempt cleanup, with a concurrent valid/changed-plan regression. Final-head results must be assessed separately from the earlier candidates.

## Boundaries

- Original lifecycle and receipt records remain unchanged; qualification uses isolated copies and deterministic remote transport.
- Authentic bound #873 and pending-publication #897 copies remain unproven because the existing relocation owner cannot safely admit their retained layouts. Synthetic tests do not close those proof gaps.
- PR #1042 exists; its latest exact-head proof, review, CI and copied-record results are recorded in the PR and retained qualification packet.
- This change does not authorize or claim a shared binary replacement, live conversion, writer activation, merge, terminal closeout, pilot completion, or sprint completion.
