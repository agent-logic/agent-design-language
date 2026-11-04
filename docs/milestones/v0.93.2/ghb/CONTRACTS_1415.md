# GHB v1 contracts and distribution — ADL #1415

This implements the data-contract portion of the [merged specification](NORMATIVE_SPEC_1414.md).
The canonical editable schema is [packet.schema.json](../../../../adl-spec/schemas/ghb/v1/packet.schema.json)
with [format and semantic boundaries](../../../../adl-spec/schemas/ghb/v1/README.md).
The synthetic task and improvement examples are proposed, unexecuted replay
records. They do not demonstrate resident improvement or authentic authority.

## Canonical format and compatibility

GHB v1 is a separate envelope and schema identity. The frozen v0.8 source,
manifest, artifact lock, schemas and examples are unchanged. Unknown v0.8
properties remain errors; v1 is not added to those closed historical records.
Historical `adopt` is a recommendation only and numeric delta is not a posterior.
An adapter must retain source version/digest, mapping and missing/unknown fields;
no automatic historical-to-v1 adapter or forged admission/activation is supplied.
A reader unable to support v1 must refuse it and retain the original artifact.

The public validator uses sorted ASCII object keys, compact UTF-8 encoding,
integer-only numbers and rejects duplicate keys before interpreting content.
Artifact references hash the entire artifact record, without a self-hash field.
SHA-256 identifies content; it does not authenticate owner authority or establish
that an external declaration is current. The caller separately supplies exact
current resident/body/incarnation/objective/policy/memory/continuity/archive
context. Replaying an internally consistent stale packet must still fail that
comparison. The examples use synthetic external absence declarations, not
receipts. Typed nullable evidence fields intentionally keep proposed protocols
non-executable when evaluator/corpus/split/rubric/threshold data is missing.

## Reused public-contract transport

The existing `adl-public-contracts` tool exposes `ghb-v1` alongside unchanged
`godel` v0.8 transport. There is no new engine, crate, authority or editable mirror.
`export` reads only committed canonical public inputs and refuses dirty source
bytes. Its deterministic archive includes one manifest, LICENSE, schema JSON
and example JSON. The manifest records the exact producer Git revision and
payload byte counts/SHA-256. The producer revision and manifest digest need an
independent trusted dependency decision; self-descriptive metadata is not one.
Verification uses the externally supplied manifest digest, rejects unexpected,
missing, tampered and aliased inputs, and extracts only into a new destination.

```sh
cargo run --locked --manifest-path tools/public_contracts/Cargo.toml --bin adl-public-contracts -- ghb-v1 export --repository "$SOURCE" --output "$NEW_ARCHIVE"
cargo run --locked --manifest-path tools/public_contracts/Cargo.toml --bin adl-public-contracts -- ghb-v1 verify --artifact "$NEW_ARCHIVE" --manifest-sha256 "$TRUSTED_MANIFEST_SHA256" --extract "$NEW_DATA_ROOT"
cargo run --locked --manifest-path tools/public_contracts/Cargo.toml --bin adl-public-contracts -- ghb-v1 validate --root "$NEW_DATA_ROOT" --manifest-sha256 "$TRUSTED_MANIFEST_SHA256" --packet "$PACKET" --context "$TRUSTED_CURRENT_CONTEXT"
```

The expected context JSON object has exactly `resident_identity_ref`, `body_ref`,
`runtime_incarnation`, `objective_ref`, `policy_ref`, `memory_head_ref`,
`continuity_head_ref` and `archive_head_ref`, copied from a separately trusted
owner observation. Copying them out of the packet is suitable only for synthetic
fixture tests, not proof of freshness or authentic operational admission.
Machine results are JSON on stdout; diagnostic failure remains stderr. No
credential, provider, service or external schema resolution is needed.

## Installed Runtime acceptance

The existing Runtime v0.8 installed reader does not support GHB v1. Runtime
[#251](https://github.com/agent-logic/agent-logic-runtime/issues/251) owns the
bounded versioned reader and installed CLI validation path, preserving the
historical reader. This public source alone cannot claim that an installed
Runtime package consumed it. That acceptance needs the exact reviewed producer
revision/manifest, actual Runtime package provenance/archive identity, isolated
installation and actual installed command outputs from an empty working
directory with only an explicit installed contract root. It must exercise valid
packets and malformed/version/stale/missing/tampered/alias refusals without an
ADL sibling or network fallback. A compiled fixture callback or schema parse is
not that proof. No shared package switch or live service restart is required.

Sprint 1 #244 remains pending until all four source children and actual installed
consumption evidence are delivered. No Gate A/B, empirical improvement, Bayesian
calibration, provider/model change or later-sprint completion follows from this
contract slice.

## Focused proof

`cargo test --locked --manifest-path tools/public_contracts/Cargo.toml --test ghb_contract --test distribution`
checks new schema/canonical/reference/refusal behavior and preserves frozen
transport proof. See [PVF inventory](VALIDATION_1415.json). Required CI integrates
the public tool change separately; actual installed Runtime proof is a distinct
owner gate. Build cache reuse is acceleration only, not proof.
