# TESTS/PROOF — findings and source coverage

Candidate `5c4a6149771c637f3c805985b86231077965eab4`; baseline `c9cdb2f13ff77a06dcc79ad4e4c332f444b2e14f`.

## Findings

- **SYN-006 P1 (historical, still current):** Independent `adl-characterization`, `adl-resilience`, and `tools/remote_validation` graphs still have no change-triggered CI owner. Three exact-candidate selector probes each return no lanes and skipped aggregation. `tests-history/syn006-routing.json` preserves the inputs/results; original severity is retained.
- **TESTS-003 P2:** #905 retest accepts two unknown base-blob digests as equal, then emits `same_base_blob=true`. `tests-905-identity-negative.json` evaluates the actual identity function offline and demonstrates acceptance with distinct FROM aliases and manifest digests. This does not assert differing weights in the retained historical run.

- **TESTS-001 P2:** Observatory accessibility regex accepts removal of both transcript aria-live attributes. Exact candidate test passed after the two attributes were removed from an isolated copy. See tests-findings.json and tests-accessibility-negative-repro.json. Existing UI attributes remain present; this is a regression-proof defect.
- **TESTS-002 P2:** The paused-clock shepherd shutdown test admits a full 600-second virtual probe timeout because its oracle merely awaits completion. An isolated same-version Tokio counterexample passes that oracle while asserting 600 virtual seconds elapsed. See tests-virtual-time-repro/result.json. The current production cancellation select is correct; this is a regression-proof defect, not an observed production shutdown delay.

## Evidence and limits

Eight bounded local commands passed, with denominators captured in tests-local-proof.json. This includes packet self-validation, CI selector/aggregator cases, artifact unit tests, mocked Observatory projection tests, and four browser-source/static Node contracts. These are not live browser, cloud, provider, installed-release, or end-to-end product proof. The isolated virtual-time counterexample adds one passing test; its zero doctests are non-proving.

The #917 Rust bridge actually delegates to its Python checks and propagates failure, checks explicit counts and negative fixtures, and rejects acceptance overclaims. Running that inherited validator against the later candidate changes correctly fails its old inventory. This is **not a finding**: #918 explicitly validates inherited #917 blobs at their pinned source revision, while independently validating the new packet. The revised release notes need not match their historical checkpoint hash.

Reviewed tests generally distinguish synthetic evidence admission from real qualification. Codefriend assessment tests cover evidence byte spans, UTF-8/CRLF boundaries, ambiguity and partial outcomes, and persistence/readback; they do not establish live provider behavior. CI contract tests execute selector and aggregate blocks with negative outcomes rather than relying solely on expected strings. Coverage routing changed-hunk assertions cover newly introduced production owners, but broad runner coverage is not claimed.

## Coverage, independence, and remaining work

This reviewer did not author the candidate implementation or the #917 validation bridge. The lead authored #917; its bridge was independently scrutinized here. `tests-coverage.json` is the authoritative per-path ledger; inventory discovery and generated fixture counts are not inspection credit. `tests-unresolved-source-paths.json` enumerates exact outstanding source/config groups for cross-lane reconciliation. The path-name selector does not itself cover every embedded production test; code lanes must account for those explicitly. This report records findings and scoped source coverage; it is not PASS, release approval, or a blanket execution claim.

## Completed source chunks and complementary ownership

All changed CodeFriend test sources, tool test sources, 60 changed ADL fixture/PVF files, small retained proof manifests, and full changed CI hunks have now been inspected. This includes actual Store/retention/no-replay consumers, two-cycle original ownership, resealed artifact corruption, relocated binary HTTP tests, source-built full 18/19-stage Journey tests, and the 916–918 validation bridges. Component Journey tests use loopback model responses and synthetic approval; they do not establish real-model, human-approved installed acceptance.

