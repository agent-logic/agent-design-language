# CSM Snapshot Segment Format 4933

Issue: `#4933`
Sprint: `WP-07`
Runtime owner: `csm`

## Decision

CSM continuity capsules keep `csm.continuity-capsule.v1` as the reviewable JSON manifest and add a hash-addressed binary snapshot segment for `continuity_checkpoint.json`.

The selected v0.91.7 substrate is a small deterministic binary envelope implemented in `adl/src/csm_continuity_capsule.rs`:

- magic: `ADLCSMSEG`
- major/minor format version: `1.0`
- segment kind: `1` for snapshot
- metadata: JSON-encoded `adl.csm.snapshot_segment.v1`
- payload: exact retained artifact bytes
- digest: SHA-256 over payload bytes in the segment header
- manifest linkage: JSON manifest records segment ref, segment SHA-256, payload SHA-256, schema, format version, and `sha256:<digest>` hash address

## Protobuf Evaluation

Protobuf remains a viable future substrate for larger agent snapshot and diff chains, but it is not adopted in this v0.91.7 issue.

Reasons:

- The current repository has no protobuf/prost dependency or build-codegen ownership in the CSM runtime path.
- Adding codegen now would widen WP-07 into toolchain, generated-source, and remote-builder policy work.
- The required acceptance surface can be proven with a deterministic binary envelope plus stable JSON metadata.
- The JSON continuity capsule manifest must remain human-reviewable and portable for the current restore/fire-up proof.

Follow-on readiness criteria for protobuf adoption:

- committed `.proto` package and stable type names
- explicit reserved-field and unknown-field policy
- generated-code ownership and remote-builder parity
- compatibility tests for old reader/new writer and new reader/old writer
- manifest linkage that remains reviewable without parsing protobuf payloads

## Compatibility Guarantees

- Current readers accept major version `1`.
- Unknown major versions fail closed.
- Same-major future minor metadata is allowed only when the header and payload hash remain valid.
- Segment refs are bundle-relative and may not escape the capsule.
- Payload bytes are preserved exactly and hash-verified before stage or restore proceeds.
- The binary segment does not replace existing JSON artifacts in `csm.continuity-capsule.v1`.

## Redaction And Portability

Segment payloads are scanned before write and during stage/restore validation.

Rejected conditions include:

- host-private absolute path markers
- JSON keys containing credential-like names such as secret, token, password, api_key, or credential
- manifest segment refs that are absolute or escape the bundle
- segment hash mismatch, payload hash mismatch, malformed header, and truncated bytes

## Integration Surface

`capture` writes:

- `segments/continuity_checkpoint.snapshot.segment`
- `binary_segments[]` entry in `continuity_capsule_manifest.json`

`stage` and `restore` validate:

- segment file exists
- segment SHA-256 matches the manifest
- decoded metadata matches manifest schema, format, and source ref
- payload SHA-256 matches the manifest
- payload passes redaction and portability scan

## Non-Claims

- This does not claim production multi-region disaster recovery.
- This does not migrate existing `csm.continuity-capsule.v1` JSON packets to a binary-only format.
- This does not adopt protobuf for v0.91.7.
- This does not store provider credentials or cloud account secrets in capsule segments.

## Local Proof Plan

Focused proof:

```sh
cargo test --manifest-path adl/Cargo.toml csm_snapshot_segment -- --nocapture
cargo test --manifest-path adl/Cargo.toml --test cli_smoke csm_continuity_capsule_captures_stages_and_rejects_unsafe_bundles -- --nocapture
```

WP-07 remote-build reporting for this issue records wuji/local, Nessus, AWS Spot, and CodeBuild timings separately in the issue SOR.

## Validation Results

Local focused proof:

| Command | Result | Timing |
| --- | --- | --- |
| `cargo test --manifest-path adl/Cargo.toml csm_snapshot_segment -- --nocapture` | PASS, 5 segment tests passed | 4m 06s Cargo build/test wall |
| `cargo test --manifest-path adl/Cargo.toml --test cli_smoke csm_continuity_capsule_captures_stages_and_rejects_unsafe_bundles -- --nocapture` | PASS, 1 integrated CSM continuity smoke passed | 0.64s command wall, 0.28s test body |
| `cargo fmt --manifest-path adl/Cargo.toml --all -- --check` | PASS | 11.1s |
| `bash adl/tools/validate_v0917_csm_continuity_capsule_4910_status.sh` | PASS | sub-second |
| `git diff --check` | PASS | sub-second |

Pre-PR review fix proof:

| Command | Result | Timing |
| --- | --- | --- |
| `cargo test --manifest-path adl/Cargo.toml csm_snapshot_segment -- --nocapture` | PASS, 5 segment tests passed after review fixes | 1m 11s |
| `cargo test --manifest-path adl/Cargo.toml --test cli_smoke csm_continuity_capsule_captures_stages_and_rejects_unsafe_bundles -- --nocapture` | PASS, 1 integrated CSM continuity smoke passed with missing-segment and divergent-payload negative cases | 30.45s compile, 0.42s test body |
| `cargo fmt --manifest-path adl/Cargo.toml --all -- --check` | PASS | 10.9s |
| `git diff --check` | PASS | sub-second |

Four-platform benchmark:

| Platform | Build seconds | Test seconds | Total benchmark seconds | Wrapper / platform wall | Status | Notes |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| wuji/local | 99 | 0 | 99 | 99s | PASS | linked target cache, test real time 0.32s |
| Nessus | 85 | 47 | 132 | 138s | PASS | builder image already present, no pull attempted |
| AWS Spot | 66 | 55 | 121 | 240s | PASS | warm EBS cache; wrapper wall includes launch, SSM, and teardown |
| AWS CodeBuild | 98 | 76 | 174 | about 203s | PASS | CodeBuild build succeeded; wrapper wall from AWS start/end timestamps |

Platform problem log:

- AWS Spot reported `role_deleted=false` for the temporary validation role after the successful run. The inline role policy was deleted manually, the role was deleted manually, and `get-role` then returned `NoSuchEntity`.
- Pre-PR review found two validation gaps: missing `binary_segments[]` was accepted, and valid segment payloads were not cross-checked against the retained artifact bytes. Both were fixed by requiring the checkpoint segment and comparing payload SHA-256 against the manifest artifact hash and source file hash.

Speed observation:

- The segment tests themselves are tiny after the Rust crate is built. The dominant cost is repeated broad Rust dependency and owner-binary compile surface across local and remote lanes.
- C-SDLC validation optimization should preserve lifecycle truth while selecting narrower validation profiles when changed-path risk permits it, then run remote platform timing in parallel after one local proving pass.
