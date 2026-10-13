# Issue 1223 source retirement guard

The immutable original register preserves 4,134 ordered identities. The current
combined ledger classifies those same rows and 22 additional paths separately.
Conditional proposals never authorize deletion. The executed boundary is 34
original removals, 36 approved component removals, and one public-cutover removal:
`RD07-2113 adl/tools/test_select_validation_lanes.sh` (71 historical total), plus
six reviewed CodeFriend helper removals (77 historical total), followed by 196
CodeFriend source-family and two obsolete workflow removals (275 historical total), then 815 C-SDLC source-family removals
(1,090 historical total), then 896 Runtime source-family removals
(1,986 historical total), followed by 212 Infrastructure source removals
(2,198 historical total), then six conversion helper/bridge retirements and
127 CodeFriend skill-family removals (**2,331 executed retirement actions**:
2,330 baseline deletions plus one bridge added and retired after baseline).
`CODEFRIEND_FAMILY_APPLIED.json` pins the latter 198 exact paths and source
preimages, reviewed caller delta and five-member supplemental rollback archive.
Retired host entrypoints do not establish current owner host/release acceptance.
`CODEFRIEND_SIX_HELPER_APPLIED.json` pins those six IDs, their exact preimages,
commit `40dd5b4ce90d6f5fc89db77501788ad30505a292`, and baseline archive members.
Its eight caller repairs preserve historical assertions and qualified owner routing.
Remaining proposals remain conditional.

`CSDLC_SOURCE_FAMILY_APPLIED.json` pins the 815 C-SDLC removals, 12 caller
modifications and three additions at commit
`f598c0711d9b270a5b304c7cde64dcdf3068619e`. Custody covers 801 unchanged
baseline-archive files and 14 exact supplemental preimages. Canonical native
authority selectors and receipts, retained operator history, public prompt
templates, lifecycle journals and original issue 872 evidence remain protected.
Historical failure-injection differences and owner package provenance do not
establish behavioral equivalence or a new conversion rehearsal pass.

`RUNTIME_SOURCE_FAMILY_APPLIED.json` records the 896 Runtime removals,
retained public source/fixtures, explicit retired-route diagnostics and exact
rollback preimages. Source delivery does not satisfy unresolved Runtime gateway,
configuration identity or operational-selector acceptance; those holds remain.

`INFRASTRUCTURE_SOURCE_FAMILY_APPLIED.json` records 212 Infrastructure removals
and 20 coupled caller repairs. Exact rollback custody includes the concise
Runtime template preimage at the source-family boundary. Offline Terraform,
static, mock and fake-tool proofs remain separate from cloud deployment and
Runtime artifact acceptance. The earlier 27-of-28 matrix failure remains
historical evidence alongside the later bounded ALB repair proof. All unrelated
holds and original inventory identities remain unchanged.

`check.py` reads committed Git objects, including approval packets. It verifies
original identities, combined counts, disjoint held groups, exact executed deletion
scope, protected/historical evidence, rollback custody and direct references.
Dirty files cannot override committed approval inputs.

## Reviewed execution reconciliation

`EXECUTION_OVERLAY.json` preserves the reviewed baseline of 77 retained-path
changes through 112 immutable Git transitions, with 25 packet references and
rollback custody. The supplemental five-file public caller repair extends the
overlay to 78 paths; its exact before/after identities and review are recorded
in `PUBLIC_OWNER_CALLER_REPAIR.json`. Its 38 original public
cutover entries preserve actual applied hashes, separately from prepared hashes.
Later supplemental reviews preserve the intervening source chains. The six-helper
batch adds six explicit removal entries and reconciles eight caller identities
without replacing the original baseline or earlier review records.
The completed manifest is pinned by `EXECUTION_APPROVAL_SHA256` in the guard.

The overlay admits only these existing reviewed changes. Conditional proposals
remain unexecuted and confer no new deletion authority. Later changes require
new exact source, review and custody evidence before updating the immutable pin.
Original approvals remain unchanged.

The exact-identity validator rejects unapproved deletion, retained source/mode
drift, duplicate paths, stale before-identities and absent provenance/custody.
The original baseline-specific validator remains available as historical logic;
the current run uses the combined ledger and execution overlay instead.

## Focused validation