The ledger currently credits 195 directly inspected source paths, 22 complete changed-hunk surfaces, 30 authored complementary source paths and 606 fixture-cohort classifications. All assigned source rows are now resolved. Embedded production tests are credited only by their code-lane source ranges. The C-SDLC supplementary ledgers retain exact review ranges and observations; no Rust integration suite was run in this final static pass.

Historical D520-TEST-001, D520-SEC-003, and D520-SEC-004 were rechecked against current source and local bounded proof in tests-history-crosswalk.json: pending-work synchronization is present; redaction checks reject four mutated fixtures; local-provider proof rejects hostile origins/redirects in two offline Rust tests. No current full provider or Runtime suite was executed.

Complementary website execution: lead ran the frozen website snapshot full suite, **141 passed, zero failures/skips/cancellations**, retained in `website-tests.log`, with locked npm install and install scripts disabled. Node26/npm11.12.1. Network cases use local fixtures; this is not deployed product acceptance.

## Exhaustive fixture classification

`tests-fixture-cohorts.json` accounts for all606 C-SDLC fixture paths individually:109 seven-role copied-record files,456 serialized historical observation files,30 predecessor-writer files and11 PVF/transport contracts. Every JSON/blob and audit row parses; Markdown is UTF8/nonempty, lock placeholders empty, PNG headers/dimensions valid. All43 portable manifest and redaction-chain hashes match. Historical prose and payload values were not manually reviewed as product logic. The consumer ranges establish their isolated copy/normalization purpose and mark observation seeds non-proving. This is exhaustive shape/provenance classification, not a claim that all606 files are independently authored tests or that conversion behavior passed.

No extra fixture finding: two nested505 redaction digests are declared intermediates consumed by the root redaction manifest; the complete chain matches current bytes. A generic top-level-generation assumption was corrected to actual semantic_commit.v3 payload/audit structure before accepting shape results.

## C-SDLC integration review completion detail

`tests-csdlc-additional-coverage.json` adds22 authored source reviews:15 complete files and7 complete changed-hunk surfaces with fixture and assertion context. `architecture-integration-test-coverage.json` credits six full sources; `architecture-coverage.json` credits the three remote test sources. `tests-installed-intent-addendum.json` credits all7,178 lines and114 test entrypoints of installed intent commands independently reviewed by the security specialist. Source counts are not executed denominators.

Conversion tests use the separate Cargo-built converter and isolated copies of seven historical roles; authentic old-binary rehearsal remains explicitly ignored without its pinned prerequisites. The full gate distinguishes seven roles/twelve scenarios/thirty faults from a filtered fault-only run. Its synthetic GitHub and generated-current observation fixtures are not live historical conversion or physical power-loss evidence.

Diagnostic tests compare primary and linked storage inventories for healthy/missing/corrupt state, constrain fake remote readback and redaction, and separate command process success from semantic helper status. Legacy writer tests intentionally stop at retirement. Direct owner/library tests are not credited as current installed operational recovery. Coordination tests count remote effects and attempts separately, reject changed child/evidence on uncertainty retry, preserve prose, and require native no-PR receipts and archives.

The independent transactions addendum found **TEST-TRANS-001 P3**: its writer can finish before the first reader iteration, so the final observed > 0 assertion fails on a valid schedule. This is static schedule analysis, not an executed flake reproduction. An explicit handshake should establish an observation before allowing completion. No other standalone finding emerged from these chunks. Missing public recovery coverage for journal activation, partial bind copy and reserved Finish before receipt is linked to **ARCH-001/002/003**, whose production findings remain actionable; successful neighboring tests do not close those windows.

`tests-transactions-addendum.json` independently reviews the entire added semantic Gate A module (1–35 and1007–2833), including30 lexical test entrypoints. It distinguishes CAS and killed-lock-holder proof from untested durable activation crashes, and file-byte inventory from full topology. The final853-path denominator has zero pending rows:195 direct full-source,22 complete-hunk,30 cross-lane,606 fixture-cohort classifications.
