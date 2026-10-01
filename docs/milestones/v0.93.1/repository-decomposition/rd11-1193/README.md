# RD11 compatibility lockset and rollback input

These source files implement existing issue1193's bounded compatibility input. They do not accept the split, activate shared tooling, install packages or edit lifecycle state. Accepted RD07 remains the final acceptance prerequisite. The supplied candidate deliberately has all seven selections pending. Its available facts preserve original evidence, not inferred current releases.

## Read-only validation

Python3.11+ uses the existing public ADL verifier's digest primitive; no new installer, downloads, build dispatch or native authority is introduced.

```sh
python3 tools/public_adl/verify_compatibility_lockset.py \
  --lockset docs/milestones/v0.93.1/repository-decomposition/rd11-1193/candidate-lockset.json \
  --graph docs/milestones/v0.93.1/repository-decomposition/rd11-1193/interface-graph.json \
  --rollback docs/milestones/v0.93.1/repository-decomposition/rd11-1193/rollback-catalog.json
```

This accepts only a well-formed candidate structure. Add `--require-qualified --evidence-root EVIDENCE_BUNDLE` to require actual referenced bytes and complete original qualification. Any lockset declaring state `accepted` always takes this stricter route; changing the string cannot bypass it. Missing selections, missing/corrupt artifacts/evidence, unresolved rollback, failed/zero/skipped operations, missing negatives or absent acceptedRD07/review references refuse with exit2. Machine-readable results go to stdout, bounded refusal diagnostics to stderr. The checker is read-only and does not convert the candidate into an accepted lockset.

## Exact contract

The validator is the executable schema: it rejects unexpected keys and duplicate JSON keys. Exactly seven unique role/repository identities are required: public_adl, csdlc, runtime, infrastructure, enterprise_adapter, codefriend, website. Each role retains available manifest observations separately from `selected`. No package generation or platform is selected by the source candidate.

A selected identity has the original seven fields: source_commit (40hex), asset_identity (`name` plus evidence-bundle-relative `path`), sha256 (64hex of actual asset bytes), package_version, dependency_lock (path/SHA), build_provenance (path/SHA), supported_consumer_versions (explicit nonempty owner-declared versions/ranges). Qualified mode reads and hashes the real artifact, locks, provenance, authentication and proof files. All references are confined to the supplied evidence root with no symlinks or traversal. Mutable latest/main/master asset names are refused. Local host paths are forbidden in public records. Package/archive/platform distinctions must remain explicit; same package version does not equate differing bytes.

Software platform rows bind exact source and artifact digest, target, independently clean/distributable-only assertions, and all three build/test/install command/results with positive executed counts and zero failed/skipped credit. The normalized row points to actual retained evidence; it does not replace that evidence. Multiple supported platforms require separate rows. Website uses explicit `bundle_api_compatible` or `preserved_no_deployed_baseline` disposition with owner evidence, not fictitious software test counts. No deployed API compatibility is inferred from static source.

Graph edges are consumer→producer and retain kind build/distribution/runtime/api, actual interface/version, optionality and source evidence. Known engine/records, remote-validation, Runtime source and policy interfaces are seeded from retained manifests. Unknown optional-enterprise/API pair names remain null; qualified mode refuses unresolved edges, or the owner explicitly removes an unsupported API claim with its website absence disposition. Build/distribution cycles, a CSDLC dependency on Runtime, a mandatory enterprise dependency in Runtime, unknown nodes and removed core dependency edges are rejected. Graph records are normalized producer-manifest claims: this checker is not a second Cargo resolver or a replacement for clean-consumer hidden-sibling tests.

Rollback contains exactly seven roles, prior observations, explicit pending/retained_prior/no_prior_accepted_baseline disposition, prior full identity, existing restore argv, and recovery/absence evidence. Pending cannot qualify. No-prior disposition does not invent a prior asset or restore command and still requires reviewed evidence. A retained prior distribution requires actual asset and metadata hashes plus an existing installer/restore command and evidence; the checker never executes it. Retained Runtime dependency-lockset is not a prior seven-role accepted RD11 lockset.

Qualified mode requires all original negative categories in the candidate contract, each with an observed refusal, positive executed denominator and hashed evidence. The original missing-product, mixed-owner and unqualified-shared-activation requirements remain mandatory; artifact/version/source/cycle/optional-adapter negatives make the original detailed plan explicit. `active_source_owners` permits one owner at most, and qualified records require one; `activation` is always none. This validator supplies no ownership lease or activation authority.

## Authentication and acceptance boundary

A hashed file, normalized PASS statement or successful checker is not producer authentication, independent review or native acceptance. The owner independently authenticates repository/head/run/job/artifact identities, supplies original proof logs/receipts and reviews normalized claims against them. The checker verifies correspondence and rejects missing/corrupt referenced material; it cannot infer semantic truth from arbitrary receipt formats. Output always states acceptance_authority=false and activation_performed=false, including on synthetic tests or records already labeled accepted. Native1193 acceptance remains with its authorized owner after actual selected-platform evidence, review and rollback requirements are satisfied.

Existing RD04/05/06 acceptance stays unchanged. RD06's retained final review includes terminal closeout; no Linuxwarm/cloud gate is added. Linux policy is not Linux Runtime service support. CodeFriend's draft resource/source packets are not accepted binary packages. Website/Beta1 historical absence is not a deployment or paid-provider instruction. The candidate records observed dependency evidence only; final package selection and current supported ranges remain owner inputs.

## Focused validation after early draft

```sh
python3 -B -m unittest discover -s tools/public_adl -p test_compatibility_lockset.py -v
```

23 source tests cover pending/qualified structure, missing/corrupt proof, byte mismatch, zero/skipped/failed results, role duplication, JSON duplicates, confinement, mutable pins, mixed ownership, forbidden activation, CSDLC independence, optional enterprise, build cycles, missing edges and rollback/negative completeness. Their complete positive fixture is explicitly synthetic and demonstrates no acceptance authority. It is not installed-product evidence. Actual product build/test/install remains performed through each existing owner verifier/installer on selected artifacts; no heavy product suite is run by this script.

## Concrete dependency and asset identities

Required consumer/producer/kind/interface tuples are fixed to the retained product contracts, including both adl-engine and adl-records and Infrastructure consuming Runtime runtime-candidate.tar. Deletion, kind downgrade, or optional substitution refuses; versions and new selected generations remain owner inputs. The Infrastructure edge is grounded in accepted source 319ad5c contract contracts/runtime-rd05.json (infrastructure.artifact-inputs.v1), with retained original four-scenario receipt; this does not select a new Runtime generation.

Selected asset uniqueness is (source commit, immutable producer asset name, artifact SHA-256, optional bundle-member path and SHA-256). A local transport path is deliberately excluded so a copied artifact cannot defeat duplicate refusal. Asset names must be authenticated by the owner; different asserted names are not proof of different producer assets. For a shared tar bundle, asset_identity may additionally contain member: {path, sha256}; qualified correspondence reads that exact unique regular member without extraction and verifies its bytes. Two distinct actual members may supply separate products. Duplicate members and overlapping whole-bundle/member selections refuse. This documents transport membership, not an installer or acceptance authority. Existing non-bundle asset identities remain unchanged.