Run `python3 -B tools/source_retirement/test_check.py` for deterministic Python
contract/negative tests (small CPU/disk, no build/network). Run
`python3 -B tools/source_retirement/check.py` only against the committed candidate;
an incomplete or changed execution manifest fails closed.
Tests do not establish installed product qualification or deletion authorization.

Provision the historical rollback archive in the resolved Git common directory
at `csdlc-v3/local/evidence/1223/retirement/baseline.tar.gz`, SHA256
`12205043566913435c25ea414b1fd9148b46727821f7bfefd5d452cfe47a1c17`.
Missing or corrupt custody fails closed. Digest checks are separate from restore
rehearsal. The extra cutover deletion also needs its exact pre-cutover Git blob
preserved; the old archive is not presumed to contain newer bytes.

Reference checks retain the three original component boundaries and exact
historical allowlists. Operational references cannot be hidden by a blanket docs
exception. Dynamic/indirect callers still require independent review. Further
historical reference additions must be reviewed individually before final proof.

## Proof storage

The installed proof honors `TMPDIR` for its fresh scratch and Cargo target.
Set it to an existing directory on the selected build volume. Evidence defaults
to this worktree's ignored `target/public-adl-proof/<issue>/<run>`;
`ADL_PUBLIC_ADL_PROOF_ROOT` can select a different evidence root. Neither path
is derived from the linked worktree's common Git directory.

The scratch exception in the macOS sandbox permits only the fresh proof
directory, excluding the synthetic credential sentinel, and literal ancestor
metadata needed by Cargo dep-info resolution. Sibling file contents, producer checkout and
network access remain denied. Run the seven small storage/sandbox regressions with
`python3 -B tools/public_adl/test_install_storage.py` under the same `TMPDIR`.
These are PVF `local_contract` path-isolation tests (small CPU/disk, no build or
external service); the five OS sandbox cases require macOS. They do not replace
the final five-package, seven-consumer, 21-UTS, five-negative proof.

## Conversion and CodeFriend skill batches

The six-file conversion and127-file skill batches bring executed retirement actions to2331:2330 files removed from the immutable baseline plus one ADL-only bridge added and then retired. The bridge remains outside the original4134 and separate22 inventories. Its exact f598c071 addition, d293b17 removal and custody are pinned; no other absent-to-absent transition is permitted. Existing owner rehearsal and original skill contracts remain separate from new live acceptance.

## C-SDLC skills, AWS source and planning helpers

Three reviewed source commits add83 C-SDLC skill resources,14 AWS source files and9 planning helpers to the prior2,331 actions. Current execution is2,437 actions:2,436 baseline deletions plus the same one transient ADL bridge. Original4,134 identities,22 additional paths and12 unresolved holds are unchanged. Packet digests, all source/caller Git identities, new-helper absence, exact removal sets and rollback archive bytes/modes are pinned.

The ten skill contracts passed with the initial ASCII-locale failure preserved and only the failed script rerun with UTF-8. AWS adapter compile/install and24 bootstrap cases are retained bounded proof; six dependency-inventory Python cases and five expected routing refusals are separate. Historical Infrastructure exit137 is a process exit code, not a test denominator. These results do not prove full installer completeness, historical CLI equivalence, live AWS execution or new native lifecycle acceptance.

## Manual and historical families

Reviewed source commits add 56 historical manual resources and 83 historical family paths to the previous 2,437 actions. Current execution is 2,576 actions: 2,575 baseline deletions plus the same single transient ADL bridge. Original 4,134 identities, 22 additional paths and 12 unresolved holds are unchanged. Existing exact-family guard checks now cover these committed packets, all source/caller identities and removal archives.

The manual packet preserves 18 nonexecuted example fixtures and the actual 42-page lookup/render proof for the earlier owner package. Only 33 of the original 34 named pages have identical later-admitted rendered bytes; the later workflow bridge appendix is source-inspected, not newly rendered. Historical family source retirement is not current native command acceptance or a new execution of its assertions.

## Public companion, CodeFriend document and Runtime demo closure

Reviewed source commits add 21 operational-skill companion schemas, 19 CodeFriend documents/fixtures and five Runtime demo helpers to the previous 2,576 actions. Current execution is 2,621 actions: 2,620 baseline deletions plus the same single transient ADL bridge. Original 4,134 identities, 22 additional paths and 12 unresolved holds remain unchanged. Twenty-seven explicit public adapters, documentation inputs and historical fixtures remain in ADL; their historical private-owner labels do not authorize removal.

