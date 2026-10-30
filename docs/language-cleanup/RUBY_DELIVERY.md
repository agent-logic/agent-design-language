# Required Ruby tooling replacement (#1346)

The native `tools/ci_contracts` crate replaces the maintained Ruby workflow
validator, deadline regression suite and UTS aggregation contract. Workflow and
cleanup callers invoke the native commands; `test_ci_runtime_contracts.sh` runs
the focused Rust fixture suite. The original valid-workflow JSON matched the
native output exactly before removal. The suite preserves seven workflow/deadline
cases, six aggregation outcomes and six workflow mutation refusals, with added
binding/UTS disconnection and Ruby-execution-trap checks.

```sh
cargo run --locked --quiet --manifest-path tools/ci_contracts/Cargo.toml -- workflow-policy
cargo run --locked --quiet --manifest-path tools/ci_contracts/Cargo.toml -- aggregation-contract
cargo test --locked --manifest-path tools/ci_contracts/Cargo.toml
```

The existing Python selector remains a transitional dependency assigned to
#1348. These commands eliminate Ruby, not all Python. Cargo's ordinary dependency
cache may need filling on a fresh host; the small native policy executable runs
in the CI policy stage and does not depend on a public product build.

## Historical helper retirement

The retained proof-gap denominator generator and its shell self-test are specific
to the completed v0.92.1 issue764 release packet. The native-receipt validator and
its shell self-test are specific to historical issue5820/27 receipt evidence.
Their maintained entrypoints had only their own paired self-test callers; other
references are historical records, the tool registry and custody/retention
ledgers. They do not establish current Runtime qualification. This issue retires
those four entrypoint files rather than importing private Runtime or C-SDLC
behavior into public ADL.

Original generated denominator data, receipt packets, reviews and the original
`NATIVE_RECEIPT_HISTORICAL_RETENTION.json` stay unchanged. That original record
truthfully reports historical retention and `retirement_authorized: false` at
its original point in time. This new reviewed delivery supplies the later exact
retirement disposition; it does not rewrite the earlier decision or claim its
old tests were rerun successfully today. Preimages are retrievable by the pinned
Git commit/path and blob in [ruby-transition.json](ruby-transition.json).

## Integrity bridge

The transition lists exact before/after identities of changed historical-scope
files and exact preimage SHA-256 values. The existing cleanup guard verifies its
pinned bytes, source identities, current candidate identities, deletion set and
retrievable custody, then reconstructs the historical view before the unchanged
older transition checks run. Unlisted drift remains visible; no directory-wide
exemption exists. The small bridge is deliberately added to an existing Python
guard and is part of #1350's native replacement obligation, not a new Python
utility or a final interpreter-free claim.

Focused guard negative cases cover restored removals, changed callers, manifest
tampering and preservation of unrelated drift. Final language removal, historical
Ruby elsewhere and removal of Python interpreter probes remain assigned to the
later cleanup issues.

PVF: local deterministic contracts, bounded CPU/disk, no provider calls. Required
focused prepublication tests plus affected repository CI. Original private-job,
nextest and backend assertions remain historical custody, not passing public
contract claims.
