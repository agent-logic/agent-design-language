# RD-01 final-predecessor ownership audit — #1181

## Result and remaining decisions

Complete census: **36,899 tracked paths**, each with one candidate product owner or an explicit accountable RD-02 decision. This completes the inventory denominator; it does not settle all ownership or establish independent extraction. Unresolved decisions block affected extraction, not acceptance of an audit that exposes them.

The original #977 findings remain relevant: public ADL still has unconditional private implementation dependencies; independent C-SDLC delivery needs owned resources and authority history; mixed build/CLI, memory, enforcement and delivery surfaces need component-level decisions. No builds, provider runs, live-runtime changes, private website inspection or extraction probes were performed for this audit. Historical failed probes and #915 qualification outcomes retain their original meaning.

## Immutable inputs and current authorization

- Original #977 census baseline: `f69019c24a9b61511e912c93f95442f96fa66d92`, 31,813 paths. This is the audited source, not #977's reviewed output head `e205107028b1f4ac735fdb9f6a2e306d122736b4` or merge `9c18d682bf28f88db8b47279ac2ab5ab2e41db68`.
- Frozen final v0.92.2 predecessor: `248f00e359e412f6bc0061ac88be9ff374e3a212`, final Sprint 11 PR #1157 merge after #1177.
- Opening integration: `08236f86026f2e317e7f405bb0381b782a1417b9`, including #1195/#1197 repairs. Its 36,900 paths are accounted separately.
- Accepted [opening checkpoint](opening-checkpoint.json), SHA256 `513082f2172880c6e5555be065cff0d3b7d0133464b05cf319c213b8a8b3570f`, authorizes bounded RD-01 under the operator's Sprint 1 execution instruction. The historical preparation-only records remain unchanged. #1178 remains OPEN; checkpoint acceptance does not close WP-01.

This audit's outputs are new evidence, outside the frozen input denominator. Later current-main changes require their own delta; neither current HEAD nor preparation repairs silently substitute for the frozen predecessor.

## Evidence and ownership decisions

[summary.json](summary.json) binds compressed full-path census and both delta ledgers by SHA256. Each census row contains exact Git path, type, mode, object and byte size, current disposition, and #977's previous classification where present. The NUL-delimited Git inventory preserves unusual names and represents symlinks/gitlinks without following them. No source text, credential contents or private repository payload is copied into these manifests.

The [decision register](decisions.json) names RD-02 #1182 and source evidence for every unresolved group:

| Decision | Paths | Required decision |
|---|---:|---|
| D-HISTORY | 31,648 | Evidence custodian, provenance, archive/privacy and cross-repository links; no wholesale active-source reassignment |
| D-MIXED-ADL | 1,937 | Component-level language/Runtime/CodeFriend interfaces, build/CLI, memory, providers and enforcement |
| D-DOCS | 849 | Cross-product manuals, ADRs and template consumers versus retained history |
| D-DEMOS | 420 | Each demo, runner, fixtures and editorial material follows its actual owner; RD-08 later verifies |
| D-DELIVERY | 334 | Product-specific host/CI/install versus reusable infrastructure and artifact contracts |
| D-GENERATION | 287 | adl-v2 adapters, characterization and resilience; retained v2 distribution or explicit retirement |
| D-ROOT | 16 | Root governance and remaining cross-product assets |

The 1,408 candidate product paths comprise C-SDLC 937, Runtime 309, CodeFriend 112 and public ADL 50. Candidate means proposed ownership, not RD-02 approval, export allowance or installed independence. In particular C-SDLC v2 has a candidate implementation owner but a separate retained-resilience distribution gate.

Unlike #977's `historical-evidence` heuristic category, this census records retained evidence as a concrete unresolved custody/policy decision. Therefore the 35,491 decision-bound paths must not be compared to #977's 9,466 unresolved paths as a regression metric. Counts changed with explicit classification policy as well as source growth. Per-path previous owner/status remains visible for comparison.

## Runtime, Observatory, documentation, tests and demos

