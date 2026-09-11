# Issue #856 validation and #526 handoff

Implementation: native v3 release-preflight and stable owner installation;
17 coordinated packages at 0.92.1, six independently versioned helpers/packages;
11 active and ten explicitly historical/superseded lockfiles inventoried.
Historical proof records and #833/#522 work products are unchanged.

Local results:

- Native owner all-target suite: 241 tests passed before the final lock-owner
  review correction; focused candidate matrix rerun passed after that correction.
- Candidate matrix: real native CLI eligible fixture and identity/version/gate/
  notes/authority negatives; empty, omitted and source-tagged lock owner cases
  reject after independent review. Each call preserves refs and checkout status.
- All 23 package manifests passed `cargo metadata --offline --locked --no-deps`;
  the 24th manifest is the virtual ADL workspace. This is manifest validation,
  not full compilation or complete dependency-resolution proof.
- Shell wrapper routing and retired-helper guard passed.
- Stable owner installer regression suite passed, including native source
  provenance, no-op reuse and mixed-manifest rejection.
- Native strict clippy and rustfmt passed; hosted CI is separate publication proof.

PVF: required deterministic local tooling/native contract checks; small CPU/Git
fixture disk; no paid resources, provider calls or release mutation. Shared
same-host dependency cache warmup was acceleration only.

Independent review found a P2: checking only present lock entries could admit a
missing owner. The repair now requires each active root/workspace lock's owning
package identities as well as versions; focused deletion/empty/source-tagged
regressions pass. Renewed exact-head review is recorded separately.

#526 consumes the PR merge revision containing this document and the exact
command in `docs/csdlc-v3/RELEASE_PREFLIGHT.md`. Request/gate bytes must be
assembled for its final proposed commit after that commit exists. Missing
candidate gate intentionally blocks the live preflight. #522 remediation,
#525 semantic review, exact candidate approval and separately authorized release
operations remain gates. A passing fixture or consistency report grants no
release authorization. No actual release gate, tag or release was created.
