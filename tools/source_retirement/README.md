# Issue 1223 source retirement guard

The completed ledger records **3,407 retirement actions**: 3,405
execution-baseline deletions and two transient helpers. See
[`COMBINED_REMAINING_DELTA.json`](../../docs/milestones/v0.93.1/repository-decomposition/rd07-1223/COMBINED_REMAINING_DELTA.json)
for the final accounting. The staged totals below describe earlier execution
boundaries; they are not the final total. Subsequent repository changes, including
the separate podcast migration, do not rewrite those immutable records.

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

The native `source-retirement-validator` reads committed Git objects, including approval packets. It verifies
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
The original baseline-specific validator remains available in Git history;
the current run uses the combined ledger and execution overlay instead.

## Focused validation

Run `cargo test --locked --manifest-path tools/source_retirement/validator/Cargo.toml --test contracts`
for native contract and adversarial cases, including provisioned historical custody.
Run `cargo test --locked --manifest-path tools/source_retirement/validator/Cargo.toml --test successors`
for portable successor transition proof (also used by CI).
Run `cargo run --locked --manifest-path tools/source_retirement/validator/Cargo.toml`
only against the committed candidate; incomplete or changed execution manifests fail closed.
Run the same binary with `--cleanup-contracts` for the focused cleanup aggregate.
The immutable original constants are literal JSON in `reviewed-identities.json`,
with a SHA256 pin in native source. No interpreter is used. The old unused
70-removal validator is retained in Git history; the current 3,407-action
execution-overlay contract is unchanged.

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

## Historical skill-completeness and issue256 wrappers

Two explicitly historical wrappers retire with original and current custody, caller documentation repairs and actual registry regeneration. Current execution is 2,823 actions: 2,822 baseline deletions, including the same seven supplemental paths, plus one transient bridge. Original 4,134 identities, 22 additional paths and ten holds remain unchanged.

The original skill-completeness inventory remains 51 families and 205 inputs, with its missing Sprint Review schema and 65 ADL-only inputs preserved as adverse evidence. No complete original pass or current owner completeness is claimed. The issue256 validator was already a refusal stub; its fixed issue256/414/424 assertions remain in original custody. Configuration rows3733/3734 and grouped editor1917 retain independent obligations. No tests or live product behavior are credited to this historical retirement.

## CSM fixture validators and retained caller routing

Six committed CSM fixture resources transfer with three unchanged original test results and their exact source and rollback custody. Current execution is 2,829 actions: 2,828 baseline deletions, including the same seven supplemental paths, plus one transient bridge. The original 4,134 identities, 22 additional paths and ten holds remain unchanged.

Qualification used six authentic Infrastructure executable inputs and eleven frozen original ADL schema/document/HTML/JS/JSON fixture inputs. All three original tests passed once; 33 literal grep assertions are only part of their preserved validator, Node smoke, JSON, existence and leakage checks, not a fabricated total. R1 could not apply the nested sandbox and started zero originals. Reviewed R2 preserved the same sandbox, proved network and outside-write denial, and passed with exact before/after input bytes and modes. These composed fixtures do not establish standalone owner delivery or live browser/Runtime behavior. The report renderer missing from the owner and the Cargo demo remain separate gaps.

Both retained visibility-validator callers keep their original assertions through an explicit absolute owner root and exact source SHA/mode adapter. Eight inert routing/refusal checks passed with dispatch mocked; no original tests were repeated for this adapter. Actual registry regeneration and exact staged modes were checked at application.

## Medium resources, obsolete refusals and issue268 Runtime resources

Six Medium skill resources and thirteen issue268 Runtime resources transfer, and twelve unconditional historical refusal helpers retire, with exact committed source and rollback custody. Current execution is 2,860 actions: 2,859 baseline deletions, including the same seven supplemental paths, plus one transient bridge. The original 4,134 identities, 22 additional paths and ten holds remain unchanged.

The existing unchanged original contract passed in an isolated source-resource fixture using the original frontmatter validator. Five skill resources are byte-exact; the schema differs only in two example host paths, and the original test checks the schema reference rather than full schema behavior. The test is supplied externally and is not claimed as owner delivery. The retained ADL contract keeps its assertions and uses an explicit six-hash owner-root preflight; seven focused prefix checks passed without repeating the original suite. Guide navigation and the actual registry were repaired. This evidence does not qualify installed packages, native lifecycle, providers, publication or the separate Medium demo.

