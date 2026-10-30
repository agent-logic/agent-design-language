# Runtime v3 Lightweight Parity Sprint Review

Issue: #5174
Review issue: #5403
Status: changes required
Remediation: #5410; shared records issue #5406

## Findings

### P1: The executable Runtime v3 path still launches the seed proof topology

`serve` calls `build_proof_runtime` at
`adl-runtime-kernel/src/bin/adl-runtime-kernel.rs:54`. That builder hard-codes
observability, Chronosense, scheduler, a simple gate, and checkpoint at
`adl-runtime-kernel/src/proof.rs:64`. Configuration registry, reasoning,
governance, and operational factories are exercised only by tests.

Impact: #5176-#5183 produced component libraries and fixtures, but the
operational Runtime v3 process is not assembled from the architecture the
sprint claims to implement.

Disposition: open. Route Runtime v3 assembly work that constructs the live
kernel from the component registry and proves lifecycle/health for the complete
required set.

### P1: Live continuity accepts an attacker-recomputable checksum

The active proof capsule at `adl-runtime-kernel/src/proof.rs:31`, line 140, and
line 443 accepts state when a public Blake3 checksum matches. Anyone able to
modify the capsule can change generation or sequence fields and recompute the
checksum. The stronger signed #5181 continuity machinery is not wired into
`serve`.

Impact: the executable path does not provide authenticity for restored
continuity state.

Disposition: open. Replace the proof checksum on the live path with the signed
continuity/checkpoint contract and add substitution/forgery tests.

### P2: Chronosense marks local wall time authoritative without SNTP qualification

After a five-millisecond delay, the component reads `SystemTime` and records it
as `Authoritative` with source `proof_sntp_adapter` at
`adl-runtime-kernel/src/proof.rs:204`.

Impact: unavailable or unsynchronized SNTP is reported authoritative instead
of explicitly degraded, contradicting the sprint contract.

Disposition: open. Use the real time-source qualification path and preserve
degraded state until synchronization evidence exists.

### P2: Independent review and lifecycle truth are not retained

`docs/architecture/RUNTIME_V3_FINAL_REVIEW_5175.md:9` says specialist reviewers
found no remaining findings, but identifies provider records only under
`.adl/local-artifacts`. `.adl/` and historical task mirrors are ignored at
`.gitignore:4` and line 51. No six-card bundles or formal GitHub reviews remain
in the merged tree.

Impact: child closure is visible, but exact SRP/SOR review and disposition truth
cannot be reconstructed independently.

Disposition: route through the shared typed-v2 records-retention remediation.

### P3: Final review footprint and inventory claims are stale

`docs/architecture/RUNTIME_V3_FINAL_REVIEW_5175.md:38` records 8,446 Rust lines,
106 tests, and 195 routed modules. The later shadow report records 207 modules
and different footprint figures at
`docs/architecture/runtime_v3_shadow_parity_report.v1.json:27`.

Impact: readers can mistake a historical snapshot for current size and parity
inventory truth.

Disposition: label the final review explicitly historical and point to the
current counted surfaces.

## Child Coverage

Reviewed #5170, #5176, #5182, #5181, #5177, #5180, #5178, #5183, #5179, and
#5175. Every child issue is closed and its PR merged.

## Validation And Limits

- The full default `adl-runtime-kernel` suite passed; one live v2/v3 test and
  seven guardian/soak tests were ignored.
- `adl-runtime` passed 115 tests including crate independence.
- All five findings above are review-discovered; no test-discovered defect is
  counted above.
- Runtime v3 remains source-independent from `adl/src/runtime_v2`; the defect
  is incomplete operational assembly and overclaimed parity, not source reuse.
- No manifest or lockfile defect was confirmed. `cargo-audit` was unavailable.

## Review Result

Changes required. Runtime v3 contains useful component implementations, but its
executable path remains a seed proof topology with weaker continuity and time
semantics than the sprint's component architecture.
