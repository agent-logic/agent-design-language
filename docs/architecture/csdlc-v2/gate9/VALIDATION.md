# C-SDLC v2 Gate 9 Validation and Budget Audit

Issue: #5239

Host: macOS arm64, same host/toolchain family as the Gate 1 baseline

Scope: standalone `csdlc-v2` workspace only

## Result

All deterministic Gate 9 hard budgets pass. The first current-revision clean
release measurement exposed a 441.81-second build caused by thin LTO plus one
codegen unit across seven binaries. Gate 9 corrected the standalone release
profile to `lto = false` and 16 codegen units. A new empty-target measurement
then completed in 198.83 seconds on the same FastWork measurement filesystem
used by Gate 2, inside the 209.275-second ceiling. This is a
measured construction fix; no acceptance threshold changed.

## Implementation size

Command:

```text
rg --files src -g '*.rs' | xargs wc -l
```

Result: 8,005 Rust implementation lines. This includes the seven installed
owners and bounded review/schedule/shepherd/import/shadow/soak helpers. It
excludes tests and docs. The 8,000 figure is a review threshold, not a reason to
omit needed code; an overage requires explicit approval and a findings-first
owner/rationale record. The five-line overage is the typed canonical-budget and
ambiguous-publication proof added during review remediation. The bounded
reviewer approved it as a 0.0625% evidence-integrity exception.

## Test and validation time

- Rust tests: 73 after the Gate 9 executable-soak, ambiguous-create, and evidence-input proofs.
- Target band: 60–100; hard ceiling: 150.
- Gate 9 executable soak inside the final full run: 8 tests, 23.47 seconds.
- Final warm `cargo test`: 65.00 seconds wall time; all 73 tests passed.
- Focused ceiling: 120 seconds; complete deterministic ceiling: 600 seconds.

The suite covers automatic cards/derived estimates, claims and worktrees, PVF
failure/retry, exact-revision review, publication reconciliation, readiness,
closeout, migration/parity, opt-in selection, persisted Store restarts, and
decision fail-closed behavior. No thousands-of-tests strategy is used.

## Construction

Exact installed set:

```text
csdlc-init csdlc-doctor csdlc-edit csdlc-bind
csdlc-validate csdlc-publish csdlc-closeout
```

Current-revision isolated empty target, release profile after correction:

- real: 198.83 seconds
- user: 237.18 seconds
- sys: 16.50 seconds
- hard ceiling: 209.275 seconds (50% of the 418.55-second v1 baseline)

Three immediate no-change builds over that target:

- real: 0.84, 0.27, and 0.22 seconds; median 0.27 seconds
- target/ceiling: 0.8125 seconds (25% of the 3.25-second v1 baseline)

The rejected pre-correction measurement was 441.81 seconds. An exact-source
empty `/tmp` target also measured 226.06 seconds while CPU user time remained
near the passing run, showing host/filesystem contention. Both are retained
because hiding slow measurements would make the decision less trustworthy.

## Installed binary size

All binaries were stripped by the release profile. Exact byte counts:

| Binary | Bytes |
| --- | ---: |
| `csdlc-init` | 1,839,424 |
| `csdlc-doctor` | 1,639,200 |
| `csdlc-edit` | 2,403,744 |
| `csdlc-bind` | 1,793,184 |
| `csdlc-validate` | 904,800 |
| `csdlc-publish` | 5,570,864 |
| `csdlc-closeout` | 5,525,392 |
| **Total** | **19,676,608** |

Largest-binary ceiling: 15,728,640 bytes. Seven-binary-set ceiling:
73,400,320 bytes. Both pass with substantial margin. Qualification and
migration helpers are not installed owner binaries and are not included.

## Local latency

The init/doctor and bind measurements each used 21 process-isolated temporary
fixtures. Nearest-rank p95 is sample 20 after sorting.

- Combined automatic six-card init plus offline doctor p95: 0.96 seconds;
  maximum 1.32 seconds. This also bounds init below its two-second ceiling.
- Bind plus idempotent rebind p95: 1.20 seconds; maximum 1.28 seconds.
- Gate 2's direct release-doctor measurement remains 0.02-second p95 over 21
  fixtures.

The combined init/doctor target of one second is stricter than the two-second
init ceiling and passed at p95. Each fixture is local and performs no network
request.

## Independence

The workspace manifest has no path or package dependency on ADL, Runtime v2,
or Runtime v3. Git is invoked through executable/argument arrays. Octocrab is
the typed GitHub boundary. Shell was used only by the operator to orchestrate
measurement commands; it is not part of the C-SDLC v2 control plane.

## Scenario and parity evidence

`soak-evidence-input.json` maps every required Strum scenario to an executable
test and/or generated packet. The three sample packets contain 18 generated
cards, three designs, three diagrams, and portable manifests. Repeated packet
generation is byte-stable.

Normalized parity compares three representative cases and reports zero
critical differences. The intentional noncritical differences are the v2
design goals: one typed canonical index, generated card projections, and a
small installed-owner boundary.