The twelve retired helpers already refused unconditionally; removing them supplies no original Runtime, GPU, cloud, native or Observatory acceptance. Five former retained classifications (1486, 1502, 1526, 2083 and 2088) are explicitly superseded while their prior records and original assertions remain preserved. Original and current bytes remain in pinned custody. The generated registry and GPU runbook reflect retirement; historical selector path hints are inert evidence. Active coverage, Guardian, the unresolved 2235 obligation and uninventoried helpers remain outside this batch.

The thirteen issue268 resources are pinned owner source at 7d0db620ad79c50ff5d8fabf0fb543183583d05c. Five unchanged original tests and one original validator passed their offline fixture; fifteen prefix checks, two inert payload transport cases and the retained 2087 script with six mocked wrapper invocations are separate proof denominators. Initial preflight failures remain preserved. Four caller scripts passed shell syntax checks. The new resource adapter verifies the complete pinned resource closure; remote payload and explicit relay-root routing preserve child arguments and status, without claiming remote execution. Legacy relay Cargo build behavior, installer1404/2017, installed issue607 consumption and live Runtime/provider/cloud acceptance remain separate obligations. Actual registry composition and both original/current and registry custody are pinned.

## Stock-league and historical workflow families

Six exact Runtime owner resources retire with original/current custody, owner guide navigation and regenerated registry. Ten historical native workflow wrappers also retire with current/frozen custody and preserved public CI policy: six refusal mutations, the valid fixture and seven deadline assertions passed. Their retired workflow acceptance is not relabeled as current native Runtime proof. The archive pair transfers with one original script, five producer invocations and 11 checks in R2 PASS; R1 executed zero originals and remains preserved. Exact owner/current custody is retained and no original test is repeated. The stock-league original Bash test passed, preserving 22 Python assertion expressions and three grep checks; file-loop assertions remain dynamic. All six input identities remained unchanged. No live discovery, model/provider, broker, cloud, native or Rust acceptance is claimed; adjacent v0.90 Rust stock-league contracts remain separate.

Current total: 2,878 executed actions, 2,877 baseline deletions (including seven supplemental paths), plus one transient bridge. Original 4,134 identities, 22 additional paths and ten holds remain unchanged.

## Metrics, Observatory and provider-neutral families

The v0.91.6 issue4441 regeneration helper and companion typed-v2 test retire under the prior reviewed historical disposition. One original test and six assertions remain in custody, and the original CSV/JSON/Markdown outputs remain byte-identical. Current tooling navigation marks their old refresh command historical; no obsolete ABI execution or native metrics equivalence is claimed.

Provider-neutral twelve resources transfer after one unchanged original Bash fixture test passed: nine Python expressions, one producer, one valid validator, five negative validator cases and one feed emission. Eight inert adapter cases passed separately. Seven historical JSON resources remain custody rather than new live receipt acceptance; no provider, browser, HTTPS, native or installed acceptance is implied.

Current total: 2,915 actions, 2,914 baseline deletions including seven prior supplemental and three separately observed paths, plus one transient bridge. Immutable 4,134 identities and separate 22 paths remain unchanged. Seven prior supplemental paths remain unchanged; three governed-room paths absent from the frozen inventory but present at the observed baseline are separately enumerated. Exactly seven Observatory holds resolve, leaving three holds. The 21 Node, 16 selector and 8 Layer8 results remain separate; browser/live/installed acceptance remains unexecuted and transferred, not claimed.

## Runtime operational selector pair

The exact reviewed selector and original companion test transfer with pinned custody and the applied packet proof limits. Only hold2110 resolves; distributed2235/2236 remain unresolved. No new execution or native admission is inferred by ledger integration.

Current total: 2,917 actions and 2,916 baseline deletions including seven prior supplemental and three separately observed paths, plus one transient bridge. Immutable4,134 identities and22 additional paths remain unchanged; two holds remain.

## Infrastructure packet, arXiv resources and serious review fixture

These three separately qualified families retire 7 Infrastructure packet tools, 6 arXiv resources and 3 CodeFriend serious-review fixture tools. Infrastructure proof is three unchanged offline Bash consumers. ArXiv proof is the unchanged retained ADL consumer with its shared frontmatter validator, not a standalone Runtime suite. CodeFriend proof is one unchanged original script: 14 required artifact paths, 8 grep assertions, 2 producer invocations, 3 leak-negative validator calls, and existing-output refusal/sentinel preservation. Its producer narrows temporary-output deletion; no byte-equivalence claim is made for that change. Documentary Cargo/skill commands were not executed. No live skill/provider, native lifecycle, installed binary, publication or product-build acceptance is inferred.

Current total: 2,933 actions and 2,932 baseline deletions, including seven prior supplemental and three separately observed paths, plus one transient bridge. Original 4,134 identities and 22 additional paths are unchanged; distributed holds2235/2236 remain.

