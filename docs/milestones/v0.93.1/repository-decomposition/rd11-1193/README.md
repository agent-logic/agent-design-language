# RD11 compatibility lockset and rollback input

These source files implement issue 1193's bounded compatibility candidate. They do not accept the split, activate shared tooling, install packages, or manufacture producer proof. All seven roles now have immutable selections grounded in retained evidence. CodeFriend consumes its already-accepted Linux build/install/test proof through an exact correspondence row; no product run is repeated. `public_adl` selects the exact five-package 1889916a qualification report and artifacts; the operator deferred issue 1223 source retirement to a separate path so it does not block Sprint 1. C-SDLC selects the final PR 88 package with hash-backed installed-result correspondence while preserving native-admission, full-original-acceptance, active-prefix and fresh audio/provider non-claims. Runtime issue 39 qualification remains unproven and was explicitly moved outside Sprint 1 under WorkerBee #17; accepted RD05/RD08 product evidence remains distinct. Available observations remain separate from selected identity, and this candidate remains non-accepted.

## Read-only validation

Python3.11+ uses the existing public ADL verifier's digest primitive; no new installer, downloads, build dispatch or native authority is introduced.

```sh
python3 tools/public_adl/verify_compatibility_lockset.py \
  --lockset docs/milestones/v0.93.1/repository-decomposition/rd11-1193/candidate-lockset.json \
  --graph docs/milestones/v0.93.1/repository-decomposition/rd11-1193/interface-graph.json \
  --rollback docs/milestones/v0.93.1/repository-decomposition/rd11-1193/rollback-catalog.json
```

This accepts only a well-formed candidate structure. Add `--require-qualified --evidence-root EVIDENCE_BUNDLE` to require actual referenced bytes and complete original qualification. Any lockset declaring state `accepted` always takes this stricter route; changing the string cannot bypass it. Missing selections, missing/corrupt artifacts/evidence, unresolved rollback, failed/zero/skipped operations, missing negatives or absent acceptedRD07/review references refuse with exit2. Machine-readable results go to stdout, bounded refusal diagnostics to stderr. The checker is read-only and does not convert the candidate into an accepted lockset.

`evidence-manifest.json` inventories every distinct evidence-root path and expected SHA-256 digests referenced by the lockset, graph, rollback catalog, and selected artifacts. The evidence bytes remain in the separately retained Sprint 1 execution bundle; the tracked manifest does not copy, authenticate, or activate them.

## Exact contract

The validator is the executable schema: it rejects unexpected keys and duplicate JSON keys. Exactly seven unique role/repository identities are required: public_adl, csdlc, runtime, infrastructure, enterprise_adapter, codefriend, website. Each role retains available manifest observations separately from `selected`. The source candidate selects exact retained generations; it does not claim platform qualification for rows that still lack original owner proof.

A selected identity has the original seven fields: source_commit (40hex), asset_identity (`name` plus evidence-bundle-relative `path`), sha256 (64hex of actual asset bytes), package_version, dependency_lock (path/SHA), build_provenance (path/SHA), supported_consumer_versions (explicit nonempty owner-declared versions/ranges). Qualified mode reads and hashes the real artifact, locks, provenance, authentication and proof files. All references are confined to the supplied evidence root with no symlinks or traversal. Mutable latest/main/master asset names are refused. Local host paths are forbidden in public records. Package/archive/platform distinctions must remain explicit; same package version does not equate differing bytes.

Software platform rows bind exact source and artifact digest, target, independently clean/distributable-only assertions, and either three separately executed build/test/install commands or one exact adapter invocation with separately counted build/test/install phase outcomes. Both forms require positive phase denominators and zero failed/skipped credit; a single adapter invocation cannot be counted three times. One narrow Runtime-only retained-acceptance form preserves the already accepted RD05 fresh build, installed six-binary identity, seven independently reviewed scenarios, and exact artifact while explicitly recording that the historical aggregate install argv was not retained and current issue 39 qualification was operator-deferred outside Sprint 1 to WorkerBee #17. It cannot be used for another role or relabeled as issue 39 completion. The normalized row points to actual retained evidence; it does not replace that evidence. Multiple supported platforms require separate rows. Website uses explicit `bundle_api_compatible` or `preserved_no_deployed_baseline` disposition with owner evidence, not fictitious software test counts. No deployed API compatibility is inferred from static source.

Graph edges are consumer→producer and retain kind build/distribution/runtime/api, actual interface/version, optionality and source evidence. Known engine/records, remote-validation, Runtime source and policy interfaces are seeded from retained manifests. Graph v2 permits one narrow `independently_built_no_dependency` disposition for CodeFriend's historical Runtime source edge when the disposition exactly matches the selected CodeFriend source, artifact, dependency lock, build provenance, authentication, and hash-backed final-delivery evidence. This records the final standalone product manifest; it is not a general edge waiver. Unknown optional-enterprise/API pair names remain null; qualified mode refuses unresolved edges. Build/distribution cycles, a C-SDLC dependency on Runtime, a mandatory enterprise dependency in Runtime, unknown nodes, and removed core dependency edges are rejected.