The companion packet preserves original contract adapters and their declared external roots. The CodeFriend packet distinguishes 14 byte-identical owner resources from five explicit document supersessions and retains three Group-D documentation inputs. Runtime demo caller repairs preserve bounded prefix assertions and explicit unqualified runtime/platform limits. Exact source/caller identities and custody are checked; source correspondence and documentation transfer do not establish all advertised operational, hosted or live acceptance.

## Legacy, ANRM and Infrastructure tool closure

Reviewed source commits add eight legacy helpers, two ANRM producer/test resources and 20 Infrastructure tool resources to the previous 2,621 actions. Current execution is 2,651 actions: 2,650 baseline deletions plus the same single transient ADL bridge. Original 4,134 identities, 22 additional paths and 12 unresolved holds remain unchanged. Twenty-seven explicit public or historical compatibility retentions include the repaired public v2 language/compiler runner, eight ADL cache/logging helpers, and 18 Infrastructure-labelled diagnostics or references.

Original ANRM, private-Wuji, continuity/bootstrap and release-assembly evidence retain their separate exact denominators. Offline mocks and Terraform validation do not establish genuine Runtime, Linux service, cloud, model or host shutdown acceptance. Source-clone release assembly was explicitly superseded by artifact inputs; no original source-recipe equivalence is claimed. Public v2 routing checks use an inert validation wrapper and do not count as Cargo tests. Exact source/caller transitions and custody are checked through the existing family guard.

## Mixed CI, builder and native sunset closure

Reviewed source commits add one obsolete private CI check, ten builder resources and ten native sunset helpers to the previous 2,651 actions. Current execution is 2,672 actions: 2,671 baseline deletions plus the same single transient ADL bridge. Original 4,134 identities, 22 additional paths and 12 unresolved holds remain unchanged. The 13 explicit public retentions are separately recorded; modified navigation and pending owner acceptance do not imply retention.

Public CI guards retain their current assertions while obsolete private job contracts remain in custody. Builder qualification preserves four original commands (four Python methods and three shell contracts), with mock Docker and static publication/import checks; no real image build, AWS or installed-image acceptance is claimed. Public Group-D and legacy builder callers route their original tests through exact owner resource hashes. Native sunset assertion and acceptance limits remain those of the applied packet.

## Retained readiness metrics caller

The reviewed readiness-metrics repair adds no removals. RD07-2058 preserves its original assertions through exact authenticated historical dependency bytes; RD07-2076 retains its already-routed policy assertions. Both remain retained ADL fixture contracts. The existing 2,672 execution actions (2,671 baseline deletions and one transient bridge), 4,134 original identities, 22 additional paths and 12 holds are unchanged. Five routing-prefix cases are separate from reused original consumer proof; no fresh whole-suite or native lifecycle acceptance is claimed.

## Runtime164 gateway and generation contracts

Two reviewed source removals bring execution to 2,674 actions: 2,673 baseline deletions and one transient bridge. RD07-2100 and 2108 leave the held set; ten holds remain. Original 4,134 identities and 22 additional paths are unchanged. Current source custody and original pre-stub 2108 custody are both pinned. Actual Runtime164 CI preserves original mock gateway and real cross-binary config assertions; the ADL caller requires exact owner Git resources and an explicit executable without Cargo fallback. No selected-binary authentication, live 1513/2223 proof or native runtime acceptance is inferred.

## Sprint, CodeFriend and historical v0.87.1 closure

Reviewed source commits add 12 sprint-conductor resources, 12 CodeFriend host/memory resources and 17 historical v0.87.1 aggregate resources. These 41 deletions comprise 38 original inventory rows and three separately enumerated supplemental source paths. Current execution is 2,715 actions: 2,714 baseline deletions (including the three supplemental paths), plus the same single transient ADL bridge. Original 4,134 identities and 22 additional paths remain unchanged; ten unresolved holds remain. The supplemental paths are not silently added to either immutable inventory.