## Saved-plan helper and PAIR accounting pair

The committed issue607 saved-plan helper retirement and the separately qualified PAIR producer/test pair add three exact captured removals. PAIR proof is 19 unchanged original unittest methods and two Python CLI children, with inline accounting and isolated loopback HTTP fixtures. Matrix attempts and parameterized subtests are not extra test methods; accounting success does not establish real Ollama/model or two-node Runtime qualification. The saved-plan batch retains its own applied-packet proof and historical command boundaries. No tests are rerun by ledger integration.

Current total: 2,936 actions and 2,935 baseline deletions, including seven prior supplemental and three separately observed paths, plus one transient bridge. Original 4,134 identities and 22 additional paths remain unchanged; distributed holds2235/2236 remain unresolved.

## Historical tooling, retired model demo and Runtime901 report contracts

Five historical helper contracts and the three-file Codex/Ollama demo family are retired with original assertions/custody preserved; no obsolete ABI, provider or model execution is credited. Runtime901 separately transfers four exact owner files after two unchanged original scripts passed 19+6 synthetic report-validation methods. This is not real provider or Runtime loss/recovery qualification.

Three existing ADL adapters1909/2087/2059 are explicitly retained under the pinned RETAINED_ADAPTERS_THREE record. ArXiv retains its ADL shared-validator resource contract; issue268 retains six mocked wrapper invocations; records-hygiene retains the complete-root historical installer/analyzer contract. Composed136/138 evidence does not establish a full2059 pass.

Current total: 2,948 actions and 2,947 baseline deletions, including seven prior supplemental and three separately observed paths, plus one transient bridge. Original 4,134 identities and 22 additional paths remain unchanged; distributed holds2235/2236 remain unresolved.

## Runtime905 speculative-retest safety pair

Two exact Runtime owner source files retire after the unchanged original 11-method safety suite passed with its existing lifecycle ModuleType and Ollama/subprocess/killpg mocks. This proves the original identity/refusal/cleanup report assertions, not live model execution, speculative-performance benefit, installed Runtime or native acceptance. The production lifecycle helper remains separate.

Current total: 2,950 actions and 2,949 baseline deletions, including seven prior supplemental and three separately observed paths, plus one transient bridge. Original 4,134 identities and 22 additional paths remain unchanged; distributed holds2235/2236 remain unresolved.

## Runtime documents, resilience and retained public contracts

Thirty Runtime guides, seven separately reviewed remaining guide/readback paths and three original-qualified resilience resources retire with exact source/custody boundaries. The seven-path batch preserves changed-owner prose differences, retires the AWS unconditional refusal entrypoint, and reuses the original five-case GCP fake-tool proof without a live cloud claim. Current navigation routes to immutable owner sources. No live/provider/cloud or original scenario acceptance is inferred from document custody. Two manuscript source documents remain local because the retained original test checks their existence; operational JSON fixtures remain unchanged. The shared provider setup page remains retained with its reviewed outgoing owner-link correction. Twelve documentation/fixture decisions and four public repository-quality/planning helpers are explicitly retained for their current consumers; no new test-pass claim is added.

Current total: 2,990 actions and 2,989 baseline deletions, including seven prior supplemental and three separately observed paths, plus one transient bridge. Original 4,134 identities and 22 additional paths remain unchanged; distributed holds2235/2236 remain unresolved.

## Historical aggregate retirement and retained public boundaries

Six v0.91.4 aggregate producer/tests retire as explicitly historical monolithic contracts; no original signed-CLI or lifecycle-filter acceptance is inferred. Two vLLM accounting sources transfer with original ten successful cases plus a corrected run of only the three setup-failed cases; initial failures remain retained and no GPU/model/performance claim is added. Four v0.87 reviewer-family helpers retire with their good/bad fixture assertions preserved historically and independent child demos unchanged. The suite aggregate is one explicitly enumerated additional supplemental baseline path outside both captured inventories. Two native command guides transfer to immutable selected owner documentation with explicit changed coordination-contract limits and no operation executed. Four inactive construction/example resources transfer as historical templates with exact owner custody, preserving all live deployment and behavioral qualification limits. The public ADL README repair and ten public or historical input/schema/document-tool retentions preserve real local consumer boundaries without owner equivalence claims.

Current total: 3,008 actions, 3,007 baseline deletions and one transient bridge. Baseline deletions include eight supplemental and three separately observed paths. Original 4,134 identities and 22 additional paths remain unchanged; distributed holds2235/2236 remain unresolved.

## Paper Sonata transfer and discontinued Unity retirement