At the frozen revision, `adl-runtime/Cargo.toml:34–35` requires resilience and kernel; kernel's manifest requires provider-core. The mixed `adl/Cargo.toml:88–91` unconditionally imports provider-core, UTS, Runtime and kernel. Public ADL independence remains unproven. Runtime core enforcement, continuity, resident lifecycle, provider execution, CSM and Observatory belong with Runtime; enterprise policy may narrow authority through a versioned interface, not replace baseline enforcement.

`adl/src/lib.rs` exports both portable language/planning modules and CSM, provider, memory and CodeFriend modules. `adl/src/runtime_v2/mod.rs` combines operational contracts with private-state, security, governance, demo and CodeFriend-adapter obligations. The module name does not establish retirement or settle every shared schema. These mixed files remain D-MIXED-ADL until RD-02 records component/API ownership. Likewise `memory_palace.rs` must be split by consumers, not its name.

The 17 `demos/html-observatory/` paths are Runtime candidates: its README explicitly documents the Runtime-owned observatory feed/readiness/control contracts. Other 420 demo paths include 162 podcast paths and versioned demos/fixtures; retain them in D-DEMOS pending individual product/editorial custody. Tests and fixtures under a cohesive crate follow that candidate owner; mixed `adl/tests` and shared runners remain explicit decisions. Broad test success would not decide those boundaries.

Runtime manuals under `docs/runtime*/` and lifecycle manuals under `docs/csdlc-v3/` / `docs/cognitive-sdlc/` have cohesive candidate owners. Mixed docs, architecture decisions and portable template families require consumer-level review; milestone records have historical custody decisions. A private destination does not erase already-public history or authorize moving restricted evidence.

## Differences from #977 and opening repairs

`delta-977.jsonl.gz` records **5,086 additions and 271 modified paths, zero deletions**, with full before/after Git metadata. Unchanged paths remain represented in the census, including previous ownership. No rename inference hides additions/deletions. The net source increase is 5,086 paths, not a product-readiness metric.

`delta-opening.jsonl.gz` separately records **41 modified paths and one addition** after the frozen predecessor, each with an ownership/decision disposition. #1195 changes semantic storage and installed lifecycle tests; #1197 changes planning documents, validator/tests and historical-verification routing. These are real changes, not discarded as mere preparation, and do not alter the frozen denominator.

The [CodeFriend/infrastructure/security survey](CF_INFRA_SECURITY_SURVEY.md) checks 639 selected frozen paths (256 added, 23 modified, 360 unchanged) and five cross-product decisions. Its lexical scope is not additive to the whole census or comparable to #977's eight CodeFriend-classified paths. Its recommendations are consolidated into D-MIXED-ADL and D-DELIVERY. Current policy separates proposed `codefriend` software from existing `codefriend.ai` website, superseding #977's reuse recommendation without rewriting historical evidence.

The [ADL/C-SDLC survey](ADL_CSDLC_SURVEY.md) independently covers 28,797 frozen paths. It identifies 646 added C-SDLC source/test paths and 3,696 added lifecycle-evidence paths, confirms active v3 has no Rust Runtime dependency, and lists the command manifest, template registry, worktree policy, manuals and shared installer/CI as required distribution companions. Its narrower candidate suggestions for portable adl-v2 and mixed monolith paths remain decisions in this conservative integrated census. This prevents overlapping survey recommendations from silently becoming approved file moves.

## Validation and handoff

PVF: deterministic local docs/planning contract, bounded Git/Python and Rust harness, no network/providers/cloud. Required issue acceptance gate, not installed-product or release proof. Run `python3 docs/milestones/v0.93.1/repository-decomposition/rd01-1181/census.py` or the focused native-declared Cargo target `rd01_1181_census`.

The verifier compares complete tree metadata and both deltas to pinned Git objects, hashes, previous ownership and aggregate counts. Six negative scenarios reject omission, duplication, metadata corruption, hidden unresolved ownership, dropped historical delta and dropped opening delta. Independent review and native proof receipts bind the actual candidate and outcomes; the mere presence of this paragraph is not a test result.

RD-02 must resolve the named decisions, names/licenses/registries/access and rollback before affected extraction. No destination bootstrap, issue transfer, launch, shared-owner refresh, paid effect or merge is authorized by this audit. No later-sprint or closeout issues were created.