The sprint caller explicitly changes from original aggregate ABI to pinned mapped owner components; 421 assertion expressions are preserved without claiming full original or native acceptance. CodeFriend host/memory proof and changed interfaces remain separately bounded by their applied packet. Historical v0.87.1 aggregates are retired deliberately; provider and multi-agent children remain under independent dispositions, with no replacement equivalence or new execution claimed. Exact source transitions and both current/original rollback custody remain pinned.

## CodeFriend fitness, host smoke and CI contract closure

Reviewed commits add 8 captured source removals to the previous 2,715 actions. Current execution is 2,723 actions: 2,722 baseline deletions (including the same three separately enumerated supplemental paths) plus the single transient ADL bridge. Immutable 4,134 identities and 22 additional paths remain unchanged, as do ten holds. Five explicit retentions preserve the original fitness transport test with owner parser routing and four current owner navigation descriptors.

Original fitness assertions, owner installed proof, ten original owner CI cases and prior host smoke disposition remain separately bounded by the applied packets. Source transfer and retained navigation do not establish live website/gateway or native acceptance. Exact source/caller identity chains and rollback custody remain checked.

## WP14 historical workflows and contract-market fixtures

The reviewed WP14 and contract-market source commits add 31 captured removals to the previous 2,723 actions: three already-refusing historical WP14 workflows and 28 qualified contract-market inputs. Current execution is 2,763 actions: 2,762 baseline deletions, including the same three supplemental paths, plus the single transient ADL bridge. Immutable 4,134 identities and 22 additional paths and ten holds remain unchanged.

WP14 original native scenario obligations remain preserved historical evidence, not new native acceptance. The policy caller still proves six mutation refusals and seven deadline cases. The three original contract-market Bash consumers passed once against exactly 28 owner inputs in an isolated network-disabled Linux container; the retained setup failure precedes all test execution. This establishes offline fixture behavior only, with no provider, cloud or native Runtime acceptance. Exact source/caller chains and rollback custody remain checked.

The separately reviewed owner aggregate retirement adds three source removals and exact public selector, cache-test, guidance and registry caller repairs. Missing/same-target/cross-device cache safety assertions remain unchanged; other stale cache integration obligations are not reported passing. The main-sync retirement adds two sources whose card-restoring behavior and demanding original test are intentionally retired, with complete custody. Retained public closeout guidance reads current policy. No original aggregate, sync, provider or native lifecycle execution is claimed.

Four separately qualified Infrastructure offline sources are also retired in this integration; their exact original assertions and bounded owner proof remain in the applied packet. The active public selector repair contributes zero removals and two exact caller transitions. The first routing proof failed because the historical selector was not active; after repair, two fixture blocks covering three lanes passed through the real default manager in 0.233 seconds. This is routing proof only, with no product execution. The six batches total 40 new removals, bringing execution to 2,763 actions (2,762 baseline plus one transient); ten holds and three supplemental paths remain unchanged.

## Runtime guides, Infrastructure offline contracts and historical card wrappers

Three reviewed source commits add 14 captured removals: five Runtime guides with retained owner navigation, two Infrastructure lifecycle/redaction sources, and seven obsolete card/wrapper sources. Current execution is 2,785 actions: 2,784 baseline deletions (including seven supplemental paths), plus the single transient bridge. Immutable 4,134 identities and 22 additional paths and ten holds remain unchanged.

The unchanged Infrastructure lifecycle test passed six fake-AWS/SSH actions; the authentic retained adapter was used only for its help prerequisite. The exact original redaction assertion subset passed separately; the remaining mixed CI assertions are not qualified by that subset. Historical wrapper/card-migration assertions are preserved in custody and explicitly retired, not replaced by a native acceptance claim. The retained public batched-check negative test passed its mocked routing/source/output checks after requiring wrapper absence. Actual sequential registry postimages and all current/original rollback custody remain pinned. No new owner or live execution is claimed.

A fourth reviewed historical-quality batch retires four captured aggregates and four explicitly enumerated companion tests outside the immutable inventories. Together these batches add 22 removals, of which 18 are captured original rows and four are supplemental. The existing three supplemental paths remain unchanged, bringing that separate denominator to seven. Original inspect/filter/failure-reporting assertions remain in custody and are intentionally retired; independent child tests and current public gates are unchanged.

## Runtime API resources

