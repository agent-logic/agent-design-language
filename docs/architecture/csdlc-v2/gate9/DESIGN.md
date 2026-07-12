# C-SDLC v2 Gate 9 Opt-In Soak Design

Issue: #5239

Version: v0.91.7

Default generation during this gate: v1

## Decision boundary

Gate 9 qualifies the complete standalone C-SDLC v2 lifecycle without changing
repository authority. `csdlc-soak` is a qualification helper, not one of the
seven installed owner binaries. It may generate sample packets and evaluate
evidence, but it cannot bind worktrees, mutate live issue state, publish, merge,
close, change the default generation, or start compatibility expiry clocks.

The generation selector has one safe rule: an omitted request resolves to v1.
V2 resolves only when both the caller explicitly requests v2 and the issue is
in the typed opt-in set. A selector whose default is v2 is invalid during Gate
9.

## Automated sample construction

One command deterministically creates three representative packets:

```text
cargo run --bin csdlc-soak -- generate-samples \
  --output ../docs/architecture/csdlc-v2/gate9/samples
```

Each packet contains a design, Mermaid diagram, portable packet manifest, and
all six cards. The cards are built from typed values through the same
`initial_cards` and markdown.rs mdast render/validation path as operational v2.
The `Small` planning profile derives SPP execution estimates and VPP time/token
budgets; no operator fills them in. Repeating the command is byte-stable.

The samples are deliberately synthetic issue identities (9001–9003). They are
qualification fixtures, not hidden GitHub issues, and cannot collide with or
claim authority over repository work.

The tracked six-card files are the required pre-execution packets, so SRP/SOR
remain pre-phase there by design. The executable soak test consumes the same
three identities, runs each through the real Store, review, publication,
readiness, failure-regression, and closeout APIs, reopens persisted Store truth
between phases, and asserts final normalized outcomes. Each packet manifest
links that execution proof; the design-time SOR is not itself misrepresented as
terminal evidence.

## Soak state and failure model

The required scenario vocabulary is a Strum-backed enum. It includes docs-only
and small-Rust paths; validation failure/retry; review finding/repair; PR-check
failure/recovery; merge/closeout; restart at every persisted lifecycle phase
boundary; dirty-worktree refusal; and GitHub outage/retry reconciliation.
`Merged` and `ClosedOut` are two audit transitions inside one atomic terminal
commit, not two separately persisted operator phases. The soak therefore
injects/retries around that transaction and verifies the reopened terminal
record rather than claiming an impossible interruption inside an atomic write.

Evidence is an explicit list of scenario outcomes and references. Missing,
waiting, or reference-free evidence cannot produce `proceed`. A known hard
scenario failure, hard budget failure, or critical parity difference produces
`stop`; incomplete non-failing evidence produces `incubate`.

## Budget model

Gate 9 measures only the independent workspace. Implementation LoC excludes
tests and documentation. Test count includes all Rust tests. Installed size is
measured for exactly the seven Gate 1 owner binaries; review, schedule,
shepherd, import, shadow, and soak remain bounded helper/migration tools and are
not counted as installed owners.

Every budget record contains the observed value, target where one exists, hard
ceiling, unit, pass truth, qualification, and evidence reference. The decision
evaluator independently checks the recorded pass flag and observed ceiling.
The implementation-LoC value is a review threshold: an overage may proceed only
with an explicit approval and non-empty qualification naming the useful surface
and rationale. Other safety/performance ceilings remain hard.

## Parity and decision

Parity compares normalized lifecycle outcomes, never internal file layout or
Markdown bytes. Proceed requires at least one compared case, zero unexplained
critical differences, every required scenario passed with evidence, and every
hard budget satisfied.

Even a `proceed` packet does not authorize cutover. Gate 10 must consume the
reviewed Gate 9 decision, migrate operator contracts first, and explicitly
switch the default. Gate 9 always records both rollback and importer-expiry
clocks as not started.

## COTS and independence

The implementation uses the existing standalone v2 crate families: Serde and
Schemars for typed contracts, Strum for closed vocabularies, markdown.rs for
card ASTs, Blake3 for content identity, Clap for the helper CLI, and the
standard library for deterministic files. It imports no ADL or Runtime crate
and reuses no incumbent lifecycle implementation.

## Review questions

- Does any path select v2 without explicit issue opt-in?
- Are all six sample cards generated rather than hand-edited?
- Can incomplete evidence or a critical parity loss yield `proceed`?
- Do measurements match the reviewed Gate 1 denominator and installed set?
- Does the design avoid silently starting cutover or compatibility clocks?
