# Runtime v3 local TLS validation

Product correction: `ee49528e2`

## Focused Rust proof

- Atomic multi-file rollback unit regression: 1 passed.
- `adl-runtime/tests/local_tls.rs`: 10 passed.
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --all-targets -- -D warnings`: passed.
- `cargo fmt --manifest-path adl-runtime/Cargo.toml -- --check`: passed.
- `git diff --check`: passed.

All Cargo output used `/Volumes/FastWork/adl-wp-5713/target`.

## Native macOS proof

The repo-native `adl-runtime-local-tls-bootstrap` binary generated material under the issue worktree. A second invocation returned `local_certificate_reused` with the same SHA-256 identity. The public certificate and chain copies had identical SHA-256 digests; the private key mode was `0600`.

The platform-native command below completed successfully without changing the trust store:

```text
security verify-cert -N -L -p ssl -n localhost -c <public-certificate> -r <public-certificate> -t -v
```

The native inspection confirmed:

- ECDSA P-256 with SHA-256;
- server-auth extended key usage;
- DNS SAN `localhost`;
- IP SAN `127.0.0.1`;
- a 397-day browser-compatible validity window;
- successful SSL certificate verification when the generated self-signed certificate is supplied as the explicit trust root.

No certificate bytes, private-key bytes, or credential values are retained in this evidence.

## Remaining platform proof

Native Linux and native Windows inspection use the same Rust implementation and configuration schema. Their results are not claimed in this local macOS artifact and remain required before final publication.
