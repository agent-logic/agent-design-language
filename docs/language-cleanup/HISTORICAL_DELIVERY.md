# Historical interpreter retirement (#1347)

This candidate retires **607 historical executable/build files** from the active
ADL tree: all 403 Ruby and 132 Python files assigned by #1345, 47 shell wrappers,
nine Rust interpreter wrappers/embedded programs, and 16 Cargo manifest/lockfile
sidecars for eight retired standalone wrapper crates. Every source blob, Git mode,
byte length and SHA256 was verified before removal against committed
`7b6986179a0642d558f3ca6e1d8f547cb003b735`; all 535 original script identities
still matched the immutable inventory. No code was renamed, embedded in data,
placed in a tracked archive or hidden with Linguist attributes.

`historical-retirements.json` records each executed disposition and retrievable
Git custody. `historical-transition.json` pins the 607 deletions and three caller
changes. The native guard validates that exact successor before restoring earlier
identities for the original retirement audit. Original manifests, selectors,
receipts, journals and cards remain unchanged. The guard's pin file is excluded
from its own hash cycle, as in earlier cleanup deliveries; its code change is
reviewed directly. The historical audit still reports the original 3,407 actions;
this separate 607-file successor does not rewrite that denominator.

## Consumer decisions

These files are historical issue preparation/evidence and milestone execution
copies, not the public ADL implementation or the admitted installed C-SDLC owner.
Current `AGENTS.md` restricts execution to authenticated v3 owner routes and the
post-split public ADL scope. Removing these copies retires their historical replay
entrypoints from current ADL; it does not assert current private-owner behavioral
parity, a new owner acceptance, or a new pass of their original tests. Reproduction
of their original context requires the pinned historical Git revision.

The fresh scan covered current ADL/tool source, shell and Rust entrypoints,
configuration, workflows and Make targets, including prepared/evidence dynamic
routes and the eight standalone wrapper manifests. The 56 executable reference
candidates actually invoke or embed Python/Ruby; their wrappers are retired with
the historical source rather than left callable. Five YAML reference candidates
are documentary disposition/planning records, remain unchanged and confer no
current execution route.

Two coupled current surfaces needed explicit resolution:

- The podcast migration check still verifies the exact historical identity
  validator bytes. It now requires the pinned retirement record, confirms the
  active file is absent, and reads its exact preimage from the committed source
  revision. The original destination/source manifest and its hash remain intact.
- The retained durable-history terminal checker referenced the preparation
  validator in its output/docstring, but never invoked it. Its wording now names
  that validator as historical Git evidence and explicitly avoids claiming
  current lifecycle authority. Its required native replacement remains #1351.

The prior live planning contract is already native under #1348. Integrity guards
are native under accepted #1350. No new private sibling checkout or owner binary
is required for public validation.

## Acceptance proof

The native historical inventory test checks all 607 committed preimages and
absences, reconciles all 535 original rows and the coupled wrapper/build counts,
rejects any remaining Ruby or historical Python source, and compares every other
historical record against the source revision. This includes all five preserved
YAML candidates. Existing native successor tests retain resurrection, caller,
mode/hash, missing custody and foreign-deletion refusal cases and admit actual
HEAD. The full committed retirement audit remains required with its original
external custody prerequisites; Git preimages do not replace those prerequisites.

Focused commands:

- `cargo test --locked --manifest-path tools/language_inventory/Cargo.toml`
- `cargo test --locked --manifest-path tools/source_retirement/validator/Cargo.toml --test successors --test retirement_contract`
- `cargo run --locked --manifest-path tools/ci_contracts/Cargo.toml -- planning-contract`
- `node --test tools/test_podcast_migration.mjs`

CI now includes the historical inventory test alongside the successor guard.
Final proof/review/CI results are recorded on the implementation PR. The remaining
61 Python utility/adapter files belong to #1351; this issue does not claim the
whole eight-issue cleanup is finished. Native tooling regressions remain recorded
in C-SDLC #330/#331; Git/gh delivery uses the operator-authorized fallback.