Rollback contains exactly seven roles, prior observations, explicit pending/retained_prior/retained_installed_generation/no_prior_accepted_baseline disposition, and recovery/absence evidence. Pending cannot qualify. No-prior disposition does not invent a prior asset or restore command and still requires reviewed evidence. A retained prior distribution requires actual asset and metadata hashes plus an existing installer/restore command and evidence; the checker never executes it. The narrow retained-installed-generation form instead requires exact source, generation, platform, artifact hashes, accepted receipt, consumer identity, historical recovery evidence, and `replay_authority=false`; it forbids a replay argv when only the original historical restore transcript remains. This models Runtime's actually accepted generation rollback point without relabeling it as a complete distribution. Retained Runtime dependency-lockset alone is not a prior seven-role accepted RD11 lockset.

Qualified mode requires all original negative categories in the candidate contract, each with an observed refusal, positive executed denominator and hashed evidence. The original missing-product, mixed-owner and unqualified-shared-activation requirements remain mandatory; artifact/version/source/cycle/optional-adapter negatives make the original detailed plan explicit. `active_source_owners` permits one owner at most, and qualified records require one; `activation` is always none. This validator supplies no ownership lease or activation authority.

## Authentication and acceptance boundary

A hashed file, normalized PASS statement or successful checker is not producer authentication, independent review or native acceptance. The owner independently authenticates repository/head/run/job/artifact identities, supplies original proof logs/receipts and reviews normalized claims against them. The checker verifies correspondence and rejects missing/corrupt referenced material; it cannot infer semantic truth from arbitrary receipt formats. Output always states acceptance_authority=false and activation_performed=false, including on synthetic tests or records already labeled accepted. Native1193 acceptance remains with its authorized owner after actual selected-platform evidence, review and rollback requirements are satisfied.

Existing RD04/05/06 acceptance stays unchanged. Runtime issue 39 remains unqualified and outside Sprint 1 under the explicit operator decision; the retained RD05 row does not claim otherwise. RD06's retained final review includes terminal closeout; no Linuxwarm/cloud gate is added. Linux policy is not Linux Runtime service support. CodeFriend's draft resource/source packets are not accepted binary packages. Website/Beta1 historical absence is not a deployment or paid-provider instruction. The candidate records observed dependency evidence only; final package selection and current supported ranges remain owner inputs.

## Focused validation after early draft

```sh
PYTHONPATH=tools/public_adl python3 -m unittest \
  tools.public_adl.test_compatibility_lockset.Lockset.test_independently_built_codefriend_disposition \
  tools.public_adl.test_compatibility_lockset.Lockset.test_independent_disposition_requires_evidence \
  tools.public_adl.test_compatibility_lockset.Lockset.test_independent_disposition_matches_selected_identity_and_sources
```

These focused tests cover the graph-v2 standalone disposition's positive path, required evidence, and exact selected identity/source correspondence. Their complete fixture is explicitly synthetic and demonstrates no acceptance authority. The real candidate is checked separately with `--evidence-root`. Actual product build/test/install remains performed through each existing owner verifier or installer; this script does not rerun product qualification.

The complete synthetic contract suite remains available and currently contains 29 tests:

```sh
python3 tools/public_adl/test_compatibility_lockset.py
```

Its fixtures construct pending and qualified states independently of the evolving real candidate and treat a required interface as satisfied by either its concrete edge or the single permitted graph-v2 disposition.

## Concrete dependency and asset identities

Required consumer/producer/kind/interface tuples are fixed to the retained product contracts, including both adl-engine and adl-records and Infrastructure consuming Runtime runtime-candidate.tar. Deletion, kind downgrade, or optional substitution refuses; versions and new selected generations remain owner inputs. The Infrastructure edge is grounded in accepted source 319ad5c contract contracts/runtime-rd05.json (infrastructure.artifact-inputs.v1), with retained original four-scenario receipt; this does not select a new Runtime generation.

Selected asset uniqueness is (source commit, immutable producer asset name, artifact SHA-256, optional bundle-member path and SHA-256). A local transport path is deliberately excluded so a copied artifact cannot defeat duplicate refusal. Asset names must be authenticated by the owner; different asserted names are not proof of different producer assets. For a shared tar bundle, asset_identity may additionally contain member: {path, sha256}; qualified correspondence reads that exact unique regular member without extraction and verifies its bytes. Two distinct actual members may supply separate products. Duplicate members and overlapping whole-bundle/member selections refuse. This documents transport membership, not an installer or acceptance authority. Existing non-bundle asset identities remain unchanged.
