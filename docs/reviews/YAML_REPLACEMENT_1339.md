# YAML dependency replacement — #1339

Baseline: public ADL `8c5ce408936f86747aaedde8f98080a6d9bd1545`.

## Selection and provenance

The implementation selects `serde-saphyr =2.0.0`, resolving `granit-parser 2.0.0`.
Both packages were published October 13, 2026, are not yanked, and use MIT OR Apache-2.0.
Their MSRVs (1.89 and 1.81) fit ADL 1.92. Downloaded published archives match registry
checksums. Archive VCS revisions match upstream GitHub commits and the registry's
GitHub trusted-publishing repository/commit records; see [provenance](evidence/1339/provenance.json).
Both published parser source roots forbid unsafe code. This is a bounded provenance
check, not an audit of every transitive dependency or proof of vulnerability absence.

Primary sources: [Saphyr](https://github.com/bourumir-wyngs/serde-saphyr),
[Granit](https://github.com/bourumir-wyngs/granit-parser), and
[RustSec0449](https://rustsec.org/advisories/RUSTSEC-2024-0449.html).

Norway 0.9.42/backend 0.2.15 was the smallest compatible fork. Its repositories are
unarchived and the maintainer responded to issue 36 in June 2026, while acknowledging
limited time. Postfork backend history inspected here showed build/rename/fuzzer
maintenance but no new parser repair. Release age alone did not decide selection.
Saphyr offers current parser work and a safe-Rust backend; this costs a scoped
compatibility adapter. Noyalib's shim uses string-keyed Mapping and would also require
adaptation. No package rename alone is treated as remediation.

## Contract and consumers

`adl-legacy-contracts::yaml` supplies the YAML operations used by the existing
public loaders and tests. It retains arbitrary YAML keys, tags, integer precision,
nonfinite floats and ordered mappings until the explicit schema JSON boundary.
Parse-local decoded scalar spelling is retained only for typed string/character
visitors; dynamic values and numeric decoding retain resolved YAML types. Metadata
is owned by each parse and discarded when producing public values.
The adapter is shared by CLI tests, characterization manifests/mock admission/shadow
manifests, and the generated installed consumer. No private owner checkout is needed.
The generated consumer no longer declares its own archived serde_yaml dependency.

Saphyr optional includes, filesystem resolvers and property interpolation are disabled
at the dependency level. ADL retains its existing include traversal, symlink, cycle,
duplicate-ID, deterministic ordering and pattern-concatenation logic. Merge keys
remain ordinary entries, duplicate keys error, and YAML scalar/typed behavior is
checked against original-backend baselines. Parsing has explicit depth 128 plus the
upstream finite work budgets; these are resource limits, not sandbox claims.

## Proof and audit

Nine deterministic compatibility tests pass against both the original backend and
the replacement. Seven were established before migration; two expanded scalar
compatibility using an isolated original-backend comparison. They cover scalar types/Unicode, aliases/literal merge keys, duplicate and
multiple-document rejection, malformed input, i64/u64 limits/nonfinite floats,
non-string mapping keys, tags/enums, roundtrip, bounded nesting and recursive aliases.
The migrated legacy suite passes 88 unit + 11 retained-contract + 9 compatibility tests.
Characterization passes 37 tests including HTTP/HTTPS schema refusal, local file schema
validation and mock-only admission. Typed string spelling, aliases, untagged enum buffering and quoted numeric rejection
are covered. Canonical public/installed validation remains in progress; final exact-head results are recorded
in the native proof and PR evidence before delivery.

PVF: compatibility tests are deterministic local contract/security regressions with
bounded CPU/memory/filesystem use, no provider/network calls, required public source
gate. Installed-consumer proof is a distinct package/installation gate on macOS/Linux.
Registry/advisory observations are timestamped network evidence, not deterministic tests.

The [six-lock audit](evidence/1339/advisory-after.json) accounts for the same CLI,
legacy, schema, UTS, v2 and characterization locks as #1329. 405 distinct registry versions
were queried on October 21, 2026 UTC; no advisory matches were returned. Original
serde_yaml and unsafe-libyaml are absent from all six locks. The graph adds Saphyr's
parser/diagnostics dependencies and reuses existing public legacy/schema packages in
characterization. Unrelated selected versions are preserved. This observation does
not certify absence of undiscovered vulnerabilities or future maintenance risk.
