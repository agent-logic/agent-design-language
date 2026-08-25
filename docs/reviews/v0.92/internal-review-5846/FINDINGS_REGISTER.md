# v0.92 WP-25 Findings Register

## Register Metadata

- Exact product target: `c6792e54df1db5969fa28c59b6dfe4c714ed5559`
- Synthesis: `docs/reviews/v0.92/internal-review-5846/SYNTHESIS.md`
- Raw specialist findings: 20
- Deduplicated findings: 11
- Open: 9
- Resolved for this packet: 2
- P0: 0
- Open P1: 6
- Open P2: 2
- Open ambiguous: 1
- Release/publication verdict: blocked

## Findings

| ID | Severity | Status | Title | Source roles | Primary owner |
|---|---|---|---|---|---|
| SYN-001 | P1 | open | Canonical review/release authority has incompatible gate states | architecture, docs, release | WP-27 documentation/release truth |
| SYN-002 | P1 | open | Engineering-complete claim exceeds reconciled checklist/security proof | docs, security, release | milestone/release evidence owner |
| SYN-003 | P1 | open | Demo/proof register and D9 cannot represent corrective state | demos/integration | WP-20 proof-register owner |
| SYN-004 | P1 | open | Remote validation executes mutable unverified bootstrap artifacts | dependency | AWS remote-validation/toolchain owner |
| SYN-005 | P1 | open | Builder image is not reproducible or integrity-pinned | dependency | builder-image/release-engineering owner |
| SYN-006 | P1 | open | Independent Cargo graphs lack change-triggered CI ownership | tests, dependency | CI/PVF and package owners |
| SYN-007 | P2 | open | Dependency scaffold consumes stale lifecycle-lock evidence | dependency | review-packet/dependency tooling owner |
| SYN-008 | P2 | open | Post-merge diff-hygiene command is non-proving | docs | WP-27 documentation owner |
| SYN-009 | ambiguous | open | Derived-terminal receipts and typed doctor disagree | lifecycle/evidence | typed C-SDLC lifecycle owner |
| SYN-010 | P1/P2 | resolved_packet | Initial lane assignments and topology metadata were invalid | code, security, tests | issue-313 packet owner; generic builder residual separate |
| SYN-011 | P2 | resolved_packet_partial | Dependency assignment repaired; stale consumer remains | dependency | issue-313 packet owner / tooling owner |

## Disposition Requirements

### SYN-001

- Required: one immutable #467 authority identity and consistent blocker/unlock state across every canonical entrypoint and retained validation artifact.
- Proof to close: exact-head validator plus cross-document assertion with historical #311 state explicitly labeled non-current.

### SYN-002

- Required: evidence-backed reconciliation of engineering and ACIP/security checklist rows, or narrowed milestone claims.
- Proof to close: exact retained positive/negative/review evidence linked for checked rows; downstream release-tail rows remain explicitly pending.

### SYN-003

- Required: shared status vocabulary and fully reconciled matrix, coverage, ledger, and artifact index.
- Proof to close: D9 positive command and full negative harness both pass at the same exact head; each claimed demo row has immutable artifacts and command routing.

### SYN-004

- Required: immutable download identity and verification for every executable bootstrap source.
- Proof to close: negative tests reject absent/mismatched digests, mutable S3 objects, and unpinned latest assets; retained provenance records source, version, architecture, digest/signature, and VersionId where applicable.

### SYN-005

- Required: pinned builder base/toolchain and verified direct downloads.
- Proof to close: reproducible resolved manifest plus contract tests enforcing every pin/checksum/signature.

### SYN-006

- Required: explicit CI/PVF path ownership for `adl-characterization`, `adl-resilience`, and `tools/remote_validation`.
- Proof to close: package-only routing fixtures and locked package-local metadata/build/test/format/Clippy lanes.

### SYN-007

- Required: dependency scaffold consumes material dependency assignments and rejects lifecycle sentinels/index disagreement.
- Proof to close: deterministic scaffold test with Cargo/container/bootstrap inclusions and `.csdlc/locks/*.lock` exclusions.

### SYN-008

- Required: immutable base-to-target diff command.
- Proof to close: command checks the intended #312 candidate range after merge and fails on an injected whitespace error.

### SYN-009

- Required: typed reconciliation of derived-terminal authority versus doctor projections.
- Proof to close: typed terminal read or documented/validated rule that makes doctor non-applicable after derived terminalization.

### SYN-010

- Packet disposition: closed by repaired deterministic exact-target assignment and passing recheck.
- Residual: generic repo-packet-builder scoring and path-name worktree detection remain unremediated and must not be reported closed.

### SYN-011

- Packet disposition: dependency lane assignment repaired to 61 material paths.
- Residual: SYN-007 remains open because the scaffold still consumes stale evidence.

## Machine-Readable Draft

```json
[
  {"id":"SYN-001","severity":"P1","status":"open","roles":["architecture","docs","release"],"owner":"WP-27"},
  {"id":"SYN-002","severity":"P1","status":"open","roles":["docs","security","release"],"owner":"milestone_release_evidence"},
  {"id":"SYN-003","severity":"P1","status":"open","roles":["demos_integration"],"owner":"WP-20"},
  {"id":"SYN-004","severity":"P1","status":"open","roles":["dependency"],"owner":"aws_remote_validation"},
  {"id":"SYN-005","severity":"P1","status":"open","roles":["dependency"],"owner":"builder_image"},
  {"id":"SYN-006","severity":"P1","status":"open","roles":["tests","dependency"],"owner":"ci_pvf"},
  {"id":"SYN-007","severity":"P2","status":"open","roles":["dependency"],"owner":"review_packet_tooling"},
  {"id":"SYN-008","severity":"P2","status":"open","roles":["docs"],"owner":"WP-27"},
  {"id":"SYN-009","severity":"ambiguous","status":"open","roles":["lifecycle_evidence"],"owner":"typed_csdlc"},
  {"id":"SYN-010","severity":"P1_P2","status":"resolved_packet","roles":["code","security","tests"],"owner":"issue_313_packet"},
  {"id":"SYN-011","severity":"P2","status":"resolved_packet_partial","roles":["dependency"],"owner":"issue_313_packet"}
]
```

## Non-Claims

- Resolved packet routing does not resolve the generic packet-builder implementation.
- Passing focused tests do not resolve missing CI ownership.
- Skipped demos are not runtime failures, and they provide no proof.
- This register does not approve release, publication, deployment, merge, or lifecycle completion.
- No claim is made about excluded issue `#269`.
