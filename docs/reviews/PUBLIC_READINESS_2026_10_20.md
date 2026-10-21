# ADL public-readiness delivery report

Status: implementation delivered; final assurance and owner decisions remain open.

This report consolidates the Codex and Claude pre-publication reviews under [#1318](https://github.com/agent-logic/agent-design-language/issues/1318). The integrated implementation baseline is `ba93dbeef036f42e8e82c12ff7bf7828f5278392`. It covers the public ADL repository, not private Runtime, CodeFriend, or C-SDLC product qualification.

## Delivered changes

| Area | Issue / merged PR | Result |
|---|---|---|
| Canonical identity and security intake | #1320 / [#1334](https://github.com/agent-logic/agent-design-language/pull/1334) | Personal repository private; organization repository public with accurate About metadata; private reporting enabled; confirmed fallback contact documented. Licensing separately escalated. |
| Roadmap | #1321 / [#1330](https://github.com/agent-logic/agent-design-language/pull/1330) | Current roadmap links five existing milestones and distinguishes issue completion from release acceptance. |
| Visitor and contributor journey | #1322 / [#1332](https://github.com/agent-logic/agent-design-language/pull/1332) | Public contribution route, conduct policy, templates, current docs, compatibility matrix, historical demo index, convenience commands and docs regression checker. |
| Safety boundaries | #1323 / [#1333](https://github.com/agent-logic/agent-design-language/pull/1333) | Create-only demo output; canonical include containment; pinned toolchain action and checker; corrected demo fixtures. |
| Crash-safe locking | #1324 / [#1328](https://github.com/agent-logic/agent-design-language/pull/1328) | Persistent lock identity and OS-managed locking; crash recovery, concurrent writers and legacy-format refusal tested on Linux. |
| Controlled archive | #1325 / [#1331](https://github.com/agent-logic/agent-design-language/pull/1331) | Exact adapter-file archive with restoration identities; retained historical package rationale; lifecycle snapshot classification; successor guard composition. |

All six PRs merged after independent review and applicable CI. PR #1332 used an explicit operator-authorized `gh` merge because native publication/readiness reconciliation reused stale evidence. Its native finish/cleanup remains separate bookkeeping; no native completion is claimed for that issue. The other five implementation issues completed native finish and cleanup.

## Combined finding disposition

| Finding from either review | Disposition |
|---|---|
| Two public ADL repositories | Fixed: personal repository private; canonical organization repository public. Earlier public clones/caches are not revoked. |
| Empty canonical About box | Fixed: description, homepage and topic metadata populated and read back. |
| No canonical release | Already outdated at review: canonical v0.92.1 source release exists. No fabricated binary assets or qualification claim. |
| No milestones | Already outdated: five milestones existed. Discoverability fixed through [ROADMAP](../ROADMAP.md); no automatic milestone closure. |
| Long obsolete README and blocked-release review guide | Earlier replacements retained and improved. Public visitor guidance is distinct from maintainer governance. |
| Private/machine-specific contributor prerequisites (F04) | Fixed public fork/build/test/PR path and explicit maintainer-only obligations. |
| Missing or stale advertised demos (F05) | Current runnable examples separated from a labeled historical catalog; current demo commands validated. |
| Broken UTS/ACC references (F08) | Corrected current links and public API names; historical design records remain labeled. |
| Missing templates/customer framing (F12) | Bug/proposal/PR templates and conduct policy added; public-owner scope clarified. |
| No root Cargo manifest | Intentional explicit-manifest structure retained; Makefile provides documented entrypoints without silently changing workspaces or locks. |
| Destructive demo output resets (F01) | Fixed create-only output with adverse existing-path fixtures. |
| Include symlink escape (F02) | Fixed canonical containment relative to the initial document parent; positive and negative cases tested. |
| Missing private security route (F03) | Fixed enabled GitHub reporting form and operator-confirmed `codefriend@agent-logic.ai` fallback. |
| Stale lock after writer crash (F06) | Fixed persistent lock protocol with process-crash/concurrency proof. |
| Mutable action pin/checker gap (F09) | Fixed exact toolchain action pin and expanded external-action checking. |
| Four unselected v2 packages (F07) | Adapter package archived; three remaining historical packages retained with documented consumers and explicitly unbuilt status. |
| Committed private Runtime binaries | Already archived under #1284 with restoration evidence; no duplicate removal or restoration. |
| Podcast media/duplicate MP3s | Already migrated out of the tracked demos tree. Historical Git objects were not rewritten. |
| Large historical lifecycle/docs footprint (F11) | Source-pinned classification and a bounded archive batch delivered. Historical evidence retained deliberately; no indiscriminate deletion. |
| Personal paths/references | Current visitor surfaces corrected and checked; historical records retain historical context. No claim of rewriting every historical reference. |
| License mismatch (F10) | Owner decision pending; no license grant or manifest changed without confirmation. |
| Claimed clean secrets | Not certified. Bounded scans matched fixtures/examples; full-history absence was not established. |
| Private product strengths | Claims remain with their separate owners; no private implementation restored or public availability implied. |

## Validation and measurement

The final acceptance run uses a fresh depth-one HTTPS clone of the integrated commit, with credential-helper lookup disabled and no sibling owner checkout. The public CLI lane passed, including formatting, Clippy, selected Rust suites, public dispatch/refusal/session checks, owner-boundary checks and release-version correspondence. Registry/toolchain caches already existed and OS caches were not controlled; this is functional proof, not a cold-build benchmark.

The same fresh clone also passed `make packages` (all five selected public package lanes), `make docs` (including adverse fixtures), and `make plan` (the provider-free example). Each command ran once against the integrated commit; retained exit statuses are zero.

The archive removes exactly 14 unselected adapter files representing 91,054 logical preimage bytes. After adding custody records, documentation and guards, its net tracked-checkout delta is **7 fewer files and 31,725 fewer logical bytes** against `8cf07d2208b24906b93da27f08e3ef184bf49869`, measured at archive head `c7ebc057fb795dafdff831047cf476786219f650`. This is not Git pack, transfer, allocation or history shrinkage. See [archive custody](../archives/PUBLIC_READINESS.md).

The snapshot inventory classifies 659 committed lifecycle snapshots and 7,007 prepared files across 513 groups at that source baseline. These records are historical evidence, not current authority. Thirty historical v2 files remain deliberately retained.

## Residual decisions and follow-ups

1. **Licensing:** public manifests declare `MIT OR Apache-2.0`, while only Apache license text is supplied. The operator must confirm whether to retain dual licensing and add the missing text or align declarations to Apache-only. No incidental relicensing occurred.
2. **Dependency advisories — [#1329](https://github.com/agent-logic/agent-design-language/issues/1329):** OSV version queries matched eight versions among 430 distinct registry versions across six public lockfiles. Active default CLI dependency families include anyhow, bytes, rustls, rustls-webpki and webpki-roots. Optional Quinn is absent from the default host graph; this does not establish universal unreachability. Unsafe-libyaml has an informational unmaintained advisory. Characterization h2 requires parent/feature analysis because the cited fixed line is not a compatible patch. No dependency updates or exploitability certification are claimed.
3. **History scanning:** current-tree matches were inspected as fixtures/examples. The bounded history query covers up to 500 matching commits, not all history or the last 500 chronological commits. No exhaustive secret certification.
4. **Historical retirement guard:** the full existing guard has pre-existing v0.93.2 milestone-identity drift on unchanged main. New exact archive normalization, five adverse/positive cases, the corrected two-test historical fixture and existing unit suites pass. No blanket exception or weakened approval pin was introduced.
5. **Native bookkeeping:** [#1327](https://github.com/agent-logic/agent-design-language/issues/1327) records the installed review admission regression. Existing issue-scoped package transition resolved it without a shared binary replacement. The additional #1322 stale-publication/readiness recovery packet is preserved; operator-authorized GitHub merge is not native terminal authority.

The cleanup implementation is delivered. Publication assurance is not a security certification, deployment approval or release acceptance. Umbrella closure must not silently treat the owner decisions and follow-ups above as resolved.
