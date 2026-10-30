# Milestone releases since the canonical repository cutover

[REGISTRY.json](REGISTRY.json) maps v0.92.0 onward to exact source, package version,
accepted engineering outcome and release identity. Older releases remain in the
legacy repository. v0.93, v0.93.2 and v0.93.3 remain active or planned.

| Milestone | Accepted boundary | Record |
| --- | --- | --- |
| v0.92.0 (v0.92) | Completed engineering tail; retrospective milestone record | [Notes](v0.92.0.md) |
| v0.92.1 | Existing software release preserved | [Notes](v0.92.1.md) |
| v0.92.2 | Administrative closure; failed external attempt and deferred qualification retained | [Notes](v0.92.2.md) |
| v0.93.1 | Accepted ready to deploy, undeployed | [Notes](v0.93.1.md) |

Historical milestone record tags refer to this compact reconciliation, not to a
new package build. Original commits/trees resolve through the verified archive;
no historical ancestry needs to return to an ordinary clone. Notes disclose the
meaning of GitHub's automatic source archives. Attached record/notes/checksums
make the historical evidence and published record independently identifiable.

The four operator-accepted backlog issues #84, #251, #269 and #1226 stay unchanged.
#1352 remains open in v0.93.3. Their presence does not imply completed work.

## Closing routine

Milestone bookkeeping runs asynchronously after accepted work. It consumes
existing proof; it does not rerun product tests, qualify the same commit again,
require another review/approval, or create a closeout PR.

Before recording a milestone as released, the owner must observe the exact
repository, release/tag commit, reviewed notes and complete expected asset
names/sizes/SHA-256 values; required issues must be closed or listed in the
explicit approved carryover disposition. Missing, draft, wrong-identity,
incomplete or uncertain observations leave maintenance pending. A failed write
requires readback of that same target before any recovery, never blind replay.

An explicitly approved administrative non-release closure has its own recorded
outcome and evidence. It cannot be called software publication or product PASS.
The closing implementation is tracked by [Cognitive-SDLC #344](https://github.com/agent-logic/cognitive-sdlc/issues/344); ADL retains this public
contract and milestone evidence. Direct manual GitHub edits still require the
operator to preserve these boundaries.
