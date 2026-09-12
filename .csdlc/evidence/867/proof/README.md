# SIM-01 installed diagnostic proof

Issue #867 under existing umbrella #866. Tested source:
`40c068b11a2cca7303f5fcaed01d33b2614db743`, based on
`f1c4e2a915c215797f0d2708cb8b0568f2b80b32`. The candidate digest and retained
artifact hashes are in `index.json`. This is local fixture proof; CI and actual
PR/issue terminal state are separate lifecycle evidence.

## Outcome and acceptance evidence

| Requirement | Proving surface |
| --- | --- |
| Installed observations across applicable healthy, missing, corrupt and pending cases | `installed_diagnostics_preserve_healthy_missing_corrupt_and_pending_state`: 40 issue observations, separately 24 helper observations; `installed_pr_state_observes_fake_remote_without_replaying_pending_mutation`: eight remote cases plus settled/corrupt reconciliation observations; `exact_candidate_preflight_and_negative_matrix`: release matrix and four linked qualification cases. |
| No lifecycle or Git registration effects | Recursive before/after inventories include directories, file digests, symlinks, Git metadata, locks, cards, journals, stage/backup paths and receipts. Fixtures use primary and genuinely registered linked worktrees. Installed release checks include Git index bytes. |
| Honest failure and recovery states | Pending local work reports explicit recovery-required; pending remote work cannot be hidden by successful PR observation. Missing/corrupt state stays blocked or failed. Operational discovery no longer falls back to construction success. Settled remote receipts must retain correct identity and merge reconciliation. |
| Mutators retain recovery | Existing bind/edit crash regressions pass, with additional linked exact-request recovery and denial of a changed request. Installed remote pending-ready recovery occurs only through the explicit mutator. |
| Shared attempt corpus | `attempts/baseline.json`: 17 installed attempts, 14 completed, two blocked, one interrupted; zero abandoned/censored. Covers prepare/bind/edit/proof/review/publication, fake PR create/readback, finish, eligible cleanup, both checkout contexts and interruption/recovery. Separate negative measurement fixtures retain two failed attempts and one censored collector interruption. |
| Channels, credentials and compatibility | Failure JSON stays on stdout with human errors on stderr. Legacy observability overrides create no implicit log and do not redirect native error output. Read-only curl credentials use stdin and a minimal environment; no lifecycle credential file is created. Redaction occurs before truncation, including a credential-prefix regression. |

The helper cases exercise repository/model inputs; helper readiness is not issue
readiness or permission to mutate. Ordinary authority, topology, exact-head,
corruption and cleanup denials remain covered by the focused owner suites.

## Validation

```sh
cargo test --manifest-path csdlc-v3/Cargo.toml --lib \
  --test operational_cli_commands --test local_commands \
  --test remote_publication_commands --test release_preflight
cargo fmt --manifest-path csdlc-v3/Cargo.toml --check
git diff --check
```

The exact-source run passed 154 tests: 85 library, 37 local command, 18 operational
CLI, one release matrix and 13 remote publication tests. `validation.stdout` and
`validation.stderr` retain the run. Display-only host prefixes in published logs
are replaced by `$WORKTREE`/`$PRIMARY`; `index.json` records original and published
hashes. Original logs remain local. Focused failing regressions in `regressions/`
have trailing blank lines normalized for diff hygiene; no result text is removed.
They show lock creation, Git index refresh, remote runtime storage creation, and the
redaction/truncation boundary before their fixes. Failed runs are not pass evidence.

PVF classification is tracked in
`csdlc-v3/tests/fixtures/diagnostic_observation_pvf.json`: required deterministic
tooling proof, bounded local CPU/disk/Git, isolated candidates and fake transport,
no paid service or live external effects.

## Corpus interpretation

Counts describe declared fixture attempts, not production reliability. Completed
review/publication checks are distinguished from the separate actual fake PR
creation effect. Review principals/receipts are synthetic; readback uses the
native adapter over deterministic fake transport. The simulated remote merge is
an explicitly recorded fixture event, not a real merge invocation. Eligible
cleanup previews and removes a separate genuinely registered clean control at
the terminal head; the dirty issue checkout and evidence remain intact.

Every attempt records request fields and their count, installed entrypoint and
nested invocation counts, effects, issue versions, correlation, result/reason,
invalidation and wait owner. Invocation scope is installed CLI entrypoints and
fake remote transport; internal Git utility calls are excluded. Source/binary
digests, OS/architecture and Rust/Git versions identify the run. Injected clock
samples are synthetic; actual subprocess durations are separately recorded and
do not establish causal savings.

The recorder saves censored starts and raw results before semantic assertions.
Malformed output, process failures and collector interruption cannot disappear
from the denominator or retain a false completed verdict. Measurement artifacts
and transport counters are outside inspected fixture storage and isolated by
test-process identity. Generated runs live under
`csdlc-v3/target/sim01-corpus/<test-process>/` before explicit retention here.

## Review and release boundary

Independent implementation review at the tested source found no actionable
findings after fixes for linked recovery, failed/blocked classification, remote
receipt integrity, process-launch diagnostics and measurement retention/counting.
The reviewer independently checked formatting and diff hygiene. Final publication
revision review is retained separately; these results do not assert CI completion,
merge, terminal closeout, active binary replacement, or live writer activation.