The reviewed API resource commit adds three captured removals. Current execution is 2,788 actions: 2,787 baseline deletions, including the same seven supplemental paths, plus the single transient bridge. Immutable 4,134 identities and 22 additional paths and ten holds remain unchanged.

Eleven existing owner source-contract tests support the schema pair; the versioning guide has exact source correspondence. Observatory schema evolution adds journal operations and bearer metadata while preserving prior values. No HTTP request, live deployment or unrelated distributed/inventory resource acceptance is inferred. Original resource bytes and source/caller transitions remain pinned.

## Remaining Sprint helpers and documentation mode correction

Six captured Sprint helpers transfer with their existing metrics, parity and closeout component proofs. Current execution is 2,794 actions: 2,793 baseline deletions, including seven supplemental paths, plus the single transient bridge. The original 4,134 identities and 22 additional paths and ten holds remain unchanged.

Five helper bodies match the owner exactly; parity has explicit root requirements and refuses canonical lifecycle-state writes. Its changed interface is recorded separately. Existing original assertion blocks and native transport-double limits remain pinned; 421 expressions are not independent tests or full original acceptance. The real default-manager routing proof preserved its initial diagnostic-name assertion failure and passed the corrected affected block; no component or product execution was repeated. A separate zero-removal commit corrects the newly added Sprint README from 100755 to 100644 with identical content, preserving both transitions and custody. The original applied packet recorded the intended README mode instead of its observed source mode; a separately reviewed supplemental observed packet corrects this metadata observation while preserving the historical original packet.

## Runtime and lifecycle review resources

Two Runtime resources and sixteen finish/closeout/review skill resources now have exact committed source and rollback custody checks. Current execution is 2,812 actions: 2,811 baseline deletions, including the same seven supplemental paths, plus the single transient bridge. The original 4,134 identities, 22 additional paths and ten holds remain unchanged.

The Runtime resource packet preserves its specific owner consumer proof and unresolved live/platform obligations. For the skill family, eleven resources match the qualified owner bundle; five obsolete V1/V2 instruction or metadata resources are explicitly retired historical contracts, not native behavior replacements. Thirteen original installer correspondence rows are reused within the original 138 assertion-bearing lines, 33 named skills, 84 direct paths and 17 recursive entrypoints; full original acceptance and native admission remain separate. Retained installer and card-editor assertions still require their authentic external historical layouts. Current card/template schemas and native review, publish, finish and clean obligations are unchanged.

## Historical slow-proof family closure

Three exact historical paths retire the old four-family monolithic slow-proof config, runner, and already-refusal-only contract entrypoint. RD07-2118 explicitly supersedes its earlier retained classification because Runtime896 already removed its executable original assertions; full original/current custody remains pinned. No original slow-proof behavioral equivalence is claimed.

The public manager keeps empty family arrays and its nonempty-private-config refusal. Only unreachable removed-runner command generation is removed; direct helper calls also fail closed on nonempty private families. Existing public interfaces passed 12 cases, direct helper checks passed 2, and actual registry verification passed in the source application; those receipts are reused, not rerun here. Coverage producer/companion and the v0.92 evidence-validator twelve negatives remain unchanged.

Current total is 2,815 retirement actions, 2,814 baseline deletions and one transient bridge, including the same seven supplemental paths. Original 4,134 identities and 22 additional identities and ten holds remain unchanged.

## Infrastructure parser and producer-result resources

Six exact owner source/test resources transfer with their original consumer proof: CI timing, nextest timing and coverage producer-result verification. Current execution is 2,821 actions: 2,820 baseline deletions, including the same seven supplemental paths, plus one transient bridge. The original 4,134 identities, 22 additional paths and ten holds remain unchanged.

All three unchanged original scripts passed once against six authentic owner Git inputs in a network-isolated container without host mounts or credentials. The denominator is 12 producer process invocations and 29 assertion events (11 CI timing, 10 nextest timing and eight result-routing cases). Source bytes and modes matched before and after; stdout, empty stderr, durations and container evidence remain pinned. Synthetic nextest events are fixture data, not executed Runtime tests. The conditional toolchain test was excluded; no live CI, coverage production, cloud, provider, native lifecycle or product acceptance is claimed. Historical selector hints and owner navigation were repaired, and the actual registry was regenerated.