Thirteen Paper Sonata source/resource paths transfer using the separately recorded original fixture proofs and exact selected-owner adapter. The full review aggregate and live providers remain unqualified. Ninety-four Unity-only paths retire under the explicit operator decision that Unity is unused, preserving original assertions and all failed editor/licensing/IPC evidence without treating discontinuation as editor acceptance. Ninety-three Unity paths belong to the captured inventory; one diagnostic document is an explicitly enumerated supplemental baseline path. Non-Unity validation remains retained.

Six ADL historical v0.91.5 packet validators/tests remain with actual two unchanged original script passes and four original offline checks from the repaired mixed script. Live provider and obsolete Cargo obligations are excluded explicitly. Runtime-init navigation now names historical owner resources; no deployment or current-default proof is inferred. The mandatory-Node dashboard test and exact #4675 historical scheduler fixture are explicitly retained, without new test acceptance claims.

Current total: 3,115 actions, 3,114 baseline deletions and one transient bridge. Baseline deletions include nine supplemental and three separately observed paths. Original4,134 plus22 captured additions remain immutable; both distributed holds remain unresolved.

## Historical Guardian and v0.88 aggregate closure

The Guardian refusal wrapper and companion retire together, preserving original report-deletion assertions and the genuine100cycle ignored-soak obligation without treating refusal as acceptance. Ten v0.88 monolithic aggregate resources retire with independent child obligations retained. DeepSeek historical ADL packet validation remains after removal of ambient credential-triggered production: two valid checks and one generic nonzero passed, but the negative failed packet-reference validation before score checking, not impossible-score rejection. Historical Software Development Polis packet validator and companion remain with their ADL evidence; no new pass claim is added.

Current total: 3,127 actions and 3,126 baseline deletions plus one transient bridge. Eleven supplemental and three separately observed paths remain outside immutable4,134+22. Both distributed holds remain unresolved.

## Generation, coverage and planning family closure

The legacy generation family preserves its unchanged original synthetic installer proof without claiming modern package or live service equivalence. The mixed private coverage family retires its original changed-file and mechanical-classifier semantics explicitly; current public coverage unit routing does not reproduce those historical contracts or establish current-head instrumented coverage. Original assertion custody and prior adverse evidence remain bound. Planning source retirement uses its separate committed packet and recorded proof boundary.

Current total: 3,147 actions and 3,146 baseline deletions plus one transient bridge. Eleven supplemental and three separately observed paths remain outside immutable4,134+22. Both distributed holds remain unresolved.

## Historical provider and residual wrapper closure

These committed families retire historical provider demonstrations, the Medium resource compatibility wrapper, the old Runtime compatibility interface, and the unconditional Observatory refusal stub. Exact original assertions and adverse evidence remain in custody. RD07-1957 prior retention is explicitly superseded by historical retirement; its five original paid calls remain unqualified. Inert routing checks are not live provider qualification. The current prompt-editor absence/schema guard remains retained without a new execution claim.

Current total: 3,178 actions and 3,177 baseline deletions plus one transient bridge. Twelve supplemental and three separately observed paths remain outside immutable4,134+22. Both distributed holds remain unresolved.

## Wrapper, stock and coverage residue closure

The committed source packets retire historical wrapper, stock, mixed coverage, Godel/AEE and benchmark residues with exact original custody and bounded proof distinctions preserved. Forty-four removals belong to the original inventory; the local authoritative coverage runner is an existing member of the separate22 additional paths, not a new supplemental path. Arxiv field-document validation and its two actual local guides remain with their original assertions; no new test pass or owner equivalence is inferred.

Current total: 3,223 actions and 3,222 baseline deletions plus one transient bridge. Twelve supplemental and three separately observed paths remain outside immutable4,134+22. Both distributed holds remain unresolved.

## Distributed archival and provider helper closure

Sixty-five committed original-inventory removals bind distributed historical/manual custody, provider suitability history and seven orphan deterministic mock helpers, CSM resources and exact adapter contracts. Both distributed holds close through their explicitly reviewed archival contract transfer; original historical three-platform evidence is not current Runtime execution. Ollama1441 remains for six surviving script consumers; Gemini1436 is now retired after both Provider15 callers; CSM four independently unresolved resources remain untouched. No original test or live acceptance is inferred.

Current total: 3,288 actions and 3,287 baseline deletions plus one transient bridge. Twelve supplemental and three separately observed paths remain outside immutable4,134+22. Zero explicit holds does not establish final public-package or whole-issue qualification.

## Legacy demo, standalone and audio documentation closure

