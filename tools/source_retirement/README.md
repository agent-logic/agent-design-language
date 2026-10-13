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
