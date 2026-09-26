# RD03 public ADL package closure

The five packages below own portable declarations, pure planning, and inert
conformance. Runtime transport, profile expansion, admission, credentials, clocks,
signing, state, and execution remain outside this distribution.

| Package | Package version | Wire/API coverage |
| --- | --- | --- |
| adl-schema | 0.1.0 | ProviderSpec/ProviderMap, model identity v1, Chronosense event-anchor schema, preserved declaration syntax |
| adl-uts | 0.1.0 | UTS v1/v1.1, invocation schema, inert conformance fixtures |
| adl-language | 0.92.1 | Six-primitives ADL 0.3/0.5 declarations and validation |
| adl-compiler | 0.92.1 | Pure deterministic adl.execution-plan.v1 compilation |
| adl-legacy-contracts | 0.1.0 | Legacy ADL 0.1–0.5 declarations, schema, prompt/plan resolution, trace v1, cognitive-transition contracts |

The compiler depends on exactly adl-language 0.92.1. Legacy contracts depend on
exactly adl-schema 0.1.0. Their path edges point only to the corresponding public
sibling artifacts. Remaining dependencies are public Cargo registry packages.
The consumer lock and archive digests identify actual resolved dependency bytes.
These are versioned source-distribution archives, not claims of crates.io release.

## Compatibility and ownership

Runtime compatibility modules reexport the single public definitions; they do
not introduce parallel DTOs. The Runtime resolver expands configured profiles
before portable resolution and preserves the retained profile provenance. The
portable resolver rejects unexpanded profile declarations. Neither an expanded
profile nor an inert execution plan authorizes execution. Runtime provider
constructors and profile admission continue enforcing endpoint restrictions.
Public declaration validation preserves historical URL and key-source rejection.
Model identity's observation clock stays in provider-core; wire types are public.

Original Git history and retained evidence stay in place. `source-provenance.json`
records source Git objects for moved modules, fixture copies and license bytes.
Language/compiler retain Apache-2.0 metadata; other packages retain their existing
MIT OR Apache-2.0 metadata and repository license bytes. This work makes no new
license grant or resolution of inherited license ambiguity.

## Executable installed-consumer qualification

From a committed candidate, on macOS with Rust and public registry dependencies
already cached:

```sh
python3 tools/public_adl/verify_install.py --output /private/tmp/rd03-install-UNIQUE
```

`UNIQUE` names a new output directory; existing outputs are never overwritten.
The runner fails without OS sandbox enforcement. It constructs archives from the
exact Git revision, extracts them outside producer checkouts, and runs the Rust
consumer with network, home and producer reads denied. Only the Rust toolchain
and public registry cache are allowed through the home restriction. No credential
files are read or copied. The private-credential test uses an owned synthetic
sentinel. Positive artifact reads plus failed credential/source/network reads
prove enforcement. Cargo rejects a producer-path dependency and an incompatible
exact package version. The consumer checks five-package linkage, deterministic
plans, schema parsing, DTO roundtrip, declaration denials, unsupported UTS version,
and inert conformance. No actual tool/provider execution occurs.

The output contains all five deterministic archives, the consumer dependency
lock and a JSON report binding their hashes, exact source revision, toolchain
versions, successful consumer assertions and classified negative cases. Resolution
occurs explicitly offline; subsequent successful build/run and metadata use
`--locked`. The proof is macOS-specific; hosted Linux tests do not claim this
sandbox proof. Rollback consists of returning consumers to their previous exact
package/source pins; archive creation changes no registry or deployed consumer.

## Validation classification

Package unit/characterization tests and the Runtime adapter regression are
required RD03 deterministic CPU-only contract tests. The installed-consumer run
is a required offline CPU/disk integration qualification with OS sandbox
capability, no cloud/provider spend and no release authority. It is additional
to native lifecycle proof and CI, not a replacement for either.