48 committed original-inventory removals preserve exact source custody and each applied packet’s qualification boundaries. Historical monolith/editor-start demo interfaces are distinct from current Runtime routes. MIDI helper/fixture and audio document custody is not media acceptance. Original podcast policy2038 remains an active installer-facing fixture contract; no new original pass is inferred. The manual Hey Jude renderer, original aggregate and validators are retired historically, preserving the later explicit-source preflight and all original assertions without a media qualification claim. DeepAgents original fixture proof remains separate; CSM17 historical retirement explicitly supersedes prior retentions1514/1515/1516. Two public HTML demo launchers and two CCC report examples remain retained for actual ADL consumers.

Current total: 3,336 actions and 3,335 baseline deletions plus one transient bridge; twelve supplemental and three separately observed paths remain outside immutable4,134+22. Zero explicit holds does not establish whole-issue qualification.

## WP12, operational and status-family closure

This batch records 48 exact committed source removals: 47 captured original paths and one second transient helper introduced and retired during #1223. Original assertions and adverse evidence remain in custody; manual or historical transfers are not new live qualification. Fourteen exact reviewed retentions preserve nine ADL historical packet validators and five local host/repository status utilities. These source-retention decisions claim no new validator, platform, cloud or Runtime pass. The new transient Runtime resource adapter is restricted to its exact path, authenticated introduction4df3ff23, removal7783ee53, immutable bytes and rollback custody. The original conversion bridge restriction remains intact.

Current total: 3,384 actions, 3,382 baseline deletions and two transient removals. Twelve supplemental and three separately observed baseline paths remain outside original4,134+22; zero explicit holds is not whole-issue acceptance.

## Additional captured source closure

The final22-removal batch resolves21 remaining captured additional paths and separately retires the historical candidate builder. No original4,134 row changes. Ten unused compatibility facades leave their canonical public packages untouched; excluded CLI source and six orphaned C-SDLC paths retain original/current custody and explicit successor scope. The historical quality aggregate and three source-candidate tests retain their assertions as history; no old coverage command or candidate archive is claimed to pass at current HEAD. Current public package/CLI validation remains separate.

The builder is absent from frozen248f, introduced at8cfd5af, and present at observed9cfd; its exact current bytes/custody and removal are approved through the pinned source packet. It is the thirteenth supplemental baseline path, outside original4,134+22. Current total: 3,406 actions, 3,404 execution-baseline deletions and two unchanged transient removals; three separately observed governed-room paths remain distinct. All original/additional proposals are resolved, without claiming whole-issue acceptance.

The reviewed incoming-main merge49effa02 is a separate zero-removal source reconciliation: its exact CI, planning, registry and incoming lifecycle-record identities are chained from the first parent. Incoming-only excluded checker custody adds no retirement action. Infrastructure evidence update10a1a6a3 is a records-only reconciliation and does not alter source accounting.

## Final unused private dispatcher

RD07-0486 (`adl/src/cli/run.rs`) supersedes its obsolete pre-public consumer-repair retention. The current compiled public CLI never includes this private engine file. The exact current/original bytes, owner correspondence and original behavior boundaries remain in ORPHAN_PRIVATE_RUN_ONE_APPLIED.json. This adds one original-row removal: 3,407 actions, 3,405 baseline deletions and two unchanged transient deletions, with zero unresolved original/additional rows. The earlier 57-case retirement proof and public qualification ran at ad8204; their reuse is source-correspondence evidence, not a new execution at this integration head.

## Public checks after history archival (#1382)

Public PR CI runs these self-contained checks against the current committed tree:

```sh
cargo test --locked --manifest-path tools/source_retirement/validator/Cargo.toml --test public_boundary
cargo test --locked --manifest-path tools/source_retirement/validator/Cargo.toml --test contracts portable::
```

The boundary policy retains 157 exact provenance-manifest hashes, 4,718 explicit
retired paths, four retired prefixes and 25 public package/dispatch entrypoints.
Tests reject restored private source, missing public entrypoints, changed custody
manifests and malformed policy. Ordinary public source edits remain possible;
this is not a freeze on every retained blob. Policy changes require an explicit
reviewed amendment to the policy and its compiled SHA-256 pin.

The original `successors`, full `contracts`, `retirement_contract` and default
audit command remain historical audit tools. Run them at archived main
`21c8acd018e13dc2bc50b124210646e816c5190b` in a restored history checkout with
its separately archived custody files. They intentionally require the original
Git graph and are not public CI dependencies after a compact-history cutover.
Neither a missing old object nor a missing custody file counts as audit success.
See [the archive and cutover contract](../../docs/archives/PUBLIC_HISTORY_1382.md).
