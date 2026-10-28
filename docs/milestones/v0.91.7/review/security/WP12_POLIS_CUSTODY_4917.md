# WP-12 Polis Custody Proof (#4917)

## Metadata

- Issue: `#4917`
- Parent sprint: `#4639`
- Milestone: `v0.91.7`
- Status: worktree proof recorded; PR integration pending
- Machine-readable companion: `docs/milestones/v0.91.7/review/security/wp12_polis_custody_4917.json`

## Purpose

Record the bounded WP-12 tamper-evident custody implementation for retained
Polis continuity artifacts without expanding into durable storage, key rotation,
or break-glass policy.

This packet covers the custody manifest emitted by `csm continuity capture`,
the fail-closed validator used by capture, stage, restore, and fire-drill flows,
and the focused proof commands for positive and tamper-negative behavior.

## Custody Contract

The continuity capsule now retains `custody_manifest.json` next to
`continuity_capsule_manifest.json`. The custody manifest records:

- artifact id, kind, schema version, bundle-relative storage location, bytes,
  producer, timestamp, and SHA-256 digest;
- parent linkage from retained artifacts and binary snapshot segments back to
  the capsule manifest root artifact;
- a replay guard derived from agent instance, creation time, target host,
  manifest ref, artifact hashes, and segment hashes;
- redaction posture requiring no sensitive material, no host-private paths, and
  no redacted required fields;
- a RustCrypto P-256/ECDSA signature over canonical sorted JSON with the
  `signature` field excluded from the signed payload.

The signing key is supplied through
`ADL_CSM_CUSTODY_P256_SIGNING_PRIVATE_KEY_B64`; validation requires the
externally supplied trusted public key
`ADL_CSM_CUSTODY_TRUSTED_P256_PUBLIC_KEY_B64` and refuses to trust the public
key embedded in the custody manifest by itself. The retained artifact contains
only the public key, key id, signature bytes, and signed payload digest.

The local proof script includes a deterministic non-secret P-256 test-vector
private scalar so the proof can be reproduced without production key material.

## Readiness Result

| Requirement | Result | Evidence |
| --- | --- | --- |
| Integrity metadata | Passed | `custody_manifest.json` records SHA-256 and parent linkage for capsule manifest, retained state/log artifacts, and binary snapshot segments. |
| Cryptographic signing | Passed | RustCrypto `p256`/ECDSA signs canonical custody JSON; stage/restore/fire-drill verify signature, trusted public key, and payload digest before trust. |
| Tamper detection | Passed | Focused smoke test rejects modified artifact bytes, missing custody manifest, wrong parent, replay guard drift, redaction drift, and signature tampering. |
| Path and secret hygiene | Passed | Custody metadata uses bundle-relative refs and marks retained secret material as forbidden. |
| Evidence retention | Passed | The proof script validates custody metadata and negative cases in the retained continuity capsule proof packet. |

## Validation

Focused local validation run for this issue:

```sh
cargo fmt --manifest-path adl/Cargo.toml --all -- --check
git diff --check
cargo test --manifest-path adl/Cargo.toml --test cli_smoke csm_continuity_capsule -- --nocapture
mkdir -p adl/target/work && tmpdir=$(mktemp -d adl/target/work/adl-4917-custody-proof.XXXXXX) && bash adl/tools/run_v0917_csm_continuity_capsule_4910_proof.sh "$tmpdir"
python3 -m json.tool docs/milestones/v0.91.7/review/security/wp12_polis_custody_4917.json >/dev/null
```

The temporary proof script produced
`validate_v0917_csm_continuity_capsule_4910_status: PASS`; the retained
validator recomputes the canonical signed payload digest, checks the trusted
public key, and verifies the P-256/ECDSA signature bytes.

## Gate Disposition

This packet supplies the #4917 worktree proof for the
`tamper_evident_evidence_custody` row in
`docs/milestones/v0.91.7/review/security/wp12_security_cav_gate_4656.json`.
The row should remain child-open until this issue's PR is merged and closeout
truth records integration.

## Non-Claims

- This packet does not claim durable storage replication or retention policy.
- This packet does not claim key rotation, revocation, break-glass, or external
  timestamp authority; those remain owned by `#4920`.
- This packet does not retain production private signing keys or provider
  credentials; the shell proof retains only a deterministic non-secret test
  vector.
- This packet does not claim full CAV red-blue readiness; `#4914` remains the
  owner for adversarial CAV proof.
