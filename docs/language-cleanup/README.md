# Ruby and Python cleanup inventory

Issue [#1345](https://github.com/agent-logic/agent-design-language/issues/1345)
is the inventory and preservation gate for the v0.93.2 cleanup. The
[inventory](inventory.json) pins commit
`85967d1349e817a5d1bd0cecd13c06e43ac161e7`, not a moving working directory.
It accounts for 408 Ruby and 234 Python files, including 403 Ruby and 132 Python
files under lifecycle or milestone directories. File age and location do not
establish that a script is unused.

The intended outcome is zero executable Ruby and no Python requirement for
normal public build, CI, installation or validation. The current exception
budget is **zero files**. Any later exception requires an explicit exact path,
owner, purpose, sunset and operator acceptance. GitHub language percentages are
reporting, not proof. No Linguist exclusions, renamed scripts, interpreter
wrappers or embedded source archives are acceptable substitutes for removal.

## Inventory contract

Each script has its Git blob, mode, SHA-256, byte length, language, delivery
issue, accountable repository, preservation location, caller candidates and
retirement gates. Every baseline blob was read directly from Git and its bytes
hashed. `git show COMMIT:PATH` retrieves the preserved preimage; the recorded
mode comes from `git ls-tree`. History is not rewritten. This is verified
repository custody, not an assertion that an external backup is available.

The inventory also records 3,232 non-script files with interpreter-token
references, including 169 executable/configuration candidates. These include
shell heredocs, Rust subprocess wrappers and workflow commands. The remaining
references are documentary/serialized candidates, not automatically accepted
exclusions. Line numbers locate the original bytes without copying potentially
sensitive historical content into a new report. Text matching can both overcount
mentions and miss dynamically assembled interpreter names; the assigned delivery
issue must trace actual execution before removing a callable surface.

All five tracked tar archives were inspected without extraction for `.rb`/`.py`
members, interpreter shebangs and interpreter-token text. None contained those
candidates. The archive rows pin the exact Git object and SHA-256 so this result
cannot silently apply to a later archive. The scan does not claim arbitrary
encoded payload detection.

## Delivery ownership and ordering

| Issue | Owned outcome | Predecessors |
| --- | --- | --- |
| #1346 | Five maintained-location Ruby tools: replace necessary behavior and tests; retire obsolete helpers only with consumer proof | #1345 |
| #1347 | Historical Ruby/Python source and relevant embedded payloads: remove obsolete copies, replace live checks, preserve original evidence | #1345, #1346 |
| #1348 | Python CI policy, aggregation, coverage, workflow helpers and their tests | #1345, #1346 |
| #1349 | Python packaging, installation, compatibility, public CLI contracts and their Rust wrappers | #1345, #1346 |
| #1350 | Python source-retirement, archive, cleanup and migration integrity guards | #1345, #1348, #1349 |
| #1351 | Remaining utilities, owner adapters, scaffolds, demos, fixtures and embedded interpreter calls | #1345, #1348, #1349 |
| #1352 | Native recurrence guard, final denominator reconciliation and supported interpreter-free delivery | All predecessors |

`delivery_issue` assigns one accountable cleanup issue to each row. Ownership is
ADL's responsibility for resolving its public tree; it is not a claim that ADL
owns private Runtime, CodeFriend, infrastructure or C-SDLC implementations. Use
[the repository map](../REPOSITORIES.md) and accepted owner handoffs before
removing transferred behavior. Never require sibling private checkouts to prove
public-only acceptance. Historical references to a private owner are not proof
that a current replacement has been accepted.

## Replacement contracts and unresolved execution gates

The three script dispositions are deliberate delivery decisions:

- `replace_native_and_switch_callers`: required CI, public packaging and integrity
  behavior moves to Rust, together with positive and negative regression tests.
- `retire_after_consumer_and_preservation_gates`: historical executable source is
  assigned for removal; any discovered live behavior needs a native successor
  and switched callers first. Original reports and authority records stay intact.
- `replace_required_behavior_or_retire_with_consumer_proof`: remaining tools,
  demos, fixtures and owner adapters need an issue-local consumer decision. Port
  public behavior that remains supported; delete only behavior explicitly
  retired or accepted by the separate owner. This is a gate, not permission to
  classify all unreferenced scripts as obsolete.

The coverage contract explicitly preserves the 80% changed-file floor and the
90% aggregate/80% per-file full-run floors. #1348 can retain current CLI test
interfaces during its migration; #1349 owns the atomic coverage-harness switch
when those tests become native. The installation contract retains macOS sandbox
enforcement separately from Ubuntu CI; it does not replace isolation with a
sanitized environment. These boundaries avoid both a dependency cycle and an
unowned cross-issue caller.

The complete inventory is suitable for allocating execution, but it is **not a
bulk deletion authorization**. `retirement_authorized: false` is intentional.
The inventory links every script to one of 14 concrete replacement contracts,
recording required outputs, rejection cases, platform scope, baseline test
surfaces, caller-switch ownership and cross-issue sequencing. Each delivery
issue must record its actual decision and fresh consumer evidence before removal. Explicitly unresolved gates are:

1. Verify dynamic and external consumers for each retiring entry. Lexical
   candidates are neither exhaustive execution graphs nor proof of dead code.
2. Reconcile protected historical evidence in the existing source-retirement,
   repository-cleanup and archive guards. Preserve canonical selectors,
   authenticated receipts, journals and card bytes; introduce reviewed successor
   overlays rather than rewriting original assertions.
3. If a #1347 batch needs guard support that existing contracts cannot express,
   defer that batch until #1350 delivers it. #1350 does not depend on #1347, so
   this adds no cycle. Never weaken a guard to accelerate retirement.
4. Replace Rust wrappers that launch Python, including public install,
   compatibility, remote-distribution, group-D and source-retirement validation.
   A Rust harness around Python is not a completed migration.
5. Retain required negative cases, output/exit-code contracts, archive safety,
   coverage denominators and supported platform behavior. The compatibility
   suite's 29 cases, Ruby aggregation's six cases and deadline suite's seven
   cases are explicit starting denominators, not substitutes for other tests.

The versioned baseline remains immutable as subsequent issues change the tree.
Later delivery overlays must reconcile every row, new interpreter payload and
approved exception with its delivered commit. #1352 verifies the final active
source independently; a passing baseline inventory test never implies cleanup
completion.

## Validation and proof limits

Run from the repository root:

```sh
cargo test --manifest-path tools/language_inventory/Cargo.toml
```

The native test checks the exact committed script denominator, paths, modes,
blobs, byte lengths, SHA-256 identities, reference paths, delivery owners and
preservation identities. Negative cases reject duplicate paths, absent owners,
invented custody and invented retirement permission. It validates the declared
baseline, not absence of interpreters in the final tree. The broader reference
and archive observations were additionally inspected during inventory creation;
lexical scanning limitations are retained above.

PVF: deterministic local Git/document contract, small CPU/memory, no runtime or
provider execution. Focused prepublication gate. No new Python/Ruby utility is
added. `git diff --check` and independent source-grounded review complement the
native tests. CI integration and later interpreter-free platform qualification
remain separate evidence.

## Workflow boundary

The operator authorized Git/`gh` fallback when native tooling blocks this wave.
#1345's Git worktree was moved to the required FastWork root and its Git
registration repaired. Native relocation records remain preserved as historical
failure evidence; they are not asserted to be reconciled authority for the moved
checkout. Cognitive-SDLC #330 records that tooling gap and is not implemented by
this cleanup. All subsequent worktrees must select the FastWork destination
explicitly. Review, focused validation and actual GitHub delivery remain required.
